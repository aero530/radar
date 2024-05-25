use serde::{Deserialize, Serialize};

use nom::IResult;

mod packet;
pub use packet::*;

mod symbology_header;
pub use symbology_header::{symbology_header, SymbologyHeader};

mod symbology_layer;
pub use symbology_layer::symbology_layer;
use tracing::{debug, info};


// Symbology data packets are described in FIGURES 3-7 THRU 3-14
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SymbologyBlock {
    pub header: SymbologyHeader,
    pub layers: Vec<SymPacketData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SymPacketData {
    GenericData28(()),
    RadialDataAF1F(RadialPacket),
    DigitalRadialDataArray(DigitalRadialPacket),
    TextAndSpecialSymbol1(TextPacket),
    TextAndSpecialSymbol8(TextPacket),
}

impl SymPacketData {
    pub fn num_bins(&self) -> i16 {
        match self {
            SymPacketData::RadialDataAF1F(x) => x.header.num_bins,
            SymPacketData::DigitalRadialDataArray(x) => x.header.num_bins,
            _ => 0,
        }
    }

}

pub fn symbology(input: &[u8]) -> IResult<&[u8], SymbologyBlock> {
    info!("Decoding symbology block");
    let (input, symbology_header) = symbology_header(input)?;

    debug!("symbology header {:?}", symbology_header);

    let (input, symbology_layers) = nom::multi::count(symbology_layer, symbology_header.layers as usize)(input)?;
    Ok((
        input,
        SymbologyBlock{
            header: symbology_header, 
            layers: symbology_layers
        }
    ))
}