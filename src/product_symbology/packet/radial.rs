use serde::{Deserialize, Serialize};
use nom::{
    bytes::complete::take, combinator::peek, multi::count, number::{complete::i16 as nom_i16, Endianness::Big}, IResult
};
use tracing::{error, info};

use crate::{codes::PacketCode, product_symbology::Symbology};


/// DigitalRadialDataArray
pub fn radial(input: &[u8], packet_code: PacketCode) -> IResult<&[u8], Symbology> {
    let (input, packet_header) = packet_header(input)?;

    let mut num_bins = packet_header.num_bins as usize;
    let num_radials = packet_header.num_radials as usize;

    let (input, temp_header) = peek(radial_header)(input)?;

    match packet_code {
        PacketCode::RadialDataAF1F => {
            if temp_header.num_bytes as usize != num_bins {
                num_bins = temp_header.num_bytes as usize;
            }
        },
        _ => {}
    }

    info!("Reading {:?} radial blocks each with {:?} bins", num_radials, num_bins);

    let (input, radials) = count(|i| data_block(i, packet_code, num_bins), num_radials)(input)?;
    Ok((input, Symbology{packet_header, radials}))
}

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


// fn radial_block(input: (PacketCode, usize, &[u8]) ) -> IResult<(PacketCode, usize, &[u8]), Radial> {
fn data_block(input: &[u8], packet_code: PacketCode, num_bins: usize) -> IResult<&[u8], Radial> {
    // let (packet_code, num_bins, input) = input;
    let (input, temp_header) = radial_header(input)?;

    let (input, radial) = match packet_code {
        PacketCode::RadialDataAF1F => {
            // decode run length encoding
            let rle_size = temp_header.num_bytes as usize * 2;
            let (input, rle) = take(rle_size)(input)?;
            
            // run code then color (4bit ints)
            let data : Vec<RunLevelEncoding>= rle.iter().map(|x| RunLevelEncoding{run: x >> 4, color: x & 0b00001111}).collect();

            Ok((input, Radial { 
                header: temp_header,
                data: RadialData::AF1F(data),
            }))
        },
        PacketCode::DigitalRadialDataArray => {
            // Figure 3-11c. Digital Radial Data Array Packet - Packet Code 16
            // page 3-95
            let (input, temp_data) = take(num_bins)(input)?;
            Ok((input, Radial { 
                header: temp_header, 
                data: RadialData::Digital(temp_data.to_vec()),
            }))
        },
        _ => {
            error!("Trying to read radial block but found the wrong packet code type.");
            Ok((input, Radial::default()))
        }
    }?;
    
    // Ok(((packet_code, num_bins, input), radial))
    Ok((input, radial))
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

fn radial_header(input: &[u8]) -> IResult<&[u8], RadialHeader> {
    
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