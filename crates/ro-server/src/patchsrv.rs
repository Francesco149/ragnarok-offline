//! A stand-in for euRO's patch servers (`wpatch.euro-ro.net` over HTTP,
//! `fpatch.euro-ro.net` over FTP), so the CD's 2003 patcher (`Ragnarok.exe`)
//! can run its normal course offline: fetch the notice and the patch list,
//! find nothing it lacks, offer "Start Game".
//!
//! What the patcher does (observed under Wine against logging servers, and in
//! its code; docs/WINDOWS.md "The patcher"):
//!
//! - HTTP (WinINet `InternetOpenUrlA "http://%s%s/%s"`, User-Agent "Patch
//!   Client"): `GET /patch/patch_allow.txt` ("allow", "deny" or "xmas"), then
//!   `notice_patch.txt` (or `notice_close.txt` when denied), then
//!   `patch2.txt`. Each must come with a Content-Length or it is treated as a
//!   failure; each is saved into the game folder.
//! - FTP (`InternetConnectA` port 21, anonymous, passive): `CWD /patch`,
//!   `TYPE A`, `PASV`, `LIST` - even when there is nothing to download - then
//!   the patch files it needs.
//!
//! Both protocols serve one directory: HTTP `/patch/x` and FTP `/patch/x` are
//! `root/patch/x`. HTTP also serves `/` (`root/index.html`): the client's
//! `registrationweb` (euRO's website) points here. Only GET/HEAD and a
//! read-only FTP subset are implemented; everything is bound to 127.0.0.1.

use std::path::{Component, Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info, warn};

/// Map a request path to a file under `root`, refusing anything that would
/// leave it.
fn resolve(root: &Path, virt: &str) -> Option<PathBuf> {
    let mut p = root.to_path_buf();
    for c in Path::new(virt.trim_start_matches(['/', '\\'])).components() {
        match c {
            Component::Normal(s) => p.push(s),
            Component::CurDir => {}
            _ => return None,
        }
    }
    Some(p)
}

/// The patch list to serve this CD's patcher: euRO's archived `patch2.txt`
/// with every numbered line above `level` (the CD's `patch.inf`) commented
/// out with the list's own `//`, which the patcher skips.
///
/// Their files were not archived; and the patcher raises `patch.inf` to the
/// highest number it sees, counting bare numbers as applied, and rewrites
/// data.grf's file table whenever it does. Keeping the list at the CD's level
/// means the patcher finds nothing to do and writes nothing to the game
/// folder (no 490 MB VirtualStore copy under Program Files, no clash with
/// the server reading data.grf). `header` (one `//` line) says so to anyone
/// reading the copy the patcher saves.
pub fn patch_list_for_level(archived: &str, level: u32, header: &str) -> String {
    let mut out = vec![header.to_string()];
    for l in archived.split('\n').map(|l| l.trim_end_matches('\r')) {
        let n: String = l.chars().take_while(|c| c.is_ascii_digit()).collect();
        if n.parse::<u32>().is_ok_and(|n| n > level) {
            out.push(format!("//{l}"));
        } else {
            out.push(l.to_string());
        }
    }
    out.join("\r\n")
}

// ---- HTTP -------------------------------------------------------------------

pub async fn http(listener: std::net::TcpListener, root: PathBuf) -> Result<()> {
    let listener = TcpListener::from_std(listener)?;
    info!("patch server (HTTP) on {} serving {}", listener.local_addr()?, root.display());
    loop {
        let (sock, _) = listener.accept().await?;
        let root = root.clone();
        tokio::spawn(async move {
            if let Err(e) = http_conn(sock, &root).await {
                debug!("patch http: {e:#}");
            }
        });
    }
}

async fn http_conn(mut sock: TcpStream, root: &Path) -> Result<()> {
    let mut head = Vec::new();
    let mut buf = [0u8; 1024];
    while !head.windows(4).any(|w| w == b"\r\n\r\n") {
        let n = tokio::time::timeout(Duration::from_secs(10), sock.read(&mut buf)).await??;
        if n == 0 || head.len() > 16 * 1024 {
            return Ok(());
        }
        head.extend_from_slice(&buf[..n]);
    }
    let head = String::from_utf8_lossy(&head);
    let mut first = head.lines().next().unwrap_or("").split_whitespace();
    let (method, target) = (first.next().unwrap_or(""), first.next().unwrap_or("/"));
    let path = target.split(['?', '#']).next().unwrap_or("/");
    let file = resolve(root, path).map(|p| if p.is_dir() { p.join("index.html") } else { p }).filter(|p| p.is_file());
    let mime = match file.as_ref().and_then(|f| f.extension()).and_then(|e| e.to_str()).map(|e| e.to_ascii_lowercase()).as_deref() {
        Some("html" | "htm") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("gif") => "image/gif",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("png") => "image/png",
        _ => "text/plain",
    };
    let (status, body) = match (method, &file) {
        ("GET" | "HEAD", Some(f)) => ("200 OK", std::fs::read(f)?), // small text files
        ("GET" | "HEAD", None) => ("404 Not Found", b"not found\r\n".to_vec()),
        _ => ("405 Method Not Allowed", b"method not allowed\r\n".to_vec()),
    };
    info!("patch http: {method} {path} -> {status}");
    let resp = format!(
        "HTTP/1.1 {status}\r\nServer: ragnarok-offline\r\nContent-Type: {mime}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    sock.write_all(resp.as_bytes()).await?;
    if method != "HEAD" {
        sock.write_all(&body).await?;
    }
    sock.shutdown().await?;
    Ok(())
}

// ---- FTP --------------------------------------------------------------------

pub async fn ftp(listener: std::net::TcpListener, root: PathBuf) -> Result<()> {
    let listener = TcpListener::from_std(listener)?;
    info!("patch server (FTP) on {} serving {}", listener.local_addr()?, root.display());
    loop {
        let (sock, _) = listener.accept().await?;
        let root = root.clone();
        tokio::spawn(async move {
            if let Err(e) = ftp_conn(sock, &root).await {
                debug!("patch ftp: {e:#}");
            }
        });
    }
}

/// Normalize an FTP path against the current directory ("/" based).
fn ftp_path(cwd: &str, arg: &str) -> Option<String> {
    let joined = if arg.starts_with('/') { arg.to_string() } else { format!("{cwd}/{arg}") };
    let mut parts: Vec<&str> = Vec::new();
    for p in joined.split('/') {
        match p {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            p => parts.push(p),
        }
    }
    Some(format!("/{}", parts.join("/")))
}

async fn ftp_conn(sock: TcpStream, root: &Path) -> Result<()> {
    let (rd, mut wr) = sock.into_split();
    let mut lines = BufReader::new(rd).lines();
    let mut cwd = String::from("/");
    let mut pasv: Option<TcpListener> = None;
    let mut rest: u64 = 0;
    wr.write_all(b"220 ragnarok-offline patch server\r\n").await?;
    while let Some(line) = tokio::time::timeout(Duration::from_secs(300), lines.next_line()).await?? {
        let (cmd, arg) = match line.trim_end().split_once(' ') {
            Some((c, a)) => (c.to_ascii_uppercase(), a.trim().to_string()),
            None => (line.trim_end().to_ascii_uppercase(), String::new()),
        };
        debug!("patch ftp <- {cmd} {arg}");
        let target = || ftp_path(&cwd, &arg).and_then(|v| resolve(root, &v).map(|p| (v, p)));
        let reply: String = match cmd.as_str() {
            "USER" => "331 Any password will do\r\n".into(),
            "PASS" => "230 Logged in\r\n".into(),
            "SYST" => "215 UNIX Type: L8\r\n".into(),
            "FEAT" => "211-Features:\r\n SIZE\r\n MDTM\r\n REST STREAM\r\n211 End\r\n".into(),
            "PWD" | "XPWD" => format!("257 \"{cwd}\" is the current directory\r\n"),
            "CWD" | "XCWD" | "CDUP" => {
                let arg = if cmd == "CDUP" { ".." } else { arg.as_str() };
                match ftp_path(&cwd, arg).and_then(|v| resolve(root, &v).filter(|p| p.is_dir()).map(|_| v)) {
                    Some(v) => {
                        cwd = v;
                        format!("250 \"{cwd}\" is the current directory\r\n")
                    }
                    None => "550 No such directory\r\n".into(),
                }
            }
            "TYPE" | "MODE" | "STRU" | "OPTS" | "NOOP" => "200 OK\r\n".into(),
            "ALLO" => "202 Not needed\r\n".into(),
            "REST" => {
                rest = arg.parse().unwrap_or(0);
                format!("350 Restarting at {rest}\r\n")
            }
            "PASV" => {
                let l = TcpListener::bind(("127.0.0.1", 0)).await?;
                let port = l.local_addr()?.port();
                pasv = Some(l);
                format!("227 Entering Passive Mode (127,0,0,1,{},{})\r\n", port >> 8, port & 0xff)
            }
            "EPSV" => {
                let l = TcpListener::bind(("127.0.0.1", 0)).await?;
                let port = l.local_addr()?.port();
                pasv = Some(l);
                format!("229 Entering Extended Passive Mode (|||{port}|)\r\n")
            }
            "SIZE" => match target().filter(|(_, p)| p.is_file()) {
                Some((_, p)) => format!("213 {}\r\n", std::fs::metadata(&p)?.len()),
                None => "550 No such file\r\n".into(),
            },
            "MDTM" => match target().filter(|(_, p)| p.is_file()) {
                Some((_, p)) => format!("213 {}\r\n", mdtm(std::fs::metadata(&p)?.modified()?)),
                None => "550 No such file\r\n".into(),
            },
            "LIST" | "NLST" | "RETR" => {
                // LIST may carry ls flags ("-la"); they are not paths
                let arg_is_flags = arg.starts_with('-');
                let what = if arg.is_empty() || arg_is_flags { Some((cwd.clone(), resolve(root, &cwd))) } else { target().map(|(v, p)| (v, Some(p))) };
                let Some(l) = pasv.take() else {
                    wr.write_all(b"425 Use PASV first\r\n").await?;
                    continue;
                };
                let payload = match (cmd.as_str(), what) {
                    ("RETR", Some((_, Some(p)))) if p.is_file() => {
                        use std::io::{Read, Seek};
                        let mut f = std::fs::File::open(&p)?;
                        f.seek(std::io::SeekFrom::Start(rest))?;
                        let mut v = Vec::new();
                        f.read_to_end(&mut v)?;
                        Some(v)
                    }
                    ("LIST" | "NLST", Some((_, Some(p)))) if p.exists() => Some(listing(&p, cmd == "NLST")?.into_bytes()),
                    _ => None,
                };
                rest = 0;
                let Some(payload) = payload else {
                    wr.write_all(b"550 No such file or directory\r\n").await?;
                    continue;
                };
                info!("patch ftp: {cmd} {} ({} bytes)", if arg.is_empty() { &cwd } else { &arg }, payload.len());
                wr.write_all(b"150 Opening data connection\r\n").await?;
                let (mut data, _) = tokio::time::timeout(Duration::from_secs(20), l.accept()).await??;
                data.write_all(&payload).await?;
                data.shutdown().await?;
                "226 Transfer complete\r\n".into()
            }
            "QUIT" => {
                wr.write_all(b"221 Bye\r\n").await?;
                return Ok(());
            }
            _ => {
                warn!("patch ftp: unsupported command {cmd}");
                "502 Command not implemented\r\n".into()
            }
        };
        wr.write_all(reply.as_bytes()).await?;
    }
    Ok(())
}

/// Unix `ls -l` style lines, which is what WinINet's FtpFindFirstFile parses.
fn listing(p: &Path, names_only: bool) -> Result<String> {
    let mut entries: Vec<(String, std::fs::Metadata)> = if p.is_dir() {
        std::fs::read_dir(p)?.flatten().filter_map(|e| Some((e.file_name().into_string().ok()?, e.metadata().ok()?))).collect()
    } else {
        vec![(p.file_name().unwrap().to_string_lossy().into_owned(), std::fs::metadata(p)?)]
    };
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out = String::new();
    for (name, m) in entries {
        if names_only {
            out += &format!("{name}\r\n");
            continue;
        }
        let (y, mo, d) = civil(m.modified().unwrap_or(UNIX_EPOCH));
        const MON: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        let kind = if m.is_dir() { "drwxr-xr-x" } else { "-rw-r--r--" };
        out += &format!("{kind} 1 ro ro {:>12} {} {d:>2}  {y} {name}\r\n", m.len(), MON[mo as usize - 1]);
    }
    Ok(out)
}

fn civil(t: SystemTime) -> (i64, u32, u32) {
    // days since 1970-01-01 -> proleptic Gregorian date (Howard Hinnant's algorithm)
    let days = t.duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64 / 86400).unwrap_or(0);
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z.rem_euclid(146097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

fn mdtm(t: SystemTime) -> String {
    let secs = t.duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let (y, m, d) = civil(t);
    let s = secs % 86400;
    format!("{y:04}{m:02}{d:02}{:02}{:02}{:02}", s / 3600, s / 60 % 60, s % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_stay_inside_root() {
        let r = Path::new("/srv");
        assert_eq!(resolve(r, "/patch/patch2.txt"), Some(PathBuf::from("/srv/patch/patch2.txt")));
        assert_eq!(resolve(r, "/patch/../../etc/passwd"), None);
        assert_eq!(ftp_path("/patch", "x.gpf").as_deref(), Some("/patch/x.gpf"));
        assert_eq!(ftp_path("/patch", "..").as_deref(), Some("/"));
        assert_eq!(ftp_path("/", ".."), None);
    }

    #[test]
    fn patch_list() {
        let archived = "300\r\n303 2003-12-19gdata_g_b.gpf\r\n304 2004-01-09gdata_g.gpf\r\n\r\n403\r\n//415 x.gpf";
        let got = patch_list_for_level(archived, 303, "// note");
        assert_eq!(got, "// note\r\n300\r\n303 2003-12-19gdata_g_b.gpf\r\n//304 2004-01-09gdata_g.gpf\r\n\r\n//403\r\n//415 x.gpf");
    }

    #[test]
    fn dates() {
        assert_eq!(civil(UNIX_EPOCH), (1970, 1, 1));
        assert_eq!(civil(UNIX_EPOCH + Duration::from_secs(1_072_224_000)), (2003, 12, 24));
        assert_eq!(mdtm(UNIX_EPOCH + Duration::from_secs(1_072_224_000 + 3661)), "20031224010101");
    }
}
