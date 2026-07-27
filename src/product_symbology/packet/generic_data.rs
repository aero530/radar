//! Generic Data Packet — packet codes 28 and 29 (Figure 3-15c, page 3-110).
//!
//! The packet header is a small binary preamble; the payload is XDR-encoded
//! (see [`super::xdr`]) and deserializes to the Generic Product Format of
//! Appendix E.
//!
//! Structures implemented here:
//!
//! - Figure E-1, Product Description (the first item for packet 28)
//! - Figure E-1b, External Data Description (the first item for packet 29)
//! - Figure E-2, Parameter list
//! - Figure E-3 / E-4, Radial Component and its radials
//! - Figure E-5, Grid Component
//! - Figure E-6 / E-7a / E-7b / E-7c, Area Component and its point kinds
//! - Figure E-8, Text Component
//! - Figure E-9, Table Component
//! - Figure E-10, Event Component, which nests further components
//! - Figure E-11, Binary Data (an attributes string plus a counted array)
//! - Figure E-12, String Data
//!
//! Where the ICD's stated type disagrees with what real products contain, the
//! discrepancy is noted at the field. XDR has no 2-byte type — every integer
//! occupies 4 bytes — so fields the ICD lists as `INT*2` are read as 4-byte
//! integers.
//!
//! # Validation
//!
//! The Product Description and Radial Component readers agree with the
//! vendored Py-ART reference, which is validated against real product 176
//! files. The remaining structures — Grid, Area, Table, Event and the External
//! Data Description — are derived from the Appendix E figures alone: no file
//! containing them was available, and Py-ART does not implement them either.
//! Two things guard against a misreading going unnoticed: decoding stops at any
//! component type Appendix E does not define, and
//! [`warn_if_payload_remains`] logs when a payload was not fully consumed,
//! which is what a wrong field layout looks like. The raw payload is retained
//! on the packet either way.

use serde::{Deserialize, Serialize};
use nom::{
    number::{complete::i16 as nom_i16, Endianness::Big},
    IResult,
};
use tracing::{debug, warn};

use super::util::{fail, payload};
use super::xdr::{XdrError, XdrReader};
use crate::product_symbology::SymPacketData;

/// The binary header of a Generic Data Packet (Figure 3-15c).
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct GenericDataHeader {
    /// Packet code, 28 or 29.
    pub packet_code: i16,
    /// Reserved for future use; should be 0.
    pub reserved: i16,
    /// Number of serialized (XDR) bytes that follow, assembled from the
    /// most- and least-significant halfwords.
    pub length: i32,
}

/// A name/value product or component parameter (Figure E-2).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub value: String,
}

/// One radial of a Radial Component (Figures E-4 and E-11).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GenericRadial {
    /// Azimuth of the leading edge of the radial, degrees.
    pub azimuth: f32,
    /// Elevation angle of the radial, degrees.
    pub elevation: f32,
    /// Radial width or separation, degrees.
    pub width: f32,
    /// Number of data values along the radial.
    ///
    /// Figure E-4 lists this as `REAL*4`, but real products encode an integer
    /// here; both occupy 4 bytes so only the interpretation differs.
    pub num_bins: i32,
    /// The `Attributes` string of the Binary Data structure (Figure E-11),
    /// which describes how to interpret `data`.
    pub attributes: String,
    /// The radial's bin values.
    pub data: Vec<i32>,
}

/// A Radial Component (Figure E-3).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RadialComponent {
    pub description: String,
    /// Range extent of each bin, meters.
    pub bin_size: f32,
    /// Range to the center of the first bin, meters.
    pub first_bin_range: f32,
    pub parameters: Vec<Parameter>,
    pub radials: Vec<GenericRadial>,
}

/// A Text Component (Figure E-8).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct TextComponent {
    pub parameters: Vec<Parameter>,
    pub text: String,
}

/// A data point of an Area Component, in whichever coordinate system the
/// component's area type selects (Figures E-7a, E-7b and E-7c).
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum GenericPoint {
    /// Figure E-7a: degrees.
    LatLon { latitude: f32, longitude: f32 },
    /// Figure E-7b: kilometres, unless the component parameters say otherwise.
    XY { x: f32, y: f32 },
    /// Figure E-7c: azimuth in degrees, range in kilometres.
    AzRan { azimuth: f32, range: f32 },
}

/// How an Area Component's points are laid out, from the low half of its area
/// type (Figure E-6).
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AreaShape {
    Point,
    Area,
    Polyline,
    /// A shape code outside the documented 1 to 3.
    Unknown(i32),
}

/// A Grid Component (Figure E-5).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GridComponent {
    /// Grid dimension sizes, ordered from fastest changing to slowest.
    pub dimensions: Vec<i32>,
    /// 1 = Array, 2 = Equally spaced, 3 = Lat/Lon, 4 = Polar.
    pub grid_type: i32,
    pub parameters: Vec<Parameter>,
    /// The `Attributes` string of the grid's Binary Data structure (E-11),
    /// which describes how to interpret `data`.
    pub attributes: String,
    /// The gridded values as a flat array, first dimension varying fastest
    /// (Note 1 of Figure E-11).
    pub data: Vec<i32>,
}

/// An Area Component (Figure E-6).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AreaComponent {
    pub parameters: Vec<Parameter>,
    /// The raw area type, whose low half gives the shape and high half the
    /// coordinate system.
    pub area_type: i32,
    pub shape: AreaShape,
    pub points: Vec<GenericPoint>,
}

/// A Table Component (Figure E-9).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableComponent {
    pub parameters: Vec<Parameter>,
    pub title: String,
    pub columns: i32,
    pub rows: i32,
    pub column_labels: Vec<String>,
    pub row_labels: Vec<String>,
    /// `rows * columns` entries, row index varying fastest (Note 1 of E-11).
    pub entries: Vec<String>,
}

/// An Event Component (Figure E-10), which nests further components.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EventComponent {
    pub parameters: Vec<Parameter>,
    pub components: Vec<GenericComponent>,
}

/// One component of a generic product.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GenericComponent {
    /// Component type 1 (Figure E-3).
    Radial(RadialComponent),
    /// Component type 2 (Figure E-5).
    Grid(GridComponent),
    /// Component type 3 (Figure E-6).
    Area(AreaComponent),
    /// Component type 4 (Figure E-8).
    Text(TextComponent),
    /// Component type 5 (Figure E-9).
    Table(TableComponent),
    /// Component type 6 (Figure E-10).
    Event(EventComponent),
    /// A component type that is not one of the six Appendix E defines. Decoding
    /// stops here, because without knowing the structure there is no way to
    /// tell how many bytes to skip.
    Unsupported { component_type: i32 },
}

/// The Product Description data structure (Figure E-1), which is the first
/// item in a packet-28 payload.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GenericProductDescription {
    pub name: String,
    pub description: String,
    /// Product code, per Table II.
    pub code: i32,
    /// 1 = Volume, 2 = Elevation, 3 = Time, 4 = On Demand, 5 = On Request,
    /// 6 = Radial, 7 = External.
    pub product_type: i32,
    /// Product generation time, seconds since the Unix epoch.
    pub generation_time: u32,
    pub radar_name: String,
    pub radar_latitude: f32,
    pub radar_longitude: f32,
    /// Radar height in meters above mean sea level.
    pub radar_height: f32,
    /// Volume scan start time, seconds since the Unix epoch.
    pub volume_scan_start_time: u32,
    /// Elevation scan start time, seconds since the Unix epoch; only
    /// meaningful when `product_type == 2`.
    pub elevation_scan_start_time: u32,
    pub elevation_angle: f32,
    pub volume_scan_number: i32,
    /// 1 = Test, 2 = Clear Air, 3 = Precipitation.
    ///
    /// Listed as `INT*2` in Figure E-1; read as a 4-byte XDR integer.
    pub operational_mode: i32,
    /// Volume coverage pattern number. Listed as `INT*2`; read as 4 bytes.
    pub vcp_number: i32,
    /// Elevation number within the VCP. Listed as `INT*2`; read as 4 bytes.
    pub elevation_number: i32,
    /// First spare halfword, reserved for a future compression type.
    pub compression: i32,
    /// Second spare, reserved for a future decompressed size.
    pub uncompressed_size: i32,
    pub parameters: Vec<Parameter>,
    pub components: Vec<GenericComponent>,
}

/// The External Data Description data structure (Figure E-1b), which is the
/// first item in a packet-29 payload.
///
/// It mirrors [`GenericProductDescription`] but replaces the radar-specific
/// fields with spares, since an external product is not tied to a site.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExternalDataDescription {
    pub name: String,
    pub description: String,
    /// Product code, per Table II.
    pub code: i32,
    /// Always 7 (External) for this structure.
    pub product_type: i32,
    /// Product generation time, seconds since the Unix epoch.
    pub generation_time: u32,
    /// The five spare words between the generation time and the parameter
    /// count. The fourth is reserved for a future compression type and the
    /// fifth for a decompressed size.
    pub spares: [i32; 5],
    pub parameters: Vec<Parameter>,
    pub components: Vec<GenericComponent>,
}

/// A fully decoded Generic Data Packet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GenericDataPacket {
    pub header: GenericDataHeader,
    /// The decoded Product Description, present for packet code 28.
    pub product_description: Option<GenericProductDescription>,
    /// The decoded External Data Description, present for packet code 29.
    pub external_description: Option<ExternalDataDescription>,
    /// The raw XDR payload, kept so that callers can re-read anything this
    /// crate decoded differently than they need.
    pub raw: Vec<u8>,
}

/// Generic Data Packet, packet codes 28 and 29 (Figure 3-15c).
///
/// Packet 28's payload leads with a Product Description (Figure E-1) and packet
/// 29's with an External Data Description (Figure E-1b); both are decoded. If
/// that decoding fails the packet still parses — the raw payload is retained
/// and a warning logged — because the framing (and therefore the position of
/// everything after this packet) is well defined by the header's length field
/// regardless.
pub fn generic_data(input: &[u8]) -> IResult<&[u8], SymPacketData> {
    let (input, packet_code) = nom_i16(Big)(input)?;
    let (input, reserved) = nom_i16(Big)(input)?;
    // The byte count is split across two halfwords, most significant first.
    let (input, length_mshw) = nom_i16(Big)(input)?;
    let (input, length_lshw) = nom_i16(Big)(input)?;
    let length = ((length_mshw as i32) << 16) | (length_lshw as i32 & 0xFFFF);

    let len = match usize::try_from(length) {
        Ok(len) => len,
        Err(_) => {
            return fail(
                input,
                &format!("Generic data packet declares a negative length ({length})"),
            )
        }
    };
    let (input, body) = payload(input, len)?;

    let header = GenericDataHeader {
        packet_code,
        reserved,
        length,
    };
    debug!("{:?}", header);

    let mut product_description = None;
    let mut external_description = None;
    match packet_code {
        28 => match decode_product_description(body) {
            Ok(pd) => product_description = Some(pd),
            Err(e) => warn!("Could not decode generic product description: {e}"),
        },
        29 => match decode_external_data_description(body) {
            Ok(ed) => external_description = Some(ed),
            Err(e) => warn!("Could not decode external data description: {e}"),
        },
        other => warn!("Generic data packet has unexpected code {other}; payload left undecoded"),
    }

    Ok((
        input,
        SymPacketData::GenericData(Box::new(GenericDataPacket {
            header,
            product_description,
            external_description,
            raw: body.to_vec(),
        })),
    ))
}

/// Deserializes the XDR payload of a packet-29 Generic Data Packet into the
/// External Data Description structure of Figure E-1b.
pub fn decode_external_data_description(
    body: &[u8],
) -> Result<ExternalDataDescription, XdrError> {
    let mut r = XdrReader::new(body);

    let name = r.string()?;
    let description = r.string()?;
    let code = r.int()?;
    let product_type = r.int()?;
    let generation_time = r.uint()?;
    // Five spare words sit between the generation time and the parameter count.
    // Figure E-1b draws them as two INT*4s, two INT*2s and an INT*4 — eight
    // halfword rows — and XDR widens each to a 4-byte word.
    let spares = r.ints(5)?;
    let parameters = read_parameters(&mut r)?;
    let components = read_components(&mut r)?;
    warn_if_payload_remains(&r, "external data description");

    Ok(ExternalDataDescription {
        name,
        description,
        code,
        product_type,
        generation_time,
        spares: [spares[0], spares[1], spares[2], spares[3], spares[4]],
        parameters,
        components,
    })
}

/// Warns when a payload was not fully consumed.
///
/// Appendix E structures should account for every serialized byte, so leftovers
/// mean this crate's reading of some field's layout is off. Surfacing that is
/// more useful than silently returning a half-decoded structure — the raw
/// payload is retained either way.
fn warn_if_payload_remains(r: &XdrReader<'_>, what: &str) {
    if !r.is_done() {
        warn!(
            "{} left {} of {} XDR byte(s) unread; some field layout may be wrong",
            what,
            r.remaining(),
            r.position() + r.remaining()
        );
    }
}

/// Deserializes the XDR payload of a packet-28 Generic Data Packet into the
/// Product Description structure of Figure E-1.
pub fn decode_product_description(body: &[u8]) -> Result<GenericProductDescription, XdrError> {
    let mut r = XdrReader::new(body);

    let name = r.string()?;
    let description = r.string()?;
    let code = r.int()?;
    let product_type = r.int()?;
    let generation_time = r.uint()?;
    let radar_name = r.string()?;
    let radar_latitude = r.float()?;
    let radar_longitude = r.float()?;
    let radar_height = r.float()?;
    let volume_scan_start_time = r.uint()?;
    let elevation_scan_start_time = r.uint()?;
    let elevation_angle = r.float()?;
    let volume_scan_number = r.int()?;
    let operational_mode = r.int()?;
    let vcp_number = r.int()?;
    let elevation_number = r.int()?;
    let compression = r.int()?;
    let uncompressed_size = r.int()?;
    let parameters = read_parameters(&mut r)?;
    let components = read_components(&mut r)?;
    warn_if_payload_remains(&r, "generic product description");

    Ok(GenericProductDescription {
        name,
        description,
        code,
        product_type,
        generation_time,
        radar_name,
        radar_latitude,
        radar_longitude,
        radar_height,
        volume_scan_start_time,
        elevation_scan_start_time,
        elevation_angle,
        volume_scan_number,
        operational_mode,
        vcp_number,
        elevation_number,
        compression,
        uncompressed_size,
        parameters,
        components,
    })
}

/// Reads a parameter list (Figure E-2).
///
/// Appendix E models the list as a count followed by a pointer to the
/// structure. Pointers are not meaningful once serialized, so the serializer
/// emits a placeholder integer after the count, and another between
/// successive entries; both are read and discarded.
fn read_parameters(r: &mut XdrReader<'_>) -> Result<Vec<Parameter>, XdrError> {
    let num = r.int()?;
    let _pointer = r.int()?;
    if num <= 0 {
        return Ok(Vec::new());
    }

    let mut out = Vec::new();
    for i in 0..num {
        let name = r.string()?;
        let value = r.string()?;
        out.push(Parameter { name, value });
        if i < num - 1 {
            let _pointer = r.int()?;
        }
    }
    Ok(out)
}

/// How deeply Event Components may nest before decoding gives up.
///
/// Appendix E puts no limit on nesting, but a corrupt stream could otherwise
/// drive unbounded recursion.
const MAX_COMPONENT_DEPTH: u8 = 8;

/// Reads a component list (Figure E-1 Note 3), dispatching on component type.
fn read_components(r: &mut XdrReader<'_>) -> Result<Vec<GenericComponent>, XdrError> {
    read_components_at_depth(r, 0)
}

fn read_components_at_depth(
    r: &mut XdrReader<'_>,
    depth: u8,
) -> Result<Vec<GenericComponent>, XdrError> {
    let num = r.int()?;
    let _pointer = r.int()?;
    if num <= 0 {
        return Ok(Vec::new());
    }
    if depth >= MAX_COMPONENT_DEPTH {
        warn!("Generic product components nested more than {MAX_COMPONENT_DEPTH} deep; stopping");
        return Ok(Vec::new());
    }

    let mut out = Vec::new();
    for i in 0..num {
        let component_type = r.int()?;
        let component = match component_type {
            1 => GenericComponent::Radial(read_radial_component(r)?),
            2 => GenericComponent::Grid(read_grid_component(r)?),
            3 => GenericComponent::Area(read_area_component(r)?),
            4 => GenericComponent::Text(TextComponent {
                parameters: read_parameters(r)?,
                text: r.string()?,
            }),
            5 => GenericComponent::Table(read_table_component(r)?),
            6 => GenericComponent::Event(EventComponent {
                parameters: read_parameters(r)?,
                components: read_components_at_depth(r, depth + 1)?,
            }),
            other => {
                // Without knowing the structure's length there is no way to
                // skip it, so stop here and keep what was decoded.
                warn!("Unknown generic product component type {other}; stopping");
                out.push(GenericComponent::Unsupported {
                    component_type: other,
                });
                break;
            }
        };
        out.push(component);
        if i < num - 1 {
            let _pointer = r.int()?;
        }
    }
    Ok(out)
}

/// Reads a Grid Component (Figure E-5).
fn read_grid_component(r: &mut XdrReader<'_>) -> Result<GridComponent, XdrError> {
    let num_dimensions = r.int()?;
    // "Dimensions: Pointer to INT*4" follows an explicit count, so it uses the
    // same count/pointer-placeholder/values shape as the parameter list.
    let _pointer = r.int()?;
    let dimensions = r.ints(num_dimensions.max(0) as usize)?;
    let grid_type = r.int()?;
    let parameters = read_parameters(r)?;
    // Grid Data is a Binary Data structure (Figure E-11).
    let attributes = r.string()?;
    let data = r.int_array()?;

    Ok(GridComponent {
        dimensions,
        grid_type,
        parameters,
        attributes,
        data,
    })
}

/// Reads an Area Component (Figure E-6).
fn read_area_component(r: &mut XdrReader<'_>) -> Result<AreaComponent, XdrError> {
    let parameters = read_parameters(r)?;
    let area_type = r.int()?;
    let num_points = r.int()?;
    let _pointer = r.int()?;

    // The low half of the area type gives the shape, the high half the
    // coordinate system: 0x0.. Lat/Lon, 0x1.. X/Y, 0x2.. Az/Ran.
    let shape = match area_type & 0xFFFF {
        1 => AreaShape::Point,
        2 => AreaShape::Area,
        3 => AreaShape::Polyline,
        other => AreaShape::Unknown(other),
    };
    let system = (area_type >> 16) & 0xFFFF;

    let mut points = Vec::new();
    for _ in 0..num_points.max(0) {
        let a = r.float()?;
        let b = r.float()?;
        points.push(match system {
            0 => GenericPoint::LatLon {
                latitude: a,
                longitude: b,
            },
            1 => GenericPoint::XY { x: a, y: b },
            2 => GenericPoint::AzRan {
                azimuth: a,
                range: b,
            },
            other => {
                warn!("Unknown area coordinate system {other}; reading points as lat/lon");
                GenericPoint::LatLon {
                    latitude: a,
                    longitude: b,
                }
            }
        });
    }

    Ok(AreaComponent {
        parameters,
        area_type,
        shape,
        points,
    })
}

/// Reads a Table Component (Figure E-9).
fn read_table_component(r: &mut XdrReader<'_>) -> Result<TableComponent, XdrError> {
    let parameters = read_parameters(r)?;
    let title = r.string()?;
    // Listed as INT*2 in Figure E-9, but XDR has no 2-byte integer, so both
    // occupy a 4-byte word — the same discrepancy Figure E-1 has.
    let columns = r.int()?;
    let rows = r.int()?;
    let column_labels = r.string_array()?;
    let row_labels = r.string_array()?;
    let entries = r.string_array()?;

    Ok(TableComponent {
        parameters,
        title,
        columns,
        rows,
        column_labels,
        row_labels,
        entries,
    })
}

/// Reads a Radial Component (Figure E-3) and its radials (E-4 / E-11).
fn read_radial_component(r: &mut XdrReader<'_>) -> Result<RadialComponent, XdrError> {
    let description = r.string()?;
    let bin_size = r.float()?;
    let first_bin_range = r.float()?;
    let parameters = read_parameters(r)?;

    let num_radials = r.int()?;
    let mut radials = Vec::new();
    for _ in 0..num_radials.max(0) {
        radials.push(GenericRadial {
            azimuth: r.float()?,
            elevation: r.float()?,
            width: r.float()?,
            num_bins: r.int()?,
            attributes: r.string()?,
            data: r.int_array()?,
        });
    }

    Ok(RadialComponent {
        description,
        bin_size,
        first_bin_range,
        parameters,
        radials,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Serializes a string the way XDR does: length, bytes, zero padding.
    fn xdr_string(s: &str) -> Vec<u8> {
        let mut out = (s.len() as i32).to_be_bytes().to_vec();
        out.extend_from_slice(s.as_bytes());
        out.extend(std::iter::repeat_n(0u8, (4 - (s.len() % 4)) % 4));
        out
    }

    fn xdr_int(v: i32) -> Vec<u8> {
        v.to_be_bytes().to_vec()
    }

    fn xdr_float(v: f32) -> Vec<u8> {
        v.to_be_bytes().to_vec()
    }

    /// Builds an XDR product description with one radial component holding a
    /// single radial, mirroring the shape of a real product 176.
    fn sample_payload() -> Vec<u8> {
        let mut p = Vec::new();
        p.extend(xdr_string("Digital Inst Precip Rate"));
        p.extend(xdr_string("Build 24.0"));
        p.extend(xdr_int(176)); // code
        p.extend(xdr_int(2)); // type = Elevation
        p.extend(xdr_int(1_662_812_114)); // generation time
        p.extend(xdr_string("KMKX"));
        p.extend(xdr_float(42.968)); // latitude
        p.extend(xdr_float(-88.551)); // longitude
        p.extend(xdr_float(311.0)); // height (meters)
        p.extend(xdr_int(1_662_812_000)); // volume scan start
        p.extend(xdr_int(1_662_812_050)); // elevation scan start
        p.extend(xdr_float(0.5)); // elevation angle
        p.extend(xdr_int(38)); // volume scan number
        p.extend(xdr_int(3)); // operational mode = Precipitation
        p.extend(xdr_int(35)); // vcp
        p.extend(xdr_int(1)); // elevation number
        p.extend(xdr_int(0)); // compression
        p.extend(xdr_int(0)); // uncompressed size

        // Two product parameters.
        p.extend(xdr_int(2)); // number of parameters
        p.extend(xdr_int(0)); // pointer placeholder
        p.extend(xdr_string("units"));
        p.extend(xdr_string("in/hr"));
        p.extend(xdr_int(0)); // inter-entry pointer
        p.extend(xdr_string("scale"));
        p.extend(xdr_string("1000"));

        // One radial component.
        p.extend(xdr_int(1)); // number of components
        p.extend(xdr_int(0)); // pointer placeholder
        p.extend(xdr_int(1)); // component type = radial
        p.extend(xdr_string("Precip Rate"));
        p.extend(xdr_float(250.0)); // bin size
        p.extend(xdr_float(125.0)); // range to first bin
        p.extend(xdr_int(0)); // component parameters: none
        p.extend(xdr_int(0)); // pointer placeholder
        p.extend(xdr_int(1)); // number of radials
        p.extend(xdr_float(45.5)); // azimuth
        p.extend(xdr_float(0.5)); // elevation
        p.extend(xdr_float(1.0)); // width
        p.extend(xdr_int(3)); // number of bins
        p.extend(xdr_string("type=int"));
        p.extend(xdr_int(3)); // bin value count
        p.extend(xdr_int(11));
        p.extend(xdr_int(22));
        p.extend(xdr_int(33));
        p
    }

    fn wrap_packet(code: i16, payload: &[u8]) -> Vec<u8> {
        let len = payload.len() as i32;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&code.to_be_bytes());
        bytes.extend_from_slice(&0i16.to_be_bytes()); // reserved
        bytes.extend_from_slice(&((len >> 16) as i16).to_be_bytes());
        bytes.extend_from_slice(&((len & 0xFFFF) as i16).to_be_bytes());
        bytes.extend_from_slice(payload);
        bytes
    }

    #[test]
    fn decodes_a_packet_28_product_description() {
        let bytes = wrap_packet(28, &sample_payload());
        let (rest, parsed) = generic_data(&bytes).unwrap();
        assert!(rest.is_empty());

        match parsed {
            SymPacketData::GenericData(p) => {
                assert_eq!(p.header.packet_code, 28);
                let pd = p.product_description.expect("packet 28 should decode");

                assert_eq!(pd.name, "Digital Inst Precip Rate");
                assert_eq!(pd.code, 176);
                assert_eq!(pd.product_type, 2);
                assert_eq!(pd.radar_name, "KMKX");
                assert!((pd.radar_latitude - 42.968).abs() < 1e-4);
                assert!((pd.radar_longitude + 88.551).abs() < 1e-4);
                assert_eq!(pd.vcp_number, 35);
                assert_eq!(pd.operational_mode, 3);

                assert_eq!(pd.parameters.len(), 2);
                assert_eq!(pd.parameters[0].name, "units");
                assert_eq!(pd.parameters[0].value, "in/hr");
                assert_eq!(pd.parameters[1].name, "scale");

                assert_eq!(pd.components.len(), 1);
                match &pd.components[0] {
                    GenericComponent::Radial(rc) => {
                        assert_eq!(rc.description, "Precip Rate");
                        assert_eq!(rc.bin_size, 250.0);
                        assert_eq!(rc.first_bin_range, 125.0);
                        assert_eq!(rc.radials.len(), 1);
                        assert_eq!(rc.radials[0].azimuth, 45.5);
                        assert_eq!(rc.radials[0].num_bins, 3);
                        assert_eq!(rc.radials[0].attributes, "type=int");
                        assert_eq!(rc.radials[0].data, vec![11, 22, 33]);
                    }
                    other => panic!("expected a radial component, got {other:?}"),
                }
            }
            other => panic!("expected GenericData, got {other:?}"),
        }
    }

    #[test]
    fn decodes_a_text_component() {
        let mut p = Vec::new();
        // Minimal product description up to the component list.
        p.extend(xdr_string("n"));
        p.extend(xdr_string("d"));
        for _ in 0..2 {
            p.extend(xdr_int(0));
        }
        p.extend(xdr_int(0)); // generation time
        p.extend(xdr_string("KABC"));
        for _ in 0..3 {
            p.extend(xdr_float(0.0));
        }
        for _ in 0..2 {
            p.extend(xdr_int(0));
        }
        p.extend(xdr_float(0.0)); // elevation angle
        for _ in 0..6 {
            p.extend(xdr_int(0));
        }
        p.extend(xdr_int(0)); // no parameters
        p.extend(xdr_int(0));
        p.extend(xdr_int(1)); // one component
        p.extend(xdr_int(0));
        p.extend(xdr_int(4)); // component type = text
        p.extend(xdr_int(0)); // no component parameters
        p.extend(xdr_int(0));
        p.extend(xdr_string("SEVERE WEATHER"));

        let bytes = wrap_packet(28, &p);
        let (_, parsed) = generic_data(&bytes).unwrap();
        match parsed {
            SymPacketData::GenericData(g) => {
                let pd = g.product_description.unwrap();
                match &pd.components[0] {
                    GenericComponent::Text(t) => assert_eq!(t.text, "SEVERE WEATHER"),
                    other => panic!("expected a text component, got {other:?}"),
                }
            }
            other => panic!("expected GenericData, got {other:?}"),
        }
    }

    /// A packet 29 payload too short to hold an External Data Description must
    /// still frame correctly, so whatever follows it in the layer is found.
    #[test]
    fn a_short_packet_29_payload_still_frames_the_packet() {
        let payload = vec![1u8, 2, 3, 4];
        let mut bytes = wrap_packet(29, &payload);
        bytes.extend_from_slice(&[0xAA, 0xBB]); // trailing bytes after the packet

        let (rest, parsed) = generic_data(&bytes).unwrap();
        assert_eq!(rest, &[0xAA, 0xBB]);
        match parsed {
            SymPacketData::GenericData(p) => {
                assert!(p.product_description.is_none());
                assert!(p.external_description.is_none());
                assert_eq!(p.raw, payload);
            }
            other => panic!("expected GenericData, got {other:?}"),
        }
    }

    /// Builds a packet 29 payload: an External Data Description (Figure E-1b)
    /// with one text component.
    fn external_payload() -> Vec<u8> {
        let mut p = Vec::new();
        p.extend(xdr_string("Model Soundings"));
        p.extend(xdr_string("external feed"));
        p.extend(xdr_int(202)); // code
        p.extend(xdr_int(7)); // type = External
        p.extend(xdr_int(1_662_812_114)); // generation time
        for _ in 0..5 {
            p.extend(xdr_int(0)); // five spare words
        }
        p.extend(xdr_int(0)); // no parameters
        p.extend(xdr_int(0)); // pointer placeholder
        p.extend(xdr_int(1)); // one component
        p.extend(xdr_int(0)); // pointer placeholder
        p.extend(xdr_int(4)); // component type = text
        p.extend(xdr_int(0)); // no component parameters
        p.extend(xdr_int(0)); // pointer placeholder
        p.extend(xdr_string("FREEZING LEVEL 8000 FT"));
        p
    }

    #[test]
    fn decodes_a_packet_29_external_data_description() {
        let bytes = wrap_packet(29, &external_payload());
        let (rest, parsed) = generic_data(&bytes).unwrap();
        assert!(rest.is_empty());

        match parsed {
            SymPacketData::GenericData(p) => {
                assert!(p.product_description.is_none(), "29 is not a 28");
                let ed = p.external_description.expect("packet 29 should decode");
                assert_eq!(ed.name, "Model Soundings");
                assert_eq!(ed.code, 202);
                assert_eq!(ed.product_type, 7);
                assert_eq!(ed.spares, [0; 5]);
                match &ed.components[0] {
                    GenericComponent::Text(t) => {
                        assert_eq!(t.text, "FREEZING LEVEL 8000 FT")
                    }
                    other => panic!("expected a text component, got {other:?}"),
                }
            }
            other => panic!("expected GenericData, got {other:?}"),
        }
    }

    /// Wraps `component` bytes in a minimal product description so a single
    /// component type can be exercised on its own.
    fn payload_with_component(component: &[u8]) -> Vec<u8> {
        let mut p = Vec::new();
        p.extend(xdr_string("n"));
        p.extend(xdr_string("d"));
        p.extend(xdr_int(0)); // code
        p.extend(xdr_int(0)); // type
        p.extend(xdr_int(0)); // generation time
        p.extend(xdr_string("KABC"));
        for _ in 0..3 {
            p.extend(xdr_float(0.0)); // lat, lon, height
        }
        p.extend(xdr_int(0)); // volume scan start
        p.extend(xdr_int(0)); // elevation scan start
        p.extend(xdr_float(0.0)); // elevation angle
        for _ in 0..6 {
            p.extend(xdr_int(0)); // vol num, mode, vcp, el num, 2 spares
        }
        p.extend(xdr_int(0)); // no product parameters
        p.extend(xdr_int(0)); // pointer placeholder
        p.extend(xdr_int(1)); // one component
        p.extend(xdr_int(0)); // pointer placeholder
        p.extend_from_slice(component);
        p
    }

    fn decode_single_component(component: &[u8]) -> GenericComponent {
        let pd = decode_product_description(&payload_with_component(component))
            .expect("payload should decode");
        assert_eq!(pd.components.len(), 1);
        pd.components.into_iter().next().unwrap()
    }

    #[test]
    fn decodes_a_grid_component() {
        let mut c = xdr_int(2); // component type = grid
        c.extend(xdr_int(2)); // two dimensions
        c.extend(xdr_int(0)); // pointer placeholder
        c.extend(xdr_int(3)); // fastest dimension
        c.extend(xdr_int(2)); // slowest dimension
        c.extend(xdr_int(2)); // grid type = equally spaced
        c.extend(xdr_int(0)); // no component parameters
        c.extend(xdr_int(0)); // pointer placeholder
        c.extend(xdr_string("type=int"));
        c.extend(xdr_int(6)); // six values
        for v in 1..=6 {
            c.extend(xdr_int(v));
        }

        match decode_single_component(&c) {
            GenericComponent::Grid(g) => {
                assert_eq!(g.dimensions, vec![3, 2]);
                assert_eq!(g.grid_type, 2);
                assert_eq!(g.attributes, "type=int");
                assert_eq!(g.data, vec![1, 2, 3, 4, 5, 6]);
                // The flat array holds one value per grid point.
                assert_eq!(g.data.len(), (g.dimensions[0] * g.dimensions[1]) as usize);
            }
            other => panic!("expected Grid, got {other:?}"),
        }
    }

    #[test]
    fn decodes_an_area_component_in_each_coordinate_system() {
        // Area type 0x00003 = Polyline in lat/lon.
        let mut c = xdr_int(3); // component type = area
        c.extend(xdr_int(0)); // no component parameters
        c.extend(xdr_int(0)); // pointer placeholder
        c.extend(xdr_int(0x00003));
        c.extend(xdr_int(2)); // two points
        c.extend(xdr_int(0)); // pointer placeholder
        c.extend(xdr_float(42.9));
        c.extend(xdr_float(-88.5));
        c.extend(xdr_float(43.1));
        c.extend(xdr_float(-88.1));

        match decode_single_component(&c) {
            GenericComponent::Area(a) => {
                assert_eq!(a.shape, AreaShape::Polyline);
                assert_eq!(a.points.len(), 2);
                match a.points[0] {
                    GenericPoint::LatLon { latitude, longitude } => {
                        assert!((latitude - 42.9).abs() < 1e-4);
                        assert!((longitude + 88.5).abs() < 1e-4);
                    }
                    other => panic!("expected LatLon, got {other:?}"),
                }
            }
            other => panic!("expected Area, got {other:?}"),
        }

        // Area type 0x20001 = Point in azimuth/range.
        let mut c = xdr_int(3);
        c.extend(xdr_int(0));
        c.extend(xdr_int(0));
        c.extend(xdr_int(0x20001));
        c.extend(xdr_int(1));
        c.extend(xdr_int(0));
        c.extend(xdr_float(270.0));
        c.extend(xdr_float(120.0));

        match decode_single_component(&c) {
            GenericComponent::Area(a) => {
                assert_eq!(a.shape, AreaShape::Point);
                assert_eq!(
                    a.points[0],
                    GenericPoint::AzRan {
                        azimuth: 270.0,
                        range: 120.0
                    }
                );
            }
            other => panic!("expected Area, got {other:?}"),
        }
    }

    #[test]
    fn decodes_a_table_component() {
        let mut c = xdr_int(5); // component type = table
        c.extend(xdr_int(0)); // no component parameters
        c.extend(xdr_int(0)); // pointer placeholder
        c.extend(xdr_string("STORM STRUCTURE"));
        c.extend(xdr_int(2)); // columns
        c.extend(xdr_int(1)); // rows
        c.extend(xdr_int(2)); // two column labels
        c.extend(xdr_string("CELL"));
        c.extend(xdr_string("TOP"));
        c.extend(xdr_int(1)); // one row label
        c.extend(xdr_string("1"));
        c.extend(xdr_int(2)); // rows * columns entries
        c.extend(xdr_string("A1"));
        c.extend(xdr_string("25.0"));

        match decode_single_component(&c) {
            GenericComponent::Table(t) => {
                assert_eq!(t.title, "STORM STRUCTURE");
                assert_eq!((t.columns, t.rows), (2, 1));
                assert_eq!(t.column_labels, vec!["CELL", "TOP"]);
                assert_eq!(t.row_labels, vec!["1"]);
                assert_eq!(t.entries, vec!["A1", "25.0"]);
                assert_eq!(t.entries.len(), (t.rows * t.columns) as usize);
            }
            other => panic!("expected Table, got {other:?}"),
        }
    }

    #[test]
    fn decodes_an_event_component_with_a_nested_component() {
        let mut c = xdr_int(6); // component type = event
        c.extend(xdr_int(1)); // one event parameter
        c.extend(xdr_int(0)); // pointer placeholder
        c.extend(xdr_string("intensity"));
        c.extend(xdr_string("severe"));
        c.extend(xdr_int(1)); // one nested component
        c.extend(xdr_int(0)); // pointer placeholder
        c.extend(xdr_int(4)); // nested type = text
        c.extend(xdr_int(0)); // no component parameters
        c.extend(xdr_int(0)); // pointer placeholder
        c.extend(xdr_string("TORNADO"));

        match decode_single_component(&c) {
            GenericComponent::Event(e) => {
                assert_eq!(e.parameters.len(), 1);
                assert_eq!(e.parameters[0].name, "intensity");
                assert_eq!(e.components.len(), 1);
                match &e.components[0] {
                    GenericComponent::Text(t) => assert_eq!(t.text, "TORNADO"),
                    other => panic!("expected nested Text, got {other:?}"),
                }
            }
            other => panic!("expected Event, got {other:?}"),
        }
    }

    /// A component type Appendix E does not define stops decoding, because the
    /// structure's length is unknown.
    #[test]
    fn an_unknown_component_type_stops_decoding() {
        let c = xdr_int(99);
        match decode_single_component(&c) {
            GenericComponent::Unsupported { component_type } => {
                assert_eq!(component_type, 99)
            }
            other => panic!("expected Unsupported, got {other:?}"),
        }
    }

    /// Events nesting deeper than the guard allows must terminate rather than
    /// recursing without bound.
    #[test]
    fn deeply_nested_events_terminate() {
        // Build 40 nested single-component events, well past MAX_COMPONENT_DEPTH.
        let mut c = Vec::new();
        for _ in 0..40 {
            c.extend(xdr_int(6)); // event
            c.extend(xdr_int(0)); // no parameters
            c.extend(xdr_int(0)); // pointer placeholder
            c.extend(xdr_int(1)); // one nested component
            c.extend(xdr_int(0)); // pointer placeholder
        }
        // The innermost level is left dangling; decoding stops at the depth
        // guard before it is reached.
        let pd = decode_product_description(&payload_with_component(&c))
            .expect("should stop rather than recurse away");

        // Walk down counting how deep the decode actually went.
        let mut depth = 0;
        let mut current = pd.components.first();
        while let Some(GenericComponent::Event(e)) = current {
            depth += 1;
            current = e.components.first();
        }
        assert!(depth <= MAX_COMPONENT_DEPTH as usize + 1, "depth was {depth}");
    }

    /// A payload that cannot be deserialized must not take the whole packet
    /// down: framing comes from the header length, not the XDR contents.
    #[test]
    fn malformed_xdr_still_frames_the_packet() {
        let payload = vec![0xFFu8; 8];
        let mut bytes = wrap_packet(28, &payload);
        bytes.push(0x77);

        let (rest, parsed) = generic_data(&bytes).unwrap();
        assert_eq!(rest, &[0x77]);
        match parsed {
            SymPacketData::GenericData(p) => {
                assert!(p.product_description.is_none());
                assert_eq!(p.raw.len(), 8);
            }
            other => panic!("expected GenericData, got {other:?}"),
        }
    }

    #[test]
    fn rejects_a_truncated_payload() {
        let mut bytes = wrap_packet(28, &[1, 2, 3, 4]);
        bytes.truncate(bytes.len() - 2); // fewer bytes than the header promises
        assert!(generic_data(&bytes).is_err());
    }

    #[test]
    fn assembles_a_length_spanning_both_halfwords() {
        // 0x00012000 = 73728 bytes, which needs both halfwords.
        let payload = vec![0u8; 0x12000];
        let bytes = wrap_packet(28, &payload);
        let (rest, parsed) = generic_data(&bytes).unwrap();
        assert!(rest.is_empty());
        match parsed {
            SymPacketData::GenericData(p) => assert_eq!(p.header.length, 0x12000),
            other => panic!("expected GenericData, got {other:?}"),
        }
    }
}
