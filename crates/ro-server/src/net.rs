//! TCP side: three listeners (login/char/map) feeding one game thread.

use std::collections::HashMap;
use std::io::Write as _;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, mpsc};

use anyhow::Result;
use ro_game::{NetEvent, Outbox, ServerKind};
use ro_proto::Framer;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc as tmpsc;
use tracing::{info, warn};

pub enum Event {
    Net(NetEvent),
    Console { line: String, reply: mpsc::Sender<String> },
    ReloadScripts,
}

enum Out {
    Data(Vec<u8>),
    Close,
}

pub struct PacketLog(Mutex<std::fs::File>);

impl PacketLog {
    pub fn open(path: &std::path::Path) -> Result<Self> {
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p)?;
        }
        Ok(PacketLog(Mutex::new(std::fs::OpenOptions::new().create(true).append(true).open(path)?)))
    }
    fn log(&self, sid: u32, dir: &str, data: &[u8]) {
        let mut f = self.0.lock().unwrap();
        let id = if data.len() >= 2 { format!("{:04x}", u16::from_le_bytes([data[0], data[1]])) } else { "----".into() };
        let hex: String = data.iter().map(|b| format!("{b:02x}")).collect();
        let _ = writeln!(f, "{sid} {dir} {id} {} {hex}", data.len());
    }
}

/// Outbox shared between the game thread and connection tasks.
#[derive(Clone, Default)]
pub struct NetOutbox {
    conns: Arc<Mutex<HashMap<u32, tmpsc::UnboundedSender<Out>>>>,
    log: Option<Arc<PacketLog>>,
}

impl NetOutbox {
    pub fn new(log: Option<Arc<PacketLog>>) -> Self {
        NetOutbox { conns: Default::default(), log }
    }
}

impl Outbox for NetOutbox {
    fn send(&self, sid: u32, data: Vec<u8>) {
        if let Some(l) = &self.log {
            l.log(sid, ">", &data);
        }
        if let Some(tx) = self.conns.lock().unwrap().get(&sid) {
            let _ = tx.send(Out::Data(data));
        }
    }
    fn close(&self, sid: u32) {
        if let Some(tx) = self.conns.lock().unwrap().get(&sid) {
            let _ = tx.send(Out::Close);
        }
    }
}

static NEXT_SID: AtomicU32 = AtomicU32::new(1);

/// Serve one of the three game ports on an already-bound (non-blocking) socket.
pub async fn listen(listener: std::net::TcpListener, kind: ServerKind, events: mpsc::Sender<Event>, out: NetOutbox) -> Result<()> {
    let listener = TcpListener::from_std(listener)?;
    info!("{kind:?} server listening on {}", listener.local_addr()?);
    loop {
        let (sock, peer) = listener.accept().await?;
        let _ = sock.set_nodelay(true);
        let sid = NEXT_SID.fetch_add(1, Ordering::Relaxed);
        let (tx, mut rx) = tmpsc::unbounded_channel::<Out>();
        out.conns.lock().unwrap().insert(sid, tx);
        let events = events.clone();
        let out2 = out.clone();
        tracing::debug!("{kind:?} connection {sid} from {peer}");
        let _ = events.send(Event::Net(NetEvent::Connected { sid, kind }));
        tokio::spawn(async move {
            let (mut rd, mut wr) = sock.into_split();
            let writer = tokio::spawn(async move {
                while let Some(m) = rx.recv().await {
                    match m {
                        Out::Data(d) => {
                            if wr.write_all(&d).await.is_err() {
                                break;
                            }
                        }
                        Out::Close => break,
                    }
                }
                let _ = wr.shutdown().await;
            });
            let mut framer = Framer::new();
            let mut buf = vec![0u8; 8192];
            'read: loop {
                let n = match rd.read(&mut buf).await {
                    Ok(0) | Err(_) => break,
                    Ok(n) => n,
                };
                framer.push(&buf[..n]);
                loop {
                    match framer.next_packet() {
                        Ok(Some(pkt)) => {
                            if let Some(l) = &out2.log {
                                l.log(sid, "<", &pkt);
                            }
                            if events.send(Event::Net(NetEvent::Packet { sid, data: pkt })).is_err() {
                                break 'read;
                            }
                        }
                        Ok(None) => break,
                        Err(e) => {
                            warn!("{kind:?} {sid}: {e}; dropping connection");
                            break 'read;
                        }
                    }
                }
            }
            out2.conns.lock().unwrap().remove(&sid);
            writer.abort();
            let _ = events.send(Event::Net(NetEvent::Disconnected { sid }));
        });
    }
}

/// Line-based admin console on 127.0.0.1: each line is Lua (or a `:command`).
pub async fn console(listener: std::net::TcpListener, events: mpsc::Sender<Event>) -> Result<()> {
    let listener = TcpListener::from_std(listener)?;
    info!("admin console on {} (ragnarok-offline console)", listener.local_addr()?);
    loop {
        let (sock, _) = listener.accept().await?;
        let events = events.clone();
        tokio::spawn(async move {
            let (rd, mut wr) = sock.into_split();
            let mut lines = tokio::io::BufReader::new(rd);
            let mut line = String::new();
            loop {
                line.clear();
                match tokio::io::AsyncBufReadExt::read_line(&mut lines, &mut line).await {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {}
                }
                let (tx, rx) = mpsc::channel();
                if events.send(Event::Console { line: line.trim_end().to_string(), reply: tx }).is_err() {
                    break;
                }
                let reply = tokio::task::spawn_blocking(move || rx.recv().unwrap_or_default()).await.unwrap_or_default();
                // protocol: reply lines, then a lone "." line
                let body = reply.replace("\n.\n", "\n..\n");
                if wr.write_all(format!("{body}\n.\n").as_bytes()).await.is_err() {
                    break;
                }
            }
        });
    }
}
