


mod radial;
pub use radial::{radial_data_af1f, RadialPacket};

mod digital_radial;
pub use digital_radial::{digital_radial_data_array, DigitalRadialPacket};

mod text_and_special_symbol;
pub use text_and_special_symbol::{text_and_symbol, TextPacket};


use nom::IResult;
use super::SymPacketData;
pub fn generic_data28(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    Ok((input, SymPacketData::GenericData28(())))
}
