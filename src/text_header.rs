use serde::{Deserialize, Serialize};
use nom::{
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::map_res,
    IResult, Parser,
};

/// The WMO/AWIPS text header that precedes every NEXRAD Level 3 product
/// message.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TextHeader {
    /// WMO product category code, e.g. `73` in `SDUS73`.
    pub xx: usize,
    /// Originating radar site identifier, e.g. `KMKX`.
    pub location: String,
    /// Six digit `DDHHMM` issuance timestamp, as raw text (day/hour/minute
    /// of month, UTC).
    pub timestamp: String,
    /// First three characters of the six character AWIPS product id.
    pub aaa: String,
    /// Last three characters of the six character AWIPS product id.
    pub bbb: String,
}

/// Converts a byte slice known to be ASCII into an owned `String`, instead
/// of panicking on unexpected non-UTF-8 bytes.
fn ascii_string(bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    Ok(std::str::from_utf8(bytes)?.to_owned())
}

/// Parses the text header block that begins every NEXRAD Level 3 file.
///
/// Format: `SDUS<xx> <location> <DDHHMM>\r\r\n<aaa><bbb>\r\r\n`
///
/// Never panics: malformed or truncated input yields a `nom` parse error
/// instead.
pub fn text_header(input: &[u8]) -> IResult<&[u8], TextHeader> {
    // remove SDUS from input
    let (input, _) = tag("SDUS".as_bytes())(input)?;

    // grab the digits after SDUS (at least one digit is required, since an
    // empty match here cannot be parsed as a number)
    let (input, xx) = map_res(digit1, |bytes: &[u8]| -> Result<usize, Box<dyn std::error::Error>> {
        Ok(std::str::from_utf8(bytes)?.parse()?)
    }).parse(input)?;

    // space
    let (input, _) = tag(" ".as_bytes())(input)?;

    // location
    let (input, location) = map_res(take(4usize), ascii_string).parse(input)?;

    // space
    let (input, _) = tag(" ".as_bytes())(input)?;

    // date
    let (input, timestamp) = map_res(take(6usize), ascii_string).parse(input)?;

    // line breaks
    let (input, _) = tag([0x0D, 0x0D, 0x0A].as_slice())(input)?;

    // other bits
    let (input, aaa) = map_res(take(3usize), ascii_string).parse(input)?;
    let (input, bbb) = map_res(take(3usize), ascii_string).parse(input)?;

    // line breaks
    let (input, _) = tag([0x0D, 0x0D, 0x0A].as_slice())(input)?;

    Ok((
        input,
        TextHeader {
            xx,
            location,
            timestamp,
            aaa,
            bbb,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_HEADER: &[u8] = b"SDUS73 KMKX 091253\r\r\nN0ZMKX\r\r\n";

    #[test]
    fn parses_a_well_formed_header() {
        let (rest, header) = text_header(SAMPLE_HEADER).unwrap();

        assert!(rest.is_empty());
        assert_eq!(
            header,
            TextHeader {
                xx: 73,
                location: "KMKX".to_string(),
                timestamp: "091253".to_string(),
                aaa: "N0Z".to_string(),
                bbb: "MKX".to_string(),
            }
        );
    }

    #[test]
    fn rejects_input_missing_the_sdus_tag() {
        assert!(text_header(b"XDUS73 KMKX 091253\r\r\nN0ZMKX\r\r\n").is_err());
    }

    #[test]
    fn rejects_missing_product_code_instead_of_panicking() {
        // No digits follow "SDUS": previously this hit `.unwrap()` on an
        // empty-string parse and crashed the whole process.
        assert!(text_header(b"SDUS KMKX 091253\r\r\nN0ZMKX\r\r\n").is_err());
    }

    #[test]
    fn rejects_truncated_input_instead_of_panicking() {
        assert!(text_header(b"SDUS73 KMKX 09").is_err());
    }

    #[test]
    fn rejects_non_ascii_bytes_instead_of_panicking() {
        let mut input = SAMPLE_HEADER.to_vec();
        input[7] = 0xFF; // corrupt a byte inside the "KMKX" location field
        assert!(text_header(&input).is_err());
    }
}
