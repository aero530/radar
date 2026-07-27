//! Decoding a product's raw data levels into physical values.
//!
//! Turning a data level code into a measurement takes the product-dependent
//! threshold information in halfwords 31-46 of the Product Description Block.
//! Note 1 of Figure 3-6 defines four different encodings, and which one applies
//! depends on the product:
//!
//! | Encoding | Products | Modelled by |
//! | -- | -- | -- |
//! | Packed flag/value threshold halfwords | everything not listed below | [`LevelDecoding::Thresholds`] |
//! | Signed scale and offset in tenths or hundredths | 32, 94, 99, 138, 182, 186 | [`LevelDecoding::Linear`] |
//! | IEEE float scale and offset, `F = (N - OFFSET) / SCALE` | 159, 161, 163, 167, 168, 170, 172-176 | [`LevelDecoding::Linear`] |
//! | Modified 16-bit float, linear below a threshold and logarithmic above | 134 | [`LevelDecoding::LinearThenLog`] |
//! | Data/topped masks read from halfwords 31-34 | 135 | [`LevelDecoding::EnhancedEchoTops`] |
//!
//! The formulas are taken from Note 1 itself; where the vendored Py-ART
//! reference (`nexrad_level3.py`) implements the same product its reading
//! agrees, and it is noted where this goes further than Py-ART does.

use crate::product_description::ProductDescription;

/// A qualifier a threshold's flag byte can attach to its value (Note 1 of
/// Figure 3-6, bits 4 to 7).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Qualifier {
    /// Bit 4: the value is a lower bound.
    GreaterThan,
    /// Bit 5: the value is an upper bound.
    LessThan,
    /// Bit 6.
    Plus,
    /// Bit 7: the value is negative.
    Minus,
}

/// A categorical threshold code, used when a threshold's flag byte has bit 0
/// set (Note 1 of Figure 3-6).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ThresholdCode {
    Blank,
    /// Below threshold.
    BelowThreshold,
    /// No data.
    NoData,
    /// Range folded.
    RangeFolded,
    /// Biological.
    Biological,
    /// AP / ground clutter.
    GroundClutter,
    IceCrystals,
    Graupel,
    WetSnow,
    DrySnow,
    /// Light and moderate rain.
    Rain,
    HeavyRain,
    BigDrops,
    /// Hail and rain mixed.
    HailAndRain,
    Unknown,
    LargeHail,
    GiantHail,
    /// A code outside the documented 0 to 16.
    Other(u8),
}

impl ThresholdCode {
    fn from_byte(code: u8) -> Self {
        match code {
            0 => ThresholdCode::Blank,
            1 => ThresholdCode::BelowThreshold,
            2 => ThresholdCode::NoData,
            3 => ThresholdCode::RangeFolded,
            4 => ThresholdCode::Biological,
            5 => ThresholdCode::GroundClutter,
            6 => ThresholdCode::IceCrystals,
            7 => ThresholdCode::Graupel,
            8 => ThresholdCode::WetSnow,
            9 => ThresholdCode::DrySnow,
            10 => ThresholdCode::Rain,
            11 => ThresholdCode::HeavyRain,
            12 => ThresholdCode::BigDrops,
            13 => ThresholdCode::HailAndRain,
            14 => ThresholdCode::Unknown,
            15 => ThresholdCode::LargeHail,
            16 => ThresholdCode::GiantHail,
            other => ThresholdCode::Other(other),
        }
    }

    /// The two-letter abbreviation the ICD gives this code.
    pub fn abbreviation(&self) -> &'static str {
        match self {
            ThresholdCode::Blank => "BLANK",
            ThresholdCode::BelowThreshold => "TH",
            ThresholdCode::NoData => "ND",
            ThresholdCode::RangeFolded => "RF",
            ThresholdCode::Biological => "BI",
            ThresholdCode::GroundClutter => "GC",
            ThresholdCode::IceCrystals => "IC",
            ThresholdCode::Graupel => "GR",
            ThresholdCode::WetSnow => "WS",
            ThresholdCode::DrySnow => "DS",
            ThresholdCode::Rain => "RA",
            ThresholdCode::HeavyRain => "HR",
            ThresholdCode::BigDrops => "BD",
            ThresholdCode::HailAndRain => "HA",
            ThresholdCode::Unknown => "UK",
            ThresholdCode::LargeHail => "LH",
            ThresholdCode::GiantHail => "GH",
            ThresholdCode::Other(_) => "??",
        }
    }
}

/// One decoded entry of a packed threshold table.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LevelThreshold {
    /// A numeric threshold, already scaled and signed.
    Value {
        value: f32,
        /// The qualifier the flag byte attached, if any.
        qualifier: Option<Qualifier>,
    },
    /// A categorical code rather than a measurement.
    Code(ThresholdCode),
}

/// How a product's raw levels map to values.
#[derive(Clone, Debug, PartialEq)]
pub enum LevelDecoding {
    /// `value = raw * scale + offset`.
    Linear { scale: f32, offset: f32 },
    /// One threshold per data level, from the packed flag/value halfwords.
    Thresholds(Vec<LevelThreshold>),
    /// Linear below `log_start`, logarithmic at or above it, as product 134
    /// (High Resolution VIL) requires.
    LinearThenLog {
        linear_scale: f32,
        linear_offset: f32,
        log_start: i16,
        log_scale: f32,
        log_offset: f32,
    },
    /// Product 135's packed altitude plus "topped" flag, using the masks the
    /// product itself carries in halfwords 31-34.
    EnhancedEchoTops {
        data_mask: u8,
        data_scale: f32,
        data_offset: f32,
        topped_mask: u8,
    },
}

/// How to turn a raw data level into a physical value, for one product.
#[derive(Clone, Debug, PartialEq)]
pub struct LevelScaling {
    decoding: LevelDecoding,
    /// The units the decoded value is in, e.g. `"dBZ"`. Empty when the quantity
    /// is dimensionless or this crate does not know it.
    pub units: &'static str,
    /// Levels below this are status flags rather than measurements.
    pub first_data_level: u8,
    /// The level meaning "range folded", when the product defines one.
    pub range_folded_level: Option<u8>,
}

impl LevelScaling {
    /// The rule this product's levels are decoded by.
    pub fn decoding(&self) -> &LevelDecoding {
        &self.decoding
    }

    /// The scale and offset, when this product decodes by a plain linear
    /// relationship.
    pub fn linear_params(&self) -> Option<(f32, f32)> {
        match self.decoding {
            LevelDecoding::Linear { scale, offset } => Some((scale, offset)),
            _ => None,
        }
    }

    /// Decodes a raw level, returning `None` for levels that are flags rather
    /// than measurements.
    ///
    /// Also `None` if the arithmetic does not produce a finite number, which the
    /// logarithmic branch of product 134 can do for coefficients that overflow.
    pub fn value(&self, raw: u8) -> Option<f32> {
        self.raw_value(raw).filter(|v| v.is_finite())
    }

    fn raw_value(&self, raw: u8) -> Option<f32> {
        match &self.decoding {
            LevelDecoding::Linear { scale, offset } => {
                if raw < self.first_data_level {
                    return None;
                }
                Some(raw as f32 * scale + offset)
            }

            LevelDecoding::Thresholds(thresholds) => match thresholds.get(raw as usize)? {
                LevelThreshold::Value { value, .. } => Some(*value),
                LevelThreshold::Code(_) => None,
            },

            LevelDecoding::LinearThenLog {
                linear_scale,
                linear_offset,
                log_start,
                log_scale,
                log_offset,
            } => {
                if raw < self.first_data_level {
                    return None;
                }
                if (raw as i16) < *log_start {
                    if *linear_scale == 0.0 {
                        return None;
                    }
                    Some((raw as f32 - linear_offset) / linear_scale)
                } else {
                    if *log_scale == 0.0 {
                        return None;
                    }
                    Some(((raw as f32 - log_offset) / log_scale).exp())
                }
            }

            LevelDecoding::EnhancedEchoTops {
                data_mask,
                data_scale,
                data_offset,
                ..
            } => {
                // Note 1 spells this out: 0 is below threshold, 1 is bad data,
                // and otherwise the altitude is the masked bits scaled and
                // offset.
                if raw < self.first_data_level {
                    return None;
                }
                if *data_scale == 0.0 {
                    return None;
                }
                Some((raw & data_mask) as f32 / data_scale - data_offset)
            }
        }
    }

    /// The decoded threshold for a level, for products that carry a packed
    /// threshold table. This exposes the categorical codes that [`Self::value`]
    /// reports as `None`.
    pub fn threshold(&self, raw: u8) -> Option<LevelThreshold> {
        match &self.decoding {
            LevelDecoding::Thresholds(thresholds) => thresholds.get(raw as usize).copied(),
            _ => None,
        }
    }

    /// Whether a level carries product 135's "topped" flag, meaning the echo top
    /// reached the highest elevation scanned.
    ///
    /// `None` for every other product.
    pub fn is_topped(&self, raw: u8) -> Option<bool> {
        match self.decoding {
            LevelDecoding::EnhancedEchoTops { topped_mask, .. } => {
                if raw < self.first_data_level {
                    None
                } else {
                    Some(raw & topped_mask != 0)
                }
            }
            _ => None,
        }
    }

    /// Whether `raw` is this product's "range folded" flag.
    pub fn is_range_folded(&self, raw: u8) -> bool {
        if self.range_folded_level == Some(raw) {
            return true;
        }
        matches!(
            self.threshold(raw),
            Some(LevelThreshold::Code(ThresholdCode::RangeFolded))
        )
    }
}

/// Converts the ICD's modified 16-bit float to an `f32`.
///
/// Note 1 of Figure 3-6 adapts IEEE 754 to a halfword as one sign bit, five
/// exponent bits and ten fraction bits, with a bias of 16 rather than 15 and a
/// different subnormal rule. The worked example in the ICD is `0x5BB4`, which
/// resolves to 123.25.
pub fn int16_to_float16(raw: i16) -> f32 {
    let bits = raw as u16;
    let sign = if bits & 0b1000_0000_0000_0000 != 0 { -1.0 } else { 1.0 };
    let exponent = ((bits & 0b0111_1100_0000_0000) >> 10) as i32;
    let fraction = (bits & 0b0000_0011_1111_1111) as f32 / 1024.0;
    if exponent == 0 {
        sign * 2.0 * fraction
    } else {
        sign * 2f32.powi(exponent - 16) * (1.0 + fraction)
    }
}

/// Products whose threshold halfwords are *not* the packed flag/value form.
///
/// Note 1 of Figure 3-6: "Except for Products 32, 81, 93, 94, 99, 134, 135, 138,
/// 153, 154, 155, 159 161, 163, 177, 189, 190, 191, 192, 193, 195 and 197 the
/// Data Level Threshold halfwords are coded as follows".
const PACKED_THRESHOLD_EXCEPTIONS: [i16; 22] = [
    32, 81, 93, 94, 99, 134, 135, 138, 153, 154, 155, 159, 161, 163, 177, 189, 190, 191, 192, 193,
    195, 197,
];

impl ProductDescription {
    /// The first two threshold halfwords read as signed integers.
    fn threshold_i16_pair(&self) -> Option<(i16, i16)> {
        let d = self.threshold_data.get(..4)?;
        Some((
            i16::from_be_bytes([d[0], d[1]]),
            i16::from_be_bytes([d[2], d[3]]),
        ))
    }

    /// The first two threshold halfword pairs read as IEEE floats.
    fn threshold_f32_pair(&self) -> Option<(f32, f32)> {
        let d = self.threshold_data.get(..8)?;
        Some((
            f32::from_be_bytes([d[0], d[1], d[2], d[3]]),
            f32::from_be_bytes([d[4], d[5], d[6], d[7]]),
        ))
    }

    /// The threshold halfwords as signed integers.
    fn threshold_halfwords(&self) -> Vec<i16> {
        self.threshold_data
            .chunks_exact(2)
            .map(|c| i16::from_be_bytes([c[0], c[1]]))
            .collect()
    }

    /// Decodes the packed flag/value threshold table (Note 1 of Figure 3-6).
    ///
    /// Each halfword's high byte is a flag byte and its low byte the value. Bit
    /// 0 of the flags makes the low byte a categorical code instead; bits 1-3
    /// scale it by 100, 20 or 10; bits 4-7 attach a qualifier, of which bit 7
    /// makes the value negative.
    ///
    /// Py-ART reads only bits 0, 2, 3 and 7, and takes the scale from level 0
    /// for the whole table. This applies every documented bit, per level.
    fn packed_thresholds(&self) -> Vec<LevelThreshold> {
        self.threshold_data
            .chunks_exact(2)
            .map(|pair| {
                let flags = pair[0];
                let raw_value = pair[1];

                if flags & 0x80 != 0 {
                    return LevelThreshold::Code(ThresholdCode::from_byte(raw_value));
                }

                let scale = if flags & 0x40 != 0 {
                    100.0
                } else if flags & 0x20 != 0 {
                    20.0
                } else if flags & 0x10 != 0 {
                    10.0
                } else {
                    1.0
                };
                let magnitude = raw_value as f32 / scale;
                let negative = flags & 0x01 != 0;

                let qualifier = if flags & 0x08 != 0 {
                    Some(Qualifier::GreaterThan)
                } else if flags & 0x04 != 0 {
                    Some(Qualifier::LessThan)
                } else if flags & 0x02 != 0 {
                    Some(Qualifier::Plus)
                } else if negative {
                    Some(Qualifier::Minus)
                } else {
                    None
                };

                LevelThreshold::Value {
                    value: if negative { -magnitude } else { magnitude },
                    qualifier,
                }
            })
            .collect()
    }

    /// How to decode this product's raw data levels into physical values.
    ///
    /// Returns `None` only when the threshold data is too short to read, or for
    /// the handful of products whose levels are category indices carrying no
    /// scaling at all.
    pub fn level_scaling(&self) -> Option<LevelScaling> {
        match self.product_code {
            // Digital Hybrid Scan Reflectivity: scale and offset in tenths,
            // applied directly to the raw level.
            32 => {
                let (offset_tenths, scale_tenths) = self.threshold_i16_pair()?;
                Some(LevelScaling {
                    decoding: LevelDecoding::Linear {
                        scale: scale_tenths as f32 / 10.0,
                        offset: offset_tenths as f32 / 10.0,
                    },
                    units: "dBZ",
                    first_data_level: 2,
                    range_folded_level: Some(1),
                })
            }

            // Base Reflectivity / Base Velocity data arrays. Same tenths
            // encoding, but the level scale starts at level 2, so the raw level
            // is biased by two before scaling.
            94 | 99 | 182 | 186 => {
                let (offset_tenths, scale_tenths) = self.threshold_i16_pair()?;
                let scale = scale_tenths as f32 / 10.0;
                Some(LevelScaling {
                    decoding: LevelDecoding::Linear {
                        scale,
                        // (raw - 2) * scale + offset, folded into y = raw*scale + b
                        offset: offset_tenths as f32 / 10.0 - 2.0 * scale,
                    },
                    units: match self.product_code {
                        99 | 182 => "kt",
                        _ => "dBZ",
                    },
                    first_data_level: 2,
                    range_folded_level: Some(1),
                })
            }

            // Digital Storm Total Precipitation: hundredths rather than tenths.
            138 => {
                let (offset_hundredths, scale_hundredths) = self.threshold_i16_pair()?;
                Some(LevelScaling {
                    decoding: LevelDecoding::Linear {
                        scale: scale_hundredths as f32 / 100.0,
                        offset: offset_hundredths as f32 / 100.0,
                    },
                    units: "in",
                    first_data_level: 0,
                    range_folded_level: None,
                })
            }

            // High Resolution VIL: linear below halfword 33, logarithmic above,
            // with the coefficients in the ICD's modified 16-bit float format.
            // Note 1: "For product 134, data level codes 0 and 1 correspond to
            // 'Below threshold' and 'flagged data'".
            134 => {
                let hw = self.threshold_halfwords();
                if hw.len() < 5 {
                    return None;
                }
                Some(LevelScaling {
                    decoding: LevelDecoding::LinearThenLog {
                        linear_scale: int16_to_float16(hw[0]),
                        linear_offset: int16_to_float16(hw[1]),
                        log_start: hw[2],
                        log_scale: int16_to_float16(hw[3]),
                        log_offset: int16_to_float16(hw[4]),
                    },
                    units: "kg/m2",
                    first_data_level: 2,
                    range_folded_level: None,
                })
            }

            // Enhanced Echo Tops: the masks come from the product itself, so
            // read them rather than assuming the documented defaults.
            135 => {
                let hw = self.threshold_halfwords();
                if hw.len() < 4 {
                    return None;
                }
                Some(LevelScaling {
                    decoding: LevelDecoding::EnhancedEchoTops {
                        data_mask: (hw[0] & 0xFF) as u8,
                        data_scale: hw[1] as f32,
                        data_offset: hw[2] as f32,
                        topped_mask: (hw[3] & 0xFF) as u8,
                    },
                    units: "kft",
                    // 0 is below threshold and 1 is bad data.
                    first_data_level: 2,
                    range_folded_level: None,
                })
            }

            // The float scale/offset family, which Note 1 lists explicitly:
            // "For products 159, 161, 163, 167, 168, 170, 172, 173, 174, 175 and
            // 176 ... F = (N - OFFSET) / SCALE".
            159 | 161 | 163 | 167 | 168 | 170 | 172 | 173 | 174 | 175 | 176 => {
                let (scale, offset) = self.threshold_f32_pair()?;
                if scale == 0.0 || !scale.is_finite() || !offset.is_finite() {
                    return None;
                }
                // The accumulation products report hundredths of an inch.
                let hundredths = matches!(self.product_code, 170 | 172 | 173 | 174 | 175);
                let unit_scale = if hundredths { 0.01 } else { 1.0 };
                Some(LevelScaling {
                    decoding: LevelDecoding::Linear {
                        scale: unit_scale / scale,
                        offset: -offset * unit_scale / scale,
                    },
                    units: match self.product_code {
                        159 => "dB",
                        163 => "deg/km",
                        167 => "",       // correlation coefficient, unitless
                        168 => "deg",    // differential phase
                        176 => "in/hr",
                        161 => "",       // correlation coefficient, unitless
                        _ => "in",
                    },
                    first_data_level: if hundredths || self.product_code == 176 { 1 } else { 2 },
                    range_folded_level: if hundredths || self.product_code == 176 {
                        None
                    } else {
                        Some(1)
                    },
                })
            }

            // Everything else uses the packed flag/value threshold halfwords.
            code if !PACKED_THRESHOLD_EXCEPTIONS.contains(&code) => {
                let thresholds = self.packed_thresholds();
                if thresholds.is_empty() {
                    return None;
                }
                // The flags themselves say which levels are data and which are
                // flags, so derive both from the table.
                let first_data_level = thresholds
                    .iter()
                    .position(|t| matches!(t, LevelThreshold::Value { .. }))
                    .unwrap_or(0) as u8;
                let range_folded_level = thresholds
                    .iter()
                    .position(|t| {
                        matches!(t, LevelThreshold::Code(ThresholdCode::RangeFolded))
                    })
                    .map(|i| i as u8);

                Some(LevelScaling {
                    decoding: LevelDecoding::Thresholds(thresholds),
                    units: packed_threshold_units(code),
                    first_data_level,
                    range_folded_level,
                })
            }

            // The remaining exceptions carry no scaling: their levels are
            // category indices (177, Hybrid Hydrometeor Classification) or the
            // format is not documented in Note 1 (81, 93, 153-155, 189-195).
            _ => None,
        }
    }

    /// The product's maximum-value annotation, when Table V puts one in
    /// halfword 47 and this crate knows its units.
    ///
    /// This is what the reference plots print as `MAX: 56 DBZ`.
    pub fn max_value_annotation(&self) -> Option<(i16, &'static str)> {
        let hw47 = self
            .halfwords_47_53
            .get(..2)
            .map(|d| i16::from_be_bytes([d[0], d[1]]))?;

        // Table V, "MSG CODE / HWORD# 47" rows.
        let units = match self.product_code {
            // Max reflectivity
            32 | 37 | 38 | 94 | 97 | 98 | 153 | 193 | 195 => "DBZ",
            // Layer composite reflectivity products
            65 | 66 | 67 | 90 | 137 => "DBZ",
            // Echo tops, enhanced echo tops
            41 | 135 => "KFT",
            // High resolution VIL is unitless; VIL is kg/m^2
            57 => "KG/M2",
            134 => "",
            // Accumulation products report inches
            78 | 79 | 80 | 138 | 169 | 170 | 172 | 173 => "IN",
            _ => return None,
        };
        Some((hw47, units))
    }
}

/// Units for the products whose levels come from packed thresholds, where this
/// crate knows the quantity from Table V or the Product Specification.
fn packed_threshold_units(product_code: i16) -> &'static str {
    match product_code {
        // Reflectivity products
        19 | 20 | 181 | 35 | 36 | 63 | 64 | 65 | 66 | 67 | 89 | 90 | 137 => "dBZ",
        // Velocity and spectrum width products
        25 | 27 | 28 | 30 | 56 => "kt",
        // Echo tops
        41 => "kft",
        // Vertically integrated liquid
        57 => "kg/m2",
        // Accumulation products
        78 | 79 | 80 | 169 | 171 => "in",
        // Snow products
        144 | 145 | 146 | 147 | 150 | 151 => "in",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product_description::product_description;

    /// Builds a product description with the given code and threshold bytes.
    fn description(product_code: i16, threshold: &[u8]) -> ProductDescription {
        let mut b = Vec::new();
        b.extend_from_slice(&(-1i16).to_be_bytes()); // divider
        b.extend_from_slice(&42968i32.to_be_bytes()); // latitude
        b.extend_from_slice(&(-88551i32).to_be_bytes()); // longitude
        b.extend_from_slice(&1022i16.to_be_bytes()); // height
        b.extend_from_slice(&product_code.to_be_bytes());
        b.extend_from_slice(&2i16.to_be_bytes()); // operational mode
        b.extend_from_slice(&212i16.to_be_bytes()); // vcp
        b.extend_from_slice(&0i16.to_be_bytes()); // sequence
        b.extend_from_slice(&1i16.to_be_bytes()); // volume scan number
        b.extend_from_slice(&1i16.to_be_bytes()); // volume scan date
        b.extend_from_slice(&0i32.to_be_bytes()); // volume scan time
        b.extend_from_slice(&1i16.to_be_bytes()); // product date
        b.extend_from_slice(&0i32.to_be_bytes()); // product time
        b.extend_from_slice(&[0u8; 4]); // halfwords 27-28
        b.extend_from_slice(&0i16.to_be_bytes()); // elevation number
        b.extend_from_slice(&[0u8; 2]); // halfword 30
        let mut th = threshold.to_vec();
        th.resize(32, 0);
        b.extend_from_slice(&th); // halfwords 31-46
        b.extend_from_slice(&[0u8; 14]); // halfwords 47-53
        b.push(0); // version
        b.push(0); // spot blank
        b.extend_from_slice(&0i32.to_be_bytes());
        b.extend_from_slice(&0i32.to_be_bytes());
        b.extend_from_slice(&0i32.to_be_bytes());
        product_description(&b).unwrap().1
    }

    fn reference_dhr() -> ProductDescription {
        let file = include_bytes!("../data/sn_DC.radar_DS.32dhr_KMKX.last");
        let (rest, _) = crate::text_header(file).unwrap();
        let (rest, _) = crate::message_header(rest).unwrap();
        product_description(rest).unwrap().1
    }

    /// The bundled product 32 fixture: threshold halfwords -320 and 5, giving
    /// `dBZ = raw * 0.5 - 32.0`.
    #[test]
    fn decodes_the_reference_dhr_product() {
        let scaling = reference_dhr().level_scaling().expect("32 should decode");
        assert_eq!(scaling.units, "dBZ");
        assert_eq!(scaling.linear_params(), Some((0.5, -32.0)));

        assert_eq!(scaling.value(0), None);
        assert_eq!(scaling.value(1), None);
        assert!(scaling.is_range_folded(1));
        assert!(!scaling.is_range_folded(0));

        assert_eq!(scaling.value(2), Some(-31.0));
        assert_eq!(scaling.value(64), Some(0.0));
        assert_eq!(scaling.value(176), Some(56.0));
        assert_eq!(scaling.value(255), Some(95.5));
    }

    #[test]
    fn reads_the_reference_max_value_annotation() {
        assert_eq!(reference_dhr().max_value_annotation(), Some((56, "DBZ")));
    }

    #[test]
    fn data_array_products_bias_the_level_by_two() {
        let pd = description(94, &[0xFE, 0xC0, 0x00, 0x05]);
        let s = pd.level_scaling().unwrap();
        assert_eq!(s.units, "dBZ");
        assert_eq!(s.value(2), Some(-32.0));
        assert_eq!(s.value(3), Some(-31.5));
        assert_eq!(s.value(0), None);
    }

    #[test]
    fn velocity_data_arrays_report_knots() {
        assert_eq!(
            description(99, &[0xFE, 0xC0, 0x00, 0x05])
                .level_scaling()
                .unwrap()
                .units,
            "kt"
        );
    }

    #[test]
    fn dual_polarisation_products_use_float_scale_and_offset() {
        let mut th = 10.0f32.to_be_bytes().to_vec();
        th.extend_from_slice(&2.0f32.to_be_bytes());
        let s = description(159, &th).level_scaling().unwrap();
        assert_eq!(s.units, "dB");
        assert!((s.value(2).unwrap() - 0.0).abs() < 1e-6);
        assert!((s.value(12).unwrap() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn accumulation_products_report_hundredths_of_an_inch() {
        let mut th = 1.0f32.to_be_bytes().to_vec();
        th.extend_from_slice(&0.0f32.to_be_bytes());
        let s = description(172, &th).level_scaling().unwrap();
        assert_eq!(s.units, "in");
        assert!((s.value(100).unwrap() - 1.0).abs() < 1e-6);
    }

    /// Note 1 lists 167 and 168 in the float scale/offset family too.
    #[test]
    fn super_res_dual_pol_products_are_in_the_float_family() {
        let mut th = 10.0f32.to_be_bytes().to_vec();
        th.extend_from_slice(&0.0f32.to_be_bytes());
        for code in [167, 168] {
            let s = description(code, &th)
                .level_scaling()
                .unwrap_or_else(|| panic!("{code} should decode"));
            assert!(s.linear_params().is_some());
        }
    }

    #[test]
    fn a_zero_float_scale_is_rejected_rather_than_dividing_by_zero() {
        let mut th = 0.0f32.to_be_bytes().to_vec();
        th.extend_from_slice(&0.0f32.to_be_bytes());
        assert!(description(159, &th).level_scaling().is_none());
    }

    // ---- packed flag/value thresholds (Note 1 of Figure 3-6) ----

    /// The ICD's own example: "A data level value of (Hex) 8401 ... is
    /// interpreted as: < TH" — bit 0 set makes the low byte a code, and bit 5
    /// (0x04) adds the "<" qualifier.
    #[test]
    fn a_flagged_threshold_decodes_as_a_category_code() {
        let pd = description(19, &[0x84, 0x01]);
        let s = pd.level_scaling().unwrap();
        assert_eq!(
            s.threshold(0),
            Some(LevelThreshold::Code(ThresholdCode::BelowThreshold))
        );
        assert_eq!(s.threshold(0).unwrap(), LevelThreshold::Code(ThresholdCode::BelowThreshold));
        // A category is not a measurement.
        assert_eq!(s.value(0), None);
    }

    #[test]
    fn threshold_scale_bits_divide_the_value() {
        // Bit 3 (0x10) scales by 10: value byte 55 becomes 5.5.
        let s = description(19, &[0x10, 55]).level_scaling().unwrap();
        assert_eq!(s.value(0), Some(5.5));

        // Bit 2 (0x20) scales by 20: 50 becomes 2.5.
        let s = description(19, &[0x20, 50]).level_scaling().unwrap();
        assert_eq!(s.value(0), Some(2.5));

        // Bit 1 (0x40) scales by 100: 250 becomes 2.5. Py-ART does not read
        // this bit at all.
        let s = description(19, &[0x40, 250]).level_scaling().unwrap();
        assert_eq!(s.value(0), Some(2.5));
    }

    #[test]
    fn threshold_sign_bit_makes_the_value_negative() {
        // Bit 7 (0x01) is "-".
        let s = description(19, &[0x01, 32]).level_scaling().unwrap();
        assert_eq!(s.value(0), Some(-32.0));
        assert_eq!(
            s.threshold(0),
            Some(LevelThreshold::Value {
                value: -32.0,
                qualifier: Some(Qualifier::Minus)
            })
        );
    }

    #[test]
    fn threshold_qualifier_bits_are_reported() {
        // Bit 4 (0x08) is ">".
        let s = description(19, &[0x08, 70]).level_scaling().unwrap();
        assert_eq!(
            s.threshold(0),
            Some(LevelThreshold::Value {
                value: 70.0,
                qualifier: Some(Qualifier::GreaterThan)
            })
        );
    }

    /// A realistic 16-level reflectivity table: ND, RF, then values every
    /// 5 dBZ. The flags themselves must drive `first_data_level` and
    /// `range_folded_level`.
    #[test]
    fn packed_thresholds_derive_the_flag_levels_from_the_table() {
        let mut th = vec![0x80, 2, 0x80, 3]; // level 0 = ND, level 1 = RF
        for level in 0..14u8 {
            th.push(0x00); // numeric, no scale, positive
            th.push(level * 5);
        }
        let s = description(19, &th).level_scaling().unwrap();

        assert_eq!(s.units, "dBZ");
        assert_eq!(s.first_data_level, 2);
        assert_eq!(s.range_folded_level, Some(1));
        assert!(s.is_range_folded(1));
        assert_eq!(s.value(0), None);
        assert_eq!(s.value(1), None);
        assert_eq!(s.value(2), Some(0.0));
        assert_eq!(s.value(5), Some(15.0));
        // Past the 16 entry table there is nothing.
        assert_eq!(s.value(16), None);
    }

    #[test]
    fn hydrometeor_classification_levels_decode_as_categories() {
        // Product 165 is not in Note 1's exception list, so it uses the packed
        // form; its levels are classification codes.
        let th = vec![0x80, 2, 0x80, 10, 0x80, 11, 0x80, 15];
        let s = description(165, &th).level_scaling().unwrap();
        assert_eq!(
            s.threshold(1).unwrap(),
            LevelThreshold::Code(ThresholdCode::Rain)
        );
        assert_eq!(s.threshold(2).unwrap(), LevelThreshold::Code(ThresholdCode::HeavyRain));
        assert_eq!(
            s.threshold(3).unwrap().clone(),
            LevelThreshold::Code(ThresholdCode::LargeHail)
        );
        assert_eq!(ThresholdCode::Rain.abbreviation(), "RA");
        assert_eq!(ThresholdCode::LargeHail.abbreviation(), "LH");
    }

    // ---- product 134 (High Resolution VIL) ----

    /// The ICD's worked example: halfword value 0x5BB4 is 123.25.
    #[test]
    fn modified_float16_matches_the_icd_example() {
        assert!((int16_to_float16(0x5BB4u16 as i16) - 123.25).abs() < 1e-3);
    }

    #[test]
    fn float16_handles_sign_and_subnormals() {
        // Sign bit set negates.
        let positive = int16_to_float16(0x5BB4u16 as i16);
        let negative = int16_to_float16(0xDBB4u16 as i16);
        assert!((negative + positive).abs() < 1e-3);
        // Exponent zero takes the subnormal branch: 2 * fraction.
        assert!((int16_to_float16(0x0200) - 2.0 * 0.5).abs() < 1e-6);
        assert_eq!(int16_to_float16(0), 0.0);
    }

    #[test]
    fn vil_decodes_linearly_below_the_log_start_and_logarithmically_above() {
        // 1.0 in the ICD's float16 is exponent 16, fraction 0 -> 0x4000, and
        // 0x54DC is the log scale from the ICD's own worked example (38.875).
        const F_ONE: i16 = 0x4000u16 as i16;
        const F_ZERO: i16 = 0;
        const F_LOG_SCALE: i16 = 0x54DCu16 as i16;

        let mut th = Vec::new();
        th.extend_from_slice(&F_ONE.to_be_bytes()); // hw31 linear scale
        th.extend_from_slice(&F_ZERO.to_be_bytes()); // hw32 linear offset
        th.extend_from_slice(&100i16.to_be_bytes()); // hw33 log start
        th.extend_from_slice(&F_LOG_SCALE.to_be_bytes()); // hw34 log scale
        th.extend_from_slice(&F_ZERO.to_be_bytes()); // hw35 log offset

        let s = description(134, &th).level_scaling().unwrap();
        assert_eq!(s.units, "kg/m2");
        let log_scale = int16_to_float16(F_LOG_SCALE);
        assert!((log_scale - 38.875).abs() < 1e-3, "log scale was {log_scale}");

        // Levels 0 and 1 are below threshold and flagged data.
        assert_eq!(s.value(0), None);
        assert_eq!(s.value(1), None);

        // Below the log start the relation is linear: (raw - 0) / 1.
        assert_eq!(s.value(50), Some(50.0));
        assert_eq!(s.value(99), Some(99.0));

        // At and above it, logarithmic: exp((raw - 0) / log_scale).
        let expected = (100.0f32 / log_scale).exp();
        let got = s.value(100).unwrap();
        assert!(
            (got - expected).abs() / expected < 1e-4,
            "expected {expected}, got {got}"
        );
    }

    /// Coefficients that drive the logarithmic branch past `f32` range must
    /// report no value rather than infinity.
    #[test]
    fn vil_rejects_a_non_finite_result() {
        const F_ONE: i16 = 0x4000u16 as i16;
        const F_ZERO: i16 = 0;

        let mut th = Vec::new();
        th.extend_from_slice(&F_ONE.to_be_bytes()); // linear scale
        th.extend_from_slice(&F_ZERO.to_be_bytes()); // linear offset
        th.extend_from_slice(&100i16.to_be_bytes()); // log start
        th.extend_from_slice(&F_ONE.to_be_bytes()); // log scale 1.0 -> exp(100)
        th.extend_from_slice(&F_ZERO.to_be_bytes()); // log offset

        let s = description(134, &th).level_scaling().unwrap();
        // exp(100) overflows f32, so there is no representable value here.
        assert_eq!(s.value(100), None);
        // The linear branch is unaffected.
        assert_eq!(s.value(50), Some(50.0));
    }

    // ---- product 135 (Enhanced Echo Tops) ----

    /// Note 1 gives the masks the product carries and the decode rule:
    /// `Value = ((Data & DATA_MASK) / DATA_SCALE) - DATA_OFFSET`,
    /// `Topped = (Data & TOPPED_MASK) != 0`.
    #[test]
    fn enhanced_echo_tops_uses_the_masks_from_the_product() {
        let mut th = Vec::new();
        th.extend_from_slice(&127i16.to_be_bytes()); // DATA_MASK
        th.extend_from_slice(&1i16.to_be_bytes()); // DATA_SCALE
        th.extend_from_slice(&2i16.to_be_bytes()); // DATA_OFFSET
        th.extend_from_slice(&128i16.to_be_bytes()); // TOPPED_MASK

        let s = description(135, &th).level_scaling().unwrap();
        assert_eq!(s.units, "kft");

        // 0 is below threshold, 1 is bad data.
        assert_eq!(s.value(0), None);
        assert_eq!(s.value(1), None);

        // The documented data sets are 2-71 and 130-199.
        assert_eq!(s.value(2), Some(0.0));
        assert_eq!(s.value(71), Some(69.0));
        assert_eq!(s.is_topped(2), Some(false));

        // A topped value carries the same altitude with the high bit set.
        assert_eq!(s.value(130), Some(0.0));
        assert_eq!(s.value(199), Some(69.0));
        assert_eq!(s.is_topped(130), Some(true));
        assert_eq!(s.is_topped(199), Some(true));

        // Topped only applies to this product.
        assert!(reference_dhr().level_scaling().unwrap().is_topped(50).is_none());
    }

    #[test]
    fn products_with_no_documented_scaling_return_none() {
        // 177 is in Note 1's exception list and its levels are class indices.
        assert!(description(177, &[0, 0, 0, 0]).level_scaling().is_none());
        // 153 (super resolution reflectivity) is excluded too.
        assert!(description(153, &[0, 0, 0, 0]).level_scaling().is_none());
    }
}
