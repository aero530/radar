//! Predefined colour ramps for rendering digital data arrays.
//!
//! The Product Specification's colour tables (see [`crate::MessageCode::color_table`])
//! are 16-entry tables written against the 4-bit display categories of the
//! legacy graphic products. A Digital Radial Data Array (packet code 16)
//! instead carries 8-bit levels that decode to a physical value, so it needs a
//! *ramp* — a continuous mapping from value to colour — rather than a table.
//!
//! Two kinds of ramp are provided:
//!
//! - **Physical ramps** such as [`ColorRamp::NwsReflectivity`], whose domain is
//!   in real units (dBZ). Using one requires the product's level scaling to be
//!   known (see [`crate::LevelScaling`]).
//! - **Raw ramps** such as [`ColorRamp::Hue`] and [`ColorRamp::Grayscale`],
//!   whose domain is the raw 0-255 level itself. These work for any product but
//!   convey nothing about the underlying units.

use plotters::style::RGBColor;

/// A predefined colour ramp.
///
/// Pick one explicitly with [`crate::PlotOptions::ramp`], or leave it unset to
/// let [`ColorRamp::default_for_units`] choose.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum ColorRamp {
    /// A synthetic sweep around the hue wheel over the raw 0-255 level.
    ///
    /// This is the fallback for products whose units are unknown. It shows
    /// structure in the data but carries no physical meaning, and note that
    /// level 0 lands at the start of the wheel (red) rather than on a
    /// "no data" colour.
    #[default]
    Hue,

    /// A linear black-to-white ramp over the raw 0-255 level.
    Grayscale,

    /// The National Weather Service reflectivity ramp, in dBZ from -25 to +75.
    ///
    /// Transcribed from the legend of the reference product 32 plot in
    /// `data/sn_DC.radar_DS.32dhr_KMKX.png`; see [`NWS_REFLECTIVITY_STOPS`]
    /// for how it was recovered.
    NwsReflectivity,
}

impl ColorRamp {
    /// The ramp to use when the caller has not chosen one.
    ///
    /// Products that decode to dBZ get [`ColorRamp::NwsReflectivity`], since
    /// its domain matches theirs. Everything else falls back to
    /// [`ColorRamp::Hue`], which needs no units.
    pub fn default_for_units(units: Option<&str>) -> Self {
        match units {
            Some("dBZ") => ColorRamp::NwsReflectivity,
            _ => ColorRamp::default(),
        }
    }

    /// A short name suitable for a command line flag.
    pub fn name(&self) -> &'static str {
        match self {
            ColorRamp::Hue => "hue",
            ColorRamp::Grayscale => "grayscale",
            ColorRamp::NwsReflectivity => "nws-reflectivity",
        }
    }

    /// Every ramp, in the order they should be offered to a user.
    pub fn all() -> &'static [ColorRamp] {
        &[
            ColorRamp::Hue,
            ColorRamp::Grayscale,
            ColorRamp::NwsReflectivity,
        ]
    }

    /// Looks a ramp up by the name [`ColorRamp::name`] returns.
    pub fn from_name(name: &str) -> Option<Self> {
        let wanted = name.trim().to_ascii_lowercase();
        ColorRamp::all()
            .iter()
            .copied()
            .find(|r| r.name() == wanted)
    }

    /// The units this ramp's domain is expressed in, or `None` if its domain is
    /// the raw data level rather than a physical quantity.
    ///
    /// A ramp with units can only be used on a product whose level scaling is
    /// known and whose units match.
    pub fn units(&self) -> Option<&'static str> {
        match self {
            ColorRamp::Hue | ColorRamp::Grayscale => None,
            ColorRamp::NwsReflectivity => Some("dBZ"),
        }
    }

    /// The inclusive range this ramp covers, in [`ColorRamp::units`] (or in raw
    /// level for a ramp with no units).
    pub fn domain(&self) -> (f32, f32) {
        match self {
            ColorRamp::Hue | ColorRamp::Grayscale => (0.0, 255.0),
            ColorRamp::NwsReflectivity => (
                NWS_REFLECTIVITY_STOPS[0].0,
                NWS_REFLECTIVITY_STOPS[NWS_REFLECTIVITY_STOPS.len() - 1].0,
            ),
        }
    }

    /// The colour for `value`, clamped to the ramp's domain.
    pub fn color_at(&self, value: f32) -> RGBColor {
        let (lo, hi) = self.domain();
        let clamped = value.clamp(lo, hi);
        match self {
            ColorRamp::Hue => {
                // Matches the historical rendering: hue swept over 0-255.
                let (r, g, b) = hsl_to_rgb(clamped as f64 / 256.0, 0.5, 0.5);
                RGBColor(r, g, b)
            }
            ColorRamp::Grayscale => {
                let v = (clamped / hi * 255.0).round() as u8;
                RGBColor(v, v, v)
            }
            ColorRamp::NwsReflectivity => interpolate(&NWS_REFLECTIVITY_STOPS, clamped),
        }
    }

    /// The values to label on a colour bar for this ramp.
    ///
    /// Physical ramps label each of their stops; raw ramps label a handful of
    /// evenly spaced levels.
    pub fn ticks(&self) -> Vec<f32> {
        match self {
            ColorRamp::Hue | ColorRamp::Grayscale => {
                vec![0.0, 32.0, 64.0, 96.0, 128.0, 160.0, 192.0, 224.0, 255.0]
            }
            ColorRamp::NwsReflectivity => {
                NWS_REFLECTIVITY_STOPS.iter().map(|(v, ..)| *v).collect()
            }
        }
    }
}

impl std::fmt::Display for ColorRamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Piecewise-linear interpolation between `(value, r, g, b)` stops, which must
/// be sorted by value.
fn interpolate(stops: &[(f32, u8, u8, u8)], value: f32) -> RGBColor {
    let first = stops[0];
    let last = stops[stops.len() - 1];
    if value <= first.0 {
        return RGBColor(first.1, first.2, first.3);
    }
    if value >= last.0 {
        return RGBColor(last.1, last.2, last.3);
    }

    for pair in stops.windows(2) {
        let (lo_v, lr, lg, lb) = pair[0];
        let (hi_v, hr, hg, hb) = pair[1];
        if value >= lo_v && value <= hi_v {
            // Guard against duplicated stop values, which encode a hard step.
            if hi_v <= lo_v {
                return RGBColor(hr, hg, hb);
            }
            let t = (value - lo_v) / (hi_v - lo_v);
            let lerp = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t).round() as u8;
            return RGBColor(lerp(lr, hr), lerp(lg, hg), lerp(lb, hb));
        }
    }

    RGBColor(last.1, last.2, last.3)
}

/// Converts HSL to RGB, matching the behaviour of the `plotters` `HSLColor`
/// the plotter previously used directly.
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let h = h.rem_euclid(1.0);
    if s <= 0.0 {
        let v = (l * 255.0).round() as u8;
        return (v, v, v);
    }
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let channel = |mut t: f64| {
        t = t.rem_euclid(1.0);
        let v = if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 0.5 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        };
        (v * 255.0).round() as u8
    };
    (
        channel(h + 1.0 / 3.0),
        channel(h),
        channel(h - 1.0 / 3.0),
    )
}

/// The colour the NWS legend uses for the "range folded" data level.
///
/// Sampled from the RF swatch of the reference plot; it is the same dark purple
/// the Product Specification gives for the top level of the velocity tables.
pub const RANGE_FOLDED: RGBColor = RGBColor(0x77, 0x00, 0x7D);

/// Reflectivity colour stops in dBZ, recovered from the legend of the reference
/// product 32 plot in `data/sn_DC.radar_DS.32dhr_KMKX.png`.
///
/// That legend draws twenty 5 dBZ bands, each a linear gradient rather than a
/// flat swatch, separated by one-pixel black dividers. Each band was fitted
/// linearly down its centre column (worst residual 0.45/255, so the bands are
/// linear to well under one colour step) and extrapolated to its edges; the two
/// estimates meeting at each 5 dBZ boundary were then averaged, giving the 21
/// stops below.
///
/// Re-evaluating these stops against every gradient pixel of the original
/// legend gives a mean error of 2.8/255 and a maximum of 8.6/255, which
/// `tests/color_ramp_reference.rs` re-checks against the reference image.
pub const NWS_REFLECTIVITY_STOPS: [(f32, u8, u8, u8); 21] = [
    (-25.0, 0x02, 0x03, 0x03),
    (-20.0, 0x34, 0x52, 0x52),
    (-15.0, 0x68, 0x8D, 0x8D),
    (-10.0, 0x98, 0xCF, 0xCF),
    (-5.0, 0xAF, 0xEF, 0xEF),
    (0.0, 0xB7, 0xFE, 0xFE),
    (5.0, 0x00, 0xEA, 0xEB),
    (10.0, 0x00, 0x9B, 0xF6),
    (15.0, 0x00, 0x02, 0xF1),
    (20.0, 0x00, 0xFF, 0x00),
    (25.0, 0x00, 0xC5, 0x00),
    (30.0, 0x05, 0x91, 0x00),
    (35.0, 0xFF, 0xFF, 0x00),
    (40.0, 0xE7, 0xBE, 0x00),
    (45.0, 0xFF, 0x8C, 0x00),
    (50.0, 0xFE, 0x00, 0x00),
    (55.0, 0xD4, 0x00, 0x00),
    (60.0, 0xC0, 0x00, 0x05),
    (65.0, 0xFE, 0x02, 0xFF),
    (70.0, 0x98, 0x59, 0xC8),
    (75.0, 0xEE, 0xF1, 0xEC),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_ramp_round_trips_through_its_name() {
        for ramp in ColorRamp::all() {
            assert_eq!(ColorRamp::from_name(ramp.name()), Some(*ramp));
        }
        assert_eq!(ColorRamp::from_name("NWS-Reflectivity"), Some(ColorRamp::NwsReflectivity));
        assert_eq!(ColorRamp::from_name("nonsense"), None);
    }

    #[test]
    fn unset_ramp_defaults_by_units() {
        assert_eq!(
            ColorRamp::default_for_units(Some("dBZ")),
            ColorRamp::NwsReflectivity
        );
        // Anything else, including an unknown unit, gets the raw-level ramp.
        assert_eq!(ColorRamp::default_for_units(Some("kt")), ColorRamp::Hue);
        assert_eq!(ColorRamp::default_for_units(None), ColorRamp::Hue);
    }

    #[test]
    fn reflectivity_ramp_reports_dbz_and_its_domain() {
        let ramp = ColorRamp::NwsReflectivity;
        assert_eq!(ramp.units(), Some("dBZ"));
        assert_eq!(ramp.domain(), (-25.0, 75.0));
        assert_eq!(ramp.ticks().len(), 21);
    }

    #[test]
    fn raw_ramps_have_no_units_and_span_the_byte_range() {
        for ramp in [ColorRamp::Hue, ColorRamp::Grayscale] {
            assert_eq!(ramp.units(), None);
            assert_eq!(ramp.domain(), (0.0, 255.0));
        }
    }

    #[test]
    fn ramp_hits_its_stop_colors_exactly() {
        let ramp = ColorRamp::NwsReflectivity;
        for (value, r, g, b) in NWS_REFLECTIVITY_STOPS {
            assert_eq!(
                ramp.color_at(value),
                RGBColor(r, g, b),
                "stop at {value} dBZ should render exactly"
            );
        }
    }

    #[test]
    fn ramp_interpolates_between_stops() {
        // Halfway between -25 (2,3,3) and -20 (52,82,82): (27, 42.5, 42.5),
        // with the halves rounding away from zero.
        let mid = ColorRamp::NwsReflectivity.color_at(-22.5);
        assert_eq!(mid, RGBColor(27, 43, 43));
    }

    #[test]
    fn values_outside_the_domain_clamp_to_the_end_stops() {
        let ramp = ColorRamp::NwsReflectivity;
        assert_eq!(ramp.color_at(-100.0), ramp.color_at(-25.0));
        assert_eq!(ramp.color_at(1000.0), ramp.color_at(75.0));
        assert_eq!(ramp.color_at(f32::NEG_INFINITY), ramp.color_at(-25.0));
    }

    #[test]
    fn grayscale_spans_black_to_white() {
        let ramp = ColorRamp::Grayscale;
        assert_eq!(ramp.color_at(0.0), RGBColor(0, 0, 0));
        assert_eq!(ramp.color_at(255.0), RGBColor(255, 255, 255));
        assert_eq!(ramp.color_at(128.0), RGBColor(128, 128, 128));
    }

    /// The hue ramp must keep producing what the plotter produced before it was
    /// factored out, so existing output does not change.
    #[test]
    fn hue_ramp_matches_the_previous_hsl_rendering() {
        use plotters::style::Color as _;
        for level in [0u8, 1, 30, 120, 200, 255] {
            let expected = plotters::style::HSLColor(level as f64 / 256.0, 0.5, 0.5).rgb();
            let got = ColorRamp::Hue.color_at(level as f32);
            assert_eq!(
                (got.0, got.1, got.2),
                expected,
                "level {level} should match the original HSLColor"
            );
        }
    }

    #[test]
    fn ticks_stay_inside_the_domain() {
        for ramp in ColorRamp::all() {
            let (lo, hi) = ramp.domain();
            for tick in ramp.ticks() {
                assert!(
                    tick >= lo && tick <= hi,
                    "{ramp} tick {tick} outside domain {lo}..{hi}"
                );
            }
        }
    }
}
