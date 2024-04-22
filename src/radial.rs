use serde::{Deserialize, Serialize};
use nom::{
    number::complete::i16 as nom_i16,
    number::Endianness::Big,
    IResult,
};

/// Digital Radial Data Array Packet - Packet Code 16 (Sheet 2)
/// Figure 3-11c (Sheet 1 and 2), page 3-120
/// and
/// Radial Data Packet - Packet Code AF1F
/// Figure 3-10 (Sheet 1 and 2), page 3-113
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RadialPacketHeader {
    /// Packet Code, Type 16
    pub packet_code: i16,
    /// Location of first range bin.
    pub first_bin: i16,
    /// Number of range bins.
    pub num_bins: i16,
    /// I coordinate of center of sweep.
    pub i_sweep_center: i16,
    /// J coordinate of center of sweep.
    pub j_sweep_center: i16,
    /// Range Scale factor
    pub range_scale: i16,
    /// Total number of radials in the product
    pub num_radials: i16,
}

/// Digital Radial Data Array Packet - Packet Code 16 (Sheet 2)
/// Figure 3-11c (Sheet 1 and 2), page 3-120
/// and
/// Radial Data Packet - Packet Code AF1F
/// Figure 3-10 (Sheet 1 and 2), page 3-113
pub fn radial_packet_header(input: &[u8]) -> IResult<&[u8], RadialPacketHeader> {
    
    let (input, packet_code) = nom_i16(Big)(input)?;
    let (input, first_bin) = nom_i16(Big)(input)?;
    let (input, num_bins) = nom_i16(Big)(input)?;
    let (input, i_sweep_center) = nom_i16(Big)(input)?;
    let (input, j_sweep_center) = nom_i16(Big)(input)?;
    let (input, range_scale) = nom_i16(Big)(input)?;
    let (input, num_radials) = nom_i16(Big)(input)?;

    Ok((
        input,
        RadialPacketHeader {
            packet_code,
            first_bin,
            num_bins,
            i_sweep_center,
            j_sweep_center,
            range_scale,
            num_radials,
        },
    ))
}


#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RadialHeader {
    /// Number of bytes in the radial.
    pub num_bytes: i16,
    /// Starting angle at which data was collected.
    pub angle_start: i16,
    /// Delta angle from previous radial.
    pub angle_delta: i16,
}

pub fn radial_header(input: &[u8]) -> IResult<&[u8], RadialHeader> {
    
    let (input, num_bytes) = nom_i16(Big)(input)?;
    let (input, angle_start) = nom_i16(Big)(input)?;
    let (input, angle_delta) = nom_i16(Big)(input)?;

    Ok((
        input,
        RadialHeader {
            num_bytes,
            angle_start,
            angle_delta,
        },
    ))
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Radial {
    pub header: RadialHeader,
    pub data: RadialData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RadialData {
    Digital(Vec<u8>),
    AF1F(Vec<RunLevelEncoding>)
}

impl Default for RadialData {
    fn default() -> Self {
        RadialData::Digital(Vec::new())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RunLevelEncoding {
    pub run: u8,
    pub color: u8,
}