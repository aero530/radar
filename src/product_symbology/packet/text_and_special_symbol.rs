use serde::{Deserialize, Serialize};
use nom::{
    bytes::complete::take, number::{complete::i16 as nom_i16, Endianness::Big}, IResult
};


use crate::{codes::PacketCode, product_symbology::SymPacketData};

/// Text and Special Symbol Packets - Packet Codes 1, 2, and 8
/// Figure 3-8b, page 3-88
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
    let text_len = (length as usize).checked_sub(offset).ok_or_else(|| {
        nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Fail))
    })?;
    let (input, part) = take(text_len)(input)?;
    let text = std::str::from_utf8(part)
        .map_err(|_| nom::Err::Failure(nom::error::Error::new(part, nom::error::ErrorKind::Fail)))?
        .to_string();

    let packet = TextPacket {
        packet_code,
        length,
        i_coord,
        j_coord,
        text,
        color_level,
    };

    let symbology = match packet_code {
        PacketCode::TextAndSpecialSymbol1 => SymPacketData::TextAndSpecialSymbol1(packet),
        // Packet code 2 shares the same wire layout as code 1, and this
        // crate does not yet distinguish them with a dedicated variant.
        PacketCode::TextAndSpecialSymbol2 => SymPacketData::TextAndSpecialSymbol1(packet),
        _ => SymPacketData::TextAndSpecialSymbol8(packet),
    };

    Ok((input, symbology))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_packet1(text: &str) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&1i16.to_be_bytes()); // packet code 1
        bytes.extend_from_slice(&((4 + text.len()) as i16).to_be_bytes()); // length
        bytes.extend_from_slice(&10i16.to_be_bytes()); // i_coord
        bytes.extend_from_slice(&20i16.to_be_bytes()); // j_coord
        bytes.extend_from_slice(text.as_bytes());
        bytes
    }

    #[test]
    fn parses_packet_code_1_as_text_and_special_symbol_1() {
        let bytes = sample_packet1("HELLO");
        let (rest, parsed) = text_and_symbol(&bytes).unwrap();

        assert!(rest.is_empty());
        match parsed {
            SymPacketData::TextAndSpecialSymbol1(packet) => {
                assert_eq!(packet.text, "HELLO");
                assert_eq!(packet.i_coord, 10);
                assert_eq!(packet.j_coord, 20);
                assert_eq!(packet.color_level, None);
            }
            other => panic!("expected TextAndSpecialSymbol1, got {other:?}"),
        }
    }

    #[test]
    fn rejects_a_length_too_short_to_hold_the_fixed_fields_instead_of_panicking() {
        // length=2 is smaller than the 4-byte fixed-field offset for packet
        // code 1; this used to underflow a `usize` subtraction and panic.
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&1i16.to_be_bytes());
        bytes.extend_from_slice(&2i16.to_be_bytes());
        bytes.extend_from_slice(&0i16.to_be_bytes());
        bytes.extend_from_slice(&0i16.to_be_bytes());

        assert!(text_and_symbol(&bytes).is_err());
    }

    #[test]
    fn rejects_non_utf8_text_instead_of_panicking() {
        let mut bytes = sample_packet1("");
        bytes.push(0xFF);
        // fix up length to include the invalid byte
        let len = bytes.len() as i16 - 2 - 2 - 2 - 2; // total - packet_code - length - i - j
        bytes[2..4].copy_from_slice(&(len + 4).to_be_bytes());

        assert!(text_and_symbol(&bytes).is_err());
    }
}
