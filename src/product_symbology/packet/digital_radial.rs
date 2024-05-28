use serde::{Deserialize, Serialize};
use nom::{
    bytes::complete::take, multi::count, number::{complete::i16 as nom_i16, Endianness::Big}, IResult
};
use tracing::{debug, error, info};

use crate::product_symbology::SymPacketData;


/// DigitalRadialDataArray
pub fn digital_radial_data_array(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, packet_header) = packet_header(input)?;

    let num_bins = packet_header.num_bins as usize;
    let num_radials = packet_header.num_radials as usize;

    debug!("{:?}", packet_header);
    info!("Reading {:?} radial blocks each with {:?} bins", num_radials, num_bins);

    let (input, radials) = count(|i| data_block(i, num_bins), num_radials)(input)?;
    Ok((input, SymPacketData::DigitalRadialDataArray(DigitalRadialPacket{header: packet_header, radials}) ))
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DigitalRadialPacket {
    pub header: DigitalRadialPacketHeader,
    pub radials: Vec<DigitalRadial>,
}

/// Digital Radial Data Array Packet - Packet Code 16 (Sheet 2)
/// Figure 3-11c (Sheet 1 and 2), page 3-120
/// and
/// Radial Data Packet - Packet Code AF1F
/// Figure 3-10 (Sheet 1 and 2), page 3-113
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq)]
pub struct DigitalRadialPacketHeader {
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
/// Figure 3-11c (Sheet 1 and 2), page 3-94, 3-95
/// and
/// Radial Data Packet - Packet Code AF1F
/// Figure 3-10 (Sheet 1 and 2), page 3-91, 3-92
fn packet_header(input: &[u8]) -> IResult<&[u8], DigitalRadialPacketHeader> {
    
    let (input, packet_code) = nom_i16(Big)(input)?;
    if packet_code != 16 {
        error!("Digital Radial Data Array Packet header error");
    }
    let (input, first_bin) = nom_i16(Big)(input)?;
    let (input, num_bins) = nom_i16(Big)(input)?;
    let (input, i_sweep_center) = nom_i16(Big)(input)?;
    let (input, j_sweep_center) = nom_i16(Big)(input)?;
    let (input, range_scale) = nom_i16(Big)(input)?;
    let (input, num_radials) = nom_i16(Big)(input)?;
    
    Ok((
        input,
        DigitalRadialPacketHeader {
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


// fn radial_block(input: (PacketCode, usize, &[u8]) ) -> IResult<(PacketCode, usize, &[u8]), Radial> {
fn data_block(input: &[u8], num_bins: usize) -> IResult<&[u8], DigitalRadial> {
    // let (packet_code, num_bins, input) = input;
    let (input, temp_header) = radial_header(input)?;
    debug!("{:?}", temp_header);

    // Figure 3-11c. Digital Radial Data Array Packet - Packet Code 16
    // page 3-95
    let (input, temp_data) = take(num_bins)(input)?;
    let radial = DigitalRadial { 
        header: temp_header, 
        data: temp_data.to_vec(),
    };
    
    Ok((input, radial))
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DigitalRadialHeader {
    /// Number of bytes in the radial.
    pub num_bytes: i16,
    /// Starting angle at which data was collected.
    pub angle_start: i16,
    /// Delta angle from previous radial.
    pub angle_delta: i16,
}

fn radial_header(input: &[u8]) -> IResult<&[u8], DigitalRadialHeader> {
    
    let (input, num_bytes) = nom_i16(Big)(input)?;
    let (input, angle_start) = nom_i16(Big)(input)?;
    let (input, angle_delta) = nom_i16(Big)(input)?;

    Ok((
        input,
        DigitalRadialHeader {
            num_bytes,
            angle_start,
            angle_delta,
        },
    ))
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DigitalRadial {
    pub header: DigitalRadialHeader,
    pub data: Vec<u8>,
}