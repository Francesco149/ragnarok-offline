//! List a GRF/GPF's entries, or extract them (any version, DES included).
//!
//!   grf_dump <file> [substring]            list matching entries
//!   grf_dump <file> <substring> <outdir>   extract matching entries
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let g = ro_grf::Grf::open(&args[0]).expect("open");
    let pat = args.get(1).map(|s| s.to_lowercase()).unwrap_or_default();
    let out = args.get(2).map(std::path::PathBuf::from);
    for e in g.entries() {
        let name = e.display_name();
        if !e.is_file() || !name.to_lowercase().contains(&pat) {
            continue;
        }
        match &out {
            None => println!("{name}"),
            Some(dir) => {
                let data = g.read_entry(e).expect("read");
                let p = dir.join(name.replace('\\', "/"));
                std::fs::create_dir_all(p.parent().unwrap()).unwrap();
                std::fs::write(&p, data).unwrap();
                println!("{} ({} bytes)", p.display(), std::fs::metadata(&p).unwrap().len());
            }
        }
    }
}
