//! Helpers shared by the symbology packet parsers.

use nom::{
    number::{complete::i16 as nom_i16, Endianness::Big},
    IResult,
};
use tracing::error;

/// Builds a `nom` failure anchored at `input`, for the cases where a packet's
/// own declared sizes are internally inconsistent (as opposed to the input
/// simply being too short, which `take` reports on its own).
pub fn fail<'a, T>(input: &'a [u8], message: &str) -> IResult<&'a [u8], T> {
    error!("{message}");
    Err(nom::Err::Failure(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Fail,
    )))
}

/// Reads a packet's "length of block" halfword and converts it to a usize.
///
/// Nearly every packet in Figures 3-7 through 3-15a defines this field as
/// "number of bytes in block not including self or packet code", so the
/// returned value is exactly how many payload bytes follow.
pub fn block_length(input: &[u8]) -> IResult<&[u8], usize> {
    let (input, raw) = nom_i16(Big)(input)?;
    match usize::try_from(raw) {
        Ok(len) => Ok((input, len)),
        Err(_) => fail(input, &format!("Packet declares a negative block length ({raw})")),
    }
}

/// Splits `len` payload bytes off the front of `input`, returning
/// `(remaining_input, payload)`.
pub fn payload(input: &[u8], len: usize) -> IResult<&[u8], &[u8]> {
    nom::bytes::complete::take(len)(input)
}

/// Reads every `i16` in `bytes`, which must be a whole number of halfwords.
pub fn i16_array(bytes: &[u8]) -> Vec<i16> {
    bytes
        .chunks_exact(2)
        .map(|c| i16::from_be_bytes([c[0], c[1]]))
        .collect()
}

/// Decodes a run-length-encoded byte stream where each byte packs a 4-bit run
/// count in the high nibble and a 4-bit level in the low nibble.
///
/// This is the encoding used by the Radial Data Packet (Figure 3-10), the
/// Raster Data Packet (Figure 3-11) and the Precipitation Rate Data Array
/// Packet (Figure 3-11b).
pub fn decode_nibble_rle(bytes: &[u8]) -> Vec<(u8, u8)> {
    bytes.iter().map(|b| (b >> 4, b & 0x0F)).collect()
}
