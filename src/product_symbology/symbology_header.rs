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
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq)]
pub struct SymbologyHeader {
    /// Delineate blocks, -1
    pub divider: i16,
    /// Block ID, (Always should be 1)
    pub id: i16,
    /// Length of block in bytes
    pub block_length: i32,
    /// Number of data layers
    pub layers: i16,
}

/// Graphic Product Message: Product Symbology Block
/// Description
/// 16 byte header
/// Figure 3-6 (Sheet 8), pages 3-40
pub fn symbology_header(input: &[u8]) -> IResult<&[u8], SymbologyHeader> {
    // warn!("sym header {:?}", input);
    
    let (input, divider) = nom_i16(Big)(input)?;
    if divider != -1 {
        error!("Block divider error");
    }
    let (input, id) = nom_i16(Big)(input)?;
    if id != 1 {
        let e = nom::error::Error::new(input, nom::error::ErrorKind::Fail);
        error!("Product symbology header should have ID=1 but found {}", id);
        return Err(nom::Err::Failure(e));
    }
    let (input, block_length) = nom_i32(Big)(input)?;
    let (input, layers) = nom_i16(Big)(input)?;

    // info!("Symbology block is {} bytes. {} data layers. Data layers are {} bytes total.", block_length, layers, layer_length);
    info!("Symbology block is {} bytes. {} data layers", block_length, layers);

    Ok((
        input,
        SymbologyHeader {
            divider,
            id,
            block_length,
            layers,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header_bytes(id: i16, block_length: i32, layers: i16) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(-1i16).to_be_bytes());
        bytes.extend_from_slice(&id.to_be_bytes());
        bytes.extend_from_slice(&block_length.to_be_bytes());
        bytes.extend_from_slice(&layers.to_be_bytes());
        bytes
    }

    #[test]
    fn parses_a_well_formed_header() {
        let bytes = header_bytes(1, 17590, 1);
        let (rest, header) = symbology_header(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(header.divider, -1);
        assert_eq!(header.id, 1);
        assert_eq!(header.block_length, 17590);
        assert_eq!(header.layers, 1);
    }

    #[test]
    fn rejects_an_id_whose_low_byte_happens_to_be_1() {
        // 257 = 0x0101; truncating to a u8 (the previous, buggy check) would
        // incorrectly read this as a valid id of 1.
        let bytes = header_bytes(257, 100, 1);
        assert!(symbology_header(&bytes).is_err());
    }

    #[test]
    fn rejects_truncated_input() {
        assert!(symbology_header(&[0xFF, 0xFF, 0, 1]).is_err());
    }
}