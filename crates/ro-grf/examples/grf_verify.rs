//! Inflate every entry of the given archives; report failures.
fn main() {
    for path in std::env::args().skip(1) {
        let g = ro_grf::Grf::open(&path).expect("open");
        let (mut ok, mut bad, mut bytes) = (0, 0, 0u64);
        let mut enc = [0usize; 3];
        for e in g.entries() {
            if e.flags & 2 != 0 { enc[1] += 1 } else if e.flags & 4 != 0 { enc[2] += 1 } else { enc[0] += 1 }
            match g.read_entry(e) {
                Ok(d) => { ok += 1; bytes += d.len() as u64 }
                Err(err) => { bad += 1; if bad <= 5 { eprintln!("  FAIL {err}") } }
            }
        }
        println!("{path}: v{:#x} entries={} ok={ok} bad={bad} bytes={bytes} plain/mixed/header={enc:?}",
            g.version(), g.entries().len());
    }
}
