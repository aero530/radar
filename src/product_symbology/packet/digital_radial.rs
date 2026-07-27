use serde::{Deserialize, Serialize};
use nom::{
    bytes::complete::take, multi::count, number::{complete::i16 as nom_i16, Endianness::Big}, IResult, Parser,
};
use tracing::{debug, error, info, warn};

use crate::product_symbology::SymPacketData;


/// DigitalRadialDataArray
pub fn digital_radial_data_array(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, packet_header) = packet_header(input)?;

    let num_bins = packet_header.num_bins as usize;
    let num_radials = packet_header.num_radials as usize;

    debug!("{:?}", packet_header);
    info!("Reading {:?} radial blocks each with {:?} bins", num_radials, num_bins);

    let (input, radials) = count(|i| data_block(i, num_bins), num_radials).parse(input)?;
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
        error!("Digital Radial Data Array Packet header should have packet code 16 but found {}", packet_code);
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
    //
    // Note 1 of that figure: "The RPG clips radials to 70 kft. This could
    // result in an odd number of bins in a radial. However, the radial will
    // always be on a halfword boundary, so the number of bytes in a radial
    // may be number of bins in a radial + 1."
    //
    // So the radial's own `num_bytes` — not the packet header's `num_bins` —
    // is what advances the cursor. Consuming only `num_bins` would leave the
    // halfword pad byte in the stream and desynchronize every radial after
    // this one.
    let num_bytes = usize::try_from(temp_header.num_bytes).map_err(|_| {
        error!("Radial declares a negative byte count ({})", temp_header.num_bytes);
        nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Fail))
    })?;
    if num_bytes < num_bins {
        warn!(
            "Radial declares {} bytes but the packet header declares {} bins; keeping the {} bytes present",
            num_bytes, num_bins, num_bytes
        );
    }
    let (input, payload) = take(num_bytes)(input)?;

    // Keep only the data level values, discarding any halfword pad byte.
    let data = payload[..num_bins.min(payload.len())].to_vec();
    let radial = DigitalRadial {
        header: temp_header,
        data,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_bytes(num_bins: i16, num_radials: i16) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&16i16.to_be_bytes()); // packet code
        bytes.extend_from_slice(&0i16.to_be_bytes()); // first_bin
        bytes.extend_from_slice(&num_bins.to_be_bytes());
        bytes.extend_from_slice(&0i16.to_be_bytes()); // i_sweep_center
        bytes.extend_from_slice(&0i16.to_be_bytes()); // j_sweep_center
        bytes.extend_from_slice(&1i16.to_be_bytes()); // range_scale
        bytes.extend_from_slice(&num_radials.to_be_bytes());

        for radial in 0..num_radials {
            bytes.extend_from_slice(&(num_bins).to_be_bytes()); // num_bytes
            bytes.extend_from_slice(&(radial * 10).to_be_bytes()); // angle_start
            bytes.extend_from_slice(&10i16.to_be_bytes()); // angle_delta
            bytes.extend(std::iter::repeat_n(radial as u8, num_bins as usize));
        }
        bytes
    }

    #[test]
    fn parses_a_well_formed_packet() {
        let bytes = sample_bytes(3, 2);
        let (rest, parsed) = digital_radial_data_array(&bytes).unwrap();

        assert!(rest.is_empty());
        match parsed {
            SymPacketData::DigitalRadialDataArray(packet) => {
                assert_eq!(packet.header.num_bins, 3);
                assert_eq!(packet.header.num_radials, 2);
                assert_eq!(packet.radials.len(), 2);
                assert_eq!(packet.radials[0].data, vec![0, 0, 0]);
                assert_eq!(packet.radials[1].data, vec![1, 1, 1]);
                assert_eq!(packet.radials[1].header.angle_start, 10);
            }
            other => panic!("expected DigitalRadialDataArray, got {other:?}"),
        }
    }

    #[test]
    fn rejects_the_wrong_packet_code() {
        let mut bytes = sample_bytes(3, 1);
        bytes[0..2].copy_from_slice(&99i16.to_be_bytes());
        assert!(digital_radial_data_array(&bytes).is_err());
    }

    /// Per Note 1 of Figure 3-11c, a radial with an odd bin count is padded
    /// to a halfword boundary, so `num_bytes == num_bins + 1`. The parser
    /// must advance by `num_bytes`; advancing by `num_bins` used to leave the
    /// pad byte in the stream and desynchronize every following radial.
    #[test]
    fn handles_halfword_pad_byte_on_odd_bin_counts() {
        let num_bins: i16 = 3; // odd -> one pad byte per radial
        let num_radials: i16 = 3;

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&16i16.to_be_bytes()); // packet code
        bytes.extend_from_slice(&0i16.to_be_bytes()); // first_bin
        bytes.extend_from_slice(&num_bins.to_be_bytes());
        bytes.extend_from_slice(&0i16.to_be_bytes()); // i_sweep_center
        bytes.extend_from_slice(&0i16.to_be_bytes()); // j_sweep_center
        bytes.extend_from_slice(&1i16.to_be_bytes()); // range_scale
        bytes.extend_from_slice(&num_radials.to_be_bytes());

        for radial in 0..num_radials {
            bytes.extend_from_slice(&(num_bins + 1).to_be_bytes()); // num_bytes = bins + pad
            bytes.extend_from_slice(&(radial * 10).to_be_bytes()); // angle_start
            bytes.extend_from_slice(&10i16.to_be_bytes()); // angle_delta
            bytes.extend(std::iter::repeat_n(radial as u8 + 1, num_bins as usize));
            bytes.push(0xEE); // halfword pad byte, not a data level
        }

        let (rest, parsed) = digital_radial_data_array(&bytes).unwrap();

        assert!(rest.is_empty(), "{} unconsumed byte(s) left over", rest.len());
        match parsed {
            SymPacketData::DigitalRadialDataArray(packet) => {
                assert_eq!(packet.radials.len(), 3);
                // Each radial keeps exactly num_bins data levels, and the pad
                // byte (0xEE) is never mistaken for one.
                assert_eq!(packet.radials[0].data, vec![1, 1, 1]);
                assert_eq!(packet.radials[1].data, vec![2, 2, 2]);
                assert_eq!(packet.radials[2].data, vec![3, 3, 3]);
                // Angles stay aligned, which is what desynchronization broke.
                assert_eq!(packet.radials[1].header.angle_start, 10);
                assert_eq!(packet.radials[2].header.angle_start, 20);
            }
            other => panic!("expected DigitalRadialDataArray, got {other:?}"),
        }
    }

    #[test]
    fn rejects_truncated_radial_data_instead_of_panicking() {
        let mut bytes = sample_bytes(3, 1);
        bytes.truncate(bytes.len() - 1); // one byte short of the last radial's data
        assert!(digital_radial_data_array(&bytes).is_err());
    }
}