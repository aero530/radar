use nom::{
    combinator::peek, error, IResult,
    number::complete::{i16 as nom_i16, i32 as nom_i32},
    number::Endianness::Big,
};
use packet::*;
use tracing::{debug, error, info,};

use crate::{codes::PacketCode, product_symbology::packet};

use super::SymPacketData;


pub fn symbology_layer(input: &[u8]) -> IResult<&[u8], SymPacketData> {

    let (input, layer_divider) = nom_i16(Big)(input)?;
    if layer_divider != -1 {
        error!("Symbology layer divider error. Found {} but expected -1", layer_divider);
    }
    let (input, layer_length) = nom_i32(Big)(input)?;
    info!("Layer data section is {} bytes.", layer_length);

    // peek at packet code in start of data layer (after symbology_header)
    let (_, packet_code_int) = peek(nom_i16(Big))(input)?;
    
    let packet_code = <PacketCode as num::FromPrimitive>::from_i16(packet_code_int).unwrap_or_default();
    if !packet_code.is_supported_product() {
        let e = nom::error::Error::new(input, error::ErrorKind::Fail);
        error!("Packet Code is {:?} ({:?}) which is not supported", packet_code, packet_code_int);
        return Err(nom::Err::Failure(e));
    }

    debug!("Packet Code {:?}", packet_code);

    let (input, symbology) = match packet_code {
        PacketCode::LinkedVector6 => todo!(),
        PacketCode::LinkedVector9 => todo!(),
        PacketCode::UnlinkedVector7 => todo!(),
        PacketCode::UnlinkedVector10 => todo!(),
        PacketCode::ContourVector0E03 => todo!(),
        PacketCode::ContourVector0802 => todo!(),
        PacketCode::ContourVector3501 => todo!(),
        PacketCode::TextAndSpecialSymbol1 => todo!(),
        PacketCode::TextAndSpecialSymbol8 => todo!(),
        PacketCode::TextAndSpecialSymbol2 => todo!(),
        PacketCode::MapMessage0E23 => todo!(),
        PacketCode::MapMessage4E00 => todo!(),
        PacketCode::MapMessage3521 => todo!(),
        PacketCode::MapMessage4E01 => todo!(),
        PacketCode::RadialDataAF1F => radial_data_af1f(input),
        PacketCode::RasterDataBA0F => todo!(),
        PacketCode::RasterDataBA07 => todo!(),
        PacketCode::DigitalPrecipitationDataArray => todo!(),
        PacketCode::PrecipitationRateDataArray => todo!(),
        PacketCode::DigitalRadialDataArray => digital_radial_data_array(input),
        PacketCode::VectorArrowData => todo!(),
        PacketCode::WindBarbData => todo!(),
        PacketCode::SpecialGraphicSymbol3 => todo!(),
        PacketCode::SpecialGraphicSymbol11 => todo!(),
        PacketCode::SpecialGraphicSymbol12 => todo!(),
        PacketCode::SpecialGraphicSymbol13 => todo!(),
        PacketCode::SpecialGraphicSymbol14 => todo!(),
        PacketCode::SpecialGraphicSymbol15 => todo!(),
        PacketCode::SpecialGraphicSymbol19 => todo!(),
        PacketCode::SpecialGraphicSymbol23 => todo!(),
        PacketCode::SpecialGraphicSymbol24 => todo!(),
        PacketCode::SpecialGraphicSymbol25 => todo!(),
        PacketCode::SpecialGraphicSymbol26 => todo!(),
        PacketCode::SpecialGraphicSymbol20 => todo!(),
        PacketCode::CellTrendData => todo!(),
        PacketCode::CellTrendVolumeScanTimes => todo!(),
        PacketCode::GenericData28 => generic_data28(input),
        PacketCode::GenericData29 => todo!(),
        PacketCode::Other => todo!(),
    }?;

    // trace!("{:?}", symbology);
    Ok((
        input,
        symbology
    ))

}