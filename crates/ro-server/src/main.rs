//! ragnarok-offline: single-player server for the 2003-12-18 euRO client.

use std::io::{BufRead, Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ro_game::Game;
use ro_server::config::{self, Config};
use ro_server::net::NetOutbox;
use ro_server::{server, setup};
use tracing::info;

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
        /// Do nothing if the folder is already prepared for this server.
        #[arg(long)]
        if_needed: bool,
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
        Cmd::Setup { client, address, port, name, if_needed } => {
            if if_needed && setup::is_prepared(&client, &address, port, &name) {
                println!("{}: already prepared", client.display());
                return Ok(());
            }
            setup::run(&client, &address, port, &name)
        }
        Cmd::Console => console(&cli.config, None),
        Cmd::Eval { code } => console(&cli.config, Some(code.join(" "))),
        Cmd::Grf { cmd } => grf(cmd),
        Cmd::Check => check(&cli.config),
    }
}

fn check(path: &Path) -> Result<()> {
    let cfg = Config::load(path)?;
    let res = server::load_resources(&cfg)?;
    let game = Game::new(cfg.game.clone(), res, Box::new(NetOutbox::default()))?;
    let mut missing = std::collections::BTreeMap::<String, usize>::new();
    for m in game.script_maps() {
        if !game.res.maps.contains(&m) {
            *missing.entry(m).or_default() += 1;
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
    let mut game = game;
    let a = ro_game::audit::audit(&mut game);
    let list = |title: &str, v: &[String]| {
        if !v.is_empty() {
            println!("{title} ({}):", v.len());
            for l in v {
                println!("  {l}");
            }
        }
    };
    println!("world: {} maps can be reached from the start point", a.maps_reached);
    list("warps that cannot be used", &a.bad_warps);
    list("script destinations on blocked cells", &a.bad_destinations);
    if !a.unreached_maps.is_empty() {
        println!("maps with NPCs or monsters that nothing leads to ({}):", a.unreached_maps.len());
        for (m, n) in &a.unreached_maps {
            println!("  {m}: {n}");
        }
    }
    list("NPCs out of sight of anywhere the player can stand", &a.unreached_npcs);
    list("warp tiles the player cannot step on", &a.unreached_warps);
    list("ids the database lacks", &a.unknown_ids);
    println!("ok");
    Ok(())
}

fn run(path: &Path) -> Result<()> {
    let cfg = Config::load(path)?;
    if !path.exists() {
        info!("no {} found; using defaults (run `ragnarok-offline init`)", path.display());
    }
    let log_dir = path.parent().unwrap_or(Path::new(".")).join("logs");
    server::start(cfg, server::Options { ctrl_c: true, log_dir })?.wait();
    Ok(())
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
