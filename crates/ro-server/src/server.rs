//! The running server: login/char/map listeners (+ the admin console) on a
//! small tokio runtime, the game (Lua is single-threaded) on its own thread.
//!
//! [`start`] returns once every port is bound and the game has loaded, so
//! callers (the CLI, the Windows launcher) can report "port in use" or a
//! broken client folder before anything else happens.

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant, SystemTime};

use anyhow::{Context, Result, anyhow};
use ro_game::{Game, Resources, ServerKind};
use tracing::{error, info};

use crate::config::Config;
use crate::net::{self, Event, NetOutbox};

pub struct Options {
    /// Stop (and save) on Ctrl-C. The CLI wants this; a GUI has no console.
    pub ctrl_c: bool,
    /// Where logs/packets.log goes when `packet_log` is on.
    pub log_dir: PathBuf,
}

/// A started server. Dropping it does not stop it; call [`Handle::shutdown`].
pub struct Handle {
    tx: mpsc::Sender<Event>,
    thread: Option<std::thread::JoinHandle<()>>,
    /// The patch server is enabled and both of its ports are bound.
    pub patch_server: bool,
}

impl Handle {
    /// Save everything, stop the servers and wait for the game thread.
    pub fn shutdown(mut self) {
        let (rtx, rrx) = mpsc::channel();
        if self.tx.send(Event::Console { line: ":shutdown".into(), reply: rtx }).is_ok() {
            let _ = rrx.recv_timeout(Duration::from_secs(15));
        }
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }

    /// Block until the server stops by itself (Ctrl-C, console `:shutdown`).
    pub fn wait(mut self) {
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

pub fn load_resources(cfg: &Config) -> Result<Resources> {
    let vfs = crate::setup::client_vfs(&cfg.client_dir)
        .with_context(|| format!("client_dir = {} (see ragnarok-offline.toml)", cfg.client_dir.display()))?;
    let res = Resources::new(vfs);
    info!("client data: {} archives, {} maps", res.vfs.layers().len(), res.maps.len());
    Ok(res)
}

fn bind(ip: std::net::IpAddr, port: u16, what: &str) -> Result<std::net::TcpListener> {
    let l = std::net::TcpListener::bind(SocketAddr::new(ip, port)).with_context(|| {
        format!("{what} port {ip}:{port} is in use or not allowed - is another ragnarok-offline already running?")
    })?;
    l.set_nonblocking(true)?;
    Ok(l)
}

pub fn start(cfg: Config, opts: Options) -> Result<Handle> {
    let ip: std::net::IpAddr = cfg.bind.parse().context("bind address")?;
    let mut listeners = Vec::new();
    for (port, kind) in [(cfg.login_port, ServerKind::Login), (cfg.game.char_port, ServerKind::Char), (cfg.game.map_port, ServerKind::Map)] {
        listeners.push((bind(ip, port, &format!("{kind:?}"))?, kind));
    }
    let console = if cfg.console_port != 0 {
        Some(bind([127, 0, 0, 1].into(), cfg.console_port, "console")?)
    } else {
        None
    };
    // optional, and not worth failing over: without it the client is started directly
    let patch = if cfg.patch_server.enabled {
        let ps = &cfg.patch_server;
        match (bind([127, 0, 0, 1].into(), ps.http_port, "patch server HTTP"), bind([127, 0, 0, 1].into(), ps.ftp_port, "patch server FTP")) {
            (Ok(h), Ok(f)) => Some((h, f, ps.dir.clone())),
            (h, f) => {
                for e in [h.err(), f.err()].into_iter().flatten() {
                    tracing::warn!("{e:#}");
                }
                None
            }
        }
    } else {
        None
    };
    let patch_server = patch.is_some();
    let bound = Bound { listeners, console, patch };

    let (tx, rx) = mpsc::channel::<Event>();
    let (ready_tx, ready_rx) = mpsc::channel::<Result<()>>();
    let tx2 = tx.clone();
    let thread = std::thread::Builder::new()
        .name("game".into())
        .spawn(move || game_thread(cfg, opts, bound, tx2, rx, ready_tx))?;
    match ready_rx.recv() {
        Ok(Ok(())) => Ok(Handle { tx, thread: Some(thread), patch_server }),
        Ok(Err(e)) => {
            let _ = thread.join();
            Err(e)
        }
        Err(_) => {
            let _ = thread.join();
            Err(anyhow!("server thread died while starting"))
        }
    }
}

/// Sockets bound by [`start`] before the game thread exists.
struct Bound {
    listeners: Vec<(std::net::TcpListener, ServerKind)>,
    console: Option<std::net::TcpListener>,
    /// patch server: HTTP, FTP, served directory
    patch: Option<(std::net::TcpListener, std::net::TcpListener, PathBuf)>,
}

fn game_thread(
    cfg: Config,
    opts: Options,
    Bound { listeners, console, patch }: Bound,
    tx: mpsc::Sender<Event>,
    rx: mpsc::Receiver<Event>,
    ready: mpsc::Sender<Result<()>>,
) {
    let fail = |e: anyhow::Error| {
        let _ = ready.send(Err(e));
    };
    let res = match load_resources(&cfg) {
        Ok(r) => r,
        Err(e) => return fail(e),
    };
    let log = if cfg.packet_log {
        match net::PacketLog::open(&opts.log_dir.join("packets.log")) {
            Ok(l) => Some(Arc::new(l)),
            Err(e) => return fail(e),
        }
    } else {
        None
    };
    let out = NetOutbox::new(log);
    let rt = match tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build() {
        Ok(rt) => rt,
        Err(e) => return fail(e.into()),
    };
    for (l, kind) in listeners {
        let (tx, out) = (tx.clone(), out.clone());
        rt.spawn(async move {
            if let Err(e) = net::listen(l, kind, tx, out).await {
                error!("{kind:?} server: {e:#}");
            }
        });
    }
    if let Some(l) = console {
        let tx = tx.clone();
        rt.spawn(async move {
            if let Err(e) = net::console(l, tx).await {
                error!("console: {e:#}");
            }
        });
    }
    if let Some((h, f, dir)) = patch {
        let d2 = dir.clone();
        rt.spawn(async move {
            if let Err(e) = crate::patchsrv::http(h, d2).await {
                error!("patch server HTTP: {e:#}");
            }
        });
        rt.spawn(async move {
            if let Err(e) = crate::patchsrv::ftp(f, dir).await {
                error!("patch server FTP: {e:#}");
            }
        });
    }
    if cfg.watch_scripts {
        let tx = tx.clone();
        let dir = cfg.game.script_dir.clone();
        std::thread::spawn(move || watch(&dir, tx));
    }
    if opts.ctrl_c {
        let tx = tx.clone();
        rt.spawn(async move {
            let _ = tokio::signal::ctrl_c().await;
            let (rtx, _rrx) = mpsc::channel();
            let _ = tx.send(Event::Console { line: ":shutdown".into(), reply: rtx });
        });
    }

    let mut game = match Game::new(cfg.game.clone(), res, Box::new(out)) {
        Ok(g) => g,
        Err(e) => return fail(e),
    };
    info!("ready: point the client at {}:{} (ragnarok-offline setup)", cfg.bind, cfg.login_port);
    let _ = ready.send(Ok(()));
    let tick = Duration::from_millis(20);
    let mut last = Instant::now();
    loop {
        match rx.recv_timeout(tick) {
            Ok(Event::Net(ev)) => game.handle(ev),
            Ok(Event::ReloadScripts) => match game.reload_scripts() {
                Ok(s) => info!("auto-reload: {s}"),
                Err(e) => error!("auto-reload: {e:#}"),
            },
            Ok(Event::Console { line, reply }) => {
                if line == ":shutdown" {
                    info!("saving and shutting down");
                    game.save_all();
                    let _ = reply.send("bye".into());
                    break;
                }
                let r = console_command(&mut game, &line);
                let _ = reply.send(r);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
        if last.elapsed() >= tick {
            last = Instant::now();
            game.tick();
        }
    }
    rt.shutdown_timeout(Duration::from_millis(200));
}

fn console_command(game: &mut Game, line: &str) -> String {
    match line.trim() {
        ":help" => ":players  :reload  :save  :notices  :shutdown  - anything else is Lua, run as the first online player (player.*, warp(), give_item(), spawn_mob(), ...)".into(),
        ":players" => {
            let v: Vec<String> = game
                .world
                .units
                .values()
                .filter(|u| u.is_player())
                .map(|u| format!("{} id={} {} {},{}", u.name, u.id, u.map, u.x, u.y))
                .collect();
            if v.is_empty() { "(nobody online)".into() } else { v.join("\n") }
        }
        ":reload" => match game.reload_scripts() {
            Ok(s) => s,
            Err(e) => format!("error: {e:#}"),
        },
        ":save" => {
            game.save_all();
            "saved".into()
        }
        ":notices" => {
            let n = game.notices.join("\n");
            game.notices.clear();
            if n.is_empty() { "(none)".into() } else { n }
        }
        code => {
            let p = game.first_player();
            game.console_eval(code, p)
        }
    }
}

/// Poll the script tree's newest mtime; reload when it changes.
fn watch(dir: &Path, tx: mpsc::Sender<Event>) {
    fn newest(dir: &Path) -> SystemTime {
        let mut best = SystemTime::UNIX_EPOCH;
        let mut stack = vec![dir.to_path_buf()];
        while let Some(d) = stack.pop() {
            let Ok(rd) = std::fs::read_dir(&d) else { continue };
            for e in rd.flatten() {
                let p = e.path();
                if p.is_dir() {
                    stack.push(p);
                } else if p.extension().is_some_and(|x| x == "lua")
                    && let Ok(m) = e.metadata().and_then(|m| m.modified())
                {
                    best = best.max(m);
                }
            }
        }
        best
    }
    let mut last = newest(dir);
    loop {
        std::thread::sleep(Duration::from_millis(700));
        let now = newest(dir);
        if now != last {
            last = now;
            std::thread::sleep(Duration::from_millis(300)); // let editors finish writing
            if tx.send(Event::ReloadScripts).is_err() {
                return;
            }
        }
    }
}
