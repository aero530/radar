use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tracing::info;
use nom::{
    number::complete::{i16 as nom_i16, i32 as nom_i32},
    number::Endianness::Big,
    IResult,
};

use super::MessageCode;


/// Graphic Product Message: Message Header Block
/// 18 bytes, 9 halfwords
/// Figure 3-3, page 3-6.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MessageHeader {
    /// message code
    pub code: MessageCode,
    /// date & time of message, days since 1 Jan, 1970 GMT
    pub datetime: DateTime<Utc>,
    /// Number of bytes in message including header
    pub length: i32,
    /// Source ID
    pub source: i16,
    /// Destination ID
    pub dest: i16,
    /// Header Block plus the Product Description Blocks in message
    pub nblocks: i16,
}

/// Graphic Product Message: Message Header Block
/// 18 bytes, 9 halfwords
/// Figure 3-3, page 3-6.
pub fn message_header(input: &[u8]) -> IResult<&[u8], MessageHeader> {

    let (input, c) = nom_i16(Big)(input)?;
    let code = <MessageCode as num::FromPrimitive>::from_i16(c);

    // Modified Julian Date at time of transmission (number of days since 1
    // January 1970, where 1=1 January 1970). To obtain actual Julian Date,
    // add 2,440,586.5 to the modified date
    let (input, days) = nom_i16(Big)(input)?;
    // Number of seconds after midnight, Greenwich Mean Time (GMT).
    let (input, seconds) = nom_i32(Big)(input)?;

    // The date is 1-based per Figure 3-3 ("where 1=1 January 1970"), so the
    // number of days elapsed since the Unix epoch is `days - 1`. Omitting the
    // -1 puts every parsed timestamp exactly one day in the future.
    let datetime = DateTime::from_timestamp((days as i64 - 1)*24*60*60 + (seconds as i64), 0).unwrap_or_default();

    let (input, length) = nom_i32(Big)(input)?;
    let (input, source) = nom_i16(Big)(input)?;
    let (input, dest) = nom_i16(Big)(input)?;
    let (input, nblocks) = nom_i16(Big)(input)?;

    info!("{} product description blocks", nblocks-1);
    Ok((
        input,
        MessageHeader {
            code: code.unwrap_or_default(),
            datetime,
            length,
            source,
            dest,
            nblocks,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_real_sample_files_message_header() {
        let file = include_bytes!("../data/sn_DS.p20-r_kmkx.last");
        // The text header is exactly the first 30 bytes; the message
        // header immediately follows it.
        let (rest, header) = message_header(&file[30..]).unwrap();

        assert_eq!(header.code, MessageCode::BaseReflectivity20);
        assert_eq!(header.nblocks, 3);
        assert_eq!(rest.len(), file.len() - 30 - 18);
    }

    /// The Modified Julian Date in the message header is 1-based
    /// ("where 1=1 January 1970", Figure 3-3), so day 1 must decode to
    /// 1970-01-01 rather than 1970-01-02.
    #[test]
    fn modified_julian_date_is_one_based() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&20i16.to_be_bytes()); // code: Base Spectrum Width
        bytes.extend_from_slice(&1i16.to_be_bytes()); // date: 1 == 1 January 1970
        bytes.extend_from_slice(&0i32.to_be_bytes()); // time: midnight
        bytes.extend_from_slice(&0i32.to_be_bytes()); // length
        bytes.extend_from_slice(&0i16.to_be_bytes()); // source
        bytes.extend_from_slice(&0i16.to_be_bytes()); // dest
        bytes.extend_from_slice(&1i16.to_be_bytes()); // nblocks

        let (_, header) = message_header(&bytes).unwrap();

        assert_eq!(header.datetime.to_string(), "1970-01-01 00:00:00 UTC");
    }

    /// The sample file's own WMO text header says it was issued on day 09 at
    /// 12:53 UTC (`SDUS73 KMKX 091253`), so the binary message header must
    /// decode to that same calendar day — not the day after.
    #[test]
    fn sample_file_datetime_agrees_with_its_text_header_day() {
        let file = include_bytes!("../data/sn_DS.p20-r_kmkx.last");
        let (_, header) = message_header(&file[30..]).unwrap();

        assert_eq!(header.datetime.to_string(), "2022-09-09 12:55:14 UTC");
    }

    #[test]
    fn unrecognized_code_falls_back_to_spare_instead_of_failing() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(-1i16).to_be_bytes()); // code: not a real MessageCode
        bytes.extend_from_slice(&0i16.to_be_bytes()); // date
        bytes.extend_from_slice(&0i32.to_be_bytes()); // time
        bytes.extend_from_slice(&0i32.to_be_bytes()); // length
        bytes.extend_from_slice(&0i16.to_be_bytes()); // source
        bytes.extend_from_slice(&0i16.to_be_bytes()); // dest
        bytes.extend_from_slice(&1i16.to_be_bytes()); // nblocks

        let (rest, header) = message_header(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(header.code, MessageCode::Spare);
    }

    #[test]
    fn rejects_truncated_input() {
        assert!(message_header(&[0, 20, 0, 1]).is_err());
    }
}
