//! Special Graphic Symbol Packets — Figure 3-14, pages 3-104 to 3-107.
//!
//! A family of packets that share the "packet code, length of block, then a
//! repeating fixed-size symbol record" shape, but differ in what each record
//! contains:
//!
//! | Code(s) | Record |
//! | -- | -- |
//! | 3, 11 | mesocyclone / 3-D correlated shear: I, J, radius |
//! | 12, 26 | TVS / ETVS position: I, J |
//! | 13, 14 | hail positive / hail probable: I, J |
//! | 15 | storm ID: I, J, two ASCII characters |
//! | 19 | HDA hail: I, J, prob. hail, prob. severe hail, max hail size |
//! | 20 | point feature: I, J, feature type, feature attribute |
//! | 25 | STI circle: I, J, radius |
//! | 23, 24 | SCIT past / forecast data: nested display data packets |

use serde::{Deserialize, Serialize};
use nom::{
    number::{complete::i16 as nom_i16, Endianness::Big},
    IResult,
};

use super::util::{block_length, i16_array, payload};
use crate::product_symbology::SymPacketData;

/// A symbol with a position and an associated radius (Km/4).
///
/// Used by mesocyclone (3, 11) and STI circle (25) packets. Per Figure 3-14
/// sheet 3, a radius of 0 means no mesocyclone is present and the I, J
/// coordinates are set to 0, 0.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CircleSymbol {
    pub i_position: i16,
    pub j_position: i16,
    pub radius: i16,
}

/// A symbol that carries only a position (codes 12, 13, 14, 26).
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PointSymbol {
    pub i_position: i16,
    pub j_position: i16,
}

/// A storm identifier label (code 15).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct StormIdSymbol {
    pub i_position: i16,
    pub j_position: i16,
    /// Two character storm ID: a letter followed by a digit.
    pub storm_id: String,
}

/// An HDA hail symbol (code 19).
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct HailSymbol {
    pub i_position: i16,
    pub j_position: i16,
    /// Probability of hail as a percentage, or -999 when the cell is beyond
    /// the maximum range for algorithm processing.
    pub probability_of_hail: i16,
    /// Probability of severe hail as a percentage, or -999 as above.
    pub probability_of_severe_hail: i16,
    /// Maximum expected hail size in inches.
    pub max_hail_size: i16,
}

/// A point feature symbol (code 20).
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PointFeatureSymbol {
    pub i_position: i16,
    pub j_position: i16,
    /// Feature type per Figure 3-14 sheet 4: 1/3 mesocyclone, 5-8 TVS/ETVS,
    /// 9-11 MDA circulation.
    pub feature_type: i16,
    /// Type-dependent attribute; for types 1-4 and 9-11 this is a radius in
    /// Km/4.
    pub feature_attribute: i16,
}

/// The decoded payload of a Special Graphic Symbol Packet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum SpecialSymbolPacket {
    /// Codes 3 and 11 — mesocyclone / 3-D correlated shear.
    Mesocyclone { packet_code: i16, symbols: Vec<CircleSymbol> },
    /// Codes 12, 13, 14 and 26 — TVS, ETVS and hail position symbols.
    Position { packet_code: i16, symbols: Vec<PointSymbol> },
    /// Code 15 — storm ID labels.
    StormId { symbols: Vec<StormIdSymbol> },
    /// Code 19 — HDA hail.
    Hail { symbols: Vec<HailSymbol> },
    /// Code 20 — point features.
    PointFeature { symbols: Vec<PointFeatureSymbol> },
    /// Code 25 — STI circles.
    StiCircle { symbols: Vec<CircleSymbol> },
    /// Codes 23 and 24 — SCIT past / forecast position data. The block holds
    /// nested display data packets (codes 2, 6 or 25), which are kept as raw
    /// bytes here so that the nested dispatch stays in one place
    /// ([`crate::symbology_layer`]).
    ScitData { packet_code: i16, nested: Vec<u8> },
}

/// Special Graphic Symbol Packet — every code listed in Figure 3-14.
pub fn special_graphic_symbol(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, packet_code) = nom_i16(Big)(input)?;
    let (input, len) = block_length(input)?;
    let (input, body) = payload(input, len)?;
    let words = i16_array(body);

    let packet = match packet_code {
        // I, J, radius
        3 | 11 => SpecialSymbolPacket::Mesocyclone {
            packet_code,
            symbols: words
                .chunks_exact(3)
                .map(|c| CircleSymbol {
                    i_position: c[0],
                    j_position: c[1],
                    radius: c[2],
                })
                .collect(),
        },
        25 => SpecialSymbolPacket::StiCircle {
            symbols: words
                .chunks_exact(3)
                .map(|c| CircleSymbol {
                    i_position: c[0],
                    j_position: c[1],
                    radius: c[2],
                })
                .collect(),
        },
        // I, J only
        12 | 13 | 14 | 26 => SpecialSymbolPacket::Position {
            packet_code,
            symbols: words
                .chunks_exact(2)
                .map(|c| PointSymbol {
                    i_position: c[0],
                    j_position: c[1],
                })
                .collect(),
        },
        // I, J, two ASCII characters packed into one halfword
        15 => SpecialSymbolPacket::StormId {
            symbols: body
                .chunks_exact(6)
                .map(|c| StormIdSymbol {
                    i_position: i16::from_be_bytes([c[0], c[1]]),
                    j_position: i16::from_be_bytes([c[2], c[3]]),
                    storm_id: String::from_utf8_lossy(&c[4..6]).into_owned(),
                })
                .collect(),
        },
        // I, J, prob hail, prob severe hail, max size
        19 => SpecialSymbolPacket::Hail {
            symbols: words
                .chunks_exact(5)
                .map(|c| HailSymbol {
                    i_position: c[0],
                    j_position: c[1],
                    probability_of_hail: c[2],
                    probability_of_severe_hail: c[3],
                    max_hail_size: c[4],
                })
                .collect(),
        },
        // I, J, feature type, feature attribute
        20 => SpecialSymbolPacket::PointFeature {
            symbols: words
                .chunks_exact(4)
                .map(|c| PointFeatureSymbol {
                    i_position: c[0],
                    j_position: c[1],
                    feature_type: c[2],
                    feature_attribute: c[3],
                })
                .collect(),
        },
        // Nested display data packets
        _ => SpecialSymbolPacket::ScitData {
            packet_code,
            nested: body.to_vec(),
        },
    };

    Ok((input, SymPacketData::SpecialGraphicSymbol(packet)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hw(values: &[i16]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_be_bytes()).collect()
    }

    fn packet(code: i16, body: &[u8]) -> Vec<u8> {
        let mut bytes = hw(&[code, body.len() as i16]);
        bytes.extend_from_slice(body);
        bytes
    }

    #[test]
    fn parses_mesocyclone_symbols() {
        let bytes = packet(3, &hw(&[100, 200, 12, -10, -20, 0]));
        let (rest, parsed) = special_graphic_symbol(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::SpecialGraphicSymbol(SpecialSymbolPacket::Mesocyclone {
                symbols,
                ..
            }) => {
                assert_eq!(symbols.len(), 2);
                assert_eq!(symbols[0].radius, 12);
                // Radius 0 means "no mesocyclone present" per sheet 3.
                assert_eq!(symbols[1].radius, 0);
            }
            other => panic!("expected Mesocyclone, got {other:?}"),
        }
    }

    #[test]
    fn parses_tvs_position_symbols() {
        let bytes = packet(12, &hw(&[5, 6, 7, 8]));
        let (_, parsed) = special_graphic_symbol(&bytes).unwrap();
        match parsed {
            SymPacketData::SpecialGraphicSymbol(SpecialSymbolPacket::Position { symbols, .. }) => {
                assert_eq!(symbols.len(), 2);
                assert_eq!(symbols[1].i_position, 7);
            }
            other => panic!("expected Position, got {other:?}"),
        }
    }

    #[test]
    fn parses_storm_id_labels() {
        let mut body = hw(&[10, 20]);
        body.extend_from_slice(b"A1");
        let bytes = packet(15, &body);

        let (_, parsed) = special_graphic_symbol(&bytes).unwrap();
        match parsed {
            SymPacketData::SpecialGraphicSymbol(SpecialSymbolPacket::StormId { symbols }) => {
                assert_eq!(symbols.len(), 1);
                assert_eq!(symbols[0].storm_id, "A1");
                assert_eq!(symbols[0].i_position, 10);
            }
            other => panic!("expected StormId, got {other:?}"),
        }
    }

    #[test]
    fn parses_hda_hail_symbols_including_the_out_of_range_flag() {
        let bytes = packet(19, &hw(&[1, 2, 70, 30, 2, 3, 4, -999, -999, 0]));
        let (_, parsed) = special_graphic_symbol(&bytes).unwrap();
        match parsed {
            SymPacketData::SpecialGraphicSymbol(SpecialSymbolPacket::Hail { symbols }) => {
                assert_eq!(symbols.len(), 2);
                assert_eq!(symbols[0].probability_of_hail, 70);
                assert_eq!(symbols[0].max_hail_size, 2);
                // -999 flags a cell beyond the algorithm's maximum range.
                assert_eq!(symbols[1].probability_of_hail, -999);
            }
            other => panic!("expected Hail, got {other:?}"),
        }
    }

    #[test]
    fn parses_point_features() {
        let bytes = packet(20, &hw(&[1, 2, 3, 40]));
        let (_, parsed) = special_graphic_symbol(&bytes).unwrap();
        match parsed {
            SymPacketData::SpecialGraphicSymbol(SpecialSymbolPacket::PointFeature { symbols }) => {
                assert_eq!(symbols[0].feature_type, 3);
                assert_eq!(symbols[0].feature_attribute, 40);
            }
            other => panic!("expected PointFeature, got {other:?}"),
        }
    }

    #[test]
    fn keeps_scit_nested_packets_as_raw_bytes() {
        let bytes = packet(23, &hw(&[25, 6, 1, 2, 3]));
        let (_, parsed) = special_graphic_symbol(&bytes).unwrap();
        match parsed {
            SymPacketData::SpecialGraphicSymbol(SpecialSymbolPacket::ScitData {
                packet_code,
                nested,
            }) => {
                assert_eq!(packet_code, 23);
                assert_eq!(nested.len(), 10);
            }
            other => panic!("expected ScitData, got {other:?}"),
        }
    }

    #[test]
    fn rejects_a_truncated_symbol_block() {
        let bytes = hw(&[3, 12, 1, 2]);
        assert!(special_graphic_symbol(&bytes).is_err());
    }
}
