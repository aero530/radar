use serde::{Deserialize, Serialize};
use nom::{
    bytes::complete::take, multi::count, number::{complete::i16 as nom_i16, Endianness::Big}, IResult, Parser,
};
use tracing::{debug, error, info};

use crate::product_symbology::SymPacketData;


/// DigitalRadialDataArray
pub fn radial_data_af1f(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, packet_header) = packet_header(input)?;

    let num_bins = packet_header.num_bins as usize;
    let num_radials = packet_header.num_radials as usize;

    debug!("{:?}", packet_header);
    info!("Reading {:?} radial blocks each with {:?} bins", num_radials, num_bins);

    let (input, radials) = count(data_block, num_radials).parse(input)?;
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
    if packet_code != -20705 {
        error!("Radial Data Packet should have packet code AF1F (-20705) but found {}", packet_code);
        return Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Fail)));
    }
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
    //
    // Each RLE halfword holds two run/colour byte pairs, so the byte count is
    // `num_halfwords * 2`. Casting a negative count straight to `usize` would
    // wrap to an enormous value and then overflow the multiply, so validate it
    // first (Figure 3-10 gives the range as 1 to 230).
    let rle_size = usize::try_from(temp_header.num_halfwords)
        .ok()
        .and_then(|hw| hw.checked_mul(2))
        .ok_or_else(|| {
            error!(
                "Radial declares an invalid RLE halfword count ({})",
                temp_header.num_halfwords
            );
            nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Fail))
        })?;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_bytes(rle_bytes: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(-20705i16).to_be_bytes()); // packet code AF1F
        bytes.extend_from_slice(&0i16.to_be_bytes()); // first_bin
        bytes.extend_from_slice(&0i16.to_be_bytes()); // num_bins
        bytes.extend_from_slice(&0i16.to_be_bytes()); // i_sweep_center
        bytes.extend_from_slice(&0i16.to_be_bytes()); // j_sweep_center
        bytes.extend_from_slice(&1i16.to_be_bytes()); // range_scale
        bytes.extend_from_slice(&1i16.to_be_bytes()); // num_radials

        bytes.extend_from_slice(&((rle_bytes.len() / 2) as i16).to_be_bytes()); // num_halfwords
        bytes.extend_from_slice(&100i16.to_be_bytes()); // angle_start
        bytes.extend_from_slice(&10i16.to_be_bytes()); // angle_delta
        bytes.extend_from_slice(rle_bytes);
        bytes
    }

    #[test]
    fn decodes_run_length_encoding_into_run_and_color_nibbles() {
        // 0x53 -> run=5, color=3; 0xA1 -> run=10, color=1
        let bytes = sample_bytes(&[0x53, 0xA1]);
        let (rest, parsed) = radial_data_af1f(&bytes).unwrap();

        assert!(rest.is_empty());
        match parsed {
            SymPacketData::RadialDataAF1F(packet) => {
                assert_eq!(packet.radials.len(), 1);
                assert_eq!(
                    packet.radials[0].data,
                    vec![
                        RunLevelEncoding { run: 5, color: 3 },
                        RunLevelEncoding { run: 10, color: 1 },
                    ]
                );
                assert_eq!(packet.radials[0].header.angle_start, 100);
            }
            other => panic!("expected RadialDataAF1F, got {other:?}"),
        }
    }

    #[test]
    fn rejects_the_wrong_packet_code() {
        let mut bytes = sample_bytes(&[0x53, 0xA1]);
        bytes[0..2].copy_from_slice(&16i16.to_be_bytes()); // this is packet code 16, not AF1F
        assert!(radial_data_af1f(&bytes).is_err());
    }

    #[test]
    fn rejects_truncated_run_length_data_instead_of_panicking() {
        let mut bytes = sample_bytes(&[0x53, 0xA1]);
        bytes.pop();
        assert!(radial_data_af1f(&bytes).is_err());
    }
}