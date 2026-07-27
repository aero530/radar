//! Verifies that `Radar::plot_to` lays radial data out per the ICD's
//! azimuth convention.
//!
//! Figure 3-10 of the Class 1 User ICD (2620001AD) specifies that the radial
//! start angle is in tenths of a degree and that the "Scan is always in
//! Clockwise direction". Rendered north-up, that means azimuth 0° points at
//! the top of the image and angle increases clockwise: 90° right (east),
//! 180° down (south), 270° left (west).
//!
//! Rather than eyeballing the output, these tests synthesize a product with
//! a single colored wedge at a known azimuth, render it, and assert the
//! colored pixels land in the expected quadrant of the PNG.


/// Builds a minimal but spec-shaped NEXRAD Level 3 file whose symbology
/// block holds one AF1F radial packet. Each `(azimuth_tenths, color_level)`
/// produces one 1°-wide radial whose bins are all `color_level`.
///
/// Product code 56 (Storm Relative Mean Radial Velocity) is used because it
/// is one of the products with a real color table, so the rendered levels are
/// distinguishable rather than all gray.
fn synthetic_af1f_file(num_bins: i16, radials: &[(i16, u8)]) -> Vec<u8> {
    synthetic_af1f_file_for_product(56, num_bins, radials)
}

/// As [`synthetic_af1f_file`], but for an arbitrary product code so that the
/// with-table and without-table colouring paths can both be exercised.
fn synthetic_af1f_file_for_product(product: i16, num_bins: i16, radials: &[(i16, u8)]) -> Vec<u8> {
    // --- text header: SDUSxx KYYYY DDHHMM\r\r\nAAABBB\r\r\n (30 bytes) ---
    let mut file = b"SDUS73 KMKX 091253\r\r\nN0ZMKX\r\r\n".to_vec();
    assert_eq!(file.len(), 30, "text header must be 30 bytes");

    // --- message header block: 9 halfwords (Figure 3-3) ---
    file.extend_from_slice(&product.to_be_bytes()); // message code
    file.extend_from_slice(&1i16.to_be_bytes()); // date (1 = 1 Jan 1970)
    file.extend_from_slice(&0i32.to_be_bytes()); // time
    file.extend_from_slice(&0i32.to_be_bytes()); // length
    file.extend_from_slice(&0i16.to_be_bytes()); // source
    file.extend_from_slice(&0i16.to_be_bytes()); // dest
    file.extend_from_slice(&3i16.to_be_bytes()); // number of blocks

    // --- product description block: halfwords 10-60, 102 bytes ---
    let pd_start = file.len();
    file.extend_from_slice(&(-1i16).to_be_bytes()); // divider
    file.extend_from_slice(&42968i32.to_be_bytes()); // latitude (deg * 1000)
    file.extend_from_slice(&(-88551i32).to_be_bytes()); // longitude
    file.extend_from_slice(&1022i16.to_be_bytes()); // height
    file.extend_from_slice(&product.to_be_bytes()); // product code
    file.extend_from_slice(&1i16.to_be_bytes()); // operational mode
    file.extend_from_slice(&35i16.to_be_bytes()); // vcp
    file.extend_from_slice(&0i16.to_be_bytes()); // sequence number
    file.extend_from_slice(&1i16.to_be_bytes()); // volume scan number
    file.extend_from_slice(&1i16.to_be_bytes()); // volume scan date
    file.extend_from_slice(&0i32.to_be_bytes()); // volume scan time
    file.extend_from_slice(&1i16.to_be_bytes()); // product generation date
    file.extend_from_slice(&0i32.to_be_bytes()); // product generation time
    file.extend_from_slice(&[0u8; 4]); // halfwords 27-28 (product dependent)
    file.extend_from_slice(&1i16.to_be_bytes()); // elevation number
    file.extend_from_slice(&[0u8; 2]); // halfword 30
    file.extend_from_slice(&[0u8; 32]); // halfwords 31-46 threshold data
    file.extend_from_slice(&[0u8; 14]); // halfwords 47-53
    file.push(0); // version
    file.push(0); // spot blank
    file.extend_from_slice(&60i32.to_be_bytes()); // offset to symbology (halfwords)
    file.extend_from_slice(&0i32.to_be_bytes()); // offset to graphic
    file.extend_from_slice(&0i32.to_be_bytes()); // offset to tabular
    assert_eq!(file.len() - pd_start, 102, "product description must be 102 bytes");
    assert_eq!(file.len(), 150, "header section must be 150 bytes");

    // --- product symbology block ---
    // num_bins is split into runs of at most 15 (a run code is 4 bits). Each
    // run/level pair is one byte, and an RLE *halfword* holds two of them
    // (Figure 3-10 sheet 1), so the byte stream is padded to an even length.
    let run_pairs = (num_bins as usize).div_ceil(15);
    let rle_bytes = run_pairs.next_multiple_of(2);
    let rle_halfwords = rle_bytes / 2;
    // Each radial: 3 halfwords of header + the RLE payload.
    let radial_bytes = 6 + rle_bytes;
    let layer_len = 14 + radials.len() * radial_bytes; // packet header is 14 bytes
    let block_len = 10 + 6 + layer_len;

    file.extend_from_slice(&(-1i16).to_be_bytes()); // block divider
    file.extend_from_slice(&1i16.to_be_bytes()); // block id = 1
    file.extend_from_slice(&(block_len as i32).to_be_bytes()); // length of block
    file.extend_from_slice(&1i16.to_be_bytes()); // number of layers
    file.extend_from_slice(&(-1i16).to_be_bytes()); // layer divider
    file.extend_from_slice(&(layer_len as i32).to_be_bytes()); // length of data layer

    // AF1F radial data packet header (Figure 3-10)
    file.extend_from_slice(&(-20705i16).to_be_bytes()); // packet code AF1F
    file.extend_from_slice(&0i16.to_be_bytes()); // index of first range bin
    file.extend_from_slice(&num_bins.to_be_bytes()); // number of range bins
    file.extend_from_slice(&0i16.to_be_bytes()); // i center of sweep
    file.extend_from_slice(&0i16.to_be_bytes()); // j center of sweep
    file.extend_from_slice(&1000i16.to_be_bytes()); // scale factor (1.000)
    file.extend_from_slice(&(radials.len() as i16).to_be_bytes()); // number of radials

    for &(azimuth_tenths, level) in radials {
        file.extend_from_slice(&(rle_halfwords as i16).to_be_bytes()); // RLE halfwords
        file.extend_from_slice(&azimuth_tenths.to_be_bytes()); // radial start angle
        file.extend_from_slice(&10i16.to_be_bytes()); // radial angle delta (1.0 deg)

        let mut remaining = num_bins as usize;
        for _ in 0..run_pairs {
            let run = remaining.min(15) as u8;
            remaining -= run as usize;
            // High nibble = run, low nibble = color code (Figure 3-10 sheet 1).
            file.push((run << 4) | (level & 0x0F));
        }
        // Pad to the halfword boundary with a zero-length run, which draws
        // nothing (the trailing 0000 0000 shown in Figure 3-10 sheet 1).
        file.extend(std::iter::repeat_n(0u8, rle_bytes - run_pairs));
    }

    file
}

/// Builds a file whose symbology block holds a Digital Radial Data Array
/// packet (code 16, Figure 3-11c). Each `(azimuth_tenths, level)` produces one
/// 1°-wide radial whose bins all hold that 8-bit data level.
fn synthetic_packet16_file(product: i16, num_bins: i16, radials: &[(i16, u8)]) -> Vec<u8> {
    // Reuse the AF1F builder for everything up to the symbology block, then
    // replace the block with a packet-16 one.
    let mut file = synthetic_af1f_file_for_product(product, num_bins, &[]);
    file.truncate(150); // keep just the header section

    // Each radial: 3 halfwords of header + num_bins bytes of data levels,
    // padded to a halfword boundary (Note 1 of Figure 3-11c).
    let data_bytes = (num_bins as usize).next_multiple_of(2);
    let radial_bytes = 6 + data_bytes;
    let layer_len = 14 + radials.len() * radial_bytes; // packet header is 14 bytes
    let block_len = 10 + 6 + layer_len;

    file.extend_from_slice(&(-1i16).to_be_bytes()); // block divider
    file.extend_from_slice(&1i16.to_be_bytes()); // block id = 1
    file.extend_from_slice(&(block_len as i32).to_be_bytes()); // length of block
    file.extend_from_slice(&1i16.to_be_bytes()); // number of layers
    file.extend_from_slice(&(-1i16).to_be_bytes()); // layer divider
    file.extend_from_slice(&(layer_len as i32).to_be_bytes()); // length of data layer

    // Digital Radial Data Array packet header (Figure 3-11c)
    file.extend_from_slice(&16i16.to_be_bytes()); // packet code 16
    file.extend_from_slice(&0i16.to_be_bytes()); // index of first range bin
    file.extend_from_slice(&num_bins.to_be_bytes()); // number of range bins
    file.extend_from_slice(&0i16.to_be_bytes()); // i center of sweep
    file.extend_from_slice(&0i16.to_be_bytes()); // j center of sweep
    file.extend_from_slice(&1000i16.to_be_bytes()); // range scale factor (1.000)
    file.extend_from_slice(&(radials.len() as i16).to_be_bytes()); // number of radials

    for &(azimuth_tenths, level) in radials {
        file.extend_from_slice(&(data_bytes as i16).to_be_bytes()); // bytes in radial
        file.extend_from_slice(&azimuth_tenths.to_be_bytes()); // radial start angle
        file.extend_from_slice(&10i16.to_be_bytes()); // radial delta angle (1.0 deg)
        file.extend(std::iter::repeat_n(level, num_bins as usize));
        // Halfword pad byte when the bin count is odd.
        file.extend(std::iter::repeat_n(0u8, data_bytes - num_bins as usize));
    }

    file
}

struct Rendered {
    img: image::RgbImage,
}

impl Rendered {
    fn of(file: Vec<u8>) -> Self {
        Self::with_options(file, &radar::PlotOptions::new().without_panel())
    }

    /// Renders with the annotation panel switched off, so the canvas is exactly
    /// the square radar image and these tests measure geometry rather than
    /// panel layout.
    fn with_options(file: Vec<u8>, options: &radar::PlotOptions) -> Self {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("plot.png");
        let (_, radar) = radar::Radar::from_vec(file).expect("synthetic file should parse");
        radar.plot_with(&path, options).expect("plot should succeed");
        let img = image::open(&path).expect("rendered png should decode").to_rgb8();
        assert_eq!(
            img.width(),
            img.height(),
            "with the panel off the canvas should be square"
        );
        Rendered { img }
    }

    fn center(&self) -> (i64, i64) {
        (self.img.width() as i64 / 2, self.img.height() as i64 / 2)
    }

    /// Samples the pixel `radius_frac` of the way from the image center out to
    /// the plotted radius, at `azimuth_deg` measured clockwise from up.
    fn sample_at_azimuth(&self, azimuth_deg: f64, radius_frac: f64) -> [u8; 3] {
        let (xc, yc) = self.center();
        // plot.rs draws out to 0.458 of the image side.
        let r = self.img.height() as f64 * 0.458 * radius_frac;
        let theta = (azimuth_deg - 90.0).to_radians(); // 0 deg = up, clockwise
        let x = xc + (r * theta.cos()).round() as i64;
        let y = yc + (r * theta.sin()).round() as i64;
        let p = self.img.get_pixel(x as u32, y as u32);
        [p[0], p[1], p[2]]
    }
}

/// The radar image background, which is what shows through where nothing is
/// drawn.
const BACKGROUND: [u8; 3] = [0, 0, 0];

#[test]
fn azimuth_zero_is_drawn_toward_the_top_of_the_image() {
    // A single wedge at azimuth 0.0 deg, data level 11 -> (FF FF 00) yellow
    // per the Storm Relative Mean Radial Velocity table.
    let rendered = Rendered::of(synthetic_af1f_file(230, &[(0, 11)]));

    let up = rendered.sample_at_azimuth(0.5, 0.5);
    assert_eq!(up, [0xFF, 0xFF, 0x00], "azimuth 0 should be drawn upward (north)");

    // The other three cardinal directions must be untouched background.
    for az in [90.5, 180.5, 270.5] {
        assert_eq!(
            rendered.sample_at_azimuth(az, 0.5),
            BACKGROUND,
            "nothing should be drawn at azimuth {az}"
        );
    }
}

#[test]
fn azimuth_increases_clockwise_from_north() {
    // Distinct levels at the four cardinal azimuths. Colors come from the
    // SRM table in codes.rs, verified against the Product Specification.
    let rendered = Rendered::of(synthetic_af1f_file(
        230,
        &[
            (0, 11),    // 0 deg   -> yellow      (FF FF 00)
            (900, 5),   // 90 deg  -> med green   (00 BB 00)
            (1800, 9),  // 180 deg -> med orange  (F8 87 00)
            (2700, 3),  // 270 deg -> dark blue   (32 00 96)
        ],
    ));

    assert_eq!(
        rendered.sample_at_azimuth(0.5, 0.5),
        [0xFF, 0xFF, 0x00],
        "azimuth 0 (north) belongs at the top"
    );
    assert_eq!(
        rendered.sample_at_azimuth(90.5, 0.5),
        [0x00, 0xBB, 0x00],
        "azimuth 90 (east) belongs at the right — angle must increase clockwise"
    );
    assert_eq!(
        rendered.sample_at_azimuth(180.5, 0.5),
        [0xF8, 0x87, 0x00],
        "azimuth 180 (south) belongs at the bottom"
    );
    assert_eq!(
        rendered.sample_at_azimuth(270.5, 0.5),
        [0x32, 0x00, 0x96],
        "azimuth 270 (west) belongs at the left"
    );
}

#[test]
fn run_length_runs_fill_the_radial_from_the_center_outward() {
    let rendered = Rendered::of(synthetic_af1f_file(230, &[(0, 11)]));

    // The single radial covers all 230 bins, so the wedge should be colored
    // from near the center all the way out to r_max.
    for frac in [0.1, 0.3, 0.5, 0.7, 0.95] {
        assert_eq!(
            rendered.sample_at_azimuth(0.5, frac),
            [0xFF, 0xFF, 0x00],
            "radial should be filled at {frac} of full range"
        );
    }
    // Past the plotted radius there should be nothing.
    assert_eq!(rendered.sample_at_azimuth(0.5, 1.08), BACKGROUND);
}

/// Figure 3-10 allows up to 460 range bins per radial. The run accumulator
/// in plot.rs used to be a `u8`, so any product whose runs summed past 255
/// panicked with "attempt to add with overflow" in debug builds.
#[test]
fn renders_products_with_more_than_255_range_bins() {
    let rendered = Rendered::of(synthetic_af1f_file(460, &[(0, 11)]));

    // The full 460-bin radial must still reach the outer edge, which it only
    // can if the accumulated bin index never wrapped.
    assert_eq!(rendered.sample_at_azimuth(0.5, 0.95), [0xFF, 0xFF, 0x00]);
}

/// A product with no colour table in the specification, delivered as an
/// `AF1F` packet, really does render entirely in the gray fallback — this is
/// the case the "neutral gray" warning describes.
#[test]
fn af1f_products_without_a_table_render_gray() {
    // Product code 20 (Base Reflectivity) has no table in revision AE.
    let rendered = Rendered::of(synthetic_af1f_file_for_product(20, 230, &[(0, 11)]));

    assert_eq!(
        rendered.sample_at_azimuth(0.5, 0.5),
        [0x88, 0x88, 0x88],
        "an AF1F product with no spec table should be the gray fallback"
    );
}

/// A Digital Radial Data Array (packet code 16) is coloured by a synthetic hue
/// ramp that deliberately ignores the colour table, so it comes out in colour
/// even for a product the specification defines no table for. This is why the
/// "neutral gray" warning must not be emitted for these — it used to be, and
/// contradicted the image that was actually produced.
#[test]
fn digital_data_arrays_render_in_color_without_a_spec_table() {
    // Product code 99 (Base Velocity Data Array) has no table in revision AE.
    let rendered = Rendered::of(synthetic_packet16_file(99, 230, &[(0, 200)]));

    let sampled = rendered.sample_at_azimuth(0.5, 0.5);
    assert_ne!(
        sampled, [0x88, 0x88, 0x88],
        "the digital path should not use the gray fallback"
    );
    assert_ne!(sampled, BACKGROUND, "the wedge should have been drawn");

    // Level 200 -> hue 200/256 of the way round the wheel, which is blue-ish:
    // the blue channel should dominate.
    let [r, g, b] = sampled;
    assert!(
        b > r && b > g,
        "level 200 should land in the blue part of the ramp, got {sampled:?}"
    );
}

/// Distinct 8-bit levels must map to distinct colours on the ramp, otherwise
/// the plot conveys nothing about the data.
#[test]
fn digital_data_array_levels_map_to_distinct_colors() {
    let rendered = Rendered::of(synthetic_packet16_file(
        99,
        230,
        &[(0, 30), (900, 120), (1800, 210)],
    ));

    let a = rendered.sample_at_azimuth(0.5, 0.5);
    let b = rendered.sample_at_azimuth(90.5, 0.5);
    let c = rendered.sample_at_azimuth(180.5, 0.5);

    assert_ne!(a, b);
    assert_ne!(b, c);
    assert_ne!(a, c);
}
