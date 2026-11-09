//! ragnarok-offline: single-player server for the 2003-12-18 euRO client.

mod config;
mod net;
mod setup;

use std::io::{BufRead, Read, Write};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant, SystemTime};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ro_game::{Game, Resources, ServerKind};
use tracing::{error, info};

use crate::config::Config;
use crate::net::{Event, NetOutbox};

#[derive(Parser)]
#[command(version, about = "Single-player Ragnarok Online server for the 2003-12-18 euRO client")]
struct Cli {
    /// Config file (created with defaults by `init`).
    #[arg(short, long, global = true, default_value = "ragnarok-offline.toml")]
    config: PathBuf,
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run the server (default).
    Run,
    /// Write a default ragnarok-offline.toml next to the binary's working dir.
    Init {
        /// Client install directory to put in the config.
        #[arg(long)]
        client: Option<PathBuf>,
    },
    /// Point a client install at this server (merges patches into data.grf, adds clientinfo.xml).
    Setup {
        /// Client install directory (with data.grf and Ragexe.exe).
        #[arg(long)]
        client: PathBuf,
        #[arg(long, default_value = "127.0.0.1")]
        address: String,
        #[arg(long, default_value_t = 6900)]
        port: u16,
        #[arg(long, default_value = "Offline")]
        name: String,
    },
    /// Interactive Lua console of a running server.
    Console,
    /// Evaluate one Lua line on a running server and print the result.
    Eval { code: Vec<String> },
    /// Inspect GRF/GPF archives.
    Grf {
        #[command(subcommand)]
        cmd: GrfCmd,
    },
    /// Check client data + scripts + database without starting the servers.
    Check,
}

#[derive(Subcommand)]
enum GrfCmd {
    /// List entries (optionally filtered by substring).
    Ls { file: PathBuf, filter: Option<String> },
    /// Write one entry to stdout.
    Cat { file: PathBuf, name: String },
    /// Inflate every entry and report failures.
    Verify { files: Vec<PathBuf> },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with_target(false)
        .init();
    let mut cli = Cli::parse();
    // Double-clicked on Windows: the working dir may not be the exe's folder.
    if !cli.config.exists() && cli.config.is_relative() {
        if let Some(dir) = std::env::current_exe().ok().and_then(|e| e.parent().map(Path::to_path_buf)) {
            let beside = dir.join(&cli.config);
            if beside.exists() {
                cli.config = beside;
            }
        }
    }
    let interactive = cli.cmd.is_none();
    let r = dispatch(cli);
    if let (Err(e), true) = (&r, interactive) {
        // keep a double-clicked console window open long enough to read the error
        eprintln!("error: {e:#}");
        eprintln!("press Enter to close");
        let _ = std::io::stdin().read_line(&mut String::new());
    }
    r
}

fn dispatch(cli: Cli) -> Result<()> {
    match cli.cmd.unwrap_or(Cmd::Run) {
        Cmd::Run => run(&cli.config),
        Cmd::Init { client } => {
            if cli.config.exists() {
                anyhow::bail!("{} already exists", cli.config.display());
            }
            let mut t = config::TEMPLATE.to_string();
            if let Some(c) = client {
                t = t.replace("client_dir = \"client\"", &format!("client_dir = {:?}", c.display().to_string()));
            }
            std::fs::write(&cli.config, t)?;
            println!("wrote {}", cli.config.display());
            Ok(())
        }
        Cmd::Setup { client, address, port, name } => setup::run(&client, &address, port, &name),
        Cmd::Console => console(&cli.config, None),
        Cmd::Eval { code } => console(&cli.config, Some(code.join(" "))),
        Cmd::Grf { cmd } => grf(cmd),
        Cmd::Check => check(&cli.config),
    }
}

fn load_resources(cfg: &Config) -> Result<Resources> {
    let vfs = setup::client_vfs(&cfg.client_dir)
        .with_context(|| format!("client_dir = {} (see ragnarok-offline.toml)", cfg.client_dir.display()))?;
    let res = Resources::new(vfs);
    info!("client data: {} archives, {} maps", res.vfs.layers().len(), res.maps.len());
    Ok(res)
}

fn check(path: &Path) -> Result<()> {
    let cfg = Config::load(path)?;
    let res = load_resources(&cfg)?;
    let game = Game::new(cfg.game.clone(), res, Box::new(NetOutbox::default()))?;
    let mut missing = std::collections::BTreeMap::<String, usize>::new();
    if let Some(defs) = game_defs(&game) {
        for m in defs {
            if !game.res.maps.contains(&m) {
                *missing.entry(m).or_default() += 1;
            }
        }
    }
    if !missing.is_empty() {
        println!("script entries on maps this client does not have (skipped at runtime):");
        for (m, n) in &missing {
            println!("  {m}: {n}");
        }
    }
    for n in &game.notices {
        println!("notice: {n}");
    }
    println!("ok");
    Ok(())
}

fn game_defs(game: &Game) -> Option<Vec<String>> {
    Some(game.script_maps())
}

fn run(path: &Path) -> Result<()> {
    let cfg = Config::load(path)?;
    if !path.exists() {
        info!("no {} found; using defaults (run `ragnarok-offline init`)", path.display());
    }
    let res = load_resources(&cfg)?;
    let log = if cfg.packet_log {
        Some(Arc::new(net::PacketLog::open(&path.parent().unwrap_or(Path::new(".")).join("logs/packets.log"))?))
    } else {
        None
    };
    let out = NetOutbox::new(log);
    let (tx, rx) = mpsc::channel::<Event>();

    // network on a tokio runtime in the background
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build()?;
    let ip: std::net::IpAddr = cfg.bind.parse().context("bind address")?;
    for (port, kind) in [(cfg.login_port, ServerKind::Login), (cfg.game.char_port, ServerKind::Char), (cfg.game.map_port, ServerKind::Map)] {
        let (tx, out) = (tx.clone(), out.clone());
        rt.spawn(async move {
            if let Err(e) = net::listen(SocketAddr::new(ip, port), kind, tx, out).await {
                error!("{kind:?} server on port {port}: {e:#}");
                std::process::exit(1);
            }
        });
    }
    if cfg.console_port != 0 {
        let tx = tx.clone();
        let port = cfg.console_port;
        rt.spawn(async move {
            if let Err(e) = net::console(port, tx).await {
                error!("console: {e:#}");
            }
        });
    }
    if cfg.watch_scripts {
        let tx = tx.clone();
        let dir = cfg.game.script_dir.clone();
        std::thread::spawn(move || watch(&dir, tx));
    }
    {
        let tx = tx.clone();
        rt.spawn(async move {
            let _ = tokio::signal::ctrl_c().await;
            let (rtx, _rrx) = mpsc::channel();
            let _ = tx.send(Event::Console { line: ":shutdown".into(), reply: rtx });
        });
    }

    // the game lives on this thread (Lua is single-threaded)
    let mut game = Game::new(cfg.game.clone(), res, Box::new(out))?;
    info!("ready: point the client at {}:{} (ragnarok-offline setup)", cfg.bind, cfg.login_port);
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
    Ok(())
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
                } else if p.extension().is_some_and(|x| x == "lua") {
                    if let Ok(m) = e.metadata().and_then(|m| m.modified()) {
                        best = best.max(m);
                    }
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

fn console(path: &Path, one: Option<String>) -> Result<()> {
    let cfg = Config::load(path)?;
    let mut sock = std::net::TcpStream::connect(("127.0.0.1", cfg.console_port))
        .with_context(|| format!("connecting to console port {} - is the server running?", cfg.console_port))?;
    let mut reader = std::io::BufReader::new(sock.try_clone()?);
    let mut ask = |line: &str| -> Result<String> {
        sock.write_all(format!("{line}\n").as_bytes())?;
        let mut out = String::new();
        loop {
            let mut l = String::new();
            if reader.read_line(&mut l)? == 0 {
                break;
            }
            if l == ".\n" {
                break;
            }
            out.push_str(if l == "..\n" { ".\n" } else { &l });
        }
        Ok(out)
    };
    if let Some(code) = one {
        print!("{}", ask(&code)?);
        return Ok(());
    }
    println!("ragnarok-offline console - :help for commands, Ctrl-D to quit");
    let stdin = std::io::stdin();
    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut line = String::new();
        if stdin.lock().read_line(&mut line)? == 0 {
            break;
        }
        if line.trim().is_empty() {
            continue;
        }
        print!("{}", ask(line.trim_end())?);
    }
    Ok(())
}

fn grf(cmd: GrfCmd) -> Result<()> {
    match cmd {
        GrfCmd::Ls { file, filter } => {
            let g = ro_grf::Grf::open(&file)?;
            let f = filter.map(|s| s.to_ascii_lowercase());
            for e in g.entries() {
                let n = e.display_name();
                if f.as_ref().is_none_or(|f| n.to_ascii_lowercase().contains(f)) {
                    println!("{:02x} {:>10} {}", e.flags, e.real_len, n);
                }
            }
        }
        GrfCmd::Cat { file, name } => {
            let g = ro_grf::Grf::open(&file)?;
            std::io::stdout().write_all(&g.read(&name)?)?;
        }
        GrfCmd::Verify { files } => {
            for f in files {
                let g = ro_grf::Grf::open(&f)?;
                let (mut ok, mut bad) = (0, 0);
                for e in g.entries() {
                    match g.read_entry(e) {
                        Ok(_) => ok += 1,
                        Err(err) => {
                            bad += 1;
                            println!("  {err}");
                        }
                    }
                }
                println!("{}: v{:#x} {ok} ok, {bad} bad", f.display(), g.version());
            }
        }
    }
    let _ = std::io::stdin().read(&mut []);
    Ok(())
}
