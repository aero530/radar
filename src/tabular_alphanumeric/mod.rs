//! The Tabular Alphanumeric Block (Block ID 3) — Figure 3-6 sheets 5 and 10,
//! pages 3-25 and 3-36.
//!
//! Per Note 3 of sheet 10 this block must be the last block in a product
//! message. Its layout is unusual: after the block header it repeats a full
//! Message Header Block and Product Description Block, then a divider, then
//! pages of 80-character text lines. A maximum of 17 lines per page applies.

use serde::{Deserialize, Serialize};
use nom::{
    number::{
        complete::{i16 as nom_i16, i32 as nom_i32},
        Endianness::Big,
    },
    IResult,
};
use tracing::{debug, error, warn};

use crate::message_header::{message_header, MessageHeader};
use crate::product_description::{product_description, ProductDescription};

/// Maximum lines per page, per Note 3 of Figure 3-6 sheet 10.
const MAX_LINES_PER_PAGE: usize = 17;

/// One page of tabular text.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct TabularPage {
    /// The page's lines, each up to 80 characters.
    pub lines: Vec<String>,
}

/// The Tabular Alphanumeric Block (Block ID 3), Figure 3-6 sheets 5 and 10.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TabularBlock {
    /// Block divider, always -1.
    pub divider: i16,
    /// Block ID, always 3.
    pub id: i16,
    /// Length of block in bytes, from the divider to the end of the message.
    pub block_length: i32,
    /// The repeated Message Header Block that precedes the tabular data.
    pub message_header: MessageHeader,
    /// The repeated Product Description Block that precedes the tabular data.
    pub product_description: ProductDescription,
    /// Total number of pages, 1 to 48.
    pub num_pages: i16,
    pub pages: Vec<TabularPage>,
}

/// Parses the Tabular Alphanumeric Block.
///
/// # Errors
///
/// Fails if the block ID is not 3, if the embedded second header blocks are
/// malformed, or if a line's character count runs past the end of the input.
pub fn tabular_alphanumeric(input: &[u8]) -> IResult<&[u8], TabularBlock> {
    let (input, divider) = nom_i16(Big)(input)?;
    if divider != -1 {
        error!("Tabular alphanumeric block divider should be -1 but found {divider}");
    }
    let (input, id) = nom_i16(Big)(input)?;
    if id != 3 {
        error!("Tabular alphanumeric block should have ID=3 but found {id}");
        return Err(nom::Err::Failure(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Fail,
        )));
    }
    let (input, block_length) = nom_i32(Big)(input)?;

    // The block repeats a full message header and product description before
    // the text itself (sheet 5: "SECOND HEADER AND PRODUCT DESCRIPTION BLOCK").
    let (input, message_header) = message_header(input)?;
    let (input, product_description) = product_description(input)?;

    // A second divider separates the repeated headers from the data.
    let (input, data_divider) = nom_i16(Big)(input)?;
    if data_divider != -1 {
        error!("Tabular data divider should be -1 but found {data_divider}");
    }

    let (mut input, num_pages) = nom_i16(Big)(input)?;
    debug!("Tabular alphanumeric block is {block_length} bytes, {num_pages} page(s)");

    let page_count = match usize::try_from(num_pages) {
        Ok(n) => n,
        Err(_) => {
            error!("Tabular alphanumeric block declares {num_pages} pages");
            return Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Fail,
            )));
        }
    };

    let mut pages = Vec::with_capacity(page_count.min(48));
    for _ in 0..page_count {
        let (rest, page) = tabular_page(input)?;
        pages.push(page);
        input = rest;
    }

    Ok((
        input,
        TabularBlock {
            divider,
            id,
            block_length,
            message_header,
            product_description,
            num_pages,
            pages,
        },
    ))
}

/// Reads lines until the -1 end-of-page flag.
fn tabular_page(mut input: &[u8]) -> IResult<&[u8], TabularPage> {
    let mut lines = Vec::new();

    loop {
        let (rest, num_chars) = nom_i16(Big)(input)?;

        // -1 marks the end of the page rather than a line length.
        if num_chars == -1 {
            return Ok((rest, TabularPage { lines }));
        }

        let len = match usize::try_from(num_chars) {
            Ok(n) => n,
            Err(_) => {
                error!("Tabular line declares {num_chars} characters");
                return Err(nom::Err::Failure(nom::error::Error::new(
                    rest,
                    nom::error::ErrorKind::Fail,
                )));
            }
        };
        let (rest, body) = nom::bytes::complete::take(len)(rest)?;

        // Character data is 8-bit ASCII; when the MSB is set the remaining 7
        // bits denote a special symbol rather than a character, so decode
        // lossily instead of rejecting the line.
        lines.push(String::from_utf8_lossy(body).into_owned());
        input = rest;

        if lines.len() > MAX_LINES_PER_PAGE {
            warn!(
                "Tabular page exceeded the documented maximum of {MAX_LINES_PER_PAGE} lines; \
                 continuing to the end-of-page flag"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hw(values: &[i16]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_be_bytes()).collect()
    }

    /// A valid 18-byte message header block (Figure 3-3).
    fn second_message_header() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&62i16.to_be_bytes()); // code: Storm Structure
        b.extend_from_slice(&1i16.to_be_bytes()); // date
        b.extend_from_slice(&0i32.to_be_bytes()); // time
        b.extend_from_slice(&0i32.to_be_bytes()); // length
        b.extend_from_slice(&0i16.to_be_bytes()); // source
        b.extend_from_slice(&0i16.to_be_bytes()); // dest
        b.extend_from_slice(&1i16.to_be_bytes()); // nblocks
        b
    }

    /// A valid 102-byte product description block (Figure 3-6 sheets 1-2).
    fn second_product_description() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&(-1i16).to_be_bytes()); // divider
        b.extend_from_slice(&42968i32.to_be_bytes()); // latitude
        b.extend_from_slice(&(-88551i32).to_be_bytes()); // longitude
        b.extend_from_slice(&1022i16.to_be_bytes()); // height
        b.extend_from_slice(&62i16.to_be_bytes()); // product code
        b.extend_from_slice(&1i16.to_be_bytes()); // operational mode
        b.extend_from_slice(&35i16.to_be_bytes()); // vcp
        b.extend_from_slice(&0i16.to_be_bytes()); // sequence number
        b.extend_from_slice(&1i16.to_be_bytes()); // volume scan number
        b.extend_from_slice(&1i16.to_be_bytes()); // volume scan date
        b.extend_from_slice(&0i32.to_be_bytes()); // volume scan time
        b.extend_from_slice(&1i16.to_be_bytes()); // product date
        b.extend_from_slice(&0i32.to_be_bytes()); // product time
        b.extend_from_slice(&[0u8; 4]); // halfwords 27-28
        b.extend_from_slice(&1i16.to_be_bytes()); // elevation number
        b.extend_from_slice(&[0u8; 2]); // halfword 30
        b.extend_from_slice(&[0u8; 32]); // threshold data
        b.extend_from_slice(&[0u8; 14]); // halfwords 47-53
        b.push(0); // version
        b.push(0); // spot blank
        b.extend_from_slice(&0i32.to_be_bytes()); // offset symbology
        b.extend_from_slice(&0i32.to_be_bytes()); // offset graphic
        b.extend_from_slice(&60i32.to_be_bytes()); // offset tabular
        assert_eq!(b.len(), 102);
        b
    }

    /// Builds a tabular block whose pages hold the given lines.
    fn block(pages: &[&[&str]]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&hw(&[-1])); // data divider
        body.extend_from_slice(&hw(&[pages.len() as i16]));
        for page in pages {
            for line in *page {
                body.extend_from_slice(&hw(&[line.len() as i16]));
                body.extend_from_slice(line.as_bytes());
            }
            body.extend_from_slice(&hw(&[-1])); // end of page
        }

        let mut bytes = hw(&[-1, 3]);
        let inner = second_message_header().len() + second_product_description().len() + body.len();
        bytes.extend_from_slice(&((8 + inner) as i32).to_be_bytes());
        bytes.extend_from_slice(&second_message_header());
        bytes.extend_from_slice(&second_product_description());
        bytes.extend_from_slice(&body);
        bytes
    }

    #[test]
    fn parses_a_single_page_of_lines() {
        let bytes = block(&[&["STORM STRUCTURE", "CELL  BASE  TOP", "A1     1.2  25.0"]]);

        let (rest, parsed) = tabular_alphanumeric(&bytes).unwrap();
        assert!(rest.is_empty());
        assert_eq!(parsed.id, 3);
        assert_eq!(parsed.num_pages, 1);
        assert_eq!(parsed.pages.len(), 1);
        assert_eq!(
            parsed.pages[0].lines,
            vec!["STORM STRUCTURE", "CELL  BASE  TOP", "A1     1.2  25.0"]
        );
    }

    #[test]
    fn parses_the_embedded_second_header_blocks() {
        let bytes = block(&[&["X"]]);
        let (_, parsed) = tabular_alphanumeric(&bytes).unwrap();

        assert_eq!(parsed.message_header.code, crate::MessageCode::StormStructure);
        assert_eq!(parsed.product_description.product_code, 62);
        assert_eq!(parsed.product_description.offset_tabular, 60);
    }

    #[test]
    fn parses_multiple_pages() {
        let bytes = block(&[&["PAGE ONE LINE"], &["PAGE TWO LINE A", "PAGE TWO LINE B"]]);

        let (rest, parsed) = tabular_alphanumeric(&bytes).unwrap();
        assert!(rest.is_empty());
        assert_eq!(parsed.pages.len(), 2);
        assert_eq!(parsed.pages[0].lines.len(), 1);
        assert_eq!(parsed.pages[1].lines, vec!["PAGE TWO LINE A", "PAGE TWO LINE B"]);
    }

    #[test]
    fn handles_an_empty_page() {
        let bytes = block(&[&[]]);
        let (_, parsed) = tabular_alphanumeric(&bytes).unwrap();
        assert!(parsed.pages[0].lines.is_empty());
    }

    #[test]
    fn handles_a_zero_length_line() {
        let bytes = block(&[&["", "after the blank"]]);
        let (_, parsed) = tabular_alphanumeric(&bytes).unwrap();
        assert_eq!(parsed.pages[0].lines, vec!["", "after the blank"]);
    }

    #[test]
    fn rejects_a_block_with_the_wrong_id() {
        let mut bytes = hw(&[-1, 2]); // ID 2 is the graphic block
        bytes.extend_from_slice(&16i32.to_be_bytes());
        assert!(tabular_alphanumeric(&bytes).is_err());
    }

    #[test]
    fn rejects_a_line_longer_than_the_input() {
        let mut bytes = hw(&[-1, 3]);
        bytes.extend_from_slice(&200i32.to_be_bytes());
        bytes.extend_from_slice(&second_message_header());
        bytes.extend_from_slice(&second_product_description());
        bytes.extend_from_slice(&hw(&[-1, 1])); // divider, 1 page
        bytes.extend_from_slice(&hw(&[80])); // line claims 80 characters
        bytes.extend_from_slice(b"short");
        assert!(tabular_alphanumeric(&bytes).is_err());
    }

    /// A page that never terminates must report the end of input rather than
    /// looping forever.
    #[test]
    fn rejects_a_page_missing_its_end_flag() {
        let mut bytes = hw(&[-1, 3]);
        bytes.extend_from_slice(&200i32.to_be_bytes());
        bytes.extend_from_slice(&second_message_header());
        bytes.extend_from_slice(&second_product_description());
        bytes.extend_from_slice(&hw(&[-1, 1]));
        bytes.extend_from_slice(&hw(&[1]));
        bytes.push(b'A');
        // no -1 end-of-page flag, and no more input
        assert!(tabular_alphanumeric(&bytes).is_err());
    }
}
