//! Non-Windows side: the client runs under Wine, dialogs become stderr.

use std::path::PathBuf;

use anyhow::{Result, bail};

use crate::RegValue;

pub fn error(text: &str) {
    eprintln!("error: {text}");
}

pub fn ask(text: &str) -> bool {
    eprintln!("warning: {text} (continuing)");
    true
}

pub fn starting<T: Send>(_text: &str, work: impl FnOnce() -> T + Send) -> T {
    work()
}

pub fn allow_foreground(_pid: u32) {}

pub fn run_patched(_exe: &std::path::Path, _dir: &std::path::Path, _patches: &[(usize, &[u8], Vec<u8>)]) -> Result<()> {
    bail!("the patcher redirect needs the Windows build (run RagnarokOffline.exe in Wine)")
}

pub fn is_wine() -> bool {
    false
}

pub fn local_appdata() -> PathBuf {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .unwrap_or_else(std::env::temp_dir)
}

pub fn run_elevated(_args: &str) -> Result<u32> {
    bail!("the game folder is read-only; make it writable or run `ragnarok-offline setup` with the right permissions")
}

pub fn progress<T: Send>(text: &str, work: impl FnOnce(&(dyn Fn(usize, usize) + Sync)) -> T + Send) -> T {
    eprintln!("{text}");
    work(&|_, _| {})
}

/// Wine keeps the client's registry in its prefix; use the client's Setup.exe
/// (or `wine regedit` a .reg file, see tools/client-test) to configure it.
pub fn client_settings_exist() -> bool {
    true
}

pub fn write_client_settings(_values: &[(&str, RegValue)]) -> Result<&'static str> {
    Ok("unchanged")
}
