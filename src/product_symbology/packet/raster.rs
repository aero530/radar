//! Raster and gridded-array packets:
//!
//! - Raster Data Packet, codes `0xBA0F`/`0xBA07` (Figure 3-11)
//! - Digital Precipitation Data Array Packet, code 17 (Figure 3-11a)
//! - Precipitation Rate Data Array Packet, code 18 (Figure 3-11b)
//! - Digital Raster Data Array Packet, code 33 (Figure 3-11d)

use serde::{Deserialize, Serialize};
use nom::{
    number::{complete::i16 as nom_i16, Endianness::Big},
    IResult,
};
use tracing::{debug, warn};

use super::util::{decode_nibble_rle, fail, payload};
use crate::product_symbology::SymPacketData;

/// One run of a run-length-encoded raster row.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Run {
    /// Number of consecutive cells covered by this run.
    pub run: u16,
    /// Data level / colour code for those cells.
    pub level: u16,
}

/// Raster Data Packet header — Figure 3-11 (Sheet 1 and 2), page 3-99.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct RasterPacketHeader {
    /// Packet Code, `0xBA0F` or `0xBA07`.
    pub packet_code: i16,
    /// Second packet code halfword, constant `0x8000`.
    pub packet_code_2: i16,
    /// Third packet code halfword, constant `0x00C0`.
    pub packet_code_3: i16,
    /// I coordinate of the start of the data (Km/4).
    pub i_start: i16,
    /// J coordinate of the start of the data (Km/4).
    pub j_start: i16,
    /// Integer part of the grid scaling factor in X (1 to 67).
    pub x_scale_int: i16,
    /// Fractional part of the X scale; reserved for internal PUP use.
    pub x_scale_fractional: i16,
    /// Integer part of the grid scaling factor in Y (1 to 67).
    pub y_scale_int: i16,
    /// Fractional part of the Y scale; reserved for internal PUP use.
    pub y_scale_fractional: i16,
    /// Number of rows in the layer (1 to 464).
    pub num_rows: i16,
    /// Packing descriptor, constant 2.
    pub packing_descriptor: i16,
}

/// A raster/array packet: a header plus one run-length-encoded row per row.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct RasterPacket {
    pub header: RasterPacketHeader,
    /// One entry per row, each a list of runs.
    pub rows: Vec<Vec<Run>>,
}

/// Raster Data Packet, codes `0xBA0F` and `0xBA07` (Figure 3-11).
///
/// Rows are run-length encoded with a 4-bit run count in the high nibble of
/// each byte and a 4-bit colour code in the low nibble.
pub fn raster_data(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, packet_code) = nom_i16(Big)(input)?;
    let (input, packet_code_2) = nom_i16(Big)(input)?;
    let (input, packet_code_3) = nom_i16(Big)(input)?;
    if packet_code_2 as u16 != 0x8000 || packet_code_3 as u16 != 0x00C0 {
        warn!(
            "Raster packet expects secondary codes 0x8000/0x00C0 but found {:#06x}/{:#06x}",
            packet_code_2 as u16, packet_code_3 as u16
        );
    }
    let (input, i_start) = nom_i16(Big)(input)?;
    let (input, j_start) = nom_i16(Big)(input)?;
    let (input, x_scale_int) = nom_i16(Big)(input)?;
    let (input, x_scale_fractional) = nom_i16(Big)(input)?;
    let (input, y_scale_int) = nom_i16(Big)(input)?;
    let (input, y_scale_fractional) = nom_i16(Big)(input)?;
    let (input, num_rows) = nom_i16(Big)(input)?;
    let (input, packing_descriptor) = nom_i16(Big)(input)?;

    let header = RasterPacketHeader {
        packet_code,
        packet_code_2,
        packet_code_3,
        i_start,
        j_start,
        x_scale_int,
        x_scale_fractional,
        y_scale_int,
        y_scale_fractional,
        num_rows,
        packing_descriptor,
    };
    debug!("{:?}", header);

    let (input, rows) = nibble_rle_rows(input, num_rows)?;

    Ok((input, SymPacketData::RasterData(RasterPacket { header, rows })))
}

/// Reads `num_rows` rows, each a halfword byte count followed by that many
/// bytes of nibble-packed run/level pairs.
fn nibble_rle_rows(mut input: &[u8], num_rows: i16) -> IResult<&[u8], Vec<Vec<Run>>> {
    let row_count = match usize::try_from(num_rows) {
        Ok(n) => n,
        Err(_) => return fail(input, &format!("Packet declares {num_rows} rows")),
    };

    let mut rows = Vec::with_capacity(row_count.min(1024));
    for _ in 0..row_count {
        let (rest, num_bytes) = nom_i16(Big)(input)?;
        let len = match usize::try_from(num_bytes) {
            Ok(n) => n,
            Err(_) => return fail(rest, &format!("Row declares {num_bytes} bytes")),
        };
        let (rest, body) = payload(rest, len)?;
        rows.push(
            decode_nibble_rle(body)
                .into_iter()
                .map(|(run, level)| Run {
                    run: run as u16,
                    level: level as u16,
                })
                .collect(),
        );
        input = rest;
    }
    Ok((input, rows))
}

/// Digital Precipitation Data Array Packet header — Figure 3-11a, page 3-100.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PrecipArrayHeader {
    /// Packet Code, 17 or 18.
    pub packet_code: i16,
    /// Two spare halfwords that follow the packet code.
    pub spares: [i16; 2],
    /// Number of LFM boxes in each row.
    pub num_boxes: i16,
    /// Total number of rows.
    pub num_rows: i16,
}

/// A precipitation array packet: a header plus one run-length-encoded row per
/// row.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct PrecipArrayPacket {
    pub header: PrecipArrayHeader,
    pub rows: Vec<Vec<Run>>,
}

/// Digital Precipitation Data Array Packet, code 17 (Figure 3-11a).
///
/// Unlike the other RLE packets this one uses a **full byte** for the run and
/// a full byte for the level (Sheet 2: "Run(0) 1 Byte ... 0 to 255",
/// "Level(0) 1 Byte ... 0 to 255"), so a run/level pair is two bytes.
pub fn digital_precipitation_array(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, header) = precip_array_header(input)?;
    let (input, rows) = byte_rle_rows(input, header.num_rows)?;
    Ok((
        input,
        SymPacketData::DigitalPrecipitationDataArray(PrecipArrayPacket { header, rows }),
    ))
}

/// Precipitation Rate Data Array Packet, code 18 (Figure 3-11b).
///
/// This one packs run and level into the two nibbles of a single byte, as
/// shown by "RUN (0) LEVEL (0) RUN (1) LEVEL (1)" occupying one halfword in
/// Sheet 1.
pub fn precipitation_rate_array(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, header) = precip_array_header(input)?;
    let (input, rows) = nibble_rle_rows(input, header.num_rows)?;
    Ok((
        input,
        SymPacketData::PrecipitationRateDataArray(PrecipArrayPacket { header, rows }),
    ))
}

fn precip_array_header(input: &[u8]) -> IResult<&[u8], PrecipArrayHeader> {
    let (input, packet_code) = nom_i16(Big)(input)?;
    let (input, spare_1) = nom_i16(Big)(input)?;
    let (input, spare_2) = nom_i16(Big)(input)?;
    let (input, num_boxes) = nom_i16(Big)(input)?;
    let (input, num_rows) = nom_i16(Big)(input)?;
    Ok((
        input,
        PrecipArrayHeader {
            packet_code,
            spares: [spare_1, spare_2],
            num_boxes,
            num_rows,
        },
    ))
}

/// Reads `num_rows` rows of byte-per-run, byte-per-level RLE data.
fn byte_rle_rows(mut input: &[u8], num_rows: i16) -> IResult<&[u8], Vec<Vec<Run>>> {
    let row_count = match usize::try_from(num_rows) {
        Ok(n) => n,
        Err(_) => return fail(input, &format!("Packet declares {num_rows} rows")),
    };

    let mut rows = Vec::with_capacity(row_count.min(1024));
    for _ in 0..row_count {
        let (rest, num_bytes) = nom_i16(Big)(input)?;
        let len = match usize::try_from(num_bytes) {
            Ok(n) => n,
            Err(_) => return fail(rest, &format!("Row declares {num_bytes} bytes")),
        };
        let (rest, body) = payload(rest, len)?;
        rows.push(
            body.chunks_exact(2)
                .map(|c| Run {
                    run: c[0] as u16,
                    level: c[1] as u16,
                })
                .collect(),
        );
        input = rest;
    }
    Ok((input, rows))
}

/// Digital Raster Data Array Packet header — Figure 3-11d, page 3-102.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct DigitalRasterHeader {
    /// Packet Code, 33.
    pub packet_code: i16,
    /// I coordinate of the upper left corner (pixels).
    pub i_start: i16,
    /// J coordinate of the upper left corner (pixels).
    pub j_start: i16,
    /// Vertical scale factor (1 to 10).
    pub i_scale: i16,
    /// Horizontal scale factor (1 to 10).
    pub j_scale: i16,
    /// Total number of cells in a raster row.
    pub num_cells: i16,
    /// Total number of raster rows in the product.
    pub num_rows: i16,
}

/// A digital raster packet: a header plus one row of 8-bit data levels per row.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct DigitalRasterPacket {
    pub header: DigitalRasterHeader,
    /// One entry per row, each holding that row's 8-bit data level codes.
    pub rows: Vec<Vec<u8>>,
}

/// Digital Raster Data Array Packet, code 33 (Figure 3-11d).
///
/// Rows are *not* run-length encoded — each row is a plain run of 8-bit data
/// level codes.
pub fn digital_raster_array(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, packet_code) = nom_i16(Big)(input)?;
    let (input, i_start) = nom_i16(Big)(input)?;
    let (input, j_start) = nom_i16(Big)(input)?;
    let (input, i_scale) = nom_i16(Big)(input)?;
    let (input, j_scale) = nom_i16(Big)(input)?;
    let (input, num_cells) = nom_i16(Big)(input)?;
    let (mut input, num_rows) = nom_i16(Big)(input)?;

    let header = DigitalRasterHeader {
        packet_code,
        i_start,
        j_start,
        i_scale,
        j_scale,
        num_cells,
        num_rows,
    };

    let row_count = match usize::try_from(num_rows) {
        Ok(n) => n,
        Err(_) => return fail(input, &format!("Packet declares {num_rows} rows")),
    };

    let mut rows = Vec::with_capacity(row_count.min(1024));
    for _ in 0..row_count {
        let (rest, num_bytes) = nom_i16(Big)(input)?;
        let len = match usize::try_from(num_bytes) {
            Ok(n) => n,
            Err(_) => return fail(rest, &format!("Row declares {num_bytes} bytes")),
        };
        let (rest, body) = payload(rest, len)?;
        rows.push(body.to_vec());
        input = rest;
    }

    Ok((
        input,
        SymPacketData::DigitalRasterDataArray(DigitalRasterPacket { header, rows }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hw(values: &[i16]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_be_bytes()).collect()
    }

    #[test]
    fn parses_a_raster_data_packet() {
        let mut bytes = hw(&[
            0xBA0Fu16 as i16,
            0x8000u16 as i16,
            0x00C0u16 as i16,
            -100, // i_start
            -200, // j_start
            2,    // x scale int
            0,    // x scale fractional
            3,    // y scale int
            0,    // y scale fractional
            2,    // number of rows
            2,    // packing descriptor
        ]);
        // Row 1: two runs (run 5 level 3, run 2 level 1).
        bytes.extend_from_slice(&hw(&[2]));
        bytes.extend_from_slice(&[0x53, 0x21]);
        // Row 2: one run (run 15 level 0).
        bytes.extend_from_slice(&hw(&[1]));
        bytes.push(0xF0);

        let (rest, parsed) = raster_data(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::RasterData(p) => {
                assert_eq!(p.header.i_start, -100);
                assert_eq!(p.header.num_rows, 2);
                assert_eq!(p.rows.len(), 2);
                assert_eq!(p.rows[0], vec![Run { run: 5, level: 3 }, Run { run: 2, level: 1 }]);
                assert_eq!(p.rows[1], vec![Run { run: 15, level: 0 }]);
            }
            other => panic!("expected RasterData, got {other:?}"),
        }
    }

    #[test]
    fn parses_digital_precipitation_array_with_byte_wide_runs() {
        let mut bytes = hw(&[17, 0, 0, 131, 1]);
        // One row: run 200 at level 250 — both need a full byte.
        bytes.extend_from_slice(&hw(&[2]));
        bytes.extend_from_slice(&[200, 250]);

        let (rest, parsed) = digital_precipitation_array(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::DigitalPrecipitationDataArray(p) => {
                assert_eq!(p.header.num_boxes, 131);
                assert_eq!(p.rows[0], vec![Run { run: 200, level: 250 }]);
            }
            other => panic!("expected DigitalPrecipitationDataArray, got {other:?}"),
        }
    }

    #[test]
    fn parses_precipitation_rate_array_with_nibble_runs() {
        let mut bytes = hw(&[18, 0, 0, 131, 1]);
        bytes.extend_from_slice(&hw(&[1]));
        bytes.push(0x7A); // run 7, level 10

        let (_, parsed) = precipitation_rate_array(&bytes).unwrap();
        match parsed {
            SymPacketData::PrecipitationRateDataArray(p) => {
                assert_eq!(p.rows[0], vec![Run { run: 7, level: 10 }]);
            }
            other => panic!("expected PrecipitationRateDataArray, got {other:?}"),
        }
    }

    #[test]
    fn parses_digital_raster_array_as_plain_levels() {
        let mut bytes = hw(&[33, 0, 0, 1, 1, 3, 2]);
        bytes.extend_from_slice(&hw(&[3]));
        bytes.extend_from_slice(&[10, 20, 30]);
        bytes.extend_from_slice(&hw(&[3]));
        bytes.extend_from_slice(&[40, 50, 60]);

        let (rest, parsed) = digital_raster_array(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::DigitalRasterDataArray(p) => {
                assert_eq!(p.header.num_cells, 3);
                assert_eq!(p.rows, vec![vec![10, 20, 30], vec![40, 50, 60]]);
            }
            other => panic!("expected DigitalRasterDataArray, got {other:?}"),
        }
    }

    #[test]
    fn rejects_a_negative_row_count() {
        let bytes = hw(&[33, 0, 0, 1, 1, 3, -5]);
        assert!(digital_raster_array(&bytes).is_err());
    }

    #[test]
    fn rejects_a_truncated_row() {
        let mut bytes = hw(&[33, 0, 0, 1, 1, 3, 1]);
        bytes.extend_from_slice(&hw(&[8])); // claims 8 bytes
        bytes.extend_from_slice(&[1, 2]); // supplies 2
        assert!(digital_raster_array(&bytes).is_err());
    }
}
