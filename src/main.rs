// cSpell:disable

use std::path::PathBuf;
// use chrono::{DateTime, Utc, NaiveDate, NaiveDateTime};

use nom::{
    bytes::complete::{tag, take_while_m_n, take}, character::complete::{alpha0, digit0}, combinator::map_res, sequence::tuple, AsBytes, AsChar, IResult
};
use tracing::{debug, warn, error, trace, info, Level};
use tracing_subscriber::filter::EnvFilter;

#[derive(Clone, Debug, PartialEq)]
pub struct TextHeader {
    xx: usize,
    location: String,
    timestamp: String,
    aaa: String,
    bbb: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Radar {
    text_header: TextHeader,
}
                    
/// Format of Text header is SDUSXX KYYYY DDHHMM\r\r\nAAABBB\r\r\n
fn text_header(input: &[u8]) -> IResult<&[u8], TextHeader> {
    // remove SDUS from input
    let (input, _) = tag("SDUS".as_bytes())(input)?;
    
    // grab the digits after SDUD
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

    // linebreaks for some reason 0x0D 0x0D 0x0A
    let (input, _) = tag([0x0D,0x0D,0x0A])(input)?;

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

    // linebreaks for some reason 0x0D 0x0D 0x0A
    let (input, _) = tag([0x0D,0x0D,0x0A])(input)?;

    Ok((
        input,
        TextHeader {
            xx,
            location,
            timestamp,
            aaa: aaa.clone(),
            bbb: bbb.clone(),
        }
    ))
}

fn parse(input: &[u8]) -> IResult<&[u8], Radar> {
    
    let (input,text_header) = text_header(input)?;
    
    
    Ok((
        input,
        Radar {
            text_header
        }
    ))
}

fn main() {
    let filter = EnvFilter::builder()
    .with_default_directive(Level::TRACE.into())
    .from_env().unwrap_or_default(); // set noisy logs to info
    // .add_directive("hyper::client=INFO".parse()?);
    
    let subscriber = tracing_subscriber::fmt()
    .with_ansi(true)
    .with_env_filter(filter);
    
    subscriber.init();
    
    error!("Designates very serious errors.");
    warn!("Designates hazardous situations.");
    info!("Designates useful information.");
    debug!("Designates lower priority information.");
    trace!("Designates very low priority, often extremely verbose, information.");
    
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => "".to_string(),
    };
    let filename = PathBuf::from(&path);
    let file = std::fs::read(&filename).unwrap_or_default();
    
    match parse(&file) {
        Ok((leftover, value)) => {
            // warn!("Unmatched {:?}", leftover); 
            info!("{:?}", value)
        },
        Err(e) => error!("{:?}",e)
    }
    
}