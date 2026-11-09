//! `ragnarok-offline.toml`. Relative paths are resolved against the config
//! file's directory, so the server works from any working directory.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use ro_game::GameConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// The client's install directory (holds data.grf).
    pub client_dir: PathBuf,
    /// Interface the servers listen on.
    pub bind: String,
    pub login_port: u16,
    /// Local admin console (Lua REPL); 0 disables. Always bound to 127.0.0.1.
    pub console_port: u16,
    /// Reload scripts automatically when a .lua file changes.
    pub watch_scripts: bool,
    /// Hex-dump all traffic to logs/packets.log.
    pub packet_log: bool,
    pub game: GameConfig,
    /// Stand-in for euRO's patch servers, for the CD's patcher (patchsrv.rs).
    pub patch_server: PatchServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PatchServerConfig {
    pub enabled: bool,
    /// HTTP (the patcher's `wpatch` host; any port, it is part of the URL).
    pub http_port: u16,
    /// FTP (the patcher's `fpatch` host; the patcher always dials 21).
    pub ftp_port: u16,
    /// Served directory: HTTP and FTP `/patch/x` are `dir/patch/x`.
    pub dir: PathBuf,
}

impl Default for PatchServerConfig {
    fn default() -> Self {
        PatchServerConfig { enabled: false, http_port: 6950, ftp_port: 21, dir: "patchsrv".into() }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            client_dir: PathBuf::from("client"),
            bind: "127.0.0.1".into(),
            login_port: 6900,
            console_port: 6123,
            watch_scripts: true,
            packet_log: false,
            game: GameConfig::default(),
            patch_server: PatchServerConfig::default(),
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let mut cfg: Config = if path.exists() {
            let s = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
            toml::from_str(&s).with_context(|| format!("parsing {}", path.display()))?
        } else {
            Config::default()
        };
        let base = path.parent().map(Path::to_path_buf).unwrap_or_default();
        let fix = |p: &mut PathBuf| {
            if p.is_relative() {
                *p = base.join(&*p);
            }
        };
        fix(&mut cfg.client_dir);
        fix(&mut cfg.game.save_dir);
        fix(&mut cfg.game.script_dir);
        fix(&mut cfg.game.db_dir);
        fix(&mut cfg.patch_server.dir);
        Ok(cfg)
    }
}

pub const TEMPLATE: &str = r#"# ragnarok-offline configuration. Paths are relative to this file.
# Full reference: docs/CONFIG.md

# Your Ragnarok Online install (the folder with data.grf and Ragexe.exe).
client_dir = "client"

bind = "127.0.0.1"
login_port = 6900
console_port = 6123      # local Lua console: ragnarok-offline console
watch_scripts = true     # reload scripts/*.lua when they change
packet_log = false       # hex-dump traffic to logs/packets.log

[game]
server_name = "Offline"
public_ip = [127, 0, 0, 1]
char_port = 6121
map_port = 5121
save_dir = "saves"
script_dir = "scripts"
db_dir = "data/db"
auto_register = true       # any new ID/password creates an account (ID_M / ID_F picks sex)
new_account_gm_level = 99  # @commands for your own account
start_point = { map = "new_zone01", x = 53, y = 111 }
start_zeny = 0
start_items = [[1201, 1], [2301, 1]]   # Knife, Cotton Shirt
base_exp_rate = 100   # percent; 100 = official 1x
job_exp_rate = 100
drop_rate = 100
autosave_secs = 120

# A stand-in for euRO's patch servers so the CD's own patcher (Ragnarok.exe)
# runs offline; RagnarokOffline.exe turns it on (docs/WINDOWS.md).
[patch_server]
enabled = false
http_port = 6950
ftp_port = 21            # the patcher always dials 21
dir = "patchsrv"         # serves dir/patch/* over HTTP and FTP
"#;
