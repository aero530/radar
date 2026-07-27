//! Integration test for the Graphic and Tabular Alphanumeric blocks reached
//! through the public `Radar::from_vec` API.
//!
//! The unit tests in `src/graphic_alphanumeric` and `src/tabular_alphanumeric`
//! cover the block layouts themselves. What this file checks is the wiring:
//! both blocks are located by the halfword offsets in the Product Description
//! Block (halfwords 55-60, Figure 3-6 sheet 2), which are counted "from the
//! beginning of the message header (halfword 1)". Getting that arithmetic
//! wrong would silently produce `None` or garbage, so it is exercised here
//! against a file whose block positions are known exactly.

use radar::{MessageCode, Radar, SymPacketData};

fn hw(values: &[i16]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_be_bytes()).collect()
}

/// A text packet with no colour value (packet code 1, Figure 3-8b sheet 1).
fn text_packet_1(i: i16, j: i16, text: &str) -> Vec<u8> {
    let mut body = hw(&[i, j]);
    body.extend_from_slice(text.as_bytes());
    let mut packet = hw(&[1, body.len() as i16]);
    packet.extend_from_slice(&body);
    packet
}

/// A text packet with a uniform colour value (packet code 8, sheet 2).
fn text_packet_8(value: i16, i: i16, j: i16, text: &str) -> Vec<u8> {
    let mut body = hw(&[value, i, j]);
    body.extend_from_slice(text.as_bytes());
    let mut packet = hw(&[8, body.len() as i16]);
    packet.extend_from_slice(&body);
    packet
}

/// A symbology block holding a single layer with one text packet.
fn symbology_block() -> Vec<u8> {
    let packet = text_packet_1(10, 20, "HI");
    let layer_len = packet.len() as i32;
    let block_len = 10 + 6 + layer_len;

    let mut bytes = hw(&[-1, 1]); // divider, block id 1
    bytes.extend_from_slice(&block_len.to_be_bytes());
    bytes.extend_from_slice(&hw(&[1])); // one layer
    bytes.extend_from_slice(&hw(&[-1])); // layer divider
    bytes.extend_from_slice(&layer_len.to_be_bytes());
    bytes.extend_from_slice(&packet);
    bytes
}

/// A graphic alphanumeric block with one page holding one text packet.
///
/// The text is an even number of characters so that the block stays on a
/// halfword boundary, which is what the format requires of every block.
fn graphic_block() -> Vec<u8> {
    let packet = text_packet_8(3, 0, 0, "CELL A1  TVS");
    let mut page = hw(&[1, packet.len() as i16]); // page number, length of page
    page.extend_from_slice(&packet);

    let block_len = 10 + page.len() as i32;
    let mut bytes = hw(&[-1, 2]); // divider, block id 2
    bytes.extend_from_slice(&block_len.to_be_bytes());
    bytes.extend_from_slice(&hw(&[1])); // one page
    bytes.extend_from_slice(&page);
    bytes
}

/// The 18 byte message header block repeated inside the tabular block.
fn second_message_header() -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&109i16.to_be_bytes()); // Storm Total Rainfall alpha block
    b.extend_from_slice(&1i16.to_be_bytes()); // date
    b.extend_from_slice(&0i32.to_be_bytes()); // time
    b.extend_from_slice(&0i32.to_be_bytes()); // length
    b.extend_from_slice(&0i16.to_be_bytes()); // source
    b.extend_from_slice(&0i16.to_be_bytes()); // dest
    b.extend_from_slice(&1i16.to_be_bytes()); // nblocks
    b
}

/// A 102 byte product description block.
fn product_description_block(
    product_code: i16,
    version: u8,
    offset_symbology: i32,
    offset_graphic: i32,
    offset_tabular: i32,
) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&(-1i16).to_be_bytes()); // divider
    b.extend_from_slice(&42968i32.to_be_bytes()); // latitude
    b.extend_from_slice(&(-88551i32).to_be_bytes()); // longitude
    b.extend_from_slice(&1022i16.to_be_bytes()); // height
    b.extend_from_slice(&product_code.to_be_bytes());
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
    b.push(version);
    b.push(0); // spot blank
    b.extend_from_slice(&offset_symbology.to_be_bytes());
    b.extend_from_slice(&offset_graphic.to_be_bytes());
    b.extend_from_slice(&offset_tabular.to_be_bytes());
    assert_eq!(b.len(), 102, "product description must be 102 bytes");
    b
}

/// A tabular alphanumeric block with the given pages of lines.
fn tabular_block(pages: &[&[&str]]) -> Vec<u8> {
    let mut data = hw(&[-1]); // divider between the repeated headers and the data
    data.extend_from_slice(&hw(&[pages.len() as i16]));
    for page in pages {
        for line in *page {
            data.extend_from_slice(&hw(&[line.len() as i16]));
            data.extend_from_slice(line.as_bytes());
        }
        data.extend_from_slice(&hw(&[-1])); // end of page
    }

    let header = second_message_header();
    // The repeated product description is not itself interpreted, so any
    // well-formed 102 byte block will do.
    let description = product_description_block(109, 0, 0, 0, 0);
    let block_len = (8 + header.len() + description.len() + data.len()) as i32;

    let mut bytes = hw(&[-1, 3]); // divider, block id 3
    bytes.extend_from_slice(&block_len.to_be_bytes());
    bytes.extend_from_slice(&header);
    bytes.extend_from_slice(&description);
    bytes.extend_from_slice(&data);
    bytes
}

/// Assembles a complete file. Product code 80 (Storm Total Rainfall
/// Accumulation) is used because it is in the supported-product list and is
/// one of the products the ICD documents as carrying a tabular block.
fn build_file(include_graphic: bool, include_tabular: bool, pages: &[&[&str]]) -> Vec<u8> {
    let symbology = symbology_block();
    let graphic = if include_graphic { graphic_block() } else { Vec::new() };
    let tabular = if include_tabular { tabular_block(pages) } else { Vec::new() };

    // Offsets are counted in halfwords, so every block must start on a
    // halfword boundary for the following block's offset to be expressible.
    assert_eq!(symbology.len() % 2, 0, "symbology block must be halfword aligned");
    assert_eq!(graphic.len() % 2, 0, "graphic block must be halfword aligned");

    // Offsets are halfword counts from the start of the message header, which
    // begins 30 bytes into the file. The symbology block always starts
    // immediately after the 60 halfwords of message header + product
    // description, so its offset is always 60.
    let offset_symbology = 60i32;
    let offset_graphic = if include_graphic {
        60 + (symbology.len() / 2) as i32
    } else {
        0
    };
    let offset_tabular = if include_tabular {
        60 + ((symbology.len() + graphic.len()) / 2) as i32
    } else {
        0
    };

    let mut file = b"SDUS53 KMKX 091253\r\r\nN0RMKX\r\r\n".to_vec();
    assert_eq!(file.len(), 30, "text header must be 30 bytes");

    // Message header block (Figure 3-3)
    file.extend_from_slice(&80i16.to_be_bytes()); // Storm Total Rainfall Accumulation
    file.extend_from_slice(&1i16.to_be_bytes()); // date
    file.extend_from_slice(&0i32.to_be_bytes()); // time
    file.extend_from_slice(&0i32.to_be_bytes()); // length
    file.extend_from_slice(&0i16.to_be_bytes()); // source
    file.extend_from_slice(&0i16.to_be_bytes()); // dest
    file.extend_from_slice(&4i16.to_be_bytes()); // nblocks

    // Product code 80 supports version <= 1.
    file.extend_from_slice(&product_description_block(
        80,
        1,
        offset_symbology,
        offset_graphic,
        offset_tabular,
    ));
    assert_eq!(file.len(), 150, "header section must be 150 bytes");

    file.extend_from_slice(&symbology);
    file.extend_from_slice(&graphic);
    file.extend_from_slice(&tabular);
    file
}

#[test]
fn locates_and_parses_a_tabular_block_from_its_offset() {
    let file = build_file(false, true, &[&["STORM TOTAL RAINFALL", "A1   1.25 IN"]]);
    let (_, radar) = Radar::from_vec(file).expect("file should parse");

    assert_eq!(radar.message_header.code, MessageCode::StormTotalRainfallAccumulation);
    assert!(radar.graphic.is_none(), "no graphic block was declared");

    let tabular = radar.tabular.expect("tabular block should be located and parsed");
    assert_eq!(tabular.id, 3);
    assert_eq!(tabular.num_pages, 1);
    assert_eq!(
        tabular.pages[0].lines,
        vec!["STORM TOTAL RAINFALL", "A1   1.25 IN"]
    );
    // The repeated header inside the block is decoded too.
    assert_eq!(
        tabular.message_header.code,
        MessageCode::StormTotalRainfallAccumulationAlphanumericBlock
    );
}

#[test]
fn locates_and_parses_a_graphic_block_from_its_offset() {
    let file = build_file(true, false, &[]);
    let (_, radar) = Radar::from_vec(file).expect("file should parse");

    assert!(radar.tabular.is_none(), "no tabular block was declared");

    let graphic = radar.graphic.expect("graphic block should be located and parsed");
    assert_eq!(graphic.id, 2);
    assert_eq!(graphic.num_pages, 1);
    assert_eq!(graphic.pages.len(), 1);
    assert!(
        graphic.pages[0].undecoded.is_empty(),
        "the page's text packets should all decode"
    );
    match &graphic.pages[0].packets[0] {
        SymPacketData::TextAndSpecialSymbol8(t) => {
            assert_eq!(t.text, "CELL A1  TVS");
            assert_eq!(t.color_level, Some(3));
        }
        other => panic!("expected a text packet, got {other:?}"),
    }
}

/// Both blocks present at once: each must be found at its own offset, which
/// only works if the halfword-to-byte conversion is right for both.
#[test]
fn locates_both_blocks_when_both_are_present() {
    let file = build_file(true, true, &[&["LINE ONE"], &["LINE TWO"]]);
    let (_, radar) = Radar::from_vec(file).expect("file should parse");

    let graphic = radar.graphic.expect("graphic block should be located");
    assert_eq!(graphic.id, 2);

    let tabular = radar.tabular.expect("tabular block should be located");
    assert_eq!(tabular.id, 3);
    assert_eq!(tabular.pages.len(), 2);
    assert_eq!(tabular.pages[0].lines, vec!["LINE ONE"]);
    assert_eq!(tabular.pages[1].lines, vec!["LINE TWO"]);
}

/// A zero offset means the product does not carry the block at all, and must
/// not be mistaken for "offset 0".
#[test]
fn zero_offsets_mean_no_block() {
    let file = build_file(false, false, &[]);
    let (_, radar) = Radar::from_vec(file).expect("file should parse");

    assert!(radar.symbology.is_some());
    assert!(radar.graphic.is_none());
    assert!(radar.tabular.is_none());
}

/// An offset pointing past the end of the payload is reported as a missing
/// block rather than panicking or failing the whole product.
#[test]
fn an_out_of_range_offset_yields_no_block_rather_than_an_error() {
    let symbology = symbology_block();
    let mut file = b"SDUS53 KMKX 091253\r\r\nN0RMKX\r\r\n".to_vec();
    file.extend_from_slice(&80i16.to_be_bytes());
    file.extend_from_slice(&1i16.to_be_bytes());
    file.extend_from_slice(&0i32.to_be_bytes());
    file.extend_from_slice(&0i32.to_be_bytes());
    file.extend_from_slice(&0i16.to_be_bytes());
    file.extend_from_slice(&0i16.to_be_bytes());
    file.extend_from_slice(&4i16.to_be_bytes());
    // Declare a tabular block far beyond the end of the file.
    file.extend_from_slice(&product_description_block(80, 1, 60, 0, 30_000));
    file.extend_from_slice(&symbology);

    let (_, radar) = Radar::from_vec(file).expect("the product itself should still parse");
    assert!(radar.symbology.is_some(), "symbology should still be parsed");
    assert!(radar.tabular.is_none(), "the unreachable block should be None");
}
