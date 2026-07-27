//! Prints a human-readable summary of a NEXRAD Level 3 product: this is the
//! smallest possible example of using the `radar` library directly (no
//! logging setup, no file output) — read a file, parse it, look at the
//! fields on the returned `Radar`.
//!
//! Run with the bundled sample fixture:
//!
//! ```sh
//! cargo run --example inspect
//! ```
//!
//! or against any other file:
//!
//! ```sh
//! cargo run --example inspect -- path/to/other/file
//! ```

use std::process::ExitCode;

use radar::{Radar, SymPacketData};

const DEFAULT_SAMPLE: &str = "data/sn_DS.p20-r_kmkx.last";

fn main() -> ExitCode {
    let path = std::env::args().nth(1).unwrap_or_else(|| DEFAULT_SAMPLE.to_string());

    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("failed to read {path}: {e}");
            return ExitCode::FAILURE;
        }
    };

    let (leftover, radar) = match Radar::from_vec(bytes) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("failed to parse {path}: {e}");
            return ExitCode::FAILURE;
        }
    };

    let (lat, lon, height) = (
        radar.product_description.latitude as f64 * 0.001,
        radar.product_description.longitude as f64 * 0.001,
        radar.product_description.height,
    );

    println!("Station:      {}", radar.text_header.location);
    println!("Product:      {} ({:?})", radar.text_header.aaa, radar.message_header.code);
    println!("Issued:       {}", radar.message_header.datetime);
    println!("Radar site:   {lat:.3}, {lon:.3} ({height} ft)");
    println!("VCP:          {}", radar.product_description.vcp);

    match &radar.symbology {
        Some(symbology) => {
            println!("Symbology:    {} layer(s)", symbology.layers.len());
            for (i, layer) in symbology.layers.iter().enumerate() {
                match layer {
                    SymPacketData::RadialDataAF1F(p) => {
                        println!("  layer {i}: {} radials x {} bins (run-length encoded)", p.radials.len(), p.header.num_bins);
                    }
                    SymPacketData::DigitalRadialDataArray(p) => {
                        println!("  layer {i}: {} radials x {} bins (digital)", p.radials.len(), p.header.num_bins);
                    }
                    other => println!("  layer {i}: {other:?}"),
                }
            }
        }
        None => println!("Symbology:    none"),
    }

    if !leftover.is_empty() {
        println!("Note:         {} unparsed trailing byte(s)", leftover.len());
    }

    ExitCode::SUCCESS
}
