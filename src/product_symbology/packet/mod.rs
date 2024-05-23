


mod radial;
pub use radial::{radial_data_af1f, RadialPacket};

mod digital_radial;
pub use digital_radial::{digital_radial_data_array, DigitalRadialPacket};



use nom::IResult;
use super::SymPacketData;
pub fn generic_data28(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    Ok((input, SymPacketData::GenericData28(())))
}
