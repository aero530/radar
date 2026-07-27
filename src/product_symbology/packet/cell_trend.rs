//! Cell trend packets:
//!
//! - Cell Trend Data Packet, code 21 (Figure 3-15, pages 3-108 to 3-109)
//! - Cell Trend Volume Scan Times, code 22 (Figure 3-15a, page 3-109)

use serde::{Deserialize, Serialize};
use nom::{
    number::{complete::i16 as nom_i16, Endianness::Big},
    IResult,
};

use super::util::{block_length, payload};
use crate::product_symbology::SymPacketData;

/// Which quantity a cell trend series holds — Figure 3-15 sheet 1.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TrendCode {
    CellTop = 1,
    CellBase = 2,
    MaxReflectivityHeight = 3,
    ProbabilityOfHail = 4,
    ProbabilityOfSevereHail = 5,
    CellBasedVil = 6,
    MaxReflectivity = 7,
    CentroidHeight = 8,
    /// A trend code outside the documented 1-8 range.
    Unknown = 0,
}

impl TrendCode {
    fn from_i16(value: i16) -> Self {
        match value {
            1 => TrendCode::CellTop,
            2 => TrendCode::CellBase,
            3 => TrendCode::MaxReflectivityHeight,
            4 => TrendCode::ProbabilityOfHail,
            5 => TrendCode::ProbabilityOfSevereHail,
            6 => TrendCode::CellBasedVil,
            7 => TrendCode::MaxReflectivity,
            8 => TrendCode::CentroidHeight,
            _ => TrendCode::Unknown,
        }
    }

    /// The units this trend's values are expressed in, per Note 1 of
    /// Figure 3-15 sheet 2.
    pub fn units(&self) -> &'static str {
        match self {
            TrendCode::CellTop
            | TrendCode::CellBase
            | TrendCode::MaxReflectivityHeight
            | TrendCode::CentroidHeight => "hundreds of feet",
            TrendCode::ProbabilityOfHail | TrendCode::ProbabilityOfSevereHail => "percent",
            TrendCode::CellBasedVil => "kg/m**2",
            TrendCode::MaxReflectivity => "dBZ",
            TrendCode::Unknown => "unknown",
        }
    }
}

/// One trend series within a Cell Trend Data Packet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CellTrend {
    pub trend_code: TrendCode,
    /// Number of volume scans of trend data in the circular list, 1 to 10.
    pub num_volumes: u8,
    /// One-based pointer to the latest volume scan in the circular list.
    pub latest_volume_pointer: u8,
    /// The trend values, one per volume scan in the circular list.
    ///
    /// Per Note 2, a cell top/base value over 700 has had 1000 added to it to
    /// denote that it was detected on the highest/lowest elevation scan. Per
    /// Note 3, -999 denotes an unknown value.
    pub values: Vec<i16>,
}

/// Cell Trend Data Packet, code 21 (Figure 3-15).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct CellTrendPacket {
    /// Two character cell ID: a letter followed by a digit.
    pub cell_id: String,
    /// Cell I coordinate at the latest volume scan (Km/8).
    pub i_position: i16,
    /// Cell J coordinate at the latest volume scan (Km/8).
    pub j_position: i16,
    pub trends: Vec<CellTrend>,
}

/// Cell Trend Data Packet, packet code 21 (Figure 3-15).
pub fn cell_trend_data(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, _packet_code) = nom_i16(Big)(input)?;
    let (input, len) = block_length(input)?;
    let (input, body) = payload(input, len)?;

    // Cell ID (2 ASCII bytes), then the I/J position halfwords.
    if body.len() < 6 {
        return super::util::fail(input, "Cell trend packet is too short to hold its cell header");
    }
    let cell_id = String::from_utf8_lossy(&body[0..2]).into_owned();
    let i_position = i16::from_be_bytes([body[2], body[3]]);
    let j_position = i16::from_be_bytes([body[4], body[5]]);

    // Then a variable number of trend series, each: trend code (halfword),
    // # volumes (byte), latest volume pointer (byte), then that many
    // halfword values.
    let mut rest = &body[6..];
    let mut trends = Vec::new();
    while rest.len() >= 4 {
        let trend_code = TrendCode::from_i16(i16::from_be_bytes([rest[0], rest[1]]));
        let num_volumes = rest[2];
        let latest_volume_pointer = rest[3];
        rest = &rest[4..];

        let wanted = num_volumes as usize * 2;
        let available = wanted.min(rest.len());
        let values = rest[..available]
            .chunks_exact(2)
            .map(|c| i16::from_be_bytes([c[0], c[1]]))
            .collect();
        rest = &rest[available..];

        trends.push(CellTrend {
            trend_code,
            num_volumes,
            latest_volume_pointer,
            values,
        });
    }

    Ok((
        input,
        SymPacketData::CellTrendData(CellTrendPacket {
            cell_id,
            i_position,
            j_position,
            trends,
        }),
    ))
}

/// Cell Trend Volume Scan Times, code 22 (Figure 3-15a).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct CellTrendVolumeTimesPacket {
    /// Number of cell trend volume scan times in the circular list, 1 to 10.
    pub num_volumes: i16,
    /// One-based pointer to the latest volume scan time in the circular list.
    pub latest_volume_pointer: i16,
    /// The volume scan times themselves.
    pub times: Vec<i16>,
}

/// Cell Trend Volume Scan Times packet, packet code 22 (Figure 3-15a).
pub fn cell_trend_volume_times(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, _packet_code) = nom_i16(Big)(input)?;
    let (input, len) = block_length(input)?;
    let (input, body) = payload(input, len)?;

    if body.len() < 4 {
        return super::util::fail(
            input,
            "Cell trend volume times packet is too short to hold its header",
        );
    }
    let num_volumes = i16::from_be_bytes([body[0], body[1]]);
    let latest_volume_pointer = i16::from_be_bytes([body[2], body[3]]);
    let times = body[4..]
        .chunks_exact(2)
        .map(|c| i16::from_be_bytes([c[0], c[1]]))
        .collect();

    Ok((
        input,
        SymPacketData::CellTrendVolumeScanTimes(CellTrendVolumeTimesPacket {
            num_volumes,
            latest_volume_pointer,
            times,
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hw(values: &[i16]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_be_bytes()).collect()
    }

    #[test]
    fn parses_a_cell_trend_packet_with_two_series() {
        let mut body = Vec::new();
        body.extend_from_slice(b"B7"); // cell id
        body.extend_from_slice(&hw(&[120, -240])); // i, j position

        // Trend 1: cell top, 3 volumes, latest = 3
        body.extend_from_slice(&hw(&[1]));
        body.push(3);
        body.push(3);
        body.extend_from_slice(&hw(&[300, 350, 1400]));

        // Trend 2: probability of hail, 2 volumes, latest = 1, one unknown
        body.extend_from_slice(&hw(&[4]));
        body.push(2);
        body.push(1);
        body.extend_from_slice(&hw(&[80, -999]));

        let mut bytes = hw(&[21, body.len() as i16]);
        bytes.extend_from_slice(&body);

        let (rest, parsed) = cell_trend_data(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::CellTrendData(p) => {
                assert_eq!(p.cell_id, "B7");
                assert_eq!(p.i_position, 120);
                assert_eq!(p.j_position, -240);
                assert_eq!(p.trends.len(), 2);

                assert_eq!(p.trends[0].trend_code, TrendCode::CellTop);
                assert_eq!(p.trends[0].values, vec![300, 350, 1400]);
                assert_eq!(p.trends[0].trend_code.units(), "hundreds of feet");

                assert_eq!(p.trends[1].trend_code, TrendCode::ProbabilityOfHail);
                assert_eq!(p.trends[1].values, vec![80, -999]);
                assert_eq!(p.trends[1].trend_code.units(), "percent");
            }
            other => panic!("expected CellTrendData, got {other:?}"),
        }
    }

    #[test]
    fn unknown_trend_codes_are_preserved_rather_than_failing() {
        let mut body = Vec::new();
        body.extend_from_slice(b"C1");
        body.extend_from_slice(&hw(&[0, 0]));
        body.extend_from_slice(&hw(&[99])); // not a documented trend code
        body.push(1);
        body.push(1);
        body.extend_from_slice(&hw(&[5]));

        let mut bytes = hw(&[21, body.len() as i16]);
        bytes.extend_from_slice(&body);

        let (_, parsed) = cell_trend_data(&bytes).unwrap();
        match parsed {
            SymPacketData::CellTrendData(p) => {
                assert_eq!(p.trends[0].trend_code, TrendCode::Unknown);
                assert_eq!(p.trends[0].values, vec![5]);
            }
            other => panic!("expected CellTrendData, got {other:?}"),
        }
    }

    #[test]
    fn parses_cell_trend_volume_scan_times() {
        let body = hw(&[3, 2, 1000, 2000, 3000]);
        let mut bytes = hw(&[22, body.len() as i16]);
        bytes.extend_from_slice(&body);

        let (rest, parsed) = cell_trend_volume_times(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::CellTrendVolumeScanTimes(p) => {
                assert_eq!(p.num_volumes, 3);
                assert_eq!(p.latest_volume_pointer, 2);
                assert_eq!(p.times, vec![1000, 2000, 3000]);
            }
            other => panic!("expected CellTrendVolumeScanTimes, got {other:?}"),
        }
    }

    #[test]
    fn rejects_a_cell_trend_packet_that_cannot_hold_its_header() {
        let body = hw(&[1]);
        let mut bytes = hw(&[21, body.len() as i16]);
        bytes.extend_from_slice(&body);
        assert!(cell_trend_data(&bytes).is_err());
    }
}
