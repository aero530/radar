use serde::{Deserialize, Serialize};
use nom::{
    bytes::complete::take, number::{complete::i16 as nom_i16, Endianness::Big}, IResult
};


use crate::{codes::PacketCode, product_symbology::SymPacketData};


pub fn text_and_symbol(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    
    let (input, packet_code_int) = nom_i16(Big)(input)?;
    let packet_code = <PacketCode as num::FromPrimitive>::from_i16(packet_code_int).unwrap_or_default();

    let (input, length) = nom_i16(Big)(input)?;
    let (input, color_level, offset) = match packet_code {
        PacketCode::TextAndSpecialSymbol8 => {
            let (input, cl) = nom_i16(Big)(input)?;
            (input, Some(cl), 6)
        },
        _ => (input, None, 4)
    };
    let (input, i_coord) = nom_i16(Big)(input)?;
    let (input, j_coord) = nom_i16(Big)(input)?;

    // length is # bytes not included length or packet code.  This includes i coordinate,
    // j coordinate, & maybe color_level which adds up to 4 (or 6) bytes so the 
    // text string must be length minus that offset of 4 (or 6).
    let (input, part) = take(length as usize - offset)(input)?;
    let text = match std::str::from_utf8(part) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    
    Ok((input, 
        SymPacketData::TextAndSpecialSymbol8(
            TextPacket{
                packet_code,
                length,
                i_coord,
                j_coord,
                text,
                color_level,
            }
        )
    ))
}

/// Text and Special Symbol Packets - Packet Code 1 (Sheet 4)
/// Figure 3-8b (Sheet 4), page 3-88
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TextPacket {
    pub packet_code: PacketCode,
    /// Number of bytes in block not including self or packet code
    pub length: i16,
    /// Color level of text used in TextPacket8 but not in TextPacket1
    pub color_level: Option<i16>,
    /// I coordinate for text starting point (Km/4 or Pixels [-2048 to 2047])
    pub i_coord: i16,
    /// J coordinate for text starting point (Km/4 or Pixels [-2048 to 2047])
    pub j_coord: i16,
    /// Characters (ASCII)
    pub text: String,
}
