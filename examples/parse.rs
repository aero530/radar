//! Parses a NEXRAD Level 3 file, writes the parsed structure (and any
//! trailing unparsed bytes) out as JSON, and renders the first symbology
//! layer to `image.png`.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example parse -- data/sn_DS.p20-r_kmkx.last
//! ```
//!
//! Optional flags:
//!
//! ```sh
//! # Choose the colour ramp used for digital data arrays.
//! cargo run --example parse -- data/sn_DC.radar_DS.32dhr_KMKX.last --ramp grayscale
//!
//! # Label the site line of the annotation panel, which the file itself does
//! # not carry.
//! cargo run --example parse -- <file> --site "KMKX - MILWAUKEE, WI"
//! ```

use std::{fs::File, io::Write, path::PathBuf, process::ExitCode};

use tracing::{info, warn, Level};
use tracing_subscriber::{self, EnvFilter};

use radar::{ColorRamp, PlotOptions, Radar};

fn main() -> ExitCode {
    let filter = EnvFilter::builder()
        .with_default_directive(Level::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_env_filter(filter)
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cli) = Cli::parse(&args) else {
        return ExitCode::FAILURE;
    };
    let filename = PathBuf::from(&cli.path);

    let file = match std::fs::read(&filename) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("failed to read {}: {e}", filename.display());
            return ExitCode::FAILURE;
        }
    };

    let (leftover, radar) = match Radar::from_vec(file) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("failed to parse {}: {e}", filename.display());
            return ExitCode::FAILURE;
        }
    };
    info!(?radar.message_header.code, "parsed NEXRAD product");
    if !leftover.is_empty() {
        warn!(leftover_bytes = leftover.len(), "unparsed bytes remain at the end of the file");
    }

    if let Err(e) = write_json("out.json", &radar) {
        eprintln!("failed to write out.json: {e}");
        return ExitCode::FAILURE;
    }
    if let Err(e) = write_json("out_leftover.json", &leftover) {
        eprintln!("failed to write out_leftover.json: {e}");
        return ExitCode::FAILURE;
    }

    let mut options = PlotOptions::new();
    if let Some(ramp) = cli.ramp {
        options = options.with_ramp(ramp);
    }
    if let Some(site) = cli.site {
        options = options.with_site_label(site);
    }

    match radar.plot_with("image.png", &options) {
        Ok(()) => info!("wrote image.png"),
        Err(e) => warn!("failed to plot: {e}"),
    }

    ExitCode::SUCCESS
}

struct Cli {
    path: String,
    ramp: Option<ColorRamp>,
    site: Option<String>,
}

impl Cli {
    /// Parses the command line, printing usage and returning `None` on error.
    fn parse(args: &[String]) -> Option<Self> {
        let mut path = None;
        let mut ramp = None;
        let mut site = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--ramp" => {
                    let Some(value) = args.get(i + 1) else {
                        eprintln!("--ramp needs a value");
                        usage();
                        return None;
                    };
                    match ColorRamp::from_name(value) {
                        Some(r) => ramp = Some(r),
                        None => {
                            eprintln!("unknown ramp {value:?}");
                            usage();
                            return None;
                        }
                    }
                    i += 2;
                }
                "--site" => {
                    let Some(value) = args.get(i + 1) else {
                        eprintln!("--site needs a value");
                        usage();
                        return None;
                    };
                    site = Some(value.clone());
                    i += 2;
                }
                "-h" | "--help" => {
                    usage();
                    return None;
                }
                other if other.starts_with('-') => {
                    eprintln!("unknown flag {other:?}");
                    usage();
                    return None;
                }
                other => {
                    path = Some(other.to_string());
                    i += 1;
                }
            }
        }

        match path {
            Some(path) => Some(Cli { path, ramp, site }),
            None => {
                usage();
                None
            }
        }
    }
}

fn usage() {
    let ramps: Vec<&str> = ColorRamp::all().iter().map(|r| r.name()).collect();
    eprintln!("usage: parse <path-to-nexrad-level3-file> [--ramp <name>] [--site <label>]");
    eprintln!();
    eprintln!("  --ramp <name>   colour ramp for digital data arrays; one of:");
    eprintln!("                    {}", ramps.join(", "));
    eprintln!("                  defaults to the ramp matching the product's units");
    eprintln!("  --site <label>  site line for the annotation panel, e.g.");
    eprintln!("                    \"KMKX - MILWAUKEE, WI\"");
}

fn write_json<T: serde::Serialize>(path: &str, value: &T) -> anyhow::Result<()> {
    let json = serde_json::to_string(value)?;
    File::create(path)?.write_all(json.as_bytes())?;
    Ok(())
}
