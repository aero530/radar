use serde::{Deserialize, Serialize};
use nom::{
    number::complete::{i16 as nom_i16, i32 as nom_i32},
    number::Endianness::Big,
    IResult,
};
use tracing::{error, info};

/// Graphic Product Message: Product Symbology Block
/// Description
/// 16 byte header
/// Figure 3-6 (Sheet 8), pages 3-40
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SymbologyHeader {
    /// Delineate blocks, -1
    pub divider: i16,
    /// Block ID, (Always should be 1)
    pub id: i16,
    /// Length of block in bytes
    pub block_length: i32,
    /// Number of data layers
    pub layers: i16,
    /// Delineate data layers, -1
    pub layer_divider: i16,
    /// Length of data layer in bytes
    pub layer_length: i32,
}

/// Graphic Product Message: Product Symbology Block
/// Description
/// 16 byte header
/// Figure 3-6 (Sheet 8), pages 3-40
pub fn symbology_header(input: &[u8]) -> IResult<&[u8], SymbologyHeader> {
        
    let (input, divider) = nom_i16(Big)(input)?;
    let (input, id) = nom_i16(Big)(input)?;
    if id as u8 !=  1 {
        let e = nom::error::Error::new(input, nom::error::ErrorKind::Fail);
        error!("Product symbology header should have ID=1 but found {}", id);
        return Err(nom::Err::Failure(e));
    }
    let (input, block_length) = nom_i32(Big)(input)?;
    let (input, layers) = nom_i16(Big)(input)?;
    let (input, layer_divider) = nom_i16(Big)(input)?;
    let (input, layer_length) = nom_i32(Big)(input)?;

    if divider != -1 {
        error!("Block divider error");
    }

    if id != 1 {
        error!("Symbology header block ID error");
    }

    if layer_divider != -1 {
        error!("Symbology header block layer divider error. Found {} but expected -1", layer_divider);
    }
    
    info!("Symbology block is {} bytes. {} data layers. Data layers are {} bytes total.", block_length, layers, layer_length);

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