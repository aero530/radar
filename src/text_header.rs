use serde::{Deserialize, Serialize};
use nom::{
    bytes::complete::{tag, take},
    character::complete::digit0,
    IResult, 
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TextHeader {
    pub xx: usize,
    pub location: String,
    pub timestamp: String,
    pub aaa: String,
    pub bbb: String,
}

/// Format of Text header is SDUSXX KYYYY DDHHMM\r\r\nAAABBB\r\r\n
pub fn text_header(input: &[u8]) -> IResult<&[u8], TextHeader> {
    // remove SDUS from input
    let (input, _) = tag("SDUS".as_bytes())(input)?;

    // grab the digits after SDUS
    let (input, a) = digit0(input)?;
    let xx_str = match std::str::from_utf8(a) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let xx = xx_str.parse::<usize>().unwrap();

    // space
    let (input, _) = tag(" ".as_bytes())(input)?;

    // location
    let (input, part) = take(4usize)(input)?;
    let location = match std::str::from_utf8(part) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // space
    let (input, _) = tag(" ".as_bytes())(input)?;

    // date
    let (input, part) = take(6usize)(input)?;
    let timestamp = match std::str::from_utf8(part) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // line breaks
    let (input, _) = tag([0x0D, 0x0D, 0x0A])(input)?;

    // other bits
    let (input, part) = take(3usize)(input)?;
    let aaa = match std::str::from_utf8(part) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let (input, part) = take(3usize)(input)?;
    let bbb = match std::str::from_utf8(part) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // line breaks
    let (input, _) = tag([0x0D, 0x0D, 0x0A])(input)?;

    Ok((
        input,
        TextHeader {
            xx,
            location,
            timestamp,
            aaa: aaa.clone(),
            bbb: bbb.clone(),
        },
    ))
}