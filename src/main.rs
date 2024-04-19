// cSpell:disable

#[macro_use]
extern crate num_derive;



use std::{io::Read, path::PathBuf};


use bzip2::bufread::BzDecoder;
use nom::{
    bytes::complete::take, combinator::peek, multi::count, IResult, *
};
use tracing::{debug, error, info, trace, warn, Level};
use tracing_subscriber::filter::EnvFilter;

mod codes;
use codes::{MessageCode, PacketCode};

mod message_header;
use message_header::{message_header, MessageHeader};

mod product_description;
use product_description::{product_description, ProductDescription};

mod radial;
use radial::{radial_header, radial_packet_header, Radial, RadialData, RadialPacketHeader, RunLevelEncoding};

mod symbology_header;
use symbology_header::{symbology_header, SymbologyHeader};

mod text_header;
use text_header::{text_header, TextHeader};


#[derive(Clone, Debug, PartialEq)]
pub struct Radar<'a> {
    text_header: TextHeader,
    message_header: MessageHeader,
    product_description: ProductDescription<'a>,
    symbology_header: SymbologyHeader,
    symbology: Symbology,
}




#[derive(Clone, Debug, PartialEq)]
pub struct Symbology {
    radials: Vec<Radial>,
}

// fn radial_block(input: (PacketCode, usize, &[u8]) ) -> IResult<(PacketCode, usize, &[u8]), Radial> {
fn radial_block(input: &[u8], packet_code: PacketCode, num_bins: usize) -> IResult<&[u8], Radial> {
    // let (packet_code, num_bins, input) = input;
    let (input, temp_header) = radial_header(input)?;
    let (input, radial) = match packet_code {
        PacketCode::Radial => {
            // Figure 3-11c. Digital Radial Data Array Packet - Packet Code 16
            // page 3-95
            let (input, temp_data) = take(num_bins)(input)?;
            Ok((input, Radial { 
                header: temp_header, 
                data: RadialData::Digital(temp_data.to_vec()),
            }))
        },
        PacketCode::AF1F => {
            // decode run length encoding
            let rle_size = temp_header.num_bytes as usize * 2;
            let (input, rle) = take(rle_size)(input)?;
            // run code then color (4bit ints)
            let data : Vec<RunLevelEncoding>= rle.iter().map(|x| RunLevelEncoding{run: x >> 4, color: x & 0b00001111}).collect();
            Ok((input, Radial { 
                header: temp_header, 
                data: RadialData::AF1F(data),
            }))
        },
        _ => {
            Ok((input, Radial::default()))
            // panic!("How did this happen?")
        }
    }?;
    
    // Ok(((packet_code, num_bins, input), radial))
    Ok((input, radial))
}

fn symbology_block(input: &[u8], packet_code: PacketCode) -> IResult<&[u8], Symbology> {
    let (input, packet_header) = radial_packet_header(input)?;

    let mut num_bins = packet_header.num_bins as usize;
    let num_radials = packet_header.num_radials as usize;

    let (input, temp_header) = peek(radial_header)(input)?;

    match packet_code {
        PacketCode::Radial => {
            if temp_header.num_bytes as usize != num_bins {
                num_bins = temp_header.num_bytes as usize;
            }
        },
        _ => {}
    }

    let (input, radials) = count(|i| radial_block(i, packet_code,num_bins), num_radials)(input)?;

    Ok((input, Symbology{radials}))
}

fn symbology_block_generic(input: &[u8]) -> IResult<&[u8], Symbology> {
    todo!();
    // Ok((input, Symbology{a:0}))
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

    // let (input, divider) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let (_, pc) = peek(number::complete::i16(nom::number::Endianness::Big))(input)?;
    
    let packet_code = <PacketCode as num::FromPrimitive>::from_i16(pc).unwrap_or_default();
    if !packet_code.is_supported_product() {
        let e = nom::error::Error::new(input, error::ErrorKind::Fail);
        error!("Packet Code is {:?} which is not supported", pc);
        return Err(nom::Err::Failure(e));
    }

    println!("{:?}", packet_code);

    let (input, symbology) = match packet_code {
        PacketCode::Generic => {
            symbology_block_generic(input)?
        },
        PacketCode::Radial | PacketCode::AF1F => {
            symbology_block(input, packet_code)?
        },
        _ => {
            let e = nom::error::Error::new(input, error::ErrorKind::Fail);
            error!("Packet Code is {:?} which is not supported", pc);
            return Err(nom::Err::Failure(e));
        }
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
        
        info!("{:?}", decomp_vec);
    };    
    

    match parse(&file, &decomp_vec ) {
        Ok((leftover, value)) => {
            // warn!("Unmatched {:?}", leftover);
            info!("{:?}", value);
            warn!("{:?}", leftover);
        }
        Err(e) => error!("{:?}", e),
    }
}


