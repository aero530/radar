// cSpell:disable

#[macro_use]
extern crate num_derive;

use bzip2::{bufread::BzDecoder, Decompress};


use parse_display::{Display, FromStr};
use std::{io::Read, path::PathBuf};
use chrono::{DateTime, Utc};

use nom::{
    bytes::complete::{is_a, tag, take, take_while}, character::{complete::digit0, is_digit}, 
    combinator::{map, map_parser}, 
    multi::many0, sequence::preceded, IResult, *
};
use tracing::{debug, error, info, trace, warn, Level};
use tracing_subscriber::filter::EnvFilter;

mod message_codes;
use message_codes::MessageCode;


#[derive(Clone, Debug, PartialEq)]
pub struct Radar<'a> {
    text_header: TextHeader,
    message_header: MessageHeader,
    product_description: ProductDescription<'a>,
    symbology_header: SymbologyHeader,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextHeader {
    xx: usize,
    location: String,
    timestamp: String,
    aaa: String,
    bbb: String,
}

/// Graphic Product Message: Message Header Block
/// 18 bytes, 9 halfwords
/// Figure 3-3, page 3-7.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MessageHeader {
    /// message code
    code: MessageCode,
    /// date & time of message, days since 1 Jan, 1970 GMT
    datetime: DateTime<Utc>,
    /// length of message in bytes
    length: i32,
    /// Source ID
    source: i16,
    /// Destination ID
    dest: i16,
    /// Number of blocks in the message (inclusive)
    nblocks: i16,
}

/// Graphic Product Message: Product Description Block
/// Description: section 3.3.1.1, page 3-3
/// 102 bytes, 51 halfwords (halfwords 10-60)
/// Figure 3-6, pages 3-24 and 3-25
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProductDescription<'a> {
    /// Delineate blocks, -1
    divider: i16,
    /// Latitude of radar, degrees, + for north
    latitude: i32,
    /// Longitude of radar, degrees, + for east
    longitude: i32,
    /// Height of radar, feet abouve mean sea level
    height: i16,
    /// NEXRAD product code
    product_code: i16,
    /// 0 = Maintenance, 1 = Clean Air, 2 = Precip
    operational_mode: i16,
    /// Volume Coverage Pattern of scan strategy
    vcp: i16,
    /// Sequence Number of the request.
    sequence_num: i16,
    /// Volume Scan number, 1 to 80.
    vol_scan_num: i16,
    /// Volume Scan start date, days since 1/1/1970
    vol_scan_date: i16,
    /// Volume Scan start time, sec since midnight
    vol_scan_time: i32,
    /// Product Generation Date, days since 1/1/1970
    product_date: i16,
    /// Product Generation Time, sec since midnight
    product_time: i32,
    ///  Product dependent parameters 1 and 2 TABLE V (4s)
    halfwords_27_28: &'a [u8],
    /// Elevation number within volume scan
    elevation_num: i16,
    ///  Product dependent parameter 3 --- PRODUCT DEPENDENT PARAMETERS 1 AND 2 (SEE TABLE V) (2s)
    halfwords_30: &'a [u8],
    ///  Data to determine threshold level values --- PRODUCT DEPENDENT (SEE NOTE 1) (32s)
    threshold_data: &'a [u8],
    ///  Product dependent parameters 4-10 --- PRODUCT DEPENDENT PARAMETERS 4 THROUGH 10 (SEE TABLE V, NOTE 3) (14s)
    halfwords_47_53: &'a [u8],
    /// Version, 0
    version: u8,
    /// 1 = Spot blank ON, 0 = Blanking OFF
    spot_blank: u8,
    /// halfword offset to Symbology block
    offet_symbology: i32,
    /// halfword offset to Graphic block
    offset_graphic: i32,
    /// halfword offset to Tabular block
    offset_tabular: i32,
}

/// Graphic Product Message: Product Symbology Block
/// Description
/// 16 byte header
/// Figure 3-6 (Sheet 8), pages 3-40
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SymbologyHeader {
    /// Delineate blocks, -1
    divider: i16,
    /// Block ID, 1
    id: i16,
    /// Length of block in bytes
    block_length: i32,
    /// Number of data layers
    layers: i16,
    /// Delineate data layers, -1
    layer_divider: i16,
    /// Length of data layer in bytes
    layer_length: i32,
}



/// Format of Text header is SDUSXX KYYYY DDHHMM\r\r\nAAABBB\r\r\n
fn text_header(input: &[u8]) -> IResult<&[u8], TextHeader> {
    // remove SDUS from input
    let (input, _) = tag("SDUS".as_bytes())(input)?;

    // grab the digits after SDUD
    let (input, a) = digit0(input)?;
    let xx_str = match std::str::from_utf8(a) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let xx = xx_str.parse::<usize>().unwrap();

    // space
    let (input, _) = tag(" ".as_bytes())(input)?;

    // location
    let (input, part) = take(4usize)(input)?;
    let location = match std::str::from_utf8(part) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // space
    let (input, _) = tag(" ".as_bytes())(input)?;

    // date
    let (input, part) = take(6usize)(input)?;
    let timestamp = match std::str::from_utf8(part) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // linebreaks
    let (input, _) = tag([0x0D, 0x0D, 0x0A])(input)?;

    // other bits
    let (input, part) = take(3usize)(input)?;
    let aaa = match std::str::from_utf8(part) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let (input, part) = take(3usize)(input)?;
    let bbb = match std::str::from_utf8(part) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // linebreaks
    let (input, _) = tag([0x0D, 0x0D, 0x0A])(input)?;

    Ok((
        input,
        TextHeader {
            xx,
            location,
            timestamp,
            aaa: aaa.clone(),
            bbb: bbb.clone(),
        },
    ))
}


/// Graphic Product Message: Message Header Block
/// 18 bytes, 9 halfwords
/// Figure 3-3, page 3-7.
fn message_header(input: &[u8]) -> IResult<&[u8], MessageHeader> {

    let (input, c) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let code = <MessageCode as num::FromPrimitive>::from_i16(c);

    let (input, days) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let (input, seconds) = number::complete::i32(nom::number::Endianness::Big)(input)?;
    let datetime = DateTime::from_timestamp((days as i64)*24*60*60 + (seconds as i64), 0).unwrap_or_default();

    let (input, length) = number::complete::i32(nom::number::Endianness::Big)(input)?;
    let (input, source) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let (input, dest) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let (input, nblocks) = number::complete::i16(nom::number::Endianness::Big)(input)?;

    Ok((
        input,
        MessageHeader {
            code: code.unwrap_or_default(),
            datetime,
            length,
            source,
            dest,
            nblocks,
        },
    ))
}


/// Graphic Product Message: Message Header Block
/// 18 bytes, 9 halfwords
/// Figure 3-3, page 3-7.
fn product_description(input: &[u8]) -> IResult<&[u8], ProductDescription> {


    let (input, divider) = number::complete::i16(nom::number::Endianness::Big)(input)?; //: i16,
    let (input, latitude) = number::complete::i32(nom::number::Endianness::Big)(input)?; //: i32,
    let (input, longitude) = number::complete::i32(nom::number::Endianness::Big)(input)?; //: i32,
    let (input, height) = number::complete::i16(nom::number::Endianness::Big)(input)?; //: i16,
    let (input, product_code) = number::complete::i16(nom::number::Endianness::Big)(input)?; //: i16,
    let (input, operational_mode) = number::complete::i16(nom::number::Endianness::Big)(input)?; //: i16,
    let (input, vcp) = number::complete::i16(nom::number::Endianness::Big)(input)?; //: i16,
    let (input, sequence_num) = number::complete::i16(nom::number::Endianness::Big)(input)?; //: i16,
    let (input, vol_scan_num) = number::complete::i16(nom::number::Endianness::Big)(input)?; //: i16,
    let (input, vol_scan_date) = number::complete::i16(nom::number::Endianness::Big)(input)?; //: i16,
    let (input, vol_scan_time) = number::complete::i32(nom::number::Endianness::Big)(input)?; //: i32,
    let (input, product_date) = number::complete::i16(nom::number::Endianness::Big)(input)?; //: i16,
    let (input, product_time) = number::complete::i32(nom::number::Endianness::Big)(input)?; //: i32,
    let (input, halfwords_27_28) = take(4usize)(input)?; //: [u8;4],
    let (input, elevation_num) = number::complete::i16(nom::number::Endianness::Big)(input)?; //: i16,
    let (input, halfwords_30) = take(2usize)(input)?; //: [u8;2],
    let (input, threshold_data) = take(32usize)(input)?; //: [u8;32],
    let (input, halfwords_47_53) = take(14usize)(input)?; //: [u8;14],
    let (input, version) = take(1usize)(input)?; //: u8,
    let (input, spot_blank) = take(1usize)(input)?; //: u8,
    let (input, offet_symbology) = number::complete::i32(nom::number::Endianness::Big)(input)?; //: i32,
    let (input, offset_graphic) = number::complete::i32(nom::number::Endianness::Big)(input)?; //: i32,
    let (input, offset_tabular) = number::complete::i32(nom::number::Endianness::Big)(input)?; //: i32,

    // let (input, c) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    // let code = <MessageCode as num::FromPrimitive>::from_i16(c);

    // let (input, days) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    // let (input, seconds) = number::complete::i32(nom::number::Endianness::Big)(input)?;
    // let datetime = DateTime::from_timestamp((days as i64)*24*60*60 + (seconds as i64), 0).unwrap_or_default();

    Ok((
        input,
        ProductDescription {
            divider,
            latitude,
            longitude,
            height,
            product_code,
            operational_mode,
            vcp,
            sequence_num,
            vol_scan_num,
            vol_scan_date,
            vol_scan_time,
            product_date,
            product_time,
            halfwords_27_28,
            elevation_num,
            halfwords_30,
            threshold_data,
            halfwords_47_53,
            version: version[0],
            spot_blank: spot_blank[0],
            offet_symbology,
            offset_graphic,
            offset_tabular,
        },
    ))
}




/// Graphic Product Message: Product Symbology Block
/// Description
/// 16 byte header
/// Figure 3-6 (Sheet 8), pages 3-40
fn symbology_header(input: &[u8]) -> IResult<&[u8], SymbologyHeader> {
        
    let (input, divider) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let (input, id) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let (input, block_length) = number::complete::i32(nom::number::Endianness::Big)(input)?;
    let (input, layers) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let (input, layer_divider) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let (input, layer_length) = number::complete::i32(nom::number::Endianness::Big)(input)?;

    Ok((
        input,
        SymbologyHeader {
            divider,
            id,
            block_length,
            layers,
            layer_divider,
            layer_length,
        },
    ))
}

fn parse<'a>(input: &'a [u8], decomp_input: &'a [u8]) -> IResult<&'a [u8], Radar<'a>> {

    // Text header
    let (input, text_header) = text_header(input)?;

    // Read and decode 18 byte Message Header Block
    let (input, message_header) = message_header(input)?;
        
    // fail if code is not in supported products list
    if !message_header.code.is_supported_product() {
        let e = nom::error::Error::new(input, error::ErrorKind::Fail);
        error!("Product type {:?} is not supported", message_header.code);
        return Err(nom::Err::Failure(e));
    };

    // Read and decode 102 byte Product Description Block
    let (input, product_description) = product_description(input)?;

    // Check product version number
    // if there is a supported version of this product type BUT (and) the product version is greater than the supported version
    if message_header.code.supported_version().is_some_and(|supported_version| product_description.version > supported_version) {
        let e = nom::error::Error::new(input, error::ErrorKind::Fail);
        error!("Product version is {:?} but currently only version <= {:?} are supported", product_description.version, message_header.code.supported_version().unwrap());
        return Err(nom::Err::Failure(e));
    }


    // Read and decode symbology header (check packet code)
    let (input, symbology_header) = if decomp_input.len()>0 {
        symbology_header(decomp_input)?
    } else {
        symbology_header(input)?
    };


    //
    //
    // Read data
    //
    //

    
    Ok((
        input,
        Radar {
            text_header,
            message_header,
            product_description,
            symbology_header,
        },
    ))
}

fn main() {
    let filter = EnvFilter::builder()
        .with_default_directive(Level::TRACE.into())
        .from_env()
        .unwrap_or_default(); // set noisy logs to info
                              // .add_directive("hyper::client=INFO".parse()?);

    let subscriber = tracing_subscriber::fmt()
        .with_ansi(true)
        .with_env_filter(filter);

    subscriber.init();

    error!("Designates very serious errors.");
    warn!("Designates hazardous situations.");
    info!("Designates useful information.");
    debug!("Designates lower priority information.");
    trace!("Designates very low priority, often extremely verbose, information.");

    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => "".to_string(),
    };
    let filename = PathBuf::from(&path);
    let mut file = std::fs::read(&filename).unwrap_or_default();

    let file_after_headers = file.split_off(150);


    // Uncompress symbology block if necessary
    let mut decomp_vec : Vec<u8> = Vec::new();
    if file_after_headers[0..1] == "BZ".as_bytes()[0..1] {
        let mut decoder = BzDecoder::new(file_after_headers.as_slice());
        let q = decoder.read_to_end(&mut decomp_vec);

        info!("Decompression {:?}", q);
        
        info!("{:?}", decomp_vec);
    };    
    

    match parse(&file, &decomp_vec ) {
        Ok((leftover, value)) => {
            // warn!("Unmatched {:?}", leftover);
            info!("{:?}", value)
        }
        Err(e) => error!("{:?}", e),
    }
}


