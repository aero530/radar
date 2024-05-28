//! Create assembly based on external part files
//!
//! Example use:
//! ```no_run
//! cargo run --features="assembly" --example assembly
//! ```

use std::{fs::File, io::Write, path::PathBuf};

use tracing::{error, warn, debug, trace, info, Level};
use tracing_subscriber::{self, EnvFilter};

use radar::Radar;

fn main() {

let filter = EnvFilter::builder()
    .with_default_directive(Level::TRACE.into())
    .from_env()
    .unwrap_or_default(); // set noisy logs to info
                          // .add_directive("hyper::client=INFO".parse()?);

    let subscriber = tracing_subscriber::fmt()
        .with_ansi(true)
        .with_env_filter(filter);

    subscriber.init();

    error!("Designates very serious errors.");
    warn!("Designates hazardous situations.");
    info!("Designates useful information.");
    debug!("Designates lower priority information.");
    trace!("Designates very low priority, often extremely verbose, information.");

    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => "".to_string(),
    };
    let filename = PathBuf::from(&path);
    let file = std::fs::read(&filename).unwrap_or_default();


    let (leftover, output) = Radar::from_vec(file).unwrap();

    // write to file
    let filepath: &str = "out.json";
    let mut file = File::create(filepath).expect("write error");
    let s = serde_json::to_string(&output).unwrap();
    let _ = file.write_all(s.as_bytes());

    
    let filepath: &str = "out_leftover.json";
    let mut file = File::create(filepath).expect("write error");
    let s = serde_json::to_string(&leftover).unwrap();
    let _ = file.write_all(s.as_bytes());

    let _ = output.plot();

}






