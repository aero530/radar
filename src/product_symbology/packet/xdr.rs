//! A minimal reader for External Data Representation (XDR) encoded data,
//! per RFC 1832 / RFC 4506.
//!
//! The Generic Data Packet (codes 28 and 29, Figure 3-15c) carries its
//! payload as XDR, so only the subset the Generic Product Format of Appendix
//! E actually uses is implemented here: signed and unsigned 32-bit integers,
//! 32-bit floats, counted strings, and counted arrays.
//!
//! Every XDR primitive occupies a whole number of 4-byte units, big-endian,
//! and variable-length data is zero-padded up to the next 4-byte boundary.

/// Something went wrong reading the XDR stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XdrError {
    /// The stream ended before `wanted` more bytes could be read.
    UnexpectedEnd { offset: usize, wanted: usize },
    /// A counted string or array declared a length that cannot be honoured.
    InvalidLength { offset: usize, len: i32 },
    /// A string's bytes were not valid UTF-8 / ASCII.
    InvalidString { offset: usize },
}

impl std::fmt::Display for XdrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XdrError::UnexpectedEnd { offset, wanted } => write!(
                f,
                "XDR stream ended at offset {offset} while trying to read {wanted} byte(s)"
            ),
            XdrError::InvalidLength { offset, len } => {
                write!(f, "XDR declared an invalid length of {len} at offset {offset}")
            }
            XdrError::InvalidString { offset } => {
                write!(f, "XDR string at offset {offset} is not valid text")
            }
        }
    }
}

impl std::error::Error for XdrError {}

/// A cursor over an XDR-encoded byte stream.
pub struct XdrReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> XdrReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        XdrReader { data, pos: 0 }
    }

    /// Current byte offset into the stream.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Number of bytes not yet consumed.
    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    /// Whether the whole stream has been consumed.
    pub fn is_done(&self) -> bool {
        self.remaining() == 0
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], XdrError> {
        if self.remaining() < n {
            return Err(XdrError::UnexpectedEnd {
                offset: self.pos,
                wanted: n,
            });
        }
        let slice = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(slice)
    }

    /// Reads a 4-byte signed integer.
    pub fn int(&mut self) -> Result<i32, XdrError> {
        let b = self.take(4)?;
        Ok(i32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// Reads a 4-byte unsigned integer.
    pub fn uint(&mut self) -> Result<u32, XdrError> {
        let b = self.take(4)?;
        Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// Reads a 4-byte IEEE-754 float.
    pub fn float(&mut self) -> Result<f32, XdrError> {
        let b = self.take(4)?;
        Ok(f32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// Reads a counted string: a 4-byte length, that many bytes, then
    /// however much zero padding is needed to reach a 4-byte boundary.
    pub fn string(&mut self) -> Result<String, XdrError> {
        let offset = self.pos;
        let len = self.int()?;
        let len = usize::try_from(len).map_err(|_| XdrError::InvalidLength { offset, len })?;
        let bytes = self.take(len)?;
        // Skip the padding that rounds the payload up to a 4-byte boundary.
        let padding = (4 - (len % 4)) % 4;
        self.take(padding)?;
        // Trailing NULs are part of the on-wire representation, not the value.
        let trimmed = bytes.split(|b| *b == 0).next().unwrap_or(bytes);
        std::str::from_utf8(trimmed)
            .map(|s| s.to_owned())
            .map_err(|_| XdrError::InvalidString { offset })
    }

    /// Reads a counted array of 4-byte signed integers.
    pub fn int_array(&mut self) -> Result<Vec<i32>, XdrError> {
        let len = self.array_len(4)?;
        (0..len).map(|_| self.int()).collect()
    }

    /// Reads a counted array of 4-byte floats.
    pub fn float_array(&mut self) -> Result<Vec<f32>, XdrError> {
        let len = self.array_len(4)?;
        (0..len).map(|_| self.float()).collect()
    }

    /// Reads a counted array of strings.
    pub fn string_array(&mut self) -> Result<Vec<String>, XdrError> {
        // A string is at least its 4-byte length, so that is the per-element
        // minimum the count can be checked against.
        let len = self.array_len(4)?;
        (0..len).map(|_| self.string()).collect()
    }

    /// Reads `n` signed integers without a leading count.
    pub fn ints(&mut self, n: usize) -> Result<Vec<i32>, XdrError> {
        if self.remaining() < n * 4 {
            return Err(XdrError::UnexpectedEnd {
                offset: self.pos,
                wanted: n * 4,
            });
        }
        (0..n).map(|_| self.int()).collect()
    }

    /// Reads an array's leading count, rejecting one that could not possibly be
    /// satisfied by the bytes remaining.
    ///
    /// Bounds-checking up front stops a corrupt count from driving a huge
    /// allocation before the read fails.
    fn array_len(&mut self, min_element_bytes: usize) -> Result<usize, XdrError> {
        let offset = self.pos;
        let len = self.int()?;
        let len = usize::try_from(len).map_err(|_| XdrError::InvalidLength { offset, len })?;
        if self.remaining() < len.saturating_mul(min_element_bytes) {
            return Err(XdrError::UnexpectedEnd {
                offset: self.pos,
                wanted: len * min_element_bytes,
            });
        }
        Ok(len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_integers_and_floats() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(-7i32).to_be_bytes());
        bytes.extend_from_slice(&4000000000u32.to_be_bytes());
        bytes.extend_from_slice(&1.5f32.to_be_bytes());

        let mut r = XdrReader::new(&bytes);
        assert_eq!(r.int().unwrap(), -7);
        assert_eq!(r.uint().unwrap(), 4000000000);
        assert_eq!(r.float().unwrap(), 1.5);
        assert!(r.is_done());
    }

    #[test]
    fn reads_a_string_and_skips_its_padding() {
        // "abc" is 3 bytes, so one byte of padding follows.
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&3i32.to_be_bytes());
        bytes.extend_from_slice(b"abc");
        bytes.push(0); // padding
        bytes.extend_from_slice(&42i32.to_be_bytes());

        let mut r = XdrReader::new(&bytes);
        assert_eq!(r.string().unwrap(), "abc");
        // The padding must have been consumed for this to line up.
        assert_eq!(r.int().unwrap(), 42);
        assert!(r.is_done());
    }

    #[test]
    fn reads_a_string_needing_no_padding() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&4i32.to_be_bytes());
        bytes.extend_from_slice(b"abcd");
        bytes.extend_from_slice(&7i32.to_be_bytes());

        let mut r = XdrReader::new(&bytes);
        assert_eq!(r.string().unwrap(), "abcd");
        assert_eq!(r.int().unwrap(), 7);
    }

    #[test]
    fn trims_trailing_nuls_from_strings() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&4i32.to_be_bytes());
        bytes.extend_from_slice(b"ab\0\0");

        let mut r = XdrReader::new(&bytes);
        assert_eq!(r.string().unwrap(), "ab");
    }

    #[test]
    fn reads_a_counted_int_array() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&3i32.to_be_bytes());
        for v in [10i32, 20, 30] {
            bytes.extend_from_slice(&v.to_be_bytes());
        }

        let mut r = XdrReader::new(&bytes);
        assert_eq!(r.int_array().unwrap(), vec![10, 20, 30]);
    }

    #[test]
    fn reports_the_end_of_the_stream_rather_than_panicking() {
        let bytes = [0u8, 0, 0];
        let mut r = XdrReader::new(&bytes);
        assert!(matches!(r.int(), Err(XdrError::UnexpectedEnd { .. })));
    }

    #[test]
    fn rejects_a_negative_string_length() {
        let bytes = (-1i32).to_be_bytes();
        let mut r = XdrReader::new(&bytes);
        assert!(matches!(r.string(), Err(XdrError::InvalidLength { .. })));
    }

    /// A huge declared array count must be rejected against the actual
    /// remaining length rather than attempting the allocation.
    #[test]
    fn rejects_an_array_count_larger_than_the_stream() {
        let bytes = 1_000_000i32.to_be_bytes();
        let mut r = XdrReader::new(&bytes);
        assert!(matches!(
            r.int_array(),
            Err(XdrError::UnexpectedEnd { .. })
        ));
    }
}
