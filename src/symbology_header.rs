use serde::{Deserialize, Serialize};
use nom::{
    number::complete::{i16 as nom_i16, i32 as nom_i32},
    number::Endianness::Big,
    IResult,
};

/// Graphic Product Message: Product Symbology Block
/// Description
/// 16 byte header
/// Figure 3-6 (Sheet 8), pages 3-40
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
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

/// Graphic Product Message: Product Symbology Block
/// Description
/// 16 byte header
/// Figure 3-6 (Sheet 8), pages 3-40
pub fn symbology_header(input: &[u8]) -> IResult<&[u8], SymbologyHeader> {
        
    let (input, divider) = nom_i16(Big)(input)?;
    let (input, id) = nom_i16(Big)(input)?;
    let (input, block_length) = nom_i32(Big)(input)?;
    let (input, layers) = nom_i16(Big)(input)?;
    let (input, layer_divider) = nom_i16(Big)(input)?;
    let (input, layer_length) = nom_i32(Big)(input)?;

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