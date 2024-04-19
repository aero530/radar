

use chrono::{DateTime, Utc};

use nom::{
    IResult, *
};


use super::MessageCode;


/// Graphic Product Message: Message Header Block
/// 18 bytes, 9 halfwords
/// Figure 3-3, page 3-7.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MessageHeader {
    /// message code
    pub code: MessageCode,
    /// date & time of message, days since 1 Jan, 1970 GMT
    pub datetime: DateTime<Utc>,
    /// length of message in bytes
    pub length: i32,
    /// Source ID
    pub source: i16,
    /// Destination ID
    pub dest: i16,
    /// Number of blocks in the message (inclusive)
    pub nblocks: i16,
}

/// Graphic Product Message: Message Header Block
/// 18 bytes, 9 halfwords
/// Figure 3-3, page 3-7.
pub fn message_header(input: &[u8]) -> IResult<&[u8], MessageHeader> {

    let (input, c) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let code = <MessageCode as num::FromPrimitive>::from_i16(c);

    let (input, days) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let (input, seconds) = number::complete::i32(nom::number::Endianness::Big)(input)?;
    let datetime = DateTime::from_timestamp((days as i64)*24*60*60 + (seconds as i64), 0).unwrap_or_default();

    let (input, length) = number::complete::i32(nom::number::Endianness::Big)(input)?;
    let (input, source) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let (input, dest) = number::complete::i16(nom::number::Endianness::Big)(input)?;
    let (input, nblocks) = number::complete::i16(nom::number::Endianness::Big)(input)?;

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