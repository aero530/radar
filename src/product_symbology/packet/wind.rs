//! Wind depiction packets:
//!
//! - Vector Arrow Data Packet, code 5 (Figure 3-12, page 3-103)
//! - Wind Barb Data Packet, code 4 (Figure 3-13, page 3-104)

use serde::{Deserialize, Serialize};
use nom::{
    number::{complete::i16 as nom_i16, Endianness::Big},
    IResult,
};

use super::util::{block_length, i16_array, payload};
use crate::product_symbology::SymPacketData;

/// One arrow from a Vector Arrow Data Packet (Figure 3-12).
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct VectorArrow {
    /// I coordinate the arrow is centred on (Km/4 or pixels).
    pub i_coord: i16,
    /// J coordinate the arrow is centred on (Km/4 or pixels).
    pub j_coord: i16,
    /// Arrow direction in whole degrees, 0 to 359; points with the wind field.
    pub direction: i16,
    /// Arrow length in pixels, 1 to 512.
    pub arrow_length: i16,
    /// Arrow head length in pixels, 1 to 512.
    pub arrow_head_length: i16,
}

/// Vector Arrow Data Packet, code 5 (Figure 3-12).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct VectorArrowPacket {
    pub arrows: Vec<VectorArrow>,
}

/// Vector Arrow Data Packet, packet code 5 (Figure 3-12).
///
/// The block holds a whole number of five-halfword arrow records.
pub fn vector_arrow(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, _packet_code) = nom_i16(Big)(input)?;
    let (input, len) = block_length(input)?;
    let (input, body) = payload(input, len)?;

    let arrows = i16_array(body)
        .chunks_exact(5)
        .map(|c| VectorArrow {
            i_coord: c[0],
            j_coord: c[1],
            direction: c[2],
            arrow_length: c[3],
            arrow_head_length: c[4],
        })
        .collect();

    Ok((input, SymPacketData::VectorArrowData(VectorArrowPacket { arrows })))
}

/// One barb from a Wind Barb Data Packet (Figure 3-13).
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct WindBarb {
    /// Colour level of the barb, 1 to 5; reflects the RMS value associated
    /// with the computed velocity.
    pub value: i16,
    /// X coordinate where the value starts (Km/4 or pixels).
    pub x_coord: i16,
    /// Y coordinate where the value starts (Km/4 or pixels).
    pub y_coord: i16,
    /// Wind direction in whole degrees, 0 to 359; points into the wind.
    pub direction: i16,
    /// Wind speed in knots, 0 to 195.
    pub speed: i16,
}

/// Wind Barb Data Packet, code 4 (Figure 3-13).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct WindBarbPacket {
    pub barbs: Vec<WindBarb>,
}

/// Wind Barb Data Packet, packet code 4 (Figure 3-13).
///
/// The block holds a whole number of five-halfword barb records.
pub fn wind_barb(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, _packet_code) = nom_i16(Big)(input)?;
    let (input, len) = block_length(input)?;
    let (input, body) = payload(input, len)?;

    let barbs = i16_array(body)
        .chunks_exact(5)
        .map(|c| WindBarb {
            value: c[0],
            x_coord: c[1],
            y_coord: c[2],
            direction: c[3],
            speed: c[4],
        })
        .collect();

    Ok((input, SymPacketData::WindBarbData(WindBarbPacket { barbs })))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hw(values: &[i16]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_be_bytes()).collect()
    }

    #[test]
    fn parses_two_vector_arrows() {
        let body = hw(&[100, 200, 270, 40, 10, -50, -60, 90, 30, 8]);
        let mut bytes = hw(&[5, body.len() as i16]);
        bytes.extend_from_slice(&body);

        let (rest, parsed) = vector_arrow(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::VectorArrowData(p) => {
                assert_eq!(p.arrows.len(), 2);
                assert_eq!(p.arrows[0].direction, 270);
                assert_eq!(p.arrows[0].arrow_head_length, 10);
                assert_eq!(p.arrows[1].i_coord, -50);
                assert_eq!(p.arrows[1].j_coord, -60);
                assert_eq!(p.arrows[1].arrow_head_length, 8);
            }
            other => panic!("expected VectorArrowData, got {other:?}"),
        }
    }

    #[test]
    fn parses_a_wind_barb() {
        let body = hw(&[3, 10, 20, 180, 45]);
        let mut bytes = hw(&[4, body.len() as i16]);
        bytes.extend_from_slice(&body);

        let (rest, parsed) = wind_barb(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::WindBarbData(p) => {
                assert_eq!(p.barbs.len(), 1);
                assert_eq!(p.barbs[0].value, 3);
                assert_eq!(p.barbs[0].direction, 180);
                assert_eq!(p.barbs[0].speed, 45);
            }
            other => panic!("expected WindBarbData, got {other:?}"),
        }
    }

    #[test]
    fn rejects_a_truncated_barb_block() {
        let bytes = hw(&[4, 10, 1, 2]);
        assert!(wind_barb(&bytes).is_err());
    }
}
