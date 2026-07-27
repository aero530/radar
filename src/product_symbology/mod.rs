use serde::{Deserialize, Serialize};

use nom::{IResult, Parser};

mod packet;
pub use packet::*;

mod symbology_header;
pub use symbology_header::{symbology_header, SymbologyHeader};

mod symbology_layer;
pub use symbology_layer::{symbology_layer, symbology_layer_packet};
use tracing::{debug, info};


/// The Product Symbology Block (Block ID 1), Figure 3-6 (sheets 3 and 8):
/// a 16 byte header followed by one or more data layers of symbology
/// packets. Symbology data packets are described in Figures 3-7 thru 3-14.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SymbologyBlock {
    pub header: SymbologyHeader,
    /// One entry per layer declared in `header.layers`. Only the packet
    /// codes with a real variant below (as opposed to falling into
    /// [`symbology_layer`] failing outright) are represented here.
    pub layers: Vec<SymPacketData>,
}

/// A single symbology-block data layer, tagged by which [`crate::PacketCode`]
/// it was decoded from.
///
/// Every packet code defined in Figures 3-7 through 3-15c has a variant here.
/// Codes that share an on-the-wire layout share a variant and record which
/// code produced them (for example [`SymPacketData::LinkedVector`] covers
/// both code 6 and code 9, distinguished by whether `value` is set).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SymPacketData {
    /// Packet codes 28 and 29 (Figure 3-15c), XDR-encoded.
    ///
    /// Boxed because a decoded generic product is far larger than any other
    /// packet, and an unboxed variant would inflate every `SymPacketData`.
    GenericData(Box<GenericDataPacket>),
    /// Packet code `AF1F` (Figure 3-10), run-length encoded radial data.
    RadialDataAF1F(RadialPacket),
    /// Packet code 16 (Figure 3-11c), 8-bit radial data levels.
    DigitalRadialDataArray(DigitalRadialPacket),
    /// Packet codes 1 and 2 (Figure 3-8b), text/special symbols with no value.
    TextAndSpecialSymbol1(TextPacket),
    /// Packet code 8 (Figure 3-8b), text with a uniform colour value.
    TextAndSpecialSymbol8(TextPacket),
    /// Packet codes 6 and 9 (Figure 3-7), a polyline.
    LinkedVector(LinkedVectorPacket),
    /// Packet codes 7 and 10 (Figure 3-8), independent line segments.
    UnlinkedVector(UnlinkedVectorPacket),
    /// Packet codes `0x0802`, `0x0E03` and `0x3501` (Figure 3-8a).
    ContourVector(ContourVectorPacket),
    /// Packet codes `0xBA0F` and `0xBA07` (Figure 3-11), raster rows.
    RasterData(RasterPacket),
    /// Packet code 17 (Figure 3-11a).
    DigitalPrecipitationDataArray(PrecipArrayPacket),
    /// Packet code 18 (Figure 3-11b).
    PrecipitationRateDataArray(PrecipArrayPacket),
    /// Packet code 33 (Figure 3-11d).
    DigitalRasterDataArray(DigitalRasterPacket),
    /// Packet code 5 (Figure 3-12).
    VectorArrowData(VectorArrowPacket),
    /// Packet code 4 (Figure 3-13).
    WindBarbData(WindBarbPacket),
    /// Packet codes 3, 11-15, 19, 20, 23-26 (Figure 3-14).
    SpecialGraphicSymbol(SpecialSymbolPacket),
    /// Packet code 21 (Figure 3-15).
    CellTrendData(CellTrendPacket),
    /// Packet code 22 (Figure 3-15a).
    CellTrendVolumeScanTimes(CellTrendVolumeTimesPacket),
    /// Packet codes `0x0E23`, `0x3521`, `0x4E00` and `0x4E01` (Figure 3-9),
    /// the map overlay geometry of the map products.
    MapMessage(MapMessagePacket),
}

impl SymPacketData {
    /// The number of range bins per radial, for the radial packet types;
    /// `0` for packet types where range bins do not apply.
    pub fn num_bins(&self) -> i16 {
        match self {
            SymPacketData::RadialDataAF1F(x) => x.header.num_bins,
            SymPacketData::DigitalRadialDataArray(x) => x.header.num_bins,
            _ => 0,
        }
    }
}

/// Parses the Product Symbology Block: its 16 byte header followed by
/// exactly `header.layers` data layers.
pub fn symbology(input: &[u8]) -> IResult<&[u8], SymbologyBlock> {
    info!("Decoding symbology block");
    let (input, symbology_header) = symbology_header(input)?;

    debug!("symbology header {:?}", symbology_header);

    let (input, symbology_layers) = nom::multi::count(symbology_layer, symbology_header.layers as usize).parse(input)?;
    Ok((
        input,
        SymbologyBlock{
            header: symbology_header, 
            layers: symbology_layers
        }
    ))
}