// cSpell:disable

//! A parser (and rough plotter) for NEXRAD Level 3 weather radar product
//! files.
//!
//! # Format
//!
//! A Level 3 file is a WMO/AWIPS text header ([`text_header`]) followed by
//! a binary Graphic Product Message made of, in order: a message header
//! ([`message_header`]), a product description ([`product_description`]),
//! and then zero or one each of a symbology block, a graphic alphanumeric
//! block, and a tabular alphanumeric block. See the "Format" section of
//! `README.md` for the full block diagram and links to the ICD documents
//! this parser is implemented against.
//!
//! # Usage
//!
//! ```no_run
//! let bytes = std::fs::read("data/sn_DS.p20-r_kmkx.last")?;
//! let (leftover, radar) = radar::Radar::from_vec(bytes)?;
//!
//! println!("{} at {}", radar.text_header.location, radar.message_header.datetime);
//! # Ok::<(), radar::Error>(())
//! ```
//!
//! # Status
//!
//! All three blocks are parsed, as is every symbology display data packet
//! defined in Figures 3-7 through 3-15c — including the map overlay packets of
//! Figure 3-9 and the XDR-encoded Generic Data packet (codes 28 and 29) with
//! all six Appendix E component types. Data levels decode to physical values
//! through every encoding Note 1 of Figure 3-6 defines.
//!
//! Parsing a file that needs something unimplemented returns an [`Error`]
//! rather than panicking. See `README.md` for the current list of what is
//! and isn't supported, and for the field-by-field spec conformance table.
//!
//! # Plotting
//!
//! [`Radar::plot`] renders the first symbology layer to a PNG with an
//! annotation panel and a colour bar; [`Radar::plot_with`] takes
//! [`PlotOptions`] to choose the [`ColorRamp`], the site label, and the image
//! size. Digital data arrays are coloured by decoding their levels to physical
//! values where [`ProductDescription::level_scaling`] knows how, so
//! reflectivity products plot against a real dBZ scale.

#[macro_use]
extern crate num_derive;

use serde::{Deserialize, Serialize};
use std::io::Read;

use bzip2::bufread::BzDecoder;
use tracing::{info, warn};

mod codes;
pub use codes::{ColorTable, MessageCode, PacketCode, FALLBACK_GRAY};

mod color_ramp;
pub use color_ramp::{ColorRamp, NWS_REFLECTIVITY_STOPS, RANGE_FOLDED};

mod level_scaling;
pub use level_scaling::{
    int16_to_float16, LevelDecoding, LevelScaling, LevelThreshold, Qualifier, ThresholdCode,
};

mod message_header;
pub use message_header::{message_header, MessageHeader};

mod product_description;
pub use product_description::{product_description, ProductDescription};

mod product_symbology;
pub use product_symbology::*;

mod graphic_alphanumeric;
pub use graphic_alphanumeric::{graphic_alphanumeric, GraphicBlock, GraphicPage};

mod tabular_alphanumeric;
pub use tabular_alphanumeric::{tabular_alphanumeric, TabularBlock, TabularPage};

mod text_header;
pub use text_header::{text_header, TextHeader};

mod plot;
pub use plot::PlotOptions;

mod error_r;
pub use error_r::Error;

/// A fully parsed NEXRAD Level 3 product: the text header, binary message
/// header, product description, and (if present) the symbology, graphic,
/// and tabular alphanumeric blocks that follow.
///
/// Build one with [`Radar::from_vec`].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Radar {
    pub text_header: TextHeader,
    pub message_header: MessageHeader,
    pub product_description: ProductDescription,
    pub symbology: Option<SymbologyBlock>,
    pub graphic: Option<GraphicBlock>,
    pub tabular: Option<TabularBlock>,
}

/// Number of bytes from the start of the file to the start of the message
/// header: the WMO/AWIPS text header is always exactly this long.
const TEXT_HEADER_LEN: usize = 30;

/// Number of bytes from the start of the file to the data that follows the
/// fixed headers — the text header plus the 18 byte message header plus the
/// 102 byte product description block.
const HEADER_SECTION_LEN: usize = TEXT_HEADER_LEN + 18 + 102;

/// Locates a block by its product-description halfword offset and parses it.
///
/// Per Figure 3-6 sheet 2, halfwords 55-60 hold offsets "from the beginning of
/// the message header (halfword 1) to the (-1) divider of each block". The
/// message header starts [`TEXT_HEADER_LEN`] bytes into the file and
/// `remaining_file` starts at [`HEADER_SECTION_LEN`], so a halfword offset
/// maps to `offset * 2 - (HEADER_SECTION_LEN - TEXT_HEADER_LEN)` bytes into
/// `remaining_file`.
///
/// Parsing is best effort: a block that cannot be located or decoded is logged
/// and reported as `None` rather than failing the whole product, since these
/// blocks are supplementary to the symbology data.
fn parse_offset_block<T, F>(
    remaining_file: &[u8],
    halfword_offset: i32,
    name: &str,
    parser: F,
) -> Option<T>
where
    F: for<'a> Fn(&'a [u8]) -> nom::IResult<&'a [u8], T>,
{
    let header_bytes = (HEADER_SECTION_LEN - TEXT_HEADER_LEN) as i64;
    let byte_offset = (halfword_offset as i64) * 2 - header_bytes;
    if byte_offset < 0 {
        warn!("{name} block offset {halfword_offset} falls inside the fixed header section");
        return None;
    }

    let Some(slice) = remaining_file.get(byte_offset as usize..) else {
        warn!(
            "{name} block offset {halfword_offset} is past the end of the {} byte payload",
            remaining_file.len()
        );
        return None;
    };

    match parser(slice) {
        Ok((_, block)) => Some(block),
        Err(e) => {
            warn!("Could not decode the {name} block: {e:?}");
            None
        }
    }
}

impl Radar {
    /// Parses the fixed-size header section (text header + message header +
    /// product description, always 150 bytes) and, if the product
    /// description declares one, the symbology block from `remaining_file`
    /// (already BZ-decompressed by the caller if necessary).
    fn parse<'a>(header_section: &'a [u8], remaining_file: &'a [u8]) -> Result<(&'a [u8], Radar), Error> {
        info!("File is {:?} bytes.", header_section.len());
        info!("Decode is {:?} bytes.", remaining_file.len());
        
        // Text header
        let (input_header, text_header) = text_header(header_section)?;

        // Read and decode 18 byte Message Header Block
        let (input_header, message_header) = message_header(input_header)?;

        // fail if code is not in supported products list
        if !message_header.code.is_supported_product() {
            return Err(error_r::Error::ProductType(message_header.code))
        };

        // Read and decode 102 byte Product Description Block
        let (input_header, product_description) = product_description(input_header)?;

        if !input_header.is_empty() {
            warn!("Header leftovers: {:?}", input_header);
        }
        
        info!("{:?}", product_description);

        // Check product version number
        // if there is a supported version of this product type BUT (and) the product version is greater than the supported version
        if message_header.code.supported_version().is_some_and(|supported_version| product_description.version > supported_version) {
            return Err(Error::SupportedVersion(product_description.version, message_header.code.supported_version()))
        }

        // ---------------------------
        // File contents after header
        // ---------------------------

        let (input_data, symbology) = if product_description.offset_symbology > 0 {
            let (input_data, symbology) = symbology(remaining_file)?;
            (input_data, Some(symbology))
        } else {
            (remaining_file, None)
        };

        // The graphic and tabular blocks are located by their own halfword
        // offsets rather than by following on from the symbology block, so
        // they are seeked to directly.
        let graphic = if product_description.offset_graphic > 0 {
            parse_offset_block(
                remaining_file,
                product_description.offset_graphic,
                "graphic alphanumeric",
                graphic_alphanumeric,
            )
        } else {
            None
        };

        let tabular = if product_description.offset_tabular > 0 {
            parse_offset_block(
                remaining_file,
                product_description.offset_tabular,
                "tabular alphanumeric",
                tabular_alphanumeric,
            )
        } else {
            None
        };

        Ok((
            input_data,
            Radar {
                text_header,
                message_header,
                product_description,
                symbology,
                graphic,
                tabular,
            },
        ))
    }


    /// Parses a complete NEXRAD Level 3 file.
    ///
    /// Returns the parsed [`Radar`] along with any trailing bytes that
    /// followed the last block this crate knows how to parse (empty for a
    /// fully-consumed, fully-supported file).
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] — never panics — if `file` is too short to
    /// contain the fixed-size text/message/product-description headers, if
    /// those headers are malformed, if the product type or version isn't
    /// supported, or if the symbology block uses a packet code this crate
    /// doesn't implement yet.
    pub fn from_vec(mut file: Vec<u8>) -> Result<(Vec<u8>, Radar), Error> {
        if file.len() < HEADER_SECTION_LEN {
            return Err(Error::TooShort {
                expected: HEADER_SECTION_LEN,
                actual: file.len(),
            });
        }
        let file_after_headers = file.split_off(HEADER_SECTION_LEN);

        // Uncompress symbology block if necessary
        let decomp_vec = if file_after_headers.starts_with(b"BZ") {
            let mut decoder = BzDecoder::new(file_after_headers.as_slice());
            let mut decomp_vec = Vec::new();
            decoder.read_to_end(&mut decomp_vec)?;
            decomp_vec
        } else {
            // combine file after header back on to file???
            file_after_headers
        };

        info!("File is {:?} bytes.", file.len());
        match Radar::parse(&file, &decomp_vec) {
            Ok((leftover, radar)) => {
                Ok((leftover.to_vec(), radar))
            }
            Err(e) => {
                
                Err(e)
            },
        }
    }
}