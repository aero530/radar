use nom::{
    combinator::peek, IResult, Parser,
    number::complete::{i16 as nom_i16, i32 as nom_i32},
    number::Endianness::Big,
};

use tracing::{debug, error, info,};

use crate::codes::PacketCode;

use super::{SymPacketData, packet::*};

/// Fails gracefully (rather than panicking) when a packet code's binary
/// layout has not been implemented yet. See the module-level docs for the
/// list of packet codes that are currently supported.
fn unsupported_packet(input: &[u8], packet_code: PacketCode) -> IResult<&[u8], SymPacketData> {
    error!("Packet code {:?} is not yet implemented in this crate", packet_code);
    Err(nom::Err::Failure(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Fail,
    )))
}

/// Parses one symbology data layer: the layer divider and length, then the
/// single display data packet the layer holds.
pub fn symbology_layer(input: &[u8]) -> IResult<&[u8], SymPacketData> {

    let (input, layer_divider) = nom_i16(Big)(input)?;
    if layer_divider != -1 {
        error!("Symbology layer divider error. Found {} but expected -1", layer_divider);
    }
    let (input, layer_length) = nom_i32(Big)(input)?;
    info!("Layer data section is {} bytes.", layer_length);

    symbology_layer_packet(input)
}

/// Parses a single display data packet, dispatching on its leading packet
/// code, without the surrounding layer divider and length.
///
/// This is the entry point used both by [`symbology_layer`] and by the
/// alphanumeric blocks, whose pages hold bare packets.
pub fn symbology_layer_packet(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    // peek at packet code in start of data layer (after symbology_header)
    let (_, packet_code_int) = peek(nom_i16(Big)).parse(input)?;

    let packet_code = <PacketCode as num::FromPrimitive>::from_i16(packet_code_int).unwrap_or_default();

    debug!("Packet Code {:?}", packet_code);

    let (input, symbology) = match packet_code {
        // Text and special symbols (Figure 3-8b)
        PacketCode::TextAndSpecialSymbol1
        | PacketCode::TextAndSpecialSymbol8
        | PacketCode::TextAndSpecialSymbol2 => text_and_symbol(input),

        // Radial data (Figures 3-10 and 3-11c)
        PacketCode::RadialDataAF1F => radial_data_af1f(input),
        PacketCode::DigitalRadialDataArray => digital_radial_data_array(input),

        // Vectors (Figures 3-7, 3-8 and 3-8a)
        PacketCode::LinkedVector6 | PacketCode::LinkedVector9 => linked_vector(input),
        PacketCode::UnlinkedVector7 | PacketCode::UnlinkedVector10 => unlinked_vector(input),
        PacketCode::ContourVector0E03
        | PacketCode::ContourVector0802
        | PacketCode::ContourVector3501 => contour_vector(input),

        // Raster and gridded arrays (Figures 3-11, 3-11a, 3-11b, 3-11d)
        PacketCode::RasterDataBA0F | PacketCode::RasterDataBA07 => raster_data(input),
        PacketCode::DigitalPrecipitationDataArray => digital_precipitation_array(input),
        PacketCode::PrecipitationRateDataArray => precipitation_rate_array(input),
        PacketCode::DigitalRasterDataArray => digital_raster_array(input),

        // Wind depictions (Figures 3-12 and 3-13)
        PacketCode::VectorArrowData => vector_arrow(input),
        PacketCode::WindBarbData => wind_barb(input),

        // Special graphic symbols (Figure 3-14)
        PacketCode::SpecialGraphicSymbol3
        | PacketCode::SpecialGraphicSymbol11
        | PacketCode::SpecialGraphicSymbol12
        | PacketCode::SpecialGraphicSymbol13
        | PacketCode::SpecialGraphicSymbol14
        | PacketCode::SpecialGraphicSymbol15
        | PacketCode::SpecialGraphicSymbol19
        | PacketCode::SpecialGraphicSymbol20
        | PacketCode::SpecialGraphicSymbol23
        | PacketCode::SpecialGraphicSymbol24
        | PacketCode::SpecialGraphicSymbol25
        | PacketCode::SpecialGraphicSymbol26 => special_graphic_symbol(input),

        // Cell trends (Figures 3-15 and 3-15a)
        PacketCode::CellTrendData => cell_trend_data(input),
        PacketCode::CellTrendVolumeScanTimes => cell_trend_volume_times(input),

        // Generic data (Figure 3-15c), XDR-encoded
        PacketCode::GenericData28 | PacketCode::GenericData29 => generic_data(input),

        // Map overlay geometry for the map products (Figure 3-9). Note these
        // use 1/8 km coordinates from the upper left corner, unlike the
        // symbology packets' 1/4 km from the sweep centre.
        PacketCode::MapMessage0E23
        | PacketCode::MapMessage4E00
        | PacketCode::MapMessage3521
        | PacketCode::MapMessage4E01 => map_message(input),

        // A packet code that is not defined in the ICD at all.
        PacketCode::Other => unsupported_packet(input, packet_code),
    }?;

    // trace!("{:?}", symbology);
    Ok((
        input,
        symbology
    ))

}

#[cfg(test)]
mod tests {
    use super::*;

    fn layer_bytes(packet_code: i16, trailing: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(-1i16).to_be_bytes()); // layer divider
        bytes.extend_from_slice(&(2 + trailing.len() as i32).to_be_bytes()); // layer length
        bytes.extend_from_slice(&packet_code.to_be_bytes());
        bytes.extend_from_slice(trailing);
        bytes
    }

    /// Builds a layer whose packet has a "length of block" halfword followed
    /// by `body`, the shape most Figure 3-7..3-15 packets share.
    fn layer_with_block(packet_code: i16, body: &[u8]) -> Vec<u8> {
        let mut packet = packet_code.to_be_bytes().to_vec();
        packet.extend_from_slice(&(body.len() as i16).to_be_bytes());
        packet.extend_from_slice(body);

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(-1i16).to_be_bytes()); // layer divider
        bytes.extend_from_slice(&(packet.len() as i32).to_be_bytes()); // layer length
        bytes.extend_from_slice(&packet);
        bytes
    }

    fn hw(values: &[i16]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_be_bytes()).collect()
    }

    #[test]
    fn unrecognized_packet_code_fails_gracefully() {
        // 12345 does not correspond to any known PacketCode variant.
        let bytes = layer_bytes(12345, &[0, 0, 0, 0]);
        assert!(symbology_layer(&bytes).is_err());
    }

    /// Map message packets (Figure 3-9) dispatch to the map overlay parser.
    #[test]
    fn dispatches_a_map_message_packet() {
        // 0x0E23: indicator, start point, then a length covering the end points.
        let mut packet = hw(&[0x0E23u16 as i16, 0x8000u16 as i16, 10, 20, 4]);
        packet.extend_from_slice(&hw(&[30, 40]));

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(-1i16).to_be_bytes());
        bytes.extend_from_slice(&(packet.len() as i32).to_be_bytes());
        bytes.extend_from_slice(&packet);

        let (_, parsed) = symbology_layer(&bytes).unwrap();
        assert!(matches!(parsed, SymPacketData::MapMessage(_)));
    }

    #[test]
    fn dispatches_a_wind_barb_packet() {
        let bytes = layer_with_block(4, &hw(&[3, 10, 20, 180, 45]));
        let (_, parsed) = symbology_layer(&bytes).unwrap();
        assert!(matches!(parsed, SymPacketData::WindBarbData(_)));
    }

    #[test]
    fn dispatches_a_linked_vector_packet() {
        let bytes = layer_with_block(6, &hw(&[10, 20, 30, 40]));
        let (_, parsed) = symbology_layer(&bytes).unwrap();
        assert!(matches!(parsed, SymPacketData::LinkedVector(_)));
    }

    #[test]
    fn dispatches_a_special_graphic_symbol_packet() {
        let bytes = layer_with_block(15, {
            let mut b = hw(&[10, 20]);
            b.extend_from_slice(b"A1");
            &b.clone()
        });
        let (_, parsed) = symbology_layer(&bytes).unwrap();
        assert!(matches!(parsed, SymPacketData::SpecialGraphicSymbol(_)));
    }

    #[test]
    fn dispatches_a_cell_trend_packet() {
        let mut body = Vec::new();
        body.extend_from_slice(b"A1");
        body.extend_from_slice(&hw(&[0, 0]));
        let bytes = layer_with_block(21, &body);
        let (_, parsed) = symbology_layer(&bytes).unwrap();
        assert!(matches!(parsed, SymPacketData::CellTrendData(_)));
    }

    /// Packet 28 is XDR-encoded; its header length field is what frames it.
    #[test]
    fn dispatches_a_generic_data_packet() {
        let mut packet = 28i16.to_be_bytes().to_vec();
        packet.extend_from_slice(&0i16.to_be_bytes()); // reserved
        packet.extend_from_slice(&0i16.to_be_bytes()); // length MSHW
        packet.extend_from_slice(&4i16.to_be_bytes()); // length LSHW
        packet.extend_from_slice(&[1, 2, 3, 4]); // (undecodable) payload

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(-1i16).to_be_bytes());
        bytes.extend_from_slice(&(packet.len() as i32).to_be_bytes());
        bytes.extend_from_slice(&packet);

        let (_, parsed) = symbology_layer(&bytes).unwrap();
        assert!(matches!(parsed, SymPacketData::GenericData(_)));
    }
}