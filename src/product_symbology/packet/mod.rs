//! Parsers for the individual symbology display data packets, Figures 3-7
//! through 3-15c of the Class 1 User ICD.

mod util;
pub use util::decode_nibble_rle;

pub mod xdr;

mod radial;
pub use radial::{radial_data_af1f, Radial, RadialHeader, RadialPacket, RadialPacketHeader, RunLevelEncoding};

mod digital_radial;
pub use digital_radial::{
    digital_radial_data_array, DigitalRadial, DigitalRadialHeader, DigitalRadialPacket,
    DigitalRadialPacketHeader,
};

mod text_and_special_symbol;
pub use text_and_special_symbol::{text_and_symbol, TextPacket};

mod vector;
pub use vector::{
    contour_vector, linked_vector, unlinked_vector, ContourVectorPacket, LinkedVectorPacket, Point,
    UnlinkedVectorPacket, Vector,
};

mod raster;
pub use raster::{
    digital_precipitation_array, digital_raster_array, precipitation_rate_array, raster_data,
    DigitalRasterHeader, DigitalRasterPacket, PrecipArrayHeader, PrecipArrayPacket,
    RasterPacket, RasterPacketHeader, Run,
};

mod wind;
pub use wind::{vector_arrow, wind_barb, VectorArrow, VectorArrowPacket, WindBarb, WindBarbPacket};

mod special_symbol;
pub use special_symbol::{
    special_graphic_symbol, CircleSymbol, HailSymbol, PointFeatureSymbol, PointSymbol,
    SpecialSymbolPacket, StormIdSymbol,
};

mod cell_trend;
pub use cell_trend::{
    cell_trend_data, cell_trend_volume_times, CellTrend, CellTrendPacket,
    CellTrendVolumeTimesPacket, TrendCode,
};

mod map_message;
pub use map_message::{map_message, MapMessagePacket};

mod generic_data;
pub use generic_data::{
    decode_external_data_description, decode_product_description, generic_data, AreaComponent,
    AreaShape, EventComponent, ExternalDataDescription, GenericComponent, GenericDataHeader,
    GenericDataPacket, GenericPoint, GenericProductDescription, GenericRadial, GridComponent,
    Parameter, RadialComponent, TableComponent, TextComponent,
};
