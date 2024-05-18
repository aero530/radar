use nom::{
    combinator::{cond, peek}, error, IResult,
    number::complete::i16 as nom_i16,
    number::Endianness::Big,
};
use tracing::{error, info};

use crate::{codes::PacketCode, product_symbology::{symbology_block, symbology_block_generic}};

use super::Symbology;

pub fn symbology_layers(input: &[u8]) -> IResult<&[u8], Symbology> {
    // If this is the first layer we will start right off with a packet code.  Other layers will have a layer divider first
    let (_, potential_divider) = peek(nom_i16(Big))(input)?;
    // If there was a layer divider then remove it and return the rest of the input
    let (input, _divider) = cond(potential_divider==-1, nom_i16(Big))(input)?;

    // peek at packet code in start of data layer (after symbology_header)
    let (_, packet_code_int) = peek(nom_i16(Big))(input)?;
    
    let packet_code = <PacketCode as num::FromPrimitive>::from_i16(packet_code_int).unwrap_or_default();
    if !packet_code.is_supported_product() {
        let e = nom::error::Error::new(input, error::ErrorKind::Fail);
        error!("Packet Code is {:?} which is not supported", packet_code_int);
        return Err(nom::Err::Failure(e));
    }

    info!("Packet Code {:?}", packet_code);

    let (input, symbology) = match packet_code {
        PacketCode::GenericData28 => {
            info!("Generic Data 28");
            symbology_block_generic(input)?
        },
        PacketCode::RadialDataAF1F | PacketCode::DigitalRadialDataArray => {
            info!("Digital Radial Data Array 16");
            symbology_block(input, packet_code)?
        },
        _ => {
            let e = nom::error::Error::new(input, error::ErrorKind::Fail);
            error!("Packet Code is {:?} which is not supported", packet_code_int);
            return Err(nom::Err::Failure(e));
        }
    };

    Ok((
        input,
        symbology
    ))

}