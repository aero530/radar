
use serde::{Deserialize, Serialize};
use nom::{bytes::complete::take, combinator::peek, multi::count, IResult};
use tracing::{error, info};

use crate::codes::PacketCode;

use super::{radial_header, radial_packet_header, Radial, RadialData, RadialPacketHeader, RunLevelEncoding};

// Symbology data packets are described in FIGURES 3-7 THRU 3-14

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Symbology {
    pub packet_header: RadialPacketHeader,
    pub radials: Vec<Radial>,
}

// fn radial_block(input: (PacketCode, usize, &[u8]) ) -> IResult<(PacketCode, usize, &[u8]), Radial> {
pub fn radial_block(input: &[u8], packet_code: PacketCode, num_bins: usize) -> IResult<&[u8], Radial> {
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

/// DigitalRadialDataArray
pub fn symbology_block(input: &[u8], packet_code: PacketCode) -> IResult<&[u8], Symbology> {
    let (input, packet_header) = radial_packet_header(input)?;

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

    let (input, radials) = count(|i| radial_block(i, packet_code, num_bins), num_radials)(input)?;
    Ok((input, Symbology{packet_header, radials}))
}

pub fn symbology_block_generic(_input: &[u8]) -> IResult<&[u8], Symbology> {
    todo!();
    // Ok((input, Symbology{a:0}))
}