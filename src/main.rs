// cSpell:disable

#[macro_use]
extern crate num_derive;

use serde::{Deserialize, Serialize};
use std::{fs::File, io::{Read, Write}, path::PathBuf};

use bzip2::bufread::BzDecoder;
use nom::{
    combinator::peek, IResult, *
};
use tracing::{debug, error, info, trace, warn, Level};
use tracing_subscriber::filter::EnvFilter;

mod codes;
use codes::{MessageCode, PacketCode};

mod message_header;
use message_header::{message_header, MessageHeader};

mod product_description;
use product_description::{product_description, ProductDescription};

mod product_symbology;
use product_symbology::{symbology_header, symbology_block, symbology_block_generic, Symbology, SymbologyHeader};

mod text_header;
use text_header::{text_header, TextHeader};

mod plot;
use plot::plot;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Radar {
    text_header: TextHeader,
    message_header: MessageHeader,
    product_description: ProductDescription,
    symbology_header: SymbologyHeader,
    symbology: Symbology,
}





fn parse<'a>(input: &'a [u8], decomp_input: &'a [u8]) -> IResult<&'a [u8], Radar> {

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

    // let (input, divider) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let (_, pc) = peek(number::complete::i16(nom::number::Endianness::Big))(input)?;
    
    let packet_code = <PacketCode as num::FromPrimitive>::from_i16(pc).unwrap_or_default();
    if !packet_code.is_supported_product() {
        let e = nom::error::Error::new(input, error::ErrorKind::Fail);
        error!("Packet Code is {:?} which is not supported", pc);
        return Err(nom::Err::Failure(e));
    }

    info!("{:?}", packet_code);


    let (input, symbology) = match packet_code {
        PacketCode::GenericData28 => {
            symbology_block_generic(input)?
        },
        PacketCode::RadialDataAF1F | PacketCode::DigitalRadialDataArray => {
            symbology_block(input, packet_code)?
        },
        _ => {
            let e = nom::error::Error::new(input, error::ErrorKind::Fail);
            error!("Packet Code is {:?} which is not supported", pc);
            return Err(nom::Err::Failure(e));
        }
    };

    // //
    // //
    // todo!("Should run sym block for number of layers in sym header");
    // //
    // //


    
    Ok((
        input,
        Radar {
            text_header,
            message_header,
            product_description,
            symbology_header,
            symbology,
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
    };    
    

    match parse(&file, &decomp_vec ) {
        Ok((leftover, value)) => {

            // write to file
            let filepath: &str = "out.json";
            let mut file = File::create(filepath).expect("write error");
            let s = serde_json::to_string(&value).unwrap();
            let _ = file.write_all(s.as_bytes());

            
            let filepath: &str = "out_leftover.json";
            let mut file = File::create(filepath).expect("write error");
            let s = serde_json::to_string(&leftover).unwrap();
            let _ = file.write_all(s.as_bytes());

            let _ = plot(value);
        }
        Err(_e) => {
            //   error!("{:?}", e)
            error!("Something didnt work")
        },
    }

    
}


