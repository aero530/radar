use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use nom::{IResult, *};
use tracing::info;

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

    let (input, c) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let code = <MessageCode as num::FromPrimitive>::from_i16(c);

    // Modified Julian Date at time of transmission (number of days since 1 
    // January 1970, where 1=1 January 1970). To obtain actual Julian Date, 
    // add 2,440,586.5 to the modified date
    let (input, days) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    // Number of seconds after midnight, Greenwich Mean Time (GMT).
    let (input, seconds) = number::complete::i32(nom::number::Endianness::Big)(input)?;

    let datetime = DateTime::from_timestamp((days as i64)*24*60*60 + (seconds as i64), 0).unwrap_or_default();

    let (input, length) = number::complete::i32(nom::number::Endianness::Big)(input)?;
    let (input, source) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let (input, dest) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let (input, nblocks) = number::complete::i16(nom::number::Endianness::Big)(input)?;

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