// cSpell:disable

#[macro_use]
extern crate num_derive;

use serde::{Deserialize, Serialize};
use std::{fs::File, io::{Read, Write}, path::PathBuf};

use bzip2::bufread::BzDecoder;
use nom::{error, IResult};
use tracing::{debug, error, info, trace, warn, Level};
use tracing_subscriber::filter::EnvFilter;

mod codes;
use codes::MessageCode;

mod message_header;
use message_header::{message_header, MessageHeader};

mod product_description;
use product_description::{product_description, ProductDescription};

mod product_symbology;
use product_symbology::{symbology, SymbologyBlock};

mod graphic_alphanumeric;
use graphic_alphanumeric::GraphicBlock;

mod tabular_alphanumeric;
use tabular_alphanumeric::TabularBlock;

mod text_header;
use text_header::{text_header, TextHeader};

mod plot;
use plot::plot;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Radar {
    text_header: TextHeader,
    message_header: MessageHeader,
    product_description: ProductDescription,
    symbology: Option<SymbologyBlock>,
    graphic: Option<GraphicBlock>,
    tabular: Option<TabularBlock>,
}



fn parse<'a>(header_section: &'a [u8], remaining_file: &'a [u8]) -> IResult<&'a [u8], Radar> {
    info!("File is {:?} bytes.", header_section.len());
    info!("Decode is {:?} bytes.", remaining_file.len());
    
    // Text header
    let (input, text_header) = text_header(header_section)?;

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

    info!("{:?}", product_description);

    // Check product version number
    // if there is a supported version of this product type BUT (and) the product version is greater than the supported version
    if message_header.code.supported_version().is_some_and(|supported_version| product_description.version > supported_version) {
        let e = nom::error::Error::new(input, error::ErrorKind::Fail);
        error!("Product version is {:?} but currently only version <= {:?} are supported", product_description.version, message_header.code.supported_version().unwrap());
        return Err(nom::Err::Failure(e));
    }


    let (input, symbology) = if product_description.offset_symbology > 0 {
        let (input, symbology) = symbology(remaining_file)?;
        (input, Some(symbology)) 
    } else {
        (remaining_file, None)
    };
    

    if product_description.offset_graphic > 0 {
        warn!("Should decode graphic alphanumeric block");
    }
    if product_description.offset_tabular > 0 {
        warn!("Should decode tabular alphanumeric block");
    }

    Ok((
        input,
        Radar {
            text_header,
            message_header,
            product_description,
            symbology,
            graphic: None,
            tabular: None,
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
    } else {
        // combine file after header back on to file???
        decomp_vec = file_after_headers;
    };
    
    info!("File is {:?} bytes.", file.len());
    match parse(&file, &decomp_vec) {
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

            let _ = plot(value.symbology.unwrap(), value.message_header.code);
        }
        Err(e) => {
            error!("{:?}", e);
            error!("Something didnt work")
        },
    }
}


