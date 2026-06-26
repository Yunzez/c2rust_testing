//! CLI: `analyzer <crate-dir> [--enable-metrics]`
//!
//! Prints the call-graph JSON (functions / raw_edges / indirect_calls) to stdout.
//! If the crate fails to load, prints `{"error":"load_failed",...}` and exits 2 —
//! an unloadable crate is the translator's defect, not a tool crash.

use std::path::PathBuf;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = match args.iter().skip(1).find(|a| !a.starts_with("--")) {
        Some(d) => PathBuf::from(d),
        None => {
            eprintln!("usage: analyzer <crate-dir> [--enable-metrics]");
            exit(1);
        }
    };

    match analyzer::load_crate(&dir) {
        Ok(ac) => {
            let out = ac.analyze();
            println!("{}", serde_json::to_string_pretty(&out).unwrap());
        }
        Err(e) => {
            let err = serde_json::json!({
                "error": "load_failed",
                "detail": e.to_string(),
                "crate": dir.display().to_string(),
            });
            println!("{}", err);
            exit(2);
        }
    }
}
