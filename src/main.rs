// cSpell:disable

use std::path::PathBuf;


use nom::{
    bytes::complete::{tag, take_while_m_n}, character::complete::{alpha0, digit0}, combinator::map_res, sequence::tuple, AsChar, IResult};
use tracing::{debug, warn, error, trace, info, Level};
use tracing_subscriber::filter::EnvFilter;
  
  #[derive(Debug,PartialEq)]
  pub struct Color {
    pub red:     u8,
    pub green:   u8,
    pub blue:    u8,
  }
  
//   fn from_hex(input: &[u8]) -> Result<u8, std::num::ParseIntError> {
//     u8::from_str_radix(input, 16)
//   }
  
//   fn is_hex_digit(c: u8) -> bool {
//     c.is_digit(16)
//   }
  
//   fn hex_primary(input: &[u8]) -> IResult<&str, u8> {
//     map_res(
//       take_while_m_n(2, 2, is_hex_digit),
//       from_hex
//     )(input)
//   }
  
  fn hex_color(input: &[u8]) -> IResult<&[u8], usize> {
    
    let (input, _) = tag("SDUS".as_bytes())(input)?;

    let (input, a) = digit0(input)?;    
    let number_thing = match std::str::from_utf8(a) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    trace!("{:?}", number_thing.parse::<usize>().unwrap());

    let (input, _) = tag(" ".as_bytes())(input)?;

    let (input, b) = alpha0(input)?;
    let location = match std::str::from_utf8(b) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    trace!("{}", location);

    // let (input, _) = tag(" ")(input)?;
    // let (input, location) = alpha0(input)?;
    // let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;
  
    Ok((input, number_thing.parse::<usize>().unwrap()))
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

    match hex_color(&file) {
      Ok((leftover, value)) => {warn!("Unmatched {:?}", leftover); info!("{:?}", value)},
      Err(e) => error!("{:?}",e)
    }
    
  }