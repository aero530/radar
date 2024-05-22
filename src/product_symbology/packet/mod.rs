use nom::{
    combinator::peek, multi::count, IResult
};
use tracing::info;

mod radial;
pub use radial::{
    radial,
    Radial,
    RadialData,
    RadialPacketHeader,
    RunLevelEncoding,
};

use crate::codes::PacketCode;

use super::Symbology;


pub fn generic(_input: &[u8]) -> IResult<&[u8], Symbology> {
    todo!();
    // Ok((input, Symbology{a:0}))
}
