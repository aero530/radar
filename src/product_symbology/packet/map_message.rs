//! Map Message packets — packet codes `0x0E23`, `0x4E00`, `0x3521` and
//! `0x4E01` (Figure 3-9, pages 3-95 to 3-97).
//!
//! These carry map overlay geometry for the map products (message codes 132 to
//! 198). Per Note 1 of Figure 3-9 sheet 3 they use the same shapes as the
//! ordinary no-value vector, text and special symbol packets, with "the first 8
//! bytes replaced by the code shown in sheet 1" — which is why each variant
//! begins with one or two fixed indicator halfwords before the geometry.
//!
//! Two things differ from the equivalent symbology packets and matter to any
//! caller drawing these:
//!
//! - Coordinates are in **1/8 km**, not the 1/4 km the vector and text packets
//!   use elsewhere.
//! - The origin is the **upper left corner** of the area of coverage at 0,0,
//!   rather than the centre of the sweep.

use serde::{Deserialize, Serialize};
use nom::{
    number::{complete::i16 as nom_i16, Endianness::Big},
    IResult,
};

use super::util::{block_length, fail, i16_array, payload};
use super::vector::{Point, Vector};
use crate::product_symbology::SymPacketData;

/// The decoded payload of a Map Message packet.
///
/// All coordinates are in 1/8 km from the upper left corner of the area of
/// coverage.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum MapMessagePacket {
    /// `0x0E23` — linked vectors: a starting point followed by successive end
    /// points, forming a polyline.
    LinkedVector {
        /// The `0x8000` initial point indicator halfword.
        indicator: i16,
        start: Point,
        ends: Vec<Point>,
    },

    /// `0x3521` — unlinked vectors: independent begin/end segments.
    UnlinkedVector { vectors: Vec<Vector> },

    /// `0x4E00` — text, or `0x4E01` — special symbols. Both share a layout.
    Text {
        /// `0x4E00` for text, `0x4E01` for special symbols.
        packet_code: i16,
        /// The `0x0C23` indicator halfword.
        indicator: i16,
        /// The `0x8000` indicator halfword.
        initial_point_indicator: i16,
        /// Where the string starts, in 1/8 km from the upper left corner.
        position: Point,
        /// The characters. For `0x4E01` these select special symbols rather
        /// than reading as text, so they are decoded lossily.
        text: String,
    },
}

impl MapMessagePacket {
    /// Whether this packet carries special symbol selectors rather than text.
    pub fn is_special_symbols(&self) -> bool {
        matches!(
            self,
            MapMessagePacket::Text {
                packet_code: 0x4E01,
                ..
            }
        )
    }
}

/// Reads a run of halfword pairs as points.
fn points(bytes: &[u8]) -> Vec<Point> {
    i16_array(bytes)
        .chunks_exact(2)
        .map(|c| Point { i: c[0], j: c[1] })
        .collect()
}

/// Map Message packet, codes `0x0E23`, `0x3521`, `0x4E00` and `0x4E01`
/// (Figure 3-9).
pub fn map_message(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, packet_code) = nom_i16(Big)(input)?;

    match packet_code as u16 {
        // Linked vectors: indicator, start point, then a byte length covering
        // the end points that follow (length = #vectors * 4).
        0x0E23 => {
            let (input, indicator) = nom_i16(Big)(input)?;
            let (input, i) = nom_i16(Big)(input)?;
            let (input, j) = nom_i16(Big)(input)?;
            let (input, len) = block_length(input)?;
            let (input, body) = payload(input, len)?;
            Ok((
                input,
                SymPacketData::MapMessage(MapMessagePacket::LinkedVector {
                    indicator,
                    start: Point { i, j },
                    ends: points(body),
                }),
            ))
        }

        // Unlinked vectors: a byte length (= #vectors * 8) then the vectors.
        0x3521 => {
            let (input, len) = block_length(input)?;
            let (input, body) = payload(input, len)?;
            let vectors = i16_array(body)
                .chunks_exact(4)
                .map(|c| Vector {
                    begin: Point { i: c[0], j: c[1] },
                    end: Point { i: c[2], j: c[3] },
                })
                .collect();
            Ok((
                input,
                SymPacketData::MapMessage(MapMessagePacket::UnlinkedVector { vectors }),
            ))
        }

        // Text or special symbols: two indicator halfwords, the start position,
        // then a byte length covering the characters.
        0x4E00 | 0x4E01 => {
            let (input, indicator) = nom_i16(Big)(input)?;
            let (input, initial_point_indicator) = nom_i16(Big)(input)?;
            let (input, i) = nom_i16(Big)(input)?;
            let (input, j) = nom_i16(Big)(input)?;
            let (input, len) = block_length(input)?;
            let (input, body) = payload(input, len)?;
            Ok((
                input,
                SymPacketData::MapMessage(MapMessagePacket::Text {
                    packet_code,
                    indicator,
                    initial_point_indicator,
                    position: Point { i, j },
                    // Special symbol selectors set the high bit, so this cannot
                    // be required to be valid UTF-8.
                    text: String::from_utf8_lossy(body).into_owned(),
                }),
            ))
        }

        other => fail(
            input,
            &format!("Unknown map message packet code {other:#06x}"),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hw(values: &[i16]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_be_bytes()).collect()
    }

    #[test]
    fn parses_linked_map_vectors() {
        let ends = hw(&[30, 40, 50, 60]);
        let mut bytes = hw(&[
            0x0E23u16 as i16,
            0x8000u16 as i16,
            10, // I start
            20, // J start
            ends.len() as i16,
        ]);
        bytes.extend_from_slice(&ends);

        let (rest, parsed) = map_message(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::MapMessage(MapMessagePacket::LinkedVector {
                indicator,
                start,
                ends,
            }) => {
                assert_eq!(indicator as u16, 0x8000);
                assert_eq!(start, Point { i: 10, j: 20 });
                assert_eq!(ends, vec![Point { i: 30, j: 40 }, Point { i: 50, j: 60 }]);
            }
            other => panic!("expected LinkedVector, got {other:?}"),
        }
    }

    #[test]
    fn parses_unlinked_map_vectors() {
        // Two segments, eight bytes each.
        let body = hw(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let mut bytes = hw(&[0x3521u16 as i16, body.len() as i16]);
        bytes.extend_from_slice(&body);

        let (rest, parsed) = map_message(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::MapMessage(MapMessagePacket::UnlinkedVector { vectors }) => {
                assert_eq!(vectors.len(), 2);
                assert_eq!(vectors[0].begin, Point { i: 1, j: 2 });
                assert_eq!(vectors[0].end, Point { i: 3, j: 4 });
                assert_eq!(vectors[1].end, Point { i: 7, j: 8 });
            }
            other => panic!("expected UnlinkedVector, got {other:?}"),
        }
    }

    #[test]
    fn parses_map_text() {
        let text = b"MILWAUKEE";
        let mut bytes = hw(&[
            0x4E00u16 as i16,
            0x0C23u16 as i16,
            0x8000u16 as i16,
            100, // X
            200, // Y
            text.len() as i16,
        ]);
        bytes.extend_from_slice(text);

        let (rest, parsed) = map_message(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::MapMessage(ref p @ MapMessagePacket::Text {
                packet_code,
                indicator,
                initial_point_indicator,
                position,
                ref text,
            }) => {
                assert_eq!(packet_code as u16, 0x4E00);
                assert_eq!(indicator as u16, 0x0C23);
                assert_eq!(initial_point_indicator as u16, 0x8000);
                assert_eq!(position, Point { i: 100, j: 200 });
                assert_eq!(text, "MILWAUKEE");
                assert!(!p.is_special_symbols());
            }
            other => panic!("expected Text, got {other:?}"),
        }
    }

    /// Special symbol selectors set the high bit, so the payload is not valid
    /// UTF-8 and must decode lossily rather than failing.
    #[test]
    fn parses_map_special_symbols_with_the_high_bit_set() {
        let body = [0x81u8, 0x82, 0x83, 0x84];
        let mut bytes = hw(&[
            0x4E01u16 as i16,
            0x0C23u16 as i16,
            0x8000u16 as i16,
            0,
            0,
            body.len() as i16,
        ]);
        bytes.extend_from_slice(&body);

        let (rest, parsed) = map_message(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::MapMessage(ref p @ MapMessagePacket::Text { packet_code, .. }) => {
                assert_eq!(packet_code as u16, 0x4E01);
                assert!(p.is_special_symbols());
            }
            other => panic!("expected Text, got {other:?}"),
        }
    }

    #[test]
    fn rejects_an_unknown_map_packet_code() {
        let bytes = hw(&[0x1234, 0, 0, 0]);
        assert!(map_message(&bytes).is_err());
    }

    #[test]
    fn rejects_a_truncated_map_packet() {
        // Declares 40 payload bytes but supplies four.
        let mut bytes = hw(&[0x3521u16 as i16, 40]);
        bytes.extend_from_slice(&hw(&[1, 2]));
        assert!(map_message(&bytes).is_err());
    }
}
