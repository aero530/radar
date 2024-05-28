use serde::{Deserialize, Serialize};
use nom::{
    bytes::complete::take, multi::count, number::{complete::i16 as nom_i16, Endianness::Big}, IResult
};
use tracing::{debug, info};

use crate::product_symbology::SymPacketData;


/// DigitalRadialDataArray
pub fn radial_data_af1f(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, packet_header) = packet_header(input)?;

    let num_bins = packet_header.num_bins as usize;
    let num_radials = packet_header.num_radials as usize;

    debug!("{:?}", packet_header);
    info!("Reading {:?} radial blocks each with {:?} bins", num_radials, num_bins);

    let (input, radials) = count(|i| data_block(i), num_radials)(input)?;
    Ok((input, SymPacketData::RadialDataAF1F(RadialPacket{header: packet_header, radials}) ))
}


#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RadialPacket {
    pub header: RadialPacketHeader,
    pub radials: Vec<Radial>,
}


/// Digital Radial Data Array Packet - Packet Code 16 (Sheet 2)
/// Figure 3-11c (Sheet 1 and 2), page 3-120
/// and
/// Radial Data Packet - Packet Code AF1F
/// Figure 3-10 (Sheet 1 and 2), page 3-113
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq)]
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
/// Figure 3-11c (Sheet 1 and 2), page 3-94, 3-95
/// and
/// Radial Data Packet - Packet Code AF1F
/// Figure 3-10 (Sheet 1 and 2), page 3-91, 3-92
fn packet_header(input: &[u8]) -> IResult<&[u8], RadialPacketHeader> {
    
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



fn data_block(input: &[u8]) -> IResult<&[u8], Radial> {
    // let (packet_code, num_bins, input) = input;
    let (input, temp_header) = radial_header(input)?;
    debug!("{:?}", temp_header);

    // decode run length encoding
    let rle_size = temp_header.num_halfwords as usize * 2;
    let (input, rle) = take(rle_size)(input)?;
    
    // run code then color (4bit ints)
    let data : Vec<RunLevelEncoding>= rle.iter().map(|x| RunLevelEncoding{run: x >> 4, color: x & 0b00001111}).collect();

    let radial = Radial { 
        header: temp_header,
        data,
    };
      
    // Ok(((packet_code, num_bins, input), radial))
    Ok((input, radial))
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq)]
pub struct RadialHeader {
    /// Number of half words in the radial.
    pub num_halfwords: i16,
    /// Starting angle at which data was collected.
    pub angle_start: i16,
    /// Delta angle from previous radial.
    pub angle_delta: i16,
}

fn radial_header(input: &[u8]) -> IResult<&[u8], RadialHeader> {
    
    let (input, num_halfwords) = nom_i16(Big)(input)?;
    let (input, angle_start) = nom_i16(Big)(input)?;
    let (input, angle_delta) = nom_i16(Big)(input)?;

    Ok((
        input,
        RadialHeader {
            num_halfwords,
            angle_start,
            angle_delta,
        },
    ))
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Radial {
    pub header: RadialHeader,
    pub data: Vec<RunLevelEncoding>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq)]
pub struct RunLevelEncoding {
    pub run: u8,
    pub color: u8,
}