//! Vector packets: linked (Figure 3-7), unlinked (Figure 3-8) and contour
//! (Figure 3-8a).
//!
//! All coordinates are in Km/4 or screen pixels, range -2048 to +2047.

use serde::{Deserialize, Serialize};
use nom::{
    number::{complete::i16 as nom_i16, Endianness::Big},
    IResult,
};

use super::util::{block_length, fail, i16_array, payload};
use crate::product_symbology::SymPacketData;

/// A point in the product's I/J coordinate space (Km/4 or screen pixels).
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Point {
    pub i: i16,
    pub j: i16,
}

/// A single vector with its own start and end point, as carried by the
/// unlinked vector and unlinked contour packets.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Vector {
    pub begin: Point,
    pub end: Point,
}

/// Linked Vector Packet — packet codes 6 (no value) and 9 (uniform value).
/// Figure 3-7, pages 3-88 to 3-89.
///
/// A polyline: one starting point followed by successive end points, each
/// continuing from the previous one.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkedVectorPacket {
    /// Colour level of the vectors, present only for packet code 9.
    pub value: Option<i16>,
    pub start: Point,
    /// Successive end points; the polyline runs `start` -> `ends[0]` ->
    /// `ends[1]` -> ...
    pub ends: Vec<Point>,
}

/// Unlinked Vector Packet — packet codes 7 (no value) and 10 (uniform value).
/// Figure 3-8, pages 3-89 to 3-91.
///
/// A set of independent line segments.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct UnlinkedVectorPacket {
    /// Colour level of the vectors, present only for packet code 10.
    pub value: Option<i16>,
    pub vectors: Vec<Vector>,
}

/// Reads a run of halfword pairs as points.
fn points(bytes: &[u8]) -> Vec<Point> {
    i16_array(bytes)
        .chunks_exact(2)
        .map(|c| Point { i: c[0], j: c[1] })
        .collect()
}

/// Reads a run of four-halfword groups as begin/end vectors.
fn vectors(bytes: &[u8]) -> Vec<Vector> {
    i16_array(bytes)
        .chunks_exact(4)
        .map(|c| Vector {
            begin: Point { i: c[0], j: c[1] },
            end: Point { i: c[2], j: c[3] },
        })
        .collect()
}

/// Linked Vector Packet, packet code 6 or 9 (Figure 3-7).
///
/// Code 9 carries a leading colour level; code 6 does not.
pub fn linked_vector(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, packet_code) = nom_i16(Big)(input)?;
    let (input, len) = block_length(input)?;
    let (input, body) = payload(input, len)?;

    // Code 9 prefixes the polyline with a uniform colour level.
    let (value, body) = if packet_code == 9 {
        if body.len() < 2 {
            return fail(input, "Linked vector packet 9 is too short to hold its value");
        }
        (Some(i16::from_be_bytes([body[0], body[1]])), &body[2..])
    } else {
        (None, body)
    };

    // The remainder is a starting point followed by successive end points.
    if body.len() < 4 {
        return fail(input, "Linked vector packet is too short to hold a starting point");
    }
    let start = Point {
        i: i16::from_be_bytes([body[0], body[1]]),
        j: i16::from_be_bytes([body[2], body[3]]),
    };
    let ends = points(&body[4..]);

    Ok((
        input,
        SymPacketData::LinkedVector(LinkedVectorPacket { value, start, ends }),
    ))
}

/// Unlinked Vector Packet, packet code 7 or 10 (Figure 3-8).
///
/// Code 10 carries a leading colour level; code 7 does not.
pub fn unlinked_vector(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, packet_code) = nom_i16(Big)(input)?;
    let (input, len) = block_length(input)?;
    let (input, body) = payload(input, len)?;

    let (value, body) = if packet_code == 10 {
        if body.len() < 2 {
            return fail(input, "Unlinked vector packet 10 is too short to hold its value");
        }
        (Some(i16::from_be_bytes([body[0], body[1]])), &body[2..])
    } else {
        (None, body)
    };

    Ok((
        input,
        SymPacketData::UnlinkedVector(UnlinkedVectorPacket {
            value,
            vectors: vectors(body),
        }),
    ))
}

/// Contour Vector Packet — the three variants of Figure 3-8a, pages 3-91 to
/// 3-92.
///
/// Unlike the other packets these are distinguished by hex packet codes and
/// do not all share the "length of block" convention, so each variant is
/// modelled separately.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ContourVectorPacket {
    /// `0x0802` — Set Colour Level. Carries the colour value that applies to
    /// the contour vectors that follow it in the same layer.
    SetColorLevel {
        /// The `0x0002` colour value indicator halfword that precedes the value.
        indicator: i16,
        value: i16,
    },
    /// `0x0E03` — Linked contour vectors with an initial point.
    Linked {
        /// The `0x8000` initial point indicator halfword.
        indicator: i16,
        start: Point,
        ends: Vec<Point>,
    },
    /// `0x3501` — Unlinked contour vectors.
    Unlinked { vectors: Vec<Vector> },
}

/// Contour Vector Packet, packet codes `0x0802`, `0x0E03` and `0x3501`
/// (Figure 3-8a).
pub fn contour_vector(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, packet_code) = nom_i16(Big)(input)?;

    match packet_code as u16 {
        // Set Colour Level: a colour value indicator followed by the level.
        0x0802 => {
            let (input, indicator) = nom_i16(Big)(input)?;
            let (input, value) = nom_i16(Big)(input)?;
            Ok((
                input,
                SymPacketData::ContourVector(ContourVectorPacket::SetColorLevel {
                    indicator,
                    value,
                }),
            ))
        }
        // Linked contour: initial point indicator, start point, then a byte
        // length covering the end points that follow (length = #vectors * 4).
        0x0E03 => {
            let (input, indicator) = nom_i16(Big)(input)?;
            let (input, i) = nom_i16(Big)(input)?;
            let (input, j) = nom_i16(Big)(input)?;
            let (input, len) = block_length(input)?;
            let (input, body) = payload(input, len)?;
            Ok((
                input,
                SymPacketData::ContourVector(ContourVectorPacket::Linked {
                    indicator,
                    start: Point { i, j },
                    ends: points(body),
                }),
            ))
        }
        // Unlinked contour: a byte length (= #vectors * 8) then the vectors.
        0x3501 => {
            let (input, len) = block_length(input)?;
            let (input, body) = payload(input, len)?;
            Ok((
                input,
                SymPacketData::ContourVector(ContourVectorPacket::Unlinked {
                    vectors: vectors(body),
                }),
            ))
        }
        other => fail(input, &format!("Unknown contour vector packet code {other:#06x}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hw(values: &[i16]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_be_bytes()).collect()
    }

    #[test]
    fn parses_linked_vector_without_a_value() {
        // Packet code 6: start (10,20) then two end points.
        let body = hw(&[10, 20, 30, 40, 50, 60]);
        let mut bytes = hw(&[6, body.len() as i16]);
        bytes.extend_from_slice(&body);

        let (rest, parsed) = linked_vector(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::LinkedVector(p) => {
                assert_eq!(p.value, None);
                assert_eq!(p.start, Point { i: 10, j: 20 });
                assert_eq!(p.ends, vec![Point { i: 30, j: 40 }, Point { i: 50, j: 60 }]);
            }
            other => panic!("expected LinkedVector, got {other:?}"),
        }
    }

    #[test]
    fn parses_linked_vector_with_a_uniform_value() {
        // Packet code 9: colour level 7, start (1,2), one end point.
        let body = hw(&[7, 1, 2, 3, 4]);
        let mut bytes = hw(&[9, body.len() as i16]);
        bytes.extend_from_slice(&body);

        let (_, parsed) = linked_vector(&bytes).unwrap();
        match parsed {
            SymPacketData::LinkedVector(p) => {
                assert_eq!(p.value, Some(7));
                assert_eq!(p.start, Point { i: 1, j: 2 });
                assert_eq!(p.ends, vec![Point { i: 3, j: 4 }]);
            }
            other => panic!("expected LinkedVector, got {other:?}"),
        }
    }

    #[test]
    fn parses_unlinked_vector_pairs() {
        // Packet code 7: two independent segments.
        let body = hw(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let mut bytes = hw(&[7, body.len() as i16]);
        bytes.extend_from_slice(&body);

        let (rest, parsed) = unlinked_vector(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::UnlinkedVector(p) => {
                assert_eq!(p.value, None);
                assert_eq!(p.vectors.len(), 2);
                assert_eq!(p.vectors[0].begin, Point { i: 1, j: 2 });
                assert_eq!(p.vectors[0].end, Point { i: 3, j: 4 });
                assert_eq!(p.vectors[1].end, Point { i: 7, j: 8 });
            }
            other => panic!("expected UnlinkedVector, got {other:?}"),
        }
    }

    #[test]
    fn parses_unlinked_vector_with_a_uniform_value() {
        let body = hw(&[3, 1, 2, 3, 4]);
        let mut bytes = hw(&[10, body.len() as i16]);
        bytes.extend_from_slice(&body);

        let (_, parsed) = unlinked_vector(&bytes).unwrap();
        match parsed {
            SymPacketData::UnlinkedVector(p) => {
                assert_eq!(p.value, Some(3));
                assert_eq!(p.vectors.len(), 1);
            }
            other => panic!("expected UnlinkedVector, got {other:?}"),
        }
    }

    #[test]
    fn parses_contour_set_color_level() {
        let bytes = hw(&[0x0802u16 as i16, 0x0002, 12]);
        let (rest, parsed) = contour_vector(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::ContourVector(ContourVectorPacket::SetColorLevel { value, .. }) => {
                assert_eq!(value, 12)
            }
            other => panic!("expected SetColorLevel, got {other:?}"),
        }
    }

    #[test]
    fn parses_linked_contour_vectors() {
        let ends = hw(&[30, 40, 50, 60]);
        let mut bytes = hw(&[
            0x0E03u16 as i16,
            0x8000u16 as i16,
            10,
            20,
            ends.len() as i16,
        ]);
        bytes.extend_from_slice(&ends);

        let (rest, parsed) = contour_vector(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::ContourVector(ContourVectorPacket::Linked { start, ends, .. }) => {
                assert_eq!(start, Point { i: 10, j: 20 });
                assert_eq!(ends.len(), 2);
            }
            other => panic!("expected Linked contour, got {other:?}"),
        }
    }

    #[test]
    fn parses_unlinked_contour_vectors() {
        let body = hw(&[1, 2, 3, 4]);
        let mut bytes = hw(&[0x3501u16 as i16, body.len() as i16]);
        bytes.extend_from_slice(&body);

        let (_, parsed) = contour_vector(&bytes).unwrap();
        match parsed {
            SymPacketData::ContourVector(ContourVectorPacket::Unlinked { vectors }) => {
                assert_eq!(vectors.len(), 1);
                assert_eq!(vectors[0].end, Point { i: 3, j: 4 });
            }
            other => panic!("expected Unlinked contour, got {other:?}"),
        }
    }

    #[test]
    fn rejects_a_negative_block_length() {
        let bytes = hw(&[6, -8, 0, 0]);
        assert!(linked_vector(&bytes).is_err());
    }

    #[test]
    fn rejects_a_truncated_linked_vector() {
        // Declares 12 payload bytes but supplies 4.
        let bytes = hw(&[6, 12, 1, 2]);
        assert!(linked_vector(&bytes).is_err());
    }
}
