//! Fetching the disc image from the Internet Archive, politely.
//!
//! Modelled on what the Archive's own `ia` command-line tool does
//! (internetarchive/files.py `File.download`, session.py
//! `mount_http_adapter`), so this tool asks no more of archive.org's
//! servers than `ia download` would:
//!
//! - one metadata request (`/metadata/<item>`), then one file, over one
//!   connection: no parallel or segmented downloads;
//! - `/download/<item>/<file>?cnt=0`, which redirects to the data node
//!   holding the item; `cnt=0` keeps tool downloads out of the item's view
//!   count, as `ia` does by default;
//! - a User-Agent naming this tool and where it comes from;
//! - 1 MiB reads;
//! - retries only on connection errors, timeouts, 429 and 5xx, waiting longer
//!   each time and as long as a Retry-After header asks (`ia`: urllib3 Retry
//!   with backoff_factor 1 and respect_retry_after_header, then 3 s between
//!   whole-file attempts), giving up after a few failures in a row;
//! - an interrupted download resumes with a Range request from where it
//!   stopped instead of starting over, then the whole file is checked;
//! - a file already downloaded and checked is never fetched again (`ia
//!   download --checksum`): it stays in the cache folder unless the user
//!   asks for it to be deleted.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};

use crate::job::{Cancelled, Report};

pub const ITEM: &str = "ragnarok-online-euro-cd-2003-12-ichipm01";
pub const FILE: &str = "RAGNAROK.iso";
/// docs/PRESERVATION.md
pub const ISO_SIZE: u64 = 639_496_192;
pub const ISO_SHA256: &str = "b5d579a4edd9c366e52460565e30aa5b48984eb4d40c9b193fdc6c8c35bd19a6";
const ISO_SHA1: &str = "3d3b23207b2bb74df0a92d5b1ab3d2670f0f890a";

pub fn details_url() -> String {
    format!("https://archive.org/details/{ITEM}")
}

fn user_agent() -> String {
    format!(
        "RagnarokOfflineSetupBuilder/{} (+https://github.com/Francesco149/ragnarok-offline; {})",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS
    )
}

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .user_agent(&user_agent())
        .timeout_connect(Duration::from_secs(30))
        // per read: a stalled connection fails and is resumed, a slow one is fine
        .timeout_read(Duration::from_secs(60))
        .redirects(5)
        // HTTPS_PROXY / ALL_PROXY, for networks that only go out through a proxy
        .try_proxy_from_env(true)
        .build()
}

/// Wait `secs`, waking every 100 ms to see if the user cancelled.
fn wait(r: &dyn Report, secs: u64) -> Result<()> {
    for _ in 0..secs * 10 {
        if r.cancelled() {
            return Err(Cancelled.into());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}

/// How long to wait before attempt `n` (1-based) after a failure: 3, 6, 12,
/// 24, 48 s, or what Retry-After asked for (at most 10 minutes).
fn backoff(n: u32, retry_after: Option<u64>) -> u64 {
    retry_after.map(|s| s.min(600)).unwrap_or(3u64 << (n - 1).min(4))
}

const MAX_FAILURES: u32 = 6;

enum Failure {
    /// worth another try, after the given Retry-After if any
    Transient(String, Option<u64>),
    Fatal(anyhow::Error),
}

fn classify(e: ureq::Error) -> Failure {
    match e {
        ureq::Error::Status(code, resp) if code == 429 || code >= 500 => {
            let after = resp.header("Retry-After").and_then(|v| v.trim().parse().ok());
            Failure::Transient(format!("the server answered {code}"), after)
        }
        ureq::Error::Status(code, resp) => Failure::Fatal(anyhow::anyhow!("the server answered {code} {} for {}", resp.status_text(), resp.get_url())),
        ureq::Error::Transport(t) => Failure::Transient(t.to_string(), None),
    }
}

/// Check the item still lists the file we know, as `ia` does before
/// downloading. A missing or different file stops here: this tool only
/// knows the December 2003 disc.
fn check_metadata(agent: &ureq::Agent, r: &dyn Report) -> Result<()> {
    let url = format!("https://archive.org/metadata/{ITEM}");
    let mut failures = 0;
    loop {
        if r.cancelled() {
            return Err(Cancelled.into());
        }
        match agent.get(&url).call() {
            Ok(resp) => {
                let v: serde_json::Value = serde_json::from_reader(resp.into_reader().take(16 << 20)).context("reading the item's metadata")?;
                let files = v["files"].as_array().with_context(|| format!("{ITEM}: no such item on the Internet Archive (or it is dark)"))?;
                let f = files
                    .iter()
                    .find(|f| f["name"] == FILE)
                    .with_context(|| format!("{ITEM} no longer has {FILE}"))?;
                let size = f["size"].as_str().and_then(|s| s.parse::<u64>().ok());
                if size != Some(ISO_SIZE) || f["sha1"].as_str() != Some(ISO_SHA1) {
                    bail!("{FILE} on the Internet Archive is not the disc image this builder knows (size {size:?}, sha1 {})", f["sha1"]);
                }
                return Ok(());
            }
            Err(e) => match classify(e) {
                Failure::Fatal(e) => return Err(e),
                Failure::Transient(why, after) => {
                    failures += 1;
                    if failures >= MAX_FAILURES {
                        bail!("could not reach archive.org ({why}); try again later");
                    }
                    let s = backoff(failures, after);
                    r.status(&format!("archive.org: {why}; trying again in {s} s"));
                    wait(r, s)?;
                }
            },
        }
    }
}

/// Hash a whole file, reporting progress.
pub fn sha256_file(path: &Path, label: &str, r: &dyn Report) -> Result<String> {
    let mut f = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let total = f.metadata()?.len();
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    let mut done = 0u64;
    r.status(label);
    loop {
        if r.cancelled() {
            return Err(Cancelled.into());
        }
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
        done += n as u64;
        r.progress(done, total);
    }
    Ok(crate::aio::hex(&h.finalize()))
}

/// The verified disc image in `cache`: the copy already there, or a new
/// download. Returns its path and whether it was downloaded now.
pub fn fetch(cache: &Path, r: &dyn Report) -> Result<(PathBuf, bool)> {
    fs::create_dir_all(cache).with_context(|| format!("creating {}", cache.display()))?;
    let done = cache.join(FILE);
    let part = cache.join(format!("{FILE}.part"));
    if done.metadata().map(|m| m.len() == ISO_SIZE).unwrap_or(false) {
        if sha256_file(&done, "Checking the disc image downloaded earlier", r)? == ISO_SHA256 {
            return Ok((done, false));
        }
        fs::remove_file(&done)?;
    }

    let agent = agent();
    r.status("Asking the Internet Archive for the disc image");
    check_metadata(&agent, r)?;

    let url = format!("https://archive.org/download/{ITEM}/{FILE}?cnt=0");
    let mut failures = 0u32;
    loop {
        if r.cancelled() {
            return Err(Cancelled.into());
        }
        let mut have = part.metadata().map(|m| m.len()).unwrap_or(0);
        if have > ISO_SIZE {
            fs::remove_file(&part)?;
            have = 0;
        }
        if have == ISO_SIZE {
            break;
        }
        r.status(if have > 0 { "Resuming the download from the Internet Archive" } else { "Downloading the disc image from the Internet Archive" });
        let mut req = agent.get(&url);
        if have > 0 {
            req = req.set("Range", &format!("bytes={have}-"));
        }
        let failure = match req.call() {
            Ok(resp) => {
                let start = match resp.status() {
                    206 => {
                        // "bytes <start>-<end>/<total>"
                        let cr = resp.header("Content-Range").unwrap_or("");
                        let start = cr.strip_prefix("bytes ").and_then(|s| s.split('-').next()).and_then(|s| s.parse::<u64>().ok());
                        if start != Some(have) {
                            fs::remove_file(&part).ok();
                            bail!("unexpected Content-Range {cr:?} when resuming at {have}");
                        }
                        have
                    }
                    200 => 0,
                    s => bail!("unexpected answer {s} from {}", resp.get_url()),
                };
                let mut f = OpenOptions::new().create(true).write(true).truncate(false).open(&part).with_context(|| format!("writing {}", part.display()))?;
                f.set_len(start)?;
                f.seek(SeekFrom::Start(start))?;
                let mut body = resp.into_reader();
                let mut buf = vec![0u8; 1 << 20];
                let mut got = start;
                let mut res = None;
                loop {
                    if r.cancelled() {
                        f.flush()?;
                        return Err(Cancelled.into());
                    }
                    match body.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            f.write_all(&buf[..n]).with_context(|| format!("writing {} (is the disk full?)", part.display()))?;
                            got += n as u64;
                            r.progress(got, ISO_SIZE);
                        }
                        Err(e) => {
                            res = Some(Failure::Transient(format!("connection lost ({e})"), None));
                            break;
                        }
                    }
                }
                f.flush()?;
                drop(f);
                if got > have + (1 << 20) {
                    failures = 0; // it moved: only failures in a row count
                }
                match res {
                    None if got == ISO_SIZE => break,
                    None => Some(Failure::Transient(format!("the download stopped at {got} of {ISO_SIZE} bytes"), None)),
                    other => other,
                }
            }
            Err(ureq::Error::Status(416, _)) => {
                // our partial file does not fit the remote one: start over
                fs::remove_file(&part).ok();
                Some(Failure::Transient("the server refused to resume".into(), None))
            }
            Err(e) => Some(classify(e)),
        };
        match failure {
            None => {}
            Some(Failure::Fatal(e)) => return Err(e),
            Some(Failure::Transient(why, after)) => {
                failures += 1;
                if failures >= MAX_FAILURES {
                    bail!("the download keeps failing ({why}); run this again later to resume it");
                }
                let s = backoff(failures, after);
                r.status(&format!("{why}; trying again in {s} s"));
                wait(r, s)?;
            }
        }
    }

    if sha256_file(&part, "Checking the download", r)? != ISO_SHA256 {
        fs::remove_file(&part).ok();
        bail!("the downloaded disc image is damaged (sha256 mismatch); it was deleted, run this again to download it anew");
    }
    fs::rename(&part, &done)?;
    Ok((done, true))
}

#[cfg(test)]
mod tests {
    #[test]
    fn backoff_grows_and_caps() {
        let v: Vec<u64> = (1..=6).map(|n| super::backoff(n, None)).collect();
        assert_eq!(v, [3, 6, 12, 24, 48, 48]);
        assert_eq!(super::backoff(1, Some(5000)), 600);
    }
}
