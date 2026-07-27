//! Integration test: parses the real NEXRAD Level 3 fixture in `data/` end
//! to end through the public `Radar::from_vec` API, the same way the
//! `parse` example and the CLI binary do.

use radar::{MessageCode, Radar, SymPacketData};

fn sample_bytes() -> Vec<u8> {
    include_bytes!("../data/sn_DS.p20-r_kmkx.last").to_vec()
}

#[test]
fn parses_the_bundled_fixture_end_to_end() {
    let (leftover, radar) = Radar::from_vec(sample_bytes()).expect("fixture should parse");

    // No bytes should be left unparsed: the file is exactly one product
    // description followed by one fully-consumed symbology block.
    assert!(leftover.is_empty(), "expected no leftover bytes, found {}", leftover.len());

    assert_eq!(radar.text_header.location, "KMKX");
    assert_eq!(radar.text_header.aaa, "N0Z");
    assert_eq!(radar.text_header.bbb, "MKX");

    assert_eq!(radar.message_header.code, MessageCode::BaseReflectivity20);
    assert_eq!(radar.product_description.product_code, 20);
    assert_eq!(radar.product_description.offset_symbology, 60);
    assert_eq!(radar.product_description.offset_graphic, 0);
    assert_eq!(radar.product_description.offset_tabular, 0);
    assert!(radar.graphic.is_none());
    assert!(radar.tabular.is_none());

    let symbology = radar.symbology.expect("product declares a symbology block");
    assert_eq!(symbology.header.layers, 1);
    assert_eq!(symbology.layers.len(), 1);

    match &symbology.layers[0] {
        SymPacketData::RadialDataAF1F(packet) => {
            assert_eq!(packet.header.num_radials, 360);
            assert_eq!(packet.header.num_bins, 230);
            assert_eq!(packet.radials.len(), 360);
        }
        other => panic!("expected a RadialDataAF1F layer, got {other:?}"),
    }
}

#[test]
fn plot_succeeds_on_the_bundled_fixture() {
    // Plot to a temp path rather than via `plot()`, which always writes to
    // "image.png" in the current directory — a path already committed to
    // this repo.
    let dir = tempfile::tempdir().expect("temp dir");
    let (_, radar) = Radar::from_vec(sample_bytes()).expect("fixture should parse");
    radar
        .plot_to(dir.path().join("plot.png"))
        .expect("plotting a real, fully-parsed product should not fail");
}

#[test]
fn rejects_a_file_that_is_too_short_to_be_nexrad_level_3() {
    let err = Radar::from_vec(vec![0u8; 10]).unwrap_err();
    assert!(matches!(err, radar::Error::TooShort { expected: 150, actual: 10 }));
}

#[test]
fn rejects_a_file_that_is_not_nexrad_level_3_at_all() {
    let not_nexrad = vec![0u8; 200];
    assert!(Radar::from_vec(not_nexrad).is_err());
}

#[test]
fn rejects_an_empty_file_instead_of_panicking() {
    assert!(Radar::from_vec(Vec::new()).is_err());
}
