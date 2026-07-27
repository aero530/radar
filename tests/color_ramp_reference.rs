//! Checks [`ColorRamp::NwsReflectivity`] against the legend it was extracted
//! from, so the transcription cannot drift away from its source.
//!
//! `data/sn_DC.radar_DS.32dhr_KMKX.png` is a reference plot of the bundled
//! product 32 file, produced by other software. Its legend draws twenty 5 dBZ
//! bands down a single column, each a linear gradient rather than a flat
//! swatch, separated by one-pixel black dividers. This test walks those
//! gradient pixels, works out the dBZ value each one stands for, and compares it
//! against what the ramp returns.

use radar::ColorRamp;

/// Column through the centre of the legend colour bar.
const BAR_X: u32 = 2322;
/// Y of the black divider above the topmost band.
const TOP_DIVIDER: u32 = 244;
/// Band pitch: 11 pixels of gradient plus a one-pixel divider.
const BAND_HEIGHT: u32 = 12;
const BANDS: u32 = 20;
/// dBZ at the top of the topmost band.
const TOP_DBZ: f32 = 75.0;
/// dBZ covered by each band.
const BAND_DBZ: f32 = 5.0;

fn reference_image() -> image::RgbImage {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/data/sn_DC.radar_DS.32dhr_KMKX.png"
    );
    image::open(path)
        .expect("the reference plot should be readable")
        .to_rgb8()
}

/// Every gradient pixel of the legend, as `(dBZ, reference colour)`.
fn legend_samples(img: &image::RgbImage) -> Vec<(f32, [u8; 3])> {
    let mut samples = Vec::new();
    for band in 0..BANDS {
        // Skip the divider row at the top of each band.
        for i in 1..BAND_HEIGHT {
            let y = TOP_DIVIDER + band * BAND_HEIGHT + i;
            // The band's dBZ endpoints sit on the dividers, `BAND_HEIGHT` rows
            // apart, and a row's colour is sampled at its centre — so row `i`
            // below the top divider is `(i + 0.5) / BAND_HEIGHT` of the way
            // through the band.
            let t = (i as f32 + 0.5) / BAND_HEIGHT as f32;
            let dbz = (TOP_DBZ - BAND_DBZ * band as f32) - BAND_DBZ * t;
            let p = img.get_pixel(BAR_X, y);
            samples.push((dbz, [p[0], p[1], p[2]]));
        }
    }
    samples
}

#[test]
fn ramp_reproduces_the_reference_legend() {
    let img = reference_image();
    let samples = legend_samples(&img);
    assert_eq!(samples.len(), (BANDS * (BAND_HEIGHT - 1)) as usize);

    let mut total = 0u32;
    let mut worst = 0u8;
    let mut worst_at = 0.0f32;

    for (dbz, expected) in &samples {
        let got = ColorRamp::NwsReflectivity.color_at(*dbz);
        let got = [got.0, got.1, got.2];
        let error = (0..3)
            .map(|c| got[c].abs_diff(expected[c]))
            .max()
            .unwrap_or(0);
        if error > worst {
            worst = error;
            worst_at = *dbz;
        }
        total += error as u32;
    }

    let mean = total as f32 / (samples.len() * 3) as f32;

    // The stops were recovered by fitting each band and averaging where two
    // bands meet, so a small residual is expected. These bounds are the
    // measured values with a little headroom; a real transcription error would
    // blow straight through them.
    assert!(
        worst <= 12,
        "worst channel error {worst}/255 at {worst_at} dBZ exceeds the 12/255 bound"
    );
    assert!(
        mean <= 2.0,
        "mean channel error {mean:.2}/255 exceeds the 2.0/255 bound"
    );
}

/// The ramp's endpoints must match the extremes of the reference bar, since
/// those are what everything outside the domain clamps to.
#[test]
fn ramp_endpoints_match_the_reference_bar() {
    let img = reference_image();

    // Just inside the top and bottom dividers.
    let top = img.get_pixel(BAR_X, TOP_DIVIDER + 1);
    let bottom = img.get_pixel(BAR_X, TOP_DIVIDER + BANDS * BAND_HEIGHT - 1);

    let ramp_top = ColorRamp::NwsReflectivity.color_at(75.0);
    let ramp_bottom = ColorRamp::NwsReflectivity.color_at(-25.0);

    // These compare an edge pixel against an extrapolated endpoint, so allow a
    // little more slack than the interior comparison above.
    for c in 0..3 {
        let t = [ramp_top.0, ramp_top.1, ramp_top.2][c];
        let b = [ramp_bottom.0, ramp_bottom.1, ramp_bottom.2][c];
        assert!(
            t.abs_diff(top[c]) <= 20,
            "top of bar channel {c}: ramp {t} vs reference {}",
            top[c]
        );
        assert!(
            b.abs_diff(bottom[c]) <= 20,
            "bottom of bar channel {c}: ramp {b} vs reference {}",
            bottom[c]
        );
    }
}

/// The RF swatch colour must match the reference too.
#[test]
fn range_folded_color_matches_the_reference_swatch() {
    let img = reference_image();
    // Centre of the RF swatch, below the colour bar.
    let p = img.get_pixel(BAR_X, 508);
    let rf = radar::RANGE_FOLDED;
    assert_eq!([rf.0, rf.1, rf.2], [p[0], p[1], p[2]]);
}

/// The ramp must be monotonic in the sense that stepping through dBZ never
/// produces the same colour twice in a row over a 5 dBZ span — otherwise part
/// of the bar would be flat and convey nothing.
#[test]
fn ramp_varies_across_every_band() {
    let ticks = ColorRamp::NwsReflectivity.ticks();
    for pair in ticks.windows(2) {
        let a = ColorRamp::NwsReflectivity.color_at(pair[0]);
        let b = ColorRamp::NwsReflectivity.color_at(pair[1]);
        assert_ne!(
            (a.0, a.1, a.2),
            (b.0, b.1, b.2),
            "band {} to {} dBZ is flat",
            pair[0],
            pair[1]
        );
    }
}
