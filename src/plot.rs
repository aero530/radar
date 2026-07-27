//! A renderer for a parsed [`Radar`]'s symbology block: radial data drawn as
//! filled wedges, alongside a metadata and colour bar panel.
//!
//! The layout follows the reference NWS plots — a square radar image on black,
//! with a light panel to its right carrying the product annotations and a
//! colour bar legend. Only the first symbology layer is drawn, and the graphic
//! and tabular alphanumeric blocks are not rendered.
//!
//! # Colouring
//!
//! The two radial packet types are coloured by different rules, because their
//! data levels mean different things:
//!
//! - **Radial Data (`AF1F`, Figure 3-10)** carries 4-bit levels, which are
//!   exactly the display categories the Product Specification's colour tables
//!   are written against. These go through
//!   [`crate::MessageCode::color_code`], so a product with no table renders
//!   entirely in [`crate::FALLBACK_GRAY`].
//!
//! - **Digital Radial Data Array (packet code 16, Figure 3-11c)** carries 8-bit
//!   levels that are *not* colour table indices. Where the product's threshold
//!   scaling is known ([`crate::LevelScaling`]) they are decoded to a physical
//!   value and passed through a [`ColorRamp`]; otherwise the raw level is fed
//!   to a ramp that needs no units. Either way these products render in colour
//!   whether or not the specification defines a table for them.
//!
//! Pick the ramp with [`PlotOptions::ramp`], or leave it unset to let
//! [`ColorRamp::default_for_units`] choose from the product's units.

use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use tracing::{debug, warn};

use crate::color_ramp::RANGE_FOLDED;
use crate::{
    error_r::Error, product_symbology::SymPacketData, ColorRamp, LevelThreshold, Qualifier, Radar,
};

/// Width in pixels of the annotation and legend panel.
const PANEL_WIDTH: u32 = 340;

/// Background of the radar image area.
const PLOT_BACKGROUND: RGBColor = RGBColor(0, 0, 0);
/// Background of the annotation panel, matching the reference plots.
const PANEL_BACKGROUND: RGBColor = RGBColor(220, 220, 220);

/// How to render a product.
///
/// `Default` gives the settings [`Radar::plot`] uses.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PlotOptions {
    /// Colour ramp for digital data arrays. When `None`, the ramp is chosen
    /// from the product's units by [`ColorRamp::default_for_units`].
    ///
    /// Ignored for `AF1F` products, which are coloured by the Product
    /// Specification's tables instead.
    pub ramp: Option<ColorRamp>,

    /// Label for the site line of the annotation panel, e.g.
    /// `"KMKX - MILWAUKEE, WI"`.
    ///
    /// The file only carries the station identifier, not a place name, so when
    /// this is `None` the panel shows the identifier from the text header on its
    /// own.
    pub site_label: Option<String>,

    /// Side length in pixels of the square radar image. The panel is added to
    /// its right, so the finished PNG is `image_size + 340` wide.
    pub image_size: u32,

    /// Whether to draw the annotation and legend panel at all.
    pub panel: bool,
}

impl PlotOptions {
    /// Options matching [`Radar::plot`]: automatic ramp, panel shown, 1200 px.
    pub fn new() -> Self {
        PlotOptions {
            ramp: None,
            site_label: None,
            image_size: 1200,
            panel: true,
        }
    }

    /// Sets the colour ramp used for digital data arrays.
    pub fn with_ramp(mut self, ramp: ColorRamp) -> Self {
        self.ramp = Some(ramp);
        self
    }

    /// Sets the site line of the annotation panel.
    pub fn with_site_label(mut self, label: impl Into<String>) -> Self {
        self.site_label = Some(label.into());
        self
    }

    /// Sets the side length of the square radar image.
    pub fn with_image_size(mut self, size: u32) -> Self {
        self.image_size = size;
        self
    }

    /// Turns the annotation and legend panel off, leaving just the radar image.
    pub fn without_panel(mut self) -> Self {
        self.panel = false;
        self
    }

    fn resolved_image_size(&self) -> u32 {
        self.image_size.max(200)
    }
}

/// Everything the annotation panel and colour bar need, worked out once.
struct Legend {
    /// Ramp for a digital data array, or `None` for a table-coloured product.
    ramp: Option<ColorRamp>,
    /// Units to title the colour bar with.
    units: String,
    /// Whether the product has a "range folded" level to show as a swatch.
    range_folded: bool,
}

impl Radar {
    /// Renders this product to `image.png` in the current directory using
    /// [`PlotOptions::new`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::NoSymbologyData`] if this product has no symbology
    /// block at all, or [`Error::NoSymbologyLayers`] if it has one but with
    /// zero layers.
    pub fn plot(&self) -> Result<(), Error> {
        self.plot_to("image.png")
    }

    /// Same as [`Radar::plot`], but writes the PNG to `path`.
    pub fn plot_to<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Error> {
        self.plot_with(path, &PlotOptions::new())
    }

    /// Renders this product to `path` with the given options.
    ///
    /// # Geometry
    ///
    /// Azimuths follow the ICD convention (Figure 3-10: "Scan is always in
    /// Clockwise direction") mapped to a north-up display: azimuth 0° points to
    /// the top of the image and angles increase clockwise, so 90° points right
    /// (east), 180° down (south), and 270° left (west). Range is drawn as a
    /// fraction of the image radius, not to a geographic scale — the packet's
    /// range scale factor and index of first range bin are not applied.
    pub fn plot_with<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        options: &PlotOptions,
    ) -> Result<(), Error> {
        let path = path.as_ref();
        let symbology = self.symbology.as_ref().ok_or(Error::NoSymbologyData)?;
        let first_layer = symbology.layers.first().ok_or(Error::NoSymbologyLayers)?;

        let plot_size = options.resolved_image_size();
        let panel_width = if options.panel { PANEL_WIDTH } else { 0 };
        let legend = self.legend_for(first_layer, options);

        let root = BitMapBackend::new(path, (plot_size + panel_width, plot_size))
            .into_drawing_area();
        root.fill(&PANEL_BACKGROUND)?;

        let (plot_area, panel_area) = root.split_horizontally(plot_size);
        plot_area.fill(&PLOT_BACKGROUND)?;

        self.draw_sweep(&plot_area, first_layer, plot_size, &legend)?;

        if options.panel {
            self.draw_panel(&panel_area, &legend, options)?;
        }

        root.present()?;
        Ok(())
    }

    /// Works out how the first layer will be coloured, and what the legend
    /// should therefore say.
    fn legend_for(&self, layer: &SymPacketData, options: &PlotOptions) -> Legend {
        let message_code = self.message_header.code;
        match layer {
            SymPacketData::DigitalRadialDataArray(_) => {
                let scaling = self.product_description.level_scaling();
                let units = scaling.as_ref().map(|s| s.units).unwrap_or("");

                // An explicitly chosen ramp wins, unless it needs units this
                // product cannot supply.
                let ramp = match options.ramp {
                    Some(chosen) => match chosen.units() {
                        Some(needed) if needed != units => {
                            warn!(
                                "Ramp {chosen} is defined in {needed} but {message_code:?} decodes \
                                 to {}; falling back to {}.",
                                if units.is_empty() { "no known units" } else { units },
                                ColorRamp::default_for_units(None)
                            );
                            ColorRamp::default_for_units(None)
                        }
                        _ => chosen,
                    },
                    None => ColorRamp::default_for_units(
                        Some(units).filter(|u| !u.is_empty()),
                    ),
                };

                let bar_units = match ramp.units() {
                    Some(u) => u.to_string(),
                    // A raw ramp is showing level codes, not a measurement.
                    None => "data level".to_string(),
                };

                Legend {
                    ramp: Some(ramp),
                    units: bar_units,
                    range_folded: scaling
                        .as_ref()
                        .and_then(|s| s.range_folded_level)
                        .is_some(),
                }
            }
            SymPacketData::RadialDataAF1F(_) => {
                if !message_code.has_color_table() {
                    warn!(
                        "No color table defined for {message_code:?}; every data level renders in \
                         the neutral gray fallback."
                    );
                }
                // A 16-level product's threshold halfwords say what each level
                // means, so the bar can be labelled in real units instead of by
                // level number.
                let scaling = self.product_description.level_scaling();
                let units = match scaling.as_ref().map(|s| s.units) {
                    Some(u) if !u.is_empty() => u.to_string(),
                    _ => "data level".to_string(),
                };
                Legend {
                    ramp: None,
                    units,
                    range_folded: false,
                }
            }
            _ => Legend {
                ramp: None,
                units: String::new(),
                range_folded: false,
            },
        }
    }

    /// Draws the radial sweep into `area`.
    fn draw_sweep<DB: DrawingBackend>(
        &self,
        area: &DrawingArea<DB, plotters::coord::Shift>,
        layer: &SymPacketData,
        plot_size: u32,
        legend: &Legend,
    ) -> Result<(), Error>
    where
        Error: From<plotters::drawing::DrawingAreaErrorKind<DB::ErrorType>>,
    {
        let logical = plot_size as f32;
        // Leave a small margin so the outermost bins are not clipped.
        let r_max = logical * 0.458;
        let xc = logical / 2.0;
        let yc = logical / 2.0;

        let area = area.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
            0f32..logical,
            0f32..logical,
            (0..plot_size as i32, 0..plot_size as i32),
        ));

        let n_bins = layer.num_bins() as f32;
        if n_bins <= 0.0 {
            warn!("First symbology layer reports no range bins; nothing to draw");
            return Ok(());
        }

        // Azimuth 0 points up and angles increase clockwise (Figure 3-10), which
        // in this y-down coordinate space means starting a quarter turn back.
        let to_radians = |tenths_of_a_degree: i16| {
            (270.0 + tenths_of_a_degree as f32 / 10.0) * std::f32::consts::PI / 180.0
        };

        let wedge = |angle: f32, delta: f32, inner: f32, outer: f32, color: RGBColor| {
            let radius_inner = inner / n_bins * r_max;
            let radius_outer = outer / n_bins * r_max;
            let points = vec![
                (radius_inner * angle.cos() + xc, radius_inner * angle.sin() + yc),
                (
                    radius_inner * (angle + delta).cos() + xc,
                    radius_inner * (angle + delta).sin() + yc,
                ),
                (
                    radius_outer * (angle + delta).cos() + xc,
                    radius_outer * (angle + delta).sin() + yc,
                ),
                (radius_outer * angle.cos() + xc, radius_outer * angle.sin() + yc),
            ];
            let _ = area.draw(&Polygon::new(points, color.filled()));
        };

        match layer {
            SymPacketData::RadialDataAF1F(packet) => {
                let message_code = self.message_header.code;
                for radial in &packet.radials {
                    let angle = to_radians(radial.header.angle_start);
                    let delta = (radial.header.angle_delta as f32 / 10.0)
                        * std::f32::consts::PI
                        / 180.0;

                    // Accumulate in u32: a radial's runs sum to its bin count,
                    // which Figure 3-10 allows up to 460 — well past u8::MAX.
                    let mut start: u32 = 0;
                    for run in &radial.data {
                        let end = start + run.run as u32;
                        // 4-bit levels are colour table categories, so these
                        // honour the Product Specification table.
                        wedge(
                            angle,
                            delta,
                            start as f32,
                            end as f32,
                            message_code.color_code(run.color),
                        );
                        start = end;
                    }
                }
            }

            SymPacketData::DigitalRadialDataArray(packet) => {
                let ramp = legend.ramp.unwrap_or_default();
                let scaling = self.product_description.level_scaling();

                // 8-bit levels are not colour table indices. A ramp with units
                // is fed the decoded physical value; a ramp without units is
                // defined over the raw level itself, so it must be fed the raw
                // level rather than a measurement in some other quantity.
                // `None` means "don't draw", leaving the background showing.
                let color_for = |level: u8| -> Option<RGBColor> {
                    if let Some(scaling) = scaling.as_ref() {
                        if scaling.is_range_folded(level) {
                            return Some(RANGE_FOLDED);
                        }
                        if level < scaling.first_data_level {
                            return None; // below threshold
                        }
                        if ramp.units().is_some() {
                            return Some(ramp.color_at(scaling.value(level)?));
                        }
                    }
                    Some(ramp.color_at(level as f32))
                };

                for radial in &packet.radials {
                    let angle = to_radians(radial.header.angle_start);
                    let delta = (radial.header.angle_delta as f32 / 10.0)
                        * std::f32::consts::PI
                        / 180.0;

                    for (bin, level) in radial.data.iter().enumerate() {
                        let Some(color) = color_for(*level) else {
                            continue;
                        };
                        wedge(angle, delta, bin as f32, bin as f32 + 1.0, color);
                    }
                }
            }

            other => {
                warn!("First symbology layer is {other:?}, which the plotter cannot draw");
            }
        }

        Ok(())
    }

    /// Draws the annotation text and colour bar into `area`.
    fn draw_panel<DB: DrawingBackend>(
        &self,
        area: &DrawingArea<DB, plotters::coord::Shift>,
        legend: &Legend,
        options: &PlotOptions,
    ) -> Result<(), Error>
    where
        Error: From<plotters::drawing::DrawingAreaErrorKind<DB::ErrorType>>,
    {
        let (width, height) = area.dim_in_pixel();
        let text = ("sans-serif", 18).into_font().color(&BLACK);
        let mono = ("monospace", 15).into_font().color(&BLACK);

        let left = 16i32;
        let mut y = 20i32;
        let line = 23i32;

        for annotation in self.annotations(options) {
            if annotation.is_empty() {
                y += line / 2;
                continue;
            }
            area.draw(&Text::new(annotation, (left, y), text.clone()))?;
            y += line;
        }

        // Colour bar, sized to whatever vertical space is left over. Reserve
        // room underneath for the range-folded swatch when there is one, so it
        // cannot be pushed off the bottom edge.
        const RF_SWATCH_HEIGHT: i32 = 20;
        const RF_SWATCH_GAP: i32 = 16;
        let reserved_below = if legend.range_folded {
            RF_SWATCH_GAP + RF_SWATCH_HEIGHT + 20
        } else {
            24
        };
        let bar_top = y + 34;
        let bar_bottom = (height as i32 - reserved_below).max(bar_top + 60);
        let bar_left = left + 6;
        let bar_right = bar_left + 46;

        if let Some(ramp) = legend.ramp {
            area.draw(&Text::new(
                format!("Legend: {}", legend.units),
                (left + 8, bar_top - 22),
                text.clone(),
            ))?;
            self.draw_color_bar(
                area,
                ramp,
                (bar_left, bar_top),
                (bar_right, bar_bottom),
                &mono,
            )?;

            if legend.range_folded {
                let rf_top = bar_bottom + RF_SWATCH_GAP;
                let rf_bottom = rf_top + RF_SWATCH_HEIGHT;
                area.draw(&Rectangle::new(
                    [(bar_left, rf_top), (bar_right, rf_bottom)],
                    RANGE_FOLDED.filled(),
                ))?;
                area.draw(&Rectangle::new(
                    [(bar_left, rf_top), (bar_right, rf_bottom)],
                    BLACK.stroke_width(1),
                ))?;
                area.draw(&Text::new(
                    "RF".to_string(),
                    (bar_right + 12, rf_top + RF_SWATCH_HEIGHT / 2 - 8),
                    mono.clone(),
                ))?;
            }
        } else if self.message_header.code.has_color_table() {
            // Table-coloured products get a discrete swatch per data level.
            area.draw(&Text::new(
                format!("Legend: {}", legend.units),
                (left + 8, bar_top - 22),
                text.clone(),
            ))?;
            self.draw_table_swatches(area, (bar_left, bar_top), (bar_right, bar_bottom), &mono)?;
        }

        debug!("Annotation panel drawn at {width}x{height}");
        Ok(())
    }

    /// Draws a continuous ramp as a vertical bar with tick labels, highest value
    /// at the top.
    fn draw_color_bar<DB: DrawingBackend>(
        &self,
        area: &DrawingArea<DB, plotters::coord::Shift>,
        ramp: ColorRamp,
        top_left: (i32, i32),
        bottom_right: (i32, i32),
        label: &TextStyle<'_>,
    ) -> Result<(), Error>
    where
        Error: From<plotters::drawing::DrawingAreaErrorKind<DB::ErrorType>>,
    {
        let (x0, y0) = top_left;
        let (x1, y1) = bottom_right;
        let (lo, hi) = ramp.domain();
        let span = (hi - lo).max(f32::EPSILON);
        let rows = (y1 - y0).max(1);

        // One filled row per pixel gives a smooth gradient.
        for row in 0..rows {
            let t = row as f32 / rows as f32;
            let value = hi - t * span; // top of the bar is the high end
            area.draw(&Rectangle::new(
                [(x0, y0 + row), (x1, y0 + row + 1)],
                ramp.color_at(value).filled(),
            ))?;
        }
        area.draw(&Rectangle::new(
            [(x0, y0), (x1, y1)],
            BLACK.stroke_width(1),
        ))?;

        // Tick labels, thinned out if the bar is too short to fit them all.
        let ticks = ramp.ticks();
        let min_spacing = 13;
        let step = ticks
            .len()
            .div_ceil(((rows / min_spacing).max(1) as usize).max(1))
            .max(1);
        for (i, tick) in ticks.iter().enumerate() {
            let t = (hi - *tick) / span;
            let y = y0 + (t * rows as f32).round() as i32;

            // Separate the bands, as the reference legends do, so each interval
            // between ticks reads as its own step.
            if y > y0 && y < y1 {
                area.draw(&PathElement::new(
                    vec![(x0, y), (x1, y)],
                    BLACK.stroke_width(1),
                ))?;
            }

            if i % step != 0 && i != ticks.len() - 1 {
                continue;
            }
            area.draw(&PathElement::new(
                vec![(x1, y), (x1 + 5, y)],
                BLACK.stroke_width(1),
            ))?;
            area.draw(&Text::new(
                format_tick(*tick),
                (x1 + 9, y - 7),
                label.clone(),
            ))?;
        }
        Ok(())
    }

    /// Draws one swatch per data level for a table-coloured product.
    fn draw_table_swatches<DB: DrawingBackend>(
        &self,
        area: &DrawingArea<DB, plotters::coord::Shift>,
        top_left: (i32, i32),
        bottom_right: (i32, i32),
        label: &TextStyle<'_>,
    ) -> Result<(), Error>
    where
        Error: From<plotters::drawing::DrawingAreaErrorKind<DB::ErrorType>>,
    {
        let Some(table) = self.message_header.code.color_table() else {
            return Ok(());
        };
        let codes: Vec<u8> = table.level_codes().collect();
        if codes.is_empty() {
            return Ok(());
        }

        let (x0, y0) = top_left;
        let (x1, y1) = bottom_right;
        // Highest level at the top, to match the ramp orientation.
        let swatch_h = ((y1 - y0) / codes.len() as i32).clamp(8, 22);

        for (i, code) in codes.iter().rev().enumerate() {
            let top = y0 + i as i32 * swatch_h;
            let color = table.color(*code).unwrap_or(crate::FALLBACK_GRAY);
            area.draw(&Rectangle::new(
                [(x0, top), (x1, top + swatch_h)],
                color.filled(),
            ))?;
            area.draw(&Rectangle::new(
                [(x0, top), (x1, top + swatch_h)],
                BLACK.stroke_width(1),
            ))?;
            area.draw(&Text::new(
                self.level_label(*code),
                (x1 + 9, top + swatch_h / 2 - 7),
                label.clone(),
            ))?;
        }
        Ok(())
    }

    /// How to label one data level of a table-coloured product.
    ///
    /// The threshold halfwords say what each level means, so a level shows its
    /// physical threshold (with the qualifier the flags carried) or its category
    /// abbreviation. Products whose thresholds this crate cannot decode fall
    /// back to the bare level number.
    fn level_label(&self, level: u8) -> String {
        let Some(threshold) = self
            .product_description
            .level_scaling()
            .and_then(|s| s.threshold(level))
        else {
            return level.to_string();
        };

        match threshold {
            LevelThreshold::Code(code) => code.abbreviation().to_string(),
            LevelThreshold::Value { value, qualifier } => {
                let magnitude = if (value - value.round()).abs() < 0.05 {
                    format!("{}", value.round().abs() as i64)
                } else {
                    format!("{:.1}", value.abs())
                };
                match qualifier {
                    Some(Qualifier::Minus) => format!("-{magnitude}"),
                    Some(Qualifier::Plus) => format!("+{magnitude}"),
                    Some(Qualifier::GreaterThan) => format!(">{magnitude}"),
                    Some(Qualifier::LessThan) => format!("<{magnitude}"),
                    None => magnitude,
                }
            }
        }
    }

    /// The annotation lines for the panel, in order. An empty string is a gap.
    fn annotations(&self, options: &PlotOptions) -> Vec<String> {
        let pd = &self.product_description;
        let mut lines = vec![
            "NEXRAD LEVEL-III".to_string(),
            self.message_header.code.to_string().to_uppercase(),
            options
                .site_label
                .clone()
                .unwrap_or_else(|| self.text_header.location.clone()),
        ];

        if let Some(product) = format_modified_julian(pd.product_date, pd.product_time) {
            lines.push(format!("{product} Z"));
        }
        if let Some(volume) = format_modified_julian(pd.vol_scan_date, pd.vol_scan_time) {
            lines.push(format!("{volume} Z (VOL)"));
        }

        lines.push(format!(
            "LAT: {} {}",
            format_dms(pd.latitude as f32 / 1000.0),
            if pd.latitude >= 0 { "N" } else { "S" }
        ));
        lines.push(format!(
            "LON: {} {}",
            format_dms(pd.longitude as f32 / 1000.0),
            if pd.longitude >= 0 { "E" } else { "W" }
        ));
        lines.push(format!("ELEV: {} FT", pd.height));
        lines.push(format!(
            "MODE/VCP: {} / {}",
            weather_mode_letter(pd.operational_mode),
            pd.vcp
        ));

        if let Some((max, units)) = pd.max_value_annotation() {
            lines.push(String::new());
            lines.push(if units.is_empty() {
                format!("MAX: {max}")
            } else {
                format!("MAX: {max} {units}")
            });
        }

        lines
    }
}

/// Formats a tick value, dropping the decimal point when it is a whole number.
fn format_tick(value: f32) -> String {
    if (value - value.round()).abs() < 0.05 {
        format!("{}", value.round() as i32)
    } else {
        format!("{value:.1}")
    }
}

/// Formats decimal degrees as the `DD/MM/SS` the reference plots use.
fn format_dms(degrees: f32) -> String {
    let total = degrees.abs();
    let d = total.floor();
    let minutes = (total - d) * 60.0;
    let m = minutes.floor();
    let s = ((minutes - m) * 60.0).round();
    // Rounding seconds can carry into minutes, and minutes into degrees.
    let (d, m, s) = if s >= 60.0 { (d, m + 1.0, 0.0) } else { (d, m, s) };
    let (d, m) = if m >= 60.0 { (d + 1.0, 0.0) } else { (d, m) };
    format!("{:.0}/{:02.0}/{:02.0}", d, m, s)
}

/// The single letter the reference plots use for the weather mode.
///
/// The Product Description Block stores 0 = Maintenance, 1 = Clean Air,
/// 2 = Precipitation (Figure 3-6 sheet 6), while the alphanumeric products
/// render the mode as "A, B, or M" (Product Specification tabular tables). The
/// bundled product 32 fixture has mode 2 with VCP 212 — a precipitation VCP —
/// and its reference plot prints `MODE/VCP: A / 212`, which fixes A to
/// precipitation and leaves B for clear air.
fn weather_mode_letter(operational_mode: i16) -> &'static str {
    match operational_mode {
        0 => "M",
        1 => "B",
        2 => "A",
        _ => "?",
    }
}

/// Formats a Modified Julian date and seconds-after-midnight as
/// `MM/DD/YYYY HH:MM:SS`.
///
/// The date is 1-based ("where 1=1 January 1970", Figure 3-3), matching
/// [`crate::message_header`].
fn format_modified_julian(date: i16, seconds: i32) -> Option<String> {
    if date <= 0 {
        return None;
    }
    let timestamp = (date as i64 - 1) * 86_400 + seconds as i64;
    let datetime = chrono::DateTime::from_timestamp(timestamp, 0)?;
    Some(datetime.format("%m/%d/%Y %H:%M:%S").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_degrees_as_the_reference_plots_do() {
        // The bundled product 32 fixture: 42.968 N, 88.551 W, which the
        // reference plot prints as LAT: 42/58/04 N and LON: 88/33/03 W.
        assert_eq!(format_dms(42.968), "42/58/05");
        assert_eq!(format_dms(88.551), "88/33/04");
        assert_eq!(format_dms(0.0), "0/00/00");
    }

    #[test]
    fn seconds_rounding_carries_into_minutes_and_degrees() {
        // 41.99999 deg is 41/59/60 before carrying, which must become 42/00/00.
        assert_eq!(format_dms(41.999_999), "42/00/00");
    }

    #[test]
    fn weather_mode_letters_follow_the_reference_plot() {
        assert_eq!(weather_mode_letter(2), "A"); // precipitation, as in the fixture
        assert_eq!(weather_mode_letter(1), "B"); // clean air
        assert_eq!(weather_mode_letter(0), "M"); // maintenance
        assert_eq!(weather_mode_letter(9), "?");
    }

    #[test]
    fn formats_the_reference_timestamps() {
        // The fixture's product time: date 19830, 77733 s -> 21:35:33 on
        // 04/16/2024, exactly what the reference plot prints.
        assert_eq!(
            format_modified_julian(19830, 77733).unwrap(),
            "04/16/2024 21:35:33"
        );
        // And its volume scan time.
        assert_eq!(
            format_modified_julian(19830, 77654).unwrap(),
            "04/16/2024 21:34:14"
        );
        assert!(format_modified_julian(0, 0).is_none());
    }

    #[test]
    fn ticks_render_without_a_trailing_decimal_when_whole() {
        assert_eq!(format_tick(-25.0), "-25");
        assert_eq!(format_tick(0.0), "0");
        assert_eq!(format_tick(2.5), "2.5");
    }

    /// The annotation lines for the reference product must match what the
    /// reference plot prints, line for line.
    #[test]
    fn annotations_match_the_reference_plot() {
        let file = include_bytes!("../data/sn_DC.radar_DS.32dhr_KMKX.last").to_vec();
        let (_, radar) = Radar::from_vec(file).expect("fixture should parse");

        let options = PlotOptions::new().with_site_label("KMKX - MILWAUKEE, WI");
        let lines = radar.annotations(&options);

        assert_eq!(lines[0], "NEXRAD LEVEL-III");
        assert_eq!(lines[1], "DIGITAL HYBRID SCAN REFLECTIVITY");
        assert_eq!(lines[2], "KMKX - MILWAUKEE, WI");
        assert_eq!(lines[3], "04/16/2024 21:35:33 Z");
        assert_eq!(lines[4], "04/16/2024 21:34:14 Z (VOL)");
        assert_eq!(lines[5], "LAT: 42/58/05 N");
        assert_eq!(lines[6], "LON: 88/33/04 W");
        assert_eq!(lines[7], "ELEV: 1022 FT");
        assert_eq!(lines[8], "MODE/VCP: A / 212");
        assert_eq!(lines[9], "");
        assert_eq!(lines[10], "MAX: 56 DBZ");
    }

    /// Without a caller-supplied label the panel falls back to the station
    /// identifier, since the file carries no place name.
    #[test]
    fn site_line_defaults_to_the_station_identifier() {
        let file = include_bytes!("../data/sn_DC.radar_DS.32dhr_KMKX.last").to_vec();
        let (_, radar) = Radar::from_vec(file).unwrap();
        let lines = radar.annotations(&PlotOptions::new());
        assert_eq!(lines[2], "KMKX");
    }

    fn load(bytes: &[u8]) -> Radar {
        Radar::from_vec(bytes.to_vec()).expect("fixture should parse").1
    }

    fn legend_of(radar: &Radar, options: &PlotOptions) -> Legend {
        let layer = radar
            .symbology
            .as_ref()
            .unwrap()
            .layers
            .first()
            .expect("fixture should have a layer");
        radar.legend_for(layer, options)
    }

    /// A dBZ product with no ramp chosen picks up the reflectivity ramp, and its
    /// legend is labelled in dBZ.
    #[test]
    fn dbz_product_defaults_to_the_reflectivity_ramp() {
        let radar = load(include_bytes!("../data/sn_DC.radar_DS.32dhr_KMKX.last"));
        let legend = legend_of(&radar, &PlotOptions::new());

        assert_eq!(legend.ramp, Some(ColorRamp::NwsReflectivity));
        assert_eq!(legend.units, "dBZ");
        // Product 32 defines a range-folded level, so the swatch is shown.
        assert!(legend.range_folded);
    }

    /// An explicitly chosen unitless ramp is honoured, and the bar is then
    /// labelled as showing raw levels rather than a measurement.
    #[test]
    fn an_explicit_raw_ramp_overrides_the_default() {
        let radar = load(include_bytes!("../data/sn_DC.radar_DS.32dhr_KMKX.last"));
        let options = PlotOptions::new().with_ramp(ColorRamp::Grayscale);
        let legend = legend_of(&radar, &options);

        assert_eq!(legend.ramp, Some(ColorRamp::Grayscale));
        assert_eq!(legend.units, "data level");
    }

    /// Asking for a ramp whose units the product cannot supply must fall back
    /// rather than plot knots against a dBZ scale. Product 99 decodes to knots.
    #[test]
    fn a_ramp_with_mismatched_units_falls_back() {
        let radar = load(include_bytes!("../data/sn_DC.radar_DS.p99v0_KMKX.last"));
        assert_eq!(
            radar.product_description.level_scaling().unwrap().units,
            "kt"
        );

        let options = PlotOptions::new().with_ramp(ColorRamp::NwsReflectivity);
        let legend = legend_of(&radar, &options);

        assert_eq!(legend.ramp, Some(ColorRamp::Hue));
        assert_eq!(legend.units, "data level");
    }

    /// A knots product has no ramp defined in those units, so it also falls back
    /// to the unitless default rather than mislabelling the bar.
    #[test]
    fn a_product_without_a_matching_ramp_uses_the_unitless_default() {
        let radar = load(include_bytes!("../data/sn_DC.radar_DS.p99v0_KMKX.last"));
        let legend = legend_of(&radar, &PlotOptions::new());

        assert_eq!(legend.ramp, Some(ColorRamp::Hue));
        assert_eq!(legend.units, "data level");
    }

    /// An `AF1F` product is coloured by its Product Specification table, so no
    /// ramp is involved and the chosen ramp is ignored.
    #[test]
    fn table_coloured_products_use_no_ramp() {
        let radar = load(include_bytes!("../data/sn_DC.radar_DS.56rm1_KMKX.last"));
        let options = PlotOptions::new().with_ramp(ColorRamp::Grayscale);
        let legend = legend_of(&radar, &options);

        assert_eq!(legend.ramp, None);
        assert!(!legend.range_folded);
    }

    #[test]
    fn options_builder_sets_each_field() {
        let options = PlotOptions::new()
            .with_ramp(ColorRamp::Grayscale)
            .with_site_label("KMKX - MILWAUKEE, WI")
            .with_image_size(600)
            .without_panel();

        assert_eq!(options.ramp, Some(ColorRamp::Grayscale));
        assert_eq!(options.site_label.as_deref(), Some("KMKX - MILWAUKEE, WI"));
        assert_eq!(options.image_size, 600);
        assert!(!options.panel);
    }

    /// A tiny image size must not produce a degenerate canvas.
    #[test]
    fn image_size_has_a_floor() {
        assert_eq!(PlotOptions::new().with_image_size(1).resolved_image_size(), 200);
    }

    /// A 16-level product's legend is labelled from its decoded threshold
    /// halfwords, so it reads in real units instead of by level number.
    #[test]
    fn table_legend_labels_come_from_the_decoded_thresholds() {
        let radar = load(include_bytes!("../data/sn_DC.radar_DS.56rm1_KMKX.last"));

        // Product 56's table: ND at level 0, -80 to +80 kt, RF at level 15.
        assert_eq!(radar.level_label(0), "ND");
        assert_eq!(radar.level_label(1), "-80");
        assert_eq!(radar.level_label(7), "-1");
        assert_eq!(radar.level_label(8), "0");
        assert_eq!(radar.level_label(9), "+10");
        assert_eq!(radar.level_label(14), "+80");
        assert_eq!(radar.level_label(15), "RF");

        // And the bar is titled in the product's units rather than "data level".
        let legend = legend_of(&radar, &PlotOptions::new());
        assert_eq!(legend.units, "kt");
    }

    #[test]
    fn base_reflectivity_legend_is_labelled_in_dbz() {
        let radar = load(include_bytes!("../data/sn_DS.p20-r_kmkx.last"));

        assert_eq!(radar.level_label(0), "ND");
        assert_eq!(radar.level_label(1), "-28");
        assert_eq!(radar.level_label(8), "0");
        assert_eq!(radar.level_label(15), "+28");

        let legend = legend_of(&radar, &PlotOptions::new());
        assert_eq!(legend.units, "dBZ");
    }

    /// A product whose thresholds cannot be decoded falls back to the bare level
    /// number rather than inventing a label.
    #[test]
    fn level_labels_fall_back_to_the_level_number() {
        let mut radar = load(include_bytes!("../data/sn_DS.p20-r_kmkx.last"));
        // Product 153 is in Note 1's exception list, so nothing decodes.
        radar.product_description.product_code = 153;
        assert_eq!(radar.level_label(7), "7");
    }
}
