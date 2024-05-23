use serde::{Deserialize, Serialize};
use nom::{
    bytes::complete::take, 
    IResult, 
    number::complete::{i16 as nom_i16, i32 as nom_i32},
    number::Endianness::Big,
};
use tracing::{error, info};

/// Graphic Product Message: Product Description Block
/// Description: section 3.3.1.1, page 3-3
/// 102 bytes, 51 halfwords (halfwords 10-60)
/// Figure 3-6 Sheet 6, pages 3-25 through 3-26
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProductDescription {
    /// Delineate blocks, -1
    pub divider: i16,
    /// Latitude of radar, degrees, + for north
    pub latitude: i32,
    /// Longitude of radar, degrees, + for east
    pub longitude: i32,
    /// Height of radar, feet above mean sea level
    pub height: i16,
    /// NEXRAD product code
    pub product_code: i16,
    /// 0 = Maintenance, 1 = Clean Air, 2 = Precipitation
    pub operational_mode: i16,
    /// Volume Coverage Pattern of scan strategy
    pub vcp: i16,
    /// Sequence Number of the request.
    pub sequence_num: i16,
    /// Volume Scan number, 1 to 80.
    pub vol_scan_num: i16,
    /// Volume Scan start date, days since 1/1/1970
    pub vol_scan_date: i16,
    /// Volume Scan start time, sec since midnight
    pub vol_scan_time: i32,
    /// Product Generation Date, days since 1/1/1970
    pub product_date: i16,
    /// Product Generation Time, sec since midnight
    pub product_time: i32,
    ///  Product dependent parameters 1 and 2 TABLE V (length 4s)
    pub halfwords_27_28: Vec<u8>,
    /// Elevation number within volume scan
    pub elevation_num: i16,
    ///  Product dependent parameter 3 --- PRODUCT DEPENDENT PARAMETERS 1 AND 2 (SEE TABLE V) (length 2s)
    pub halfwords_30: Vec<u8>,
    ///  Data to determine threshold level values --- PRODUCT DEPENDENT (SEE NOTE 1) (length 32s)
    pub threshold_data: Vec<u8>,
    ///  Product dependent parameters 4-10 --- PRODUCT DEPENDENT PARAMETERS 4 THROUGH 10 (SEE TABLE V, NOTE 3) (length 14s)
    pub halfwords_47_53: Vec<u8>,
    /// Version, 0
    pub version: u8,
    /// 1 = Spot blank ON, 0 = Blanking OFF
    pub spot_blank: u8,
    /// halfword offset to Symbology block
    pub offset_symbology: i32,
    /// halfword offset to Graphic block
    pub offset_graphic: i32,
    /// halfword offset to Tabular block
    pub offset_tabular: i32,
}


/// Graphic Product Message: Product Description Block
/// 51 halfwords
/// Figure 3-6 Sheet 2, 6, 7, page 3-24 to 3-26.
pub fn product_description(input: &[u8]) -> IResult<&[u8], ProductDescription> {
    let (input, divider) = nom_i16(Big)(input)?; //: i16,
    if divider != -1 {
        error!("Block divider error");
    }
    let (input, latitude) = nom_i32(Big)(input)?; //: i32,
    let (input, longitude) = nom_i32(Big)(input)?; //: i32,
    let (input, height) = nom_i16(Big)(input)?; //: i16,
    let (input, product_code) = nom_i16(Big)(input)?; //: i16,
    let (input, operational_mode) = nom_i16(Big)(input)?; //: i16,
    let (input, vcp) = nom_i16(Big)(input)?; //: i16,
    let (input, sequence_num) = nom_i16(Big)(input)?; //: i16,
    let (input, vol_scan_num) = nom_i16(Big)(input)?; //: i16,
    let (input, vol_scan_date) = nom_i16(Big)(input)?; //: i16,
    let (input, vol_scan_time) = nom_i32(Big)(input)?; //: i32,
    let (input, product_date) = nom_i16(Big)(input)?; //: i16,
    let (input, product_time) = nom_i32(Big)(input)?; //: i32,
    let (input, halfwords_27_28) = take(4usize)(input)?; //: [u8;4],
    let (input, elevation_num) = nom_i16(Big)(input)?; //: i16,
    let (input, halfwords_30) = take(2usize)(input)?; //: [u8;2],
    let (input, threshold_data) = take(32usize)(input)?; //: [u8;32],
    let (input, halfwords_47_53) = take(14usize)(input)?; //: [u8;14],
    let (input, version) = take(1usize)(input)?; //: u8,
    let (input, spot_blank) = take(1usize)(input)?; //: u8,
    let (input, offset_symbology) = nom_i32(Big)(input)?; //: i32,
    let (input, offset_graphic) = nom_i32(Big)(input)?; //: i32,
    let (input, offset_tabular) = nom_i32(Big)(input)?; //: i32,

    // let (input, c) = nom_i16(Big)(input)?;
    // let code = <MessageCode as num::FromPrimitive>::from_i16(c);

    // let (input, days) = nom_i16(Big)(input)?;
    // let (input, seconds) = nom_i32(Big)(input)?;
    // let date_time = DateTime::from_timestamp((days as i64)*24*60*60 + (seconds as i64), 0).unwrap_or_default();



    info!("Symbology located at {}", offset_symbology);
    info!("Graphic located at {}", offset_graphic);
    info!("Tabular located at {}", offset_tabular);

    Ok((
        input,
        ProductDescription {
            divider,
            latitude,
            longitude,
            height,
            product_code,
            operational_mode,
            vcp,
            sequence_num,
            vol_scan_num,
            vol_scan_date,
            vol_scan_time,
            product_date,
            product_time,
            halfwords_27_28: halfwords_27_28.to_vec(),
            elevation_num,
            halfwords_30: halfwords_30.to_vec(),
            threshold_data: threshold_data.to_vec(),
            halfwords_47_53: halfwords_47_53.to_vec(),
            version: version[0],
            spot_blank: spot_blank[0],
            offset_symbology,
            offset_graphic,
            offset_tabular,
        },
    ))
}
