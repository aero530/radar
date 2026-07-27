//! The Graphic Alphanumeric Block (Block ID 2) — Figure 3-6 sheets 4 and 9,
//! pages 3-24 and 3-35.
//!
//! This block carries the storm-attribute table that products draw at the top
//! of the geographic display. Its payload is a series of pages, each holding
//! text packets (packet code 8 of Figure 3-8b, or packet code 10 of
//! Figure 3-8).

use serde::{Deserialize, Serialize};
use nom::{
    number::{
        complete::{i16 as nom_i16, i32 as nom_i32},
        Endianness::Big,
    },
    IResult,
};
use tracing::{debug, error, warn};

use crate::product_symbology::{symbology_layer_packet, SymPacketData};

/// One page of the Graphic Alphanumeric Block.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GraphicPage {
    /// Current page number, 1 to 48.
    pub page_number: i16,
    /// Number of bytes of text packets on this page.
    pub length: i16,
    /// The page's decoded packets. A packet this crate cannot decode leaves
    /// the remainder of the page in `undecoded` rather than failing the block.
    pub packets: Vec<SymPacketData>,
    /// Any bytes of the page that could not be decoded into packets.
    pub undecoded: Vec<u8>,
}

/// The Graphic Alphanumeric Block (Block ID 2), Figure 3-6 sheets 4 and 9.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GraphicBlock {
    /// Block divider, always -1.
    pub divider: i16,
    /// Block ID, always 2.
    pub id: i16,
    /// Length of block in bytes, from the divider to the end of the message.
    pub block_length: i32,
    /// Total number of pages, 1 to 48.
    pub num_pages: i16,
    pub pages: Vec<GraphicPage>,
}

/// Parses the Graphic Alphanumeric Block.
///
/// # Errors
///
/// Fails if the block ID is not 2, if a declared page length runs past the end
/// of the input, or if the page count is negative.
pub fn graphic_alphanumeric(input: &[u8]) -> IResult<&[u8], GraphicBlock> {
    let (input, divider) = nom_i16(Big)(input)?;
    if divider != -1 {
        error!("Graphic alphanumeric block divider should be -1 but found {divider}");
    }
    let (input, id) = nom_i16(Big)(input)?;
    if id != 2 {
        error!("Graphic alphanumeric block should have ID=2 but found {id}");
        return Err(nom::Err::Failure(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Fail,
        )));
    }
    let (input, block_length) = nom_i32(Big)(input)?;
    let (mut input, num_pages) = nom_i16(Big)(input)?;

    debug!("Graphic alphanumeric block is {block_length} bytes, {num_pages} page(s)");

    let page_count = match usize::try_from(num_pages) {
        Ok(n) => n,
        Err(_) => {
            error!("Graphic alphanumeric block declares {num_pages} pages");
            return Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Fail,
            )));
        }
    };

    let mut pages = Vec::with_capacity(page_count.min(48));
    for _ in 0..page_count {
        let (rest, page_number) = nom_i16(Big)(input)?;
        let (rest, length) = nom_i16(Big)(rest)?;
        let len = match usize::try_from(length) {
            Ok(n) => n,
            Err(_) => {
                error!("Graphic alphanumeric page declares {length} bytes");
                return Err(nom::Err::Failure(nom::error::Error::new(
                    rest,
                    nom::error::ErrorKind::Fail,
                )));
            }
        };
        let (rest, body) = nom::bytes::complete::take(len)(rest)?;

        // Decode as many packets as the page's bytes allow.
        let (packets, undecoded) = decode_page_packets(body);
        pages.push(GraphicPage {
            page_number,
            length,
            packets,
            undecoded,
        });
        input = rest;
    }

    Ok((
        input,
        GraphicBlock {
            divider,
            id,
            block_length,
            num_pages,
            pages,
        },
    ))
}

/// Decodes a page's text packets, stopping at the first packet that cannot be
/// decoded and returning whatever bytes remain.
fn decode_page_packets(mut body: &[u8]) -> (Vec<SymPacketData>, Vec<u8>) {
    let mut packets = Vec::new();
    while body.len() >= 4 {
        match symbology_layer_packet(body) {
            Ok((rest, packet)) => {
                // Guard against a parser that consumes nothing, which would
                // otherwise spin here forever.
                if rest.len() == body.len() {
                    warn!("Graphic alphanumeric page packet consumed no bytes; stopping");
                    break;
                }
                packets.push(packet);
                body = rest;
            }
            Err(_) => break,
        }
    }
    (packets, body.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hw(values: &[i16]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_be_bytes()).collect()
    }

    /// Builds a packet code 8 text packet: value, I, J, then characters.
    fn text_packet_8(value: i16, i: i16, j: i16, text: &str) -> Vec<u8> {
        let mut body = hw(&[value, i, j]);
        body.extend_from_slice(text.as_bytes());
        let mut packet = hw(&[8, body.len() as i16]);
        packet.extend_from_slice(&body);
        packet
    }

    fn block(pages: &[(i16, Vec<u8>)]) -> Vec<u8> {
        let mut body = Vec::new();
        for (page_number, packets) in pages {
            body.extend_from_slice(&hw(&[*page_number, packets.len() as i16]));
            body.extend_from_slice(packets);
        }

        let block_length = 10 + body.len() as i32;
        let mut bytes = hw(&[-1, 2]);
        bytes.extend_from_slice(&block_length.to_be_bytes());
        bytes.extend_from_slice(&hw(&[pages.len() as i16]));
        bytes.extend_from_slice(&body);
        bytes
    }

    #[test]
    fn parses_a_single_page_of_text_packets() {
        let mut page = text_packet_8(3, 10, 20, "CELL A1");
        page.extend_from_slice(&text_packet_8(3, 10, 40, "TVS"));
        let bytes = block(&[(1, page)]);

        let (rest, parsed) = graphic_alphanumeric(&bytes).unwrap();
        assert!(rest.is_empty());
        assert_eq!(parsed.id, 2);
        assert_eq!(parsed.num_pages, 1);
        assert_eq!(parsed.pages.len(), 1);

        let page = &parsed.pages[0];
        assert_eq!(page.page_number, 1);
        assert!(page.undecoded.is_empty(), "page should decode fully");
        assert_eq!(page.packets.len(), 2);
        match &page.packets[0] {
            SymPacketData::TextAndSpecialSymbol8(t) => {
                assert_eq!(t.text, "CELL A1");
                assert_eq!(t.color_level, Some(3));
            }
            other => panic!("expected a text packet, got {other:?}"),
        }
    }

    #[test]
    fn parses_multiple_pages() {
        let bytes = block(&[
            (1, text_packet_8(1, 0, 0, "PAGE ONE")),
            (2, text_packet_8(1, 0, 0, "PAGE TWO")),
        ]);

        let (rest, parsed) = graphic_alphanumeric(&bytes).unwrap();
        assert!(rest.is_empty());
        assert_eq!(parsed.pages.len(), 2);
        assert_eq!(parsed.pages[1].page_number, 2);
        match &parsed.pages[1].packets[0] {
            SymPacketData::TextAndSpecialSymbol8(t) => assert_eq!(t.text, "PAGE TWO"),
            other => panic!("expected a text packet, got {other:?}"),
        }
    }

    #[test]
    fn keeps_undecodable_page_bytes_instead_of_failing() {
        // A page whose bytes are not a valid packet at all.
        let bytes = block(&[(1, vec![0xFF, 0xFF, 0xFF, 0xFF])]);

        let (_, parsed) = graphic_alphanumeric(&bytes).unwrap();
        assert!(parsed.pages[0].packets.is_empty());
        assert_eq!(parsed.pages[0].undecoded.len(), 4);
    }

    #[test]
    fn rejects_a_block_with_the_wrong_id() {
        let mut bytes = hw(&[-1, 3]); // ID 3 is the tabular block
        bytes.extend_from_slice(&16i32.to_be_bytes());
        bytes.extend_from_slice(&hw(&[1]));
        assert!(graphic_alphanumeric(&bytes).is_err());
    }

    #[test]
    fn rejects_a_page_longer_than_the_input() {
        let mut bytes = hw(&[-1, 2]);
        bytes.extend_from_slice(&100i32.to_be_bytes());
        bytes.extend_from_slice(&hw(&[1, 1, 80])); // page 1 claims 80 bytes
        bytes.extend_from_slice(&[0, 0]); // but only 2 follow
        assert!(graphic_alphanumeric(&bytes).is_err());
    }
}
