//! The patch server over real sockets, speaking the protocol the CD's patcher
//! uses (HTTP GET with Content-Length; anonymous passive FTP CWD/TYPE/PASV/LIST/RETR).

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

fn root() -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("ro-patchsrv-test-{}", std::process::id()));
    std::fs::create_dir_all(d.join("patch")).unwrap();
    std::fs::write(d.join("patch/patch_allow.txt"), "allow").unwrap();
    std::fs::write(d.join("patch/patch2.txt"), "300\r\n303 2003-12-19gdata_g_b.gpf\r\n").unwrap();
    d
}

fn start(root: &std::path::Path) -> (u16, u16, tokio::runtime::Runtime) {
    let h = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let f = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let (hp, fp) = (h.local_addr().unwrap().port(), f.local_addr().unwrap().port());
    h.set_nonblocking(true).unwrap();
    f.set_nonblocking(true).unwrap();
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build().unwrap();
    let (r1, r2) = (root.to_path_buf(), root.to_path_buf());
    rt.spawn(async move { ro_server::patchsrv::http(h, r1).await });
    rt.spawn(async move { ro_server::patchsrv::ftp(f, r2).await });
    (hp, fp, rt)
}

fn http_get(port: u16, path: &str) -> String {
    let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
    write!(s, "GET {path} HTTP/1.1\r\nHost: wpatch.euro-ro.net\r\nUser-Agent: Patch Client\r\n\r\n").unwrap();
    let mut out = String::new();
    s.read_to_string(&mut out).unwrap();
    out
}

#[test]
fn http_serves_files_with_content_length() {
    let r = root();
    let (hp, _, _rt) = start(&r);
    let resp = http_get(hp, "/patch/patch_allow.txt");
    assert!(resp.starts_with("HTTP/1.1 200 OK"), "{resp}");
    assert!(resp.contains("Content-Length: 5\r\n"), "{resp}");
    assert!(resp.ends_with("\r\n\r\nallow"), "{resp}");
    assert!(http_get(hp, "/patch/notice_xmas.txt").starts_with("HTTP/1.1 404"));
    assert!(http_get(hp, "/patch/../../../etc/passwd").starts_with("HTTP/1.1 404"));
}

#[test]
fn ftp_session_like_wininet() {
    let r = root();
    let (_, fp, _rt) = start(&r);
    let c = TcpStream::connect(("127.0.0.1", fp)).unwrap();
    let mut rd = BufReader::new(c.try_clone().unwrap());
    let mut w = c;
    let mut line = String::new();
    let mut ask = |cmd: &str| -> String {
        if !cmd.is_empty() {
            write!(w, "{cmd}\r\n").unwrap();
        }
        line.clear();
        rd.read_line(&mut line).unwrap();
        line.clone()
    };
    assert!(ask("").starts_with("220"));
    assert!(ask("USER anonymous").starts_with("331"));
    assert!(ask("PASS IEUser@").starts_with("230"));
    assert!(ask("CWD /patch").starts_with("250"));
    assert!(ask("TYPE A").starts_with("200"));
    let pasv = |reply: String| -> u16 {
        let nums: Vec<u16> = reply.split(['(', ')']).nth(1).unwrap().split(',').map(|x| x.parse().unwrap()).collect();
        nums[4] * 256 + nums[5]
    };
    let data = |port: u16| {
        let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
        let mut v = String::new();
        s.read_to_string(&mut v).unwrap();
        v
    };
    let p = pasv(ask("PASV"));
    assert!(ask("LIST").starts_with("150"));
    let listing = data(p);
    assert!(ask("").starts_with("226"));
    assert!(listing.contains("patch2.txt") && listing.starts_with("-rw-r--r-- 1 ro ro"), "{listing}");
    assert!(ask("SIZE patch2.txt").starts_with("213 "));
    let p = pasv(ask("PASV"));
    assert!(ask("RETR patch2.txt").starts_with("150"));
    assert_eq!(data(p), "300\r\n303 2003-12-19gdata_g_b.gpf\r\n");
    assert!(ask("").starts_with("226"));
    assert!(ask("REST 5").starts_with("350"));
    let p = pasv(ask("PASV"));
    assert!(ask("RETR patch2.txt").starts_with("150"));
    assert_eq!(data(p), "303 2003-12-19gdata_g_b.gpf\r\n", "REST resumes");
    assert!(ask("").starts_with("226"));
    assert!(ask("RETR 2004-01-09gdata_g.gpf").starts_with("425"), "no PASV first");
    assert!(ask("CWD /nope").starts_with("550"));
    assert!(ask("QUIT").starts_with("221"));
}
