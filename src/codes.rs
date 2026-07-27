use parse_display::{Display, FromStr};
use plotters::style::RGBColor;
use serde::{Deserialize, Serialize};

/// The message (product) code carried in every Graphic Product Message
/// header — identifies what kind of radar product a file contains (base
/// reflectivity, velocity, precipitation accumulation, and so on).
///
/// TABLE II NEXRAD MESSAGE CODE DEFINITIONS
/// TABLE III MESSAGE CODES FOR PRODUCTS
///
/// Every variant here corresponds to a real product code number from the
/// ICD, but only the ones listed in [`MessageCode::is_supported_product`]
/// can actually be parsed past the message header by this crate.
#[derive(Serialize, Deserialize, Display, FromStr, PartialEq, Debug, Copy, Clone, Default)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum MessageCode {
    #[default]
    #[display("Spare / Reserved")]
    Spare = 999,
    #[display("Product Request")]
    ProductRequest = 0,
    #[display("General Status")]
    GeneralStatus = 2,
    #[display("Request Response")]
    RequestResponse = 3,
    #[display("Maximum Connection Time Disable Request")]
    MaximumConnectionTimeDisableRequest = 4,
    #[display("External Data Message")]
    ExternalDataMessage = 5,
    #[display("Product List")]
    ProductList = 8,
    #[display("Sign-on Request Message (Dial -up Users)")]
    SignOnRequestMessage = 11,
    #[display("Command Parameter Message")]
    CommandParameterMessage = 12,
    #[display("Product Request Cancel")]
    ProductRequestCancel = 13,
    #[display("Command Control Message")]
    CommandControlMessage = 14,
    #[display("Bias Table Message")]
    BiasTableMessage = 15,
    #[display("Base Reflectivity")]
    BaseReflectivity19 = 19,
    #[display("Base Reflectivity")]
    BaseReflectivity20 = 20,
    #[display("Base Velocity")]
    BaseVelocity25 = 25,
    #[display("Base Velocity")]
    BaseVelocity27 = 27,
    #[display("Base Spectrum Width")]
    BaseSpectrumWidth28 = 28,
    #[display("Clutter Filter Control")]
    ClutterFilterControl = 34,
    #[display("Base Spectrum Width")]
    BaseSpectrumWidth = 30,
    #[display("User Selectable Storm Total Precipitation")]
    UserSelectableStormTotalPrecipitation = 31,
    #[display("Digital Hybrid Scan Reflectivity")]
    DigitalHybridScanReflectivity = 32,
    #[display("Composite Reflectivity 0.54 Nmi Resolution")]
    CompositeReflectivity0p54Nmi = 37,
    #[display("Composite Reflectivity 2.2 Nmi Resolution")]
    CompositeReflectivity2p2Nmi = 38,
    #[display("Echo Tops")]
    EchoTops = 41,
    #[display("VAD Wind Profile")]
    VADWindProfile = 48,
    #[display("Cross Section (Reflectivity)")]
    CrossSectionReflectivity = 50,
    #[display("Cross Section (Velocity) Data Level 16")]
    CrossSectionVelocityDataLevel16 = 51,
    #[display("Storm Relative Mean Radial Velocity")]
    StormRelativeMeanRadialVelocity = 56,
    #[display("Vertically Integrated Liquid")]
    VerticallyIntegratedLiquid = 57,
    #[display("Storm Tracking Information")]
    StormTrackingInformation = 58,
    #[display("Hail Index")]
    HailIndex = 59,
    #[display("SpareGeographic and Non-geographic Alpha")]
    SpareGeographicAndNonGeographicAlpha = 60,
    #[display("Tornado Vortex Signature")]
    TornadoVortexSignature = 61,
    #[display("Storm Structure")]
    StormStructure = 62,
    #[display("Layer Composite Reflectivity Layer 1 Max")]
    LayerCompositeReflectivityLayer1Max = 65,
    #[display("Layer Composite Reflectivity Layer 2 Max")]
    LayerCompositeReflectivityLayer2Max = 66,
    #[display("Layer Composite Reflectivity - AP Removed")]
    LayerCompositeReflectivityAPRemoved = 67,
    #[display("Radar Coded Message")]
    RadarCodedMessage = 74,
    #[display("Free Text Message")]
    FreeTextMessage = 75,
    #[display("PUP Text Message")]
    PUPTextMessage = 77,
    #[display("Surface Rainfall Accumulation (1 hr)")]
    SurfaceRainfallAccumulation1hr = 78,
    #[display("Surface Rainfall Accumulation. (3 hr)")]
    SurfaceRainfallAccumulation3hr = 79,
    #[display("Storm Total Rainfall Accumulation")]
    StormTotalRainfallAccumulation = 80,
    #[display("Hourly Digital Precipitation Array")]
    HourlyDigitalPrecipitationArray = 81,
    #[display("Supplemental Precipitation Data")]
    SupplementalPrecipitationData = 82,
    #[display("Velocity Azimuth Display")]
    VelocityAzimuthDisplay = 84,
    #[display("Cross Section Velocity Data Level 8")]
    CrossSectionVelocityDataLevel8 = 86,
    #[display("Layer Composite Reflectivity")]
    LayerCompositeReflectivity = 90,
    #[display("ITWS Digital Base Velocity")]
    ITWSDigitalBaseVelocity = 93,
    #[display("Base Reflectivity Data Array")]
    BaseReflectivityDataArray = 94,
    #[display("Composite Reflectivity Edited for AP 0.54 Nmi")]
    CompositeReflectivityEditedForAP0p54Nmi = 97,
    #[display("Composite Reflectivity Edited for AP 2.2 Nmi")]
    CompositeReflectivityEditedForAP2p2Nmi = 98,
    #[display("Base Velocity Data Array")]
    BaseVelocityDataArray = 99,
    #[display("Site Adaptable parameters for VAD Wind Profile (Product 48)")]
    SiteAdaptableParametersForVADWindProfile = 100,
    #[display("Storm Track Alphanumeric Block")]
    StormTrackAlphanumericBlock = 101,
    #[display("Hail Index Alphanumeric Block")]
    HailIndexAlphanumericBlock = 102,
    #[display("TVS Alphanumeric Block")]
    TVSAlphanumericBlock = 104,
    #[display("Site Adaptable Parameters for Combined Shear")]
    SiteAdaptableParametersForCombinedShear = 105,
    #[display("Surface Rainfall (1 hr) Alphanumeric Block")]
    SurfaceRainfall1HrAlphanumericBlock = 107,
    #[display("Surface Rainfall (3 hr) Alphanumeric Block")]
    SurfaceRainfall3hrAlphanumericBlock = 108,
    #[display("Storm Total Rainfall Accumulation Alphanumeric Block")]
    StormTotalRainfallAccumulationAlphanumericBlock = 109,
    #[display("Clutter Likelihood Reflectivity Alphanumeric Block")]
    ClutterLikelihoodReflectivityAlphanumericBlock = 110,
    #[display("Clutter Likelihood Doppler Alphanumeric Block")]
    ClutterLikelihoodDopplerAlphanumericBlock = 111,
    #[display("Power Removed Control Product")]
    PowerRemovedControlProduct = 113,
    #[display("Clutter Likelihood Reflectivity")]
    ClutterLikelihoodReflectivity = 132,
    #[display("Clutter Likelihood Doppler")]
    ClutterLikelihoodDoppler = 133,
    #[display("High Resolution VIL")]
    HighResolutionVIL = 134,
    #[display("Enhanced Echo Tops")]
    EnhancedEchoTops = 135,
    #[display("User Selectable Layer Composite Reflectivity")]
    UserSelectableLayerCompositeReflectivity = 137,
    #[display("Digital Storm Total Precipitation")]
    DigitalStormTotalPrecipitation = 138,
    #[display("Gust Front MIGFA")]
    GustFrontMIGFA = 140,
    #[display("Mesocyclone Detection")]
    MesocycloneDetection = 141,
    #[display("Tornado Vortex Signature Rapid Update")]
    TornadoVortexSignatureRapidUpdate = 143,
    #[display("One-hour Snow Water Equivalent")]
    OneHourSnowWaterEquivalent = 144,
    #[display("One-hour Snow Depth")]
    OneHourSnowDepth = 145,
    #[display("Storm Total Snow Water Equivalent")]
    StormTotalSnowWaterEquivalent = 146,
    #[display("Storm Total Snow Depth")]
    StormTotalSnowDepth = 147,
    #[display("Digital Mesocyclone Detection")]
    DigitalMesocycloneDetection = 149,
    #[display("User Selectable Snow Water Equivalent")]
    UserSelectableSnowWaterEquivalent = 150,
    #[display("User Selectable Snow Depth")]
    UserSelectableSnowDepth = 151,
    #[display("Archive III Status Product Generic Data Format")]
    ArchiveIIIStatusProductGenericDataFormat = 152,
    #[display("Super Resolution Reflectivity Data Array")]
    SuperResolutionReflectivityDataArray = 153,
    #[display("Super Resolution Velocity Data Array")]
    SuperResolutionVelocityDataArray = 154,
    #[display("Super Resolution Spectrum Width Data Array")]
    SuperResolutionSpectrumWidthDataArray = 155,
    #[display("Digital Differential Reflectivity")]
    DigitalDifferentialReflectivity = 159,
    #[display("Digital Correlation Coefficient")]
    DigitalCorrelationCoefficient = 161,
    #[display("Digital Specific Differential Phase")]
    DigitalSpecificDifferentialPhase = 163,
    #[display("Digital Hydrometeor Classification")]
    DigitalHydrometeorClassification = 165,
    #[display("Melting Layer")]
    MeltingLayer = 166,
    #[display("Super Res Digital Correlation Coefficient")]
    SuperResDigitalCorrelationCoefficient = 167,
    #[display("Super Res Digital Phi")]
    SuperResDigitalPhi = 168,
    #[display("One Hour Accumulation")]
    OneHourAccumulation = 169,
    #[display("Digital Accumulation Array")]
    DigitalAccumulationArray = 170,
    #[display("Storm Total Accumulation")]
    StormTotalAccumulation = 171,
    #[display("Digital Storm Total Accumulation")]
    DigitalStormTotalAccumulation = 172,
    #[display("Digital User- Selectable Accumulation")]
    DigitalUserSelectableAccumulation = 173,
    #[display("Digital One-Hour Difference Accumulation")]
    DigitalOneHourDifferenceAccumulation = 174,
    #[display("Digital Storm Total Difference Accumulation")]
    DigitalStormTotalDifferenceAccumulation = 175,
    #[display("Digital Instantaneous Precipitation Rate")]
    DigitalInstantaneousPrecipitationRate = 176,
    #[display("Hybrid Hydrometeor Classification")]
    HybridHydrometeorClassification = 177,
    #[display("Icing Hazard Level")]
    IcingHazardLevel = 178,
    #[display("Hail Hazard Layers")]
    HailHazardLayers = 179,
    #[display("Base Reflectivity")]
    BaseReflectivity181 = 181,
    #[display("Base Velocity")]
    BaseVelocity182 = 182,
    #[display("Base Reflectivity")]
    BaseReflectivity186 = 186,
    #[display("Super Resolution Digital Reflectivity Data-Quality-Edited")]
    SuperResolutionDigitalReflectivityDataQualityEdited = 193,
    #[display("Digital Reflectivity, DQA-Edited Data Array")]
    DigitalReflectivityDQAEditedDataArray = 195,
    #[display("Microburst AMDA")]
    MicroburstAMDA = 196,
    #[display("Rain Rate Classification")]
    RainRateClassification = 197,
    #[display("Shift Change ChecklistGeneric Data Format")]
    ShiftChangeChecklistGenericDataFormat = 202,
}

impl MessageCode {
    /// Whether this crate's [`crate::message_header`]/[`crate::product_description`]
    /// parsing has been validated against this product type. `Radar::parse`
    /// rejects any file whose product code is not in this list, even if the
    /// message header and product description blocks themselves would
    /// otherwise parse fine.
    pub fn is_supported_product(&self) -> bool {
        let supported_products: [u32;33] = [19, 20, 25, 27, 28, 30, 32, 34, 56, 78, 79, 80, 94, 99, 134, 135, 138, 159, 161, 163, 165, 169, 170, 171, 172, 173, 174, 175, 176, 177, 181, 182, 186];
        supported_products.contains(&(*self as u32))
    }

    /// The highest product version number this crate knows how to interpret
    /// for this product type, or `None` if the product isn't supported at
    /// all (see [`MessageCode::is_supported_product`]). A file whose
    /// `ProductDescription::version` exceeds this is rejected rather than
    /// parsed, since newer versions may have changed the product-dependent
    /// fields in ways this crate doesn't account for.
    pub fn supported_version(&self) -> Option<u8> {
        match *self as u32 {
            19 => Some(0),
            20 => Some(0),
            25 => Some(0),
            27 => Some(0),
            28 => Some(0),
            30 => Some(0),
            32 => Some(2),
            34 => Some(2),
            56 => Some(0),
            78 => Some(1),
            79 => Some(1),
            80 => Some(1),
            94 => Some(0),
            99 => Some(0),
            134 => Some(1),
            135 => Some(0),
            138 => Some(2),
            159 => Some(0),
            161 => Some(0),
            163 => Some(0),
            165 => Some(1),
            169 => Some(0),
            170 => Some(0),
            171 => Some(0),
            172 => Some(1),
            173 => Some(0),
            174 => Some(0),
            175 => Some(0),
            176 => Some(0),
            177 => Some(0),
            181 => Some(0),
            182 => Some(0),
            186 => Some(0),
            _ => None,
        }
    }

    /// Whether [`Self::color_code`] has a real color table for this product
    /// type, as opposed to falling back to a neutral gray for every level.
    pub fn has_color_table(&self) -> bool {
        self.color_table().is_some()
    }

    /// The display color table this product type uses, if the Product
    /// Specification defines one.
    ///
    /// Every table is transcribed from the Interface Control Document Product
    /// Specification, document 2620003AE (Build 24.0, 19 August 2025); the
    /// section each came from is cited on its constant.
    pub fn color_table(&self) -> Option<ColorTable> {
        // (levels, first level code, step between level codes)
        let (levels, first_code, step): (&'static [(u8, u8, u8)], u8, u8) = match self {
            // 3.2.2 (8 levels), shared by the legacy spectrum width products.
            MessageCode::BaseSpectrumWidth28 | MessageCode::BaseSpectrumWidth => {
                (&SPECTRUM_WIDTH, 0, 1)
            }

            // 8.2.2
            MessageCode::EchoTops => (&ECHO_TOPS, 0, 1),

            // 12.2.2. These are the wind barb colour levels, which match the
            // 1 to 5 range of the Wind Barb Data Packet's value field
            // (Figure 3-13) - hence a first level code of 1, not 0. The same
            // section also defines a separate eight-level reflectivity table
            // used to shade velocity points, which this does not model.
            MessageCode::VADWindProfile => (&VAD_WIND_BARB, 1, 1),

            // 16.2.2
            MessageCode::StormRelativeMeanRadialVelocity => (&STORM_RELATIVE_VELOCITY, 0, 1),

            // 17.2.2
            MessageCode::VerticallyIntegratedLiquid => (&VERTICALLY_INTEGRATED_LIQUID, 0, 1),

            // 23.2.2 (8 levels), shared by the layer composite reflectivity
            // products.
            MessageCode::LayerCompositeReflectivityLayer1Max
            | MessageCode::LayerCompositeReflectivityLayer2Max
            | MessageCode::LayerCompositeReflectivityAPRemoved
            | MessageCode::LayerCompositeReflectivity => (&LAYER_COMPOSITE_REFLECTIVITY, 0, 1),

            // 28.2.2
            MessageCode::SurfaceRainfallAccumulation1hr
            | MessageCode::SurfaceRainfallAccumulation3hr => {
                (&SURFACE_RAINFALL_ACCUMULATION, 0, 1)
            }

            // 29.2.2
            MessageCode::StormTotalRainfallAccumulation
            | MessageCode::DigitalStormTotalPrecipitation => {
                (&STORM_TOTAL_RAINFALL_ACCUMULATION, 0, 1)
            }

            // 40.2.2
            MessageCode::UserSelectableLayerCompositeReflectivity => {
                (&USER_SELECTABLE_LAYER_COMPOSITE, 0, 1)
            }

            // 42.2.2
            MessageCode::OneHourSnowWaterEquivalent | MessageCode::OneHourSnowDepth => {
                (&ONE_HOUR_SNOW_ACCUMULATION, 0, 1)
            }

            // 43.2.2
            MessageCode::StormTotalSnowWaterEquivalent | MessageCode::StormTotalSnowDepth => {
                (&STORM_TOTAL_SNOW_ACCUMULATION, 0, 1)
            }

            // 52.2.2. Only levels 0 to 3 are defined; 4 to F are "TBD".
            MessageCode::MeltingLayer => (&MELTING_LAYER, 0, 1),

            // 53.2.2
            MessageCode::OneHourAccumulation => (&ONE_HOUR_ACCUMULATION, 0, 1),

            // 68.2.1. This product's level codes step by 10 (0, 10, ... 100)
            // rather than by 1.
            MessageCode::RainRateClassification => (&RAIN_RATE_CLASSIFICATION, 0, 10),

            _ => return None,
        };
        Some(ColorTable {
            levels,
            first_code,
            step,
        })
    }

    /// Maps a raw data level code to the display color the Product
    /// Specification defines for it, used by [`crate::Radar::plot`].
    ///
    /// Falls back to [`FALLBACK_GRAY`] for any product type without a table
    /// (see [`MessageCode::color_table`]) and for any level code outside the
    /// table's range. Check [`MessageCode::has_color_table`] first if you need
    /// to distinguish "really is gray" from "no table for this product".
    ///
    /// Note that the legacy 16-level Base Reflectivity products (19/20) are
    /// among those without a table: revision AE of the Product Specification
    /// no longer defines one for them, having superseded them with the
    /// 256-level digital data array products.
    pub fn color_code(&self, code: u8) -> RGBColor {
        self.color_table()
            .and_then(|table| table.color(code))
            .unwrap_or(FALLBACK_GRAY)
    }
}

/// The color used when a product has no table defined, or when a data level
/// falls outside the table its product does define.
pub const FALLBACK_GRAY: RGBColor = RGBColor(0x88, 0x88, 0x88);

/// A product's display color table: RGB values for its data level codes.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ColorTable {
    levels: &'static [(u8, u8, u8)],
    /// The level code the first entry corresponds to. Zero for every product
    /// except VAD Wind Profile, whose barb colour levels start at 1.
    first_code: u8,
    /// Increment between successive level codes. One for every product except
    /// Rain Rate Classification, whose codes run 0, 10, 20 ... 100.
    step: u8,
}

impl ColorTable {
    /// The color for a data level code, or `None` if the code is not one this
    /// table defines.
    pub fn color(&self, code: u8) -> Option<RGBColor> {
        if self.step == 0 || code < self.first_code {
            return None;
        }
        let offset = code - self.first_code;
        if !offset.is_multiple_of(self.step) {
            return None;
        }
        self.levels
            .get((offset / self.step) as usize)
            .map(|&(r, g, b)| RGBColor(r, g, b))
    }

    /// Number of data levels the table defines.
    pub fn len(&self) -> usize {
        self.levels.len()
    }

    /// Whether the table defines no levels at all.
    pub fn is_empty(&self) -> bool {
        self.levels.is_empty()
    }

    /// The level codes this table defines, in order.
    pub fn level_codes(&self) -> impl Iterator<Item = u8> + '_ {
        (0..self.levels.len()).map(move |i| self.first_code + i as u8 * self.step)
    }
}

// ---------------------------------------------------------------------------
// Color tables transcribed from the Product Specification (2620003AE).
//
// Each entry is indexed by data level code (offset by the table's first level
// code and step); the trailing comment records the level code and the colour
// name the document gives it.
// ---------------------------------------------------------------------------

/// VAD Wind Profile wind barb colours - Product Specification 12.2.2.
/// Level codes run 1 to 5, matching the Wind Barb Data Packet's value field.
const VAD_WIND_BARB: [(u8, u8, u8); 5] = [
    (0x00, 0xFF, 0x00), // 1: green
    (0xFF, 0xFF, 0x00), // 2: yellow
    (0xFF, 0x00, 0x00), // 3: bright red
    (0x00, 0xE0, 0xFF), // 4: light blue
    (0xFF, 0x70, 0xFF), // 5: medium purple
];

/// Storm Relative Mean Radial Velocity - Product Specification 16.2.2.
const STORM_RELATIVE_VELOCITY: [(u8, u8, u8); 16] = [
    (0x00, 0x00, 0x00), // 0: black
    (0x00, 0xE0, 0xFF), // 1: light blue
    (0x00, 0x80, 0xFF), // 2: medium blue
    (0x32, 0x00, 0x96), // 3: dark blue
    (0x00, 0xFB, 0x90), // 4: light green
    (0x00, 0xBB, 0x00), // 5: medium green
    (0x00, 0x8F, 0x00), // 6: dark green
    (0xCD, 0xC0, 0x9F), // 7: light gray
    (0x76, 0x76, 0x76), // 8: dark gray
    (0xF8, 0x87, 0x00), // 9: medium orange
    (0xFF, 0xCF, 0x00), // A: medium yellow
    (0xFF, 0xFF, 0x00), // B: yellow
    (0xAE, 0x00, 0x00), // C: dark red
    (0xD0, 0x70, 0x00), // D: medium brown
    (0xFF, 0x00, 0x00), // E: bright red
    (0x77, 0x00, 0x7D), // F: dark purple
];

/// Layer Composite Reflectivity - Product Specification 23.2.2 (8 levels).
const LAYER_COMPOSITE_REFLECTIVITY: [(u8, u8, u8); 8] = [
    (0x00, 0x00, 0x00), // 0: black
    (0xFF, 0xAA, 0xAA), // 1: light pink
    (0xC9, 0x70, 0x70), // 2: dark pink
    (0x00, 0xBB, 0x00), // 3: medium green
    (0xFF, 0xFF, 0x70), // 4: light yellow
    (0xDA, 0x00, 0x00), // 5: medium red
    (0x00, 0x00, 0xFF), // 6: blue
    (0xFF, 0xFF, 0xFF), // 7: white
];

/// Melting Layer - Product Specification 52.2.2. Levels 4 to F are "TBD" in
/// the document and so are not defined here.
const MELTING_LAYER: [(u8, u8, u8); 4] = [
    (0x9C, 0x9C, 0x9C), // 0: medium gray (Top Edge)
    (0xF5, 0xF5, 0xF5), // 1: near white  (Top Center)
    (0xF5, 0xF5, 0xF5), // 2: near white  (Bottom Center)
    (0x9C, 0x9C, 0x9C), // 3: medium gray (Bottom Edge)
];

/// One-Hour Accumulation - Product Specification 53.2.2.
const ONE_HOUR_ACCUMULATION: [(u8, u8, u8); 16] = [
    (0x00, 0x00, 0x00), // 0: black
    (0xAA, 0xAA, 0xAA), // 1: gray
    (0x76, 0x76, 0x76), // 2: dark gray
    (0x00, 0xFF, 0xFF), // 3: cyan
    (0x00, 0xAF, 0xAF), // 4: dark cyan
    (0x00, 0xFF, 0x00), // 5: green
    (0x00, 0x8F, 0x00), // 6: dark green
    (0xFF, 0x00, 0xFF), // 7: magenta
    (0xAF, 0x32, 0x7D), // 8: dark magenta
    (0x00, 0x00, 0xFF), // 9: blue
    (0x32, 0x00, 0x96), // A: dark blue
    (0xFF, 0xFF, 0x00), // B: yellow
    (0xFF, 0xAA, 0x00), // C: orange
    (0xFF, 0x00, 0x00), // D: bright red
    (0xAE, 0x00, 0x00), // E: dark red
    (0xFF, 0xFF, 0xFF), // F: white
];

/// Rain Rate Classification - Product Specification 68.2.1. Level codes step
/// by 10, from 0 (No Precip) to 100 (R(Z) * multiplier).
const RAIN_RATE_CLASSIFICATION: [(u8, u8, u8); 11] = [
    (0x00, 0x00, 0x00), // 0:   black       NP  No Precip (Biota or NoEcho)
    (0x66, 0x66, 0x66), // 10:  gray        UF  Unfilled
    (0x66, 0xCC, 0x66), // 20:  light green CZ  Continental R(Z,ZDR)
    (0xC9, 0x70, 0x70), // 30:  med green   TZ  Tropical R(Z,ZDR)
    (0x00, 0xBB, 0x00), // 40:  dark green  SA  Specific Attenuation
    (0xFF, 0xFF, 0x70), // 50:  yellow      KL  R(KDP) 27 coeff.
    (0xDA, 0x00, 0x00), // 60:  red         KH  R(KDP) 44 coeff.
    (0x00, 0x00, 0xFF), // 70:  dark blue   Z1  R(Z)
    (0xCC, 0x99, 0xFF), // 80:  lavender    Z6  R(Z) * 0.6
    (0x33, 0x99, 0xFF), // 90:  med blue    Z8  R(Z) * 0.8
    (0x99, 0xCC, 0xFF), // 100: light blue  SI  R(Z) * multiplier
];

/// Spectrum Width - Product Specification 3.2.2.
const SPECTRUM_WIDTH: [(u8, u8, u8); 8] = [
    (0x00, 0x00, 0x00), // 0: black
    (0x76, 0x76, 0x76), // 1: dark gray
    (0x9C, 0x9C, 0x9C), // 2: medium gray
    (0x00, 0xBB, 0x00), // 3: medium green
    (0xFF, 0x00, 0x00), // 4: bright red
    (0xD0, 0x70, 0x00), // 5: medium brown
    (0xFF, 0xFF, 0x00), // 6: yellow
    (0x77, 0x00, 0x7D), // 7: dark purple
];

/// Echo Tops - Product Specification 8.2.2.
const ECHO_TOPS: [(u8, u8, u8); 16] = [
    (0x00, 0x00, 0x00), // 0: black
    (0x00, 0x00, 0x00), // 1: black
    (0x76, 0x76, 0x76), // 2: dark gray
    (0x00, 0xE0, 0xFF), // 3: light blue
    (0x00, 0xB0, 0xFF), // 4: lt medium blue
    (0x00, 0x90, 0xCC), // 5: dk medium blue
    (0x32, 0x00, 0x96), // 6: dark blue
    (0x00, 0xFB, 0x90), // 7: light green
    (0x00, 0xBB, 0x00), // 8: medium green
    (0x00, 0xEF, 0x00), // 9: bright green
    (0xFE, 0xBF, 0x00), // A: tan
    (0xFF, 0xFF, 0x00), // B: yellow
    (0xAE, 0x00, 0x00), // C: dark red
    (0xFF, 0x00, 0x00), // D: bright red
    (0xFF, 0xFF, 0xFF), // E: white
    (0xE7, 0x00, 0xFF), // F: purple
];

/// Vertically Integrated Liquid - Product Specification 17.2.2.
const VERTICALLY_INTEGRATED_LIQUID: [(u8, u8, u8); 16] = [
    (0x00, 0x00, 0x00), // 0: black
    (0x9C, 0x9C, 0x9C), // 1: medium gray
    (0x76, 0x76, 0x76), // 2: dark gray
    (0xFA, 0xAA, 0xAA), // 3: light pink
    (0xEE, 0x8C, 0x8C), // 4: medium pink
    (0xC9, 0x70, 0x70), // 5: dark pink
    (0x00, 0xFB, 0x90), // 6: light green
    (0x00, 0xBB, 0x00), // 7: medium green
    (0xFF, 0xFF, 0x70), // 8: light yellow
    (0xD0, 0xD0, 0x60), // 9: dark yellow
    (0xFF, 0x60, 0x60), // A: light red
    (0xDA, 0x00, 0x00), // B: medium red
    (0xAE, 0x00, 0x00), // C: dark red
    (0x00, 0x00, 0xFF), // D: blue
    (0xFF, 0xFF, 0xFF), // E: white
    (0xE7, 0x00, 0xFF), // F: purple
];

/// Surface Rainfall Accumulation - Product Specification 28.2.2.
const SURFACE_RAINFALL_ACCUMULATION: [(u8, u8, u8); 16] = [
    (0x00, 0x00, 0x00), // 0: black
    (0xAA, 0xAA, 0xAA), // 1: gray
    (0x76, 0x76, 0x76), // 2: dark gray
    (0x00, 0xFF, 0xFF), // 3: cyan
    (0x00, 0xAF, 0xAF), // 4: dark cyan
    (0x00, 0xFF, 0x00), // 5: green
    (0x00, 0x8F, 0x00), // 6: dark green
    (0xFF, 0x00, 0xFF), // 7: magenta
    (0xAF, 0x32, 0x7D), // 8: dark magenta
    (0x00, 0x00, 0xFF), // 9: blue
    (0x32, 0x00, 0x96), // A: dark blue
    (0xFF, 0xFF, 0x00), // B: yellow
    (0xFF, 0xAA, 0x00), // C: orange
    (0xFF, 0x00, 0x00), // D: bright red
    (0xAE, 0x00, 0x00), // E: dark red
    (0xFF, 0xFF, 0xFF), // F: white
];

/// Storm Total Rainfall Accumulation - Product Specification 29.2.2.
const STORM_TOTAL_RAINFALL_ACCUMULATION: [(u8, u8, u8); 16] = [
    (0x00, 0x00, 0x00), // 0: black
    (0xAA, 0xAA, 0xAA), // 1: gray
    (0x76, 0x76, 0x76), // 2: dark gray
    (0x00, 0xFF, 0xFF), // 3: cyan
    (0x00, 0xAF, 0xAF), // 4: dark cyan
    (0x00, 0xFF, 0x00), // 5: green
    (0x00, 0x8F, 0x00), // 6: dark green
    (0xFF, 0x00, 0xFF), // 7: magenta
    (0xAF, 0x32, 0x7D), // 8: dark magenta
    (0x00, 0x00, 0xFF), // 9: blue
    (0x32, 0x00, 0x96), // A: dark blue
    (0xFF, 0xFF, 0x00), // B: yellow
    (0xFF, 0xAA, 0x00), // C: orange
    (0xFF, 0x00, 0x00), // D: bright red
    (0xAE, 0x00, 0x00), // E: dark red
    (0xFF, 0xFF, 0xFF), // F: white
];

/// User Selectable Layer Composite Reflectivity - Product Specification 40.2.2.
const USER_SELECTABLE_LAYER_COMPOSITE: [(u8, u8, u8); 16] = [
    (0x00, 0x00, 0x00), // 0: black
    (0x9C, 0x9C, 0x9C), // 1: medium gray
    (0x76, 0x76, 0x76), // 2: dark gray
    (0xFF, 0xAA, 0xAA), // 3: light pink
    (0xEE, 0x8C, 0x8C), // 4: medium pink
    (0xC9, 0x70, 0x70), // 5: dark pink
    (0x00, 0xFB, 0x90), // 6: light green
    (0x00, 0xBB, 0x00), // 7: medium green
    (0xFF, 0xFF, 0x70), // 8: light yellow
    (0xD0, 0xD0, 0x60), // 9: dark yellow
    (0xFF, 0x60, 0x60), // 10: light red
    (0xDA, 0x00, 0x00), // 11: medium red
    (0xAE, 0x00, 0x00), // 12: dark red
    (0x00, 0x00, 0xFF), // 13: blue
    (0xFF, 0xFF, 0xFF), // 14: white
    (0xE7, 0x00, 0xFF), // 15: purple
];

/// One Hour Snow Accumulation - Product Specification 42.2.2.
const ONE_HOUR_SNOW_ACCUMULATION: [(u8, u8, u8); 16] = [
    (0x00, 0x00, 0x00), // 0: black
    (0xAA, 0xAA, 0xAA), // 1: gray
    (0x76, 0x76, 0x76), // 2: dark gray
    (0x00, 0xFF, 0xFF), // 3: cyan
    (0x00, 0xAF, 0xAF), // 4: dark cyan
    (0x00, 0xFF, 0x00), // 5: green
    (0x00, 0x8F, 0x00), // 6: dark green
    (0xFF, 0x00, 0xFF), // 7: magenta
    (0xAF, 0x32, 0x7D), // 8: dark magenta
    (0x00, 0x00, 0xFF), // 9: blue
    (0x32, 0x00, 0x96), // A: dark blue
    (0xFF, 0xFF, 0x00), // B: yellow
    (0xFF, 0xAA, 0x00), // C: orange
    (0xFF, 0x00, 0x00), // D: bright red
    (0xAE, 0x00, 0x00), // E: dark red
    (0xFF, 0xFF, 0xFF), // F: white
];

/// Storm Total Snow Accumulation - Product Specification 43.2.2.
const STORM_TOTAL_SNOW_ACCUMULATION: [(u8, u8, u8); 16] = [
    (0x00, 0x00, 0x00), // 0: black
    (0xAA, 0xAA, 0xAA), // 1: gray
    (0x76, 0x76, 0x76), // 2: dark gray
    (0x00, 0xFF, 0xFF), // 3: cyan
    (0x00, 0xAF, 0xAF), // 4: dark cyan
    (0x00, 0xFF, 0x00), // 5: green
    (0x00, 0x8F, 0x00), // 6: dark green
    (0xFF, 0x00, 0xFF), // 7: magenta
    (0xAF, 0x32, 0x7D), // 8: dark magenta
    (0x00, 0x00, 0xFF), // 9: blue
    (0x32, 0x00, 0x96), // A: dark blue
    (0xFF, 0xFF, 0x00), // B: yellow
    (0xFF, 0xAA, 0x00), // C: orange
    (0xFF, 0x00, 0x00), // D: bright red
    (0xAE, 0x00, 0x00), // E: dark red
    (0xFF, 0xFF, 0xFF), // F: white
];




/// The packet code at the start of each symbology-block data layer, which
/// identifies the binary layout of that layer's packet (a vector, a text
/// label, radial data, a raster image, and so on).
///
/// Figure 3-7 through 3-14. Only [`PacketCode::TextAndSpecialSymbol1`],
/// [`PacketCode::TextAndSpecialSymbol2`], [`PacketCode::TextAndSpecialSymbol8`],
/// [`PacketCode::RadialDataAF1F`], and [`PacketCode::DigitalRadialDataArray`]
/// have parsers implemented; every other code (including
/// [`PacketCode::GenericData28`], which is XDR-encoded) fails clearly
/// rather than panicking or silently producing wrong data — see
/// `README.md` for the current list.
#[derive(Serialize, Deserialize, Display, FromStr, PartialEq, Debug, Copy, Clone, Default)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum PacketCode {

    /// Figure 3-7 Sheet 1 pg 3-81
    #[display("Linked Vector")]
    LinkedVector6 = 6,

    /// Figure 3-7 Sheet 2,3 pg 3-81
    #[display("Linked Vector")]
    LinkedVector9 = 9,

    /// Figure 3-8. (Sheet 1, 3) 
    #[display("Unlinked Vector 7")]
    UnlinkedVector7 = 7,
    
    /// Figure 3-8. (Sheet 2, 4) 
    #[display("Unlinked Vector 10")]
    UnlinkedVector10 = 10,
    
    /// Figure 3-8a (Sheet 1, 2) 0x0E03=3587
    #[display("Contour Vector 0E03")]
    ContourVector0E03 = 3587,
    
    /// Figure 3-8a (Sheet 1, 2) 0x0802=2050
    #[display("Contour Vector 0802")]
    ContourVector0802 = 2050,
    
    /// Figure 3-8a (Sheet 1, 3) 0x3501=13569
    #[display("Contour Vector 3501")]
    ContourVector3501 = 13569,
    
    /// Figure 3-8b. (Sheet 1, 4) page 3-88
    #[display("Text and Special Symbol 1")]
    TextAndSpecialSymbol1 = 1,
    
    /// Figure 3-8b. (Sheet 2) 
    #[display("Text and Special Symbol 8")]
    TextAndSpecialSymbol8 = 8,
    
    /// Figure 3-8b. (Sheet 3, 5) 
    #[display("Text and Special Symbol 2")]
    TextAndSpecialSymbol2 = 2,
    
    /// Figure 3-9. (Sheet 2, 3) 0x0E23=3619
    #[display("Map Message 0E23")]
    MapMessage0E23 = 3619,
    
    /// Figure 3-9. (Sheet 2, 3) 0x4E00=19968
    #[display("Map Message 4E00")]
    MapMessage4E00 = 19968,
    
    /// Figure 3-9. (Sheet 2, 3) 0x3521=13601
    #[display("Map Message 3521")]
    MapMessage3521 = 13601,
    
    /// Figure 3-9. (Sheet 2, 3) 0x4E01=19969
    #[display("Map Message 4E01")]
    MapMessage4E01 = 19969,
    
    /// Figure 3-10. (Sheet 1, 2) 0xAF1F=-20705
    #[display("Radial Data (16 Data Levels)")]
    RadialDataAF1F = -20705,
    
    /// Figure 3-11. (Sheet 1, 2) 0xBA0F=-17905
    #[display("Raster Data BA0F")]
    RasterDataBA0F = -17905,
    
    /// Figure 3-11. (Sheet 1, 2) 0xBA07=-17913
    #[display("Raster Data BA07")]
    RasterDataBA07 = -17913,
    
    /// Figure 3-11a. (Sheet 1, 2) 
    #[display("Digital Precipitation Data Array")]
    DigitalPrecipitationDataArray = 17,
    
    /// Figure 3-11b. (Sheet 1, 2) 
    #[display("Precipitation Rate Data Array")]
    PrecipitationRateDataArray = 18,
    
    /// Figure 3-11c. (Sheet 1, 2)
    #[display("Digital Radial Data Array")]
    DigitalRadialDataArray = 16,

    /// Figure 3-11d. (Sheet 1, 2) page 3-102
    #[display("Digital Raster Data Array")]
    DigitalRasterDataArray = 33,

    /// Figure 3-12.
    #[display("Vector Arrow Data")]
    VectorArrowData = 5,
    
    /// Figure 3-13. 
    #[display("Wind Barb Data")]
    WindBarbData = 4,
    
    /// Figure 3-14. (Sheet 1, 3) 
    #[display("Special Graphic Symbol 3")]
    SpecialGraphicSymbol3 = 3,
    
    /// Figure 3-14. (Sheet 1, 3) 
    #[display("Special Graphic Symbol 11")]
    SpecialGraphicSymbol11 = 11,
    
    /// Figure 3-14. (Sheet 1, 3) 
    #[display("Special Graphic Symbol 12")]
    SpecialGraphicSymbol12 = 12,
    
    /// Figure 3-14. (Sheet 1, 3) 
    #[display("Special Graphic Symbol 13")]
    SpecialGraphicSymbol13 = 13,
    
    /// Figure 3-14. (Sheet 1, 3) 
    #[display("Special Graphic Symbol 14")]
    SpecialGraphicSymbol14 = 14,
    
    /// Figure 3-14. (Sheet 2, 3) 
    #[display("Special Graphic Symbol 15")]
    SpecialGraphicSymbol15 = 15,
    
    /// Figure 3-14. (Sheet 2, 3) 
    #[display("Special Graphic Symbol 19")]
    SpecialGraphicSymbol19 = 19,
    
    /// Figure 3-14. (Sheet 2, 3) 
    #[display("Special Graphic Symbol 23")]
    SpecialGraphicSymbol23 = 23,
    
    /// Figure 3-14. (Sheet 2, 3) 
    #[display("Special Graphic Symbol 24")]
    SpecialGraphicSymbol24 = 24,
    
    /// Figure 3-14. (Sheet 2, 3) 
    #[display("Special Graphic Symbol 25")]
    SpecialGraphicSymbol25 = 25,
    
    /// Figure 3-14. (Sheet 3) 
    #[display("Special Graphic Symbol 26")]
    SpecialGraphicSymbol26 = 26,
    
    /// Figure 3-14. (Sheet 4) 
    #[display("Special Graphic Symbol 20")]
    SpecialGraphicSymbol20 = 20,
    
    /// Figure 3-15. (Sheet 1, 2) 
    #[display("Cell Trend Data")]
    CellTrendData = 21,
    
    /// Figure 3-15a. 
    #[display("Cell Trend Volume Scan Times")]
    CellTrendVolumeScanTimes = 22,
    
    /// Figure 3-15c (Sheet 1) 
    #[display("Generic Data")]
    GenericData28 = 28,
    
    /// Figure 3-15c (Sheet 1) 
    #[display("Generic Data")]
    GenericData29 = 29,


    #[default]
    #[display("Other / Unknown")]
    Other = 0,
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Every code number that `is_supported_product`/`supported_version`
    /// claim to support must round-trip through `FromPrimitive` back to the
    /// exact same code. This is a regression test for a bug where nine of
    /// these codes (including the very common Base Reflectivity/Velocity
    /// products, 19/20/25/27/28/34/181/182/186) had no matching enum
    /// variant at all, so `FromPrimitive` silently produced `Spare` instead
    /// and every file using one of those product types was rejected as
    /// "unsupported."
    #[test]
    fn every_declared_supported_code_has_a_matching_enum_variant() {
        let supported_products: [u32; 33] = [
            19, 20, 25, 27, 28, 30, 32, 34, 56, 78, 79, 80, 94, 99, 134, 135, 138, 159, 161, 163,
            165, 169, 170, 171, 172, 173, 174, 175, 176, 177, 181, 182, 186,
        ];

        for code in supported_products {
            let parsed = <MessageCode as num::FromPrimitive>::from_u32(code)
                .unwrap_or_else(|| panic!("no MessageCode variant at all decodes to {code}"));
            assert_eq!(
                parsed as u32, code,
                "code {code} decoded to a different variant ({parsed:?} = {})",
                parsed as u32
            );
            assert!(
                parsed.is_supported_product(),
                "code {code} round-tripped to {parsed:?} but is_supported_product() is false for it"
            );
        }
    }

    #[test]
    fn unknown_codes_fall_back_to_spare_and_are_rejected() {
        let parsed = <MessageCode as num::FromPrimitive>::from_u32(65535).unwrap_or_default();
        assert_eq!(parsed, MessageCode::Spare);
        assert!(!parsed.is_supported_product());
    }

    /// Every product the Product Specification defines a color table for
    /// should report one, at the level count the document gives.
    #[test]
    fn every_product_with_a_spec_table_exposes_it() {
        // (product, number of levels, Product Specification section)
        let expected = [
            (MessageCode::BaseSpectrumWidth28, 8, "3.2.2"),
            (MessageCode::BaseSpectrumWidth, 8, "3.2.2"),
            (MessageCode::EchoTops, 16, "8.2.2"),
            (MessageCode::VADWindProfile, 5, "12.2.2"),
            (MessageCode::StormRelativeMeanRadialVelocity, 16, "16.2.2"),
            (MessageCode::VerticallyIntegratedLiquid, 16, "17.2.2"),
            (MessageCode::LayerCompositeReflectivityLayer1Max, 8, "23.2.2"),
            (MessageCode::LayerCompositeReflectivityLayer2Max, 8, "23.2.2"),
            (MessageCode::LayerCompositeReflectivityAPRemoved, 8, "23.2.2"),
            (MessageCode::LayerCompositeReflectivity, 8, "23.2.2"),
            (MessageCode::SurfaceRainfallAccumulation1hr, 16, "28.2.2"),
            (MessageCode::SurfaceRainfallAccumulation3hr, 16, "28.2.2"),
            (MessageCode::StormTotalRainfallAccumulation, 16, "29.2.2"),
            (MessageCode::DigitalStormTotalPrecipitation, 16, "29.2.2"),
            (MessageCode::UserSelectableLayerCompositeReflectivity, 16, "40.2.2"),
            (MessageCode::OneHourSnowWaterEquivalent, 16, "42.2.2"),
            (MessageCode::OneHourSnowDepth, 16, "42.2.2"),
            (MessageCode::StormTotalSnowWaterEquivalent, 16, "43.2.2"),
            (MessageCode::StormTotalSnowDepth, 16, "43.2.2"),
            (MessageCode::MeltingLayer, 4, "52.2.2"),
            (MessageCode::OneHourAccumulation, 16, "53.2.2"),
            (MessageCode::RainRateClassification, 11, "68.2.1"),
        ];

        for (product, levels, section) in expected {
            let table = product
                .color_table()
                .unwrap_or_else(|| panic!("{product:?} should have a table from section {section}"));
            assert_eq!(
                table.len(),
                levels,
                "{product:?} (section {section}) should define {levels} levels"
            );
            assert!(product.has_color_table());
            // Every level the table claims must resolve to a real color.
            for code in table.level_codes() {
                assert!(
                    table.color(code).is_some(),
                    "{product:?} level {code} should resolve"
                );
            }
        }
    }

    /// Base Reflectivity is a real, common product that revision AE of the
    /// Product Specification does *not* define a table for, so it must fall
    /// back to gray rather than panicking.
    #[test]
    fn products_without_a_spec_table_fall_back_to_gray() {
        assert!(!MessageCode::BaseReflectivity20.has_color_table());
        assert!(MessageCode::BaseReflectivity20.color_table().is_none());
        assert_eq!(MessageCode::BaseReflectivity20.color_code(5), FALLBACK_GRAY);

        // Clutter Likelihood Reflectivity is listed in the spec but every
        // level except 0 is "TBD", so no table is transcribed for it.
        assert!(!MessageCode::ClutterLikelihoodReflectivity.has_color_table());
    }

    #[test]
    fn level_codes_outside_a_table_fall_back_to_gray() {
        // Melting Layer defines only levels 0-3; 4-F are "TBD" in the spec.
        assert_ne!(MessageCode::MeltingLayer.color_code(3), FALLBACK_GRAY);
        assert_eq!(MessageCode::MeltingLayer.color_code(4), FALLBACK_GRAY);
        assert_eq!(MessageCode::MeltingLayer.color_code(15), FALLBACK_GRAY);
    }

    /// Rain Rate Classification level codes step by 10, so intermediate
    /// values are not table entries.
    #[test]
    fn rain_rate_classification_level_codes_step_by_ten() {
        let rrc = MessageCode::RainRateClassification;
        assert_eq!(rrc.color_code(0), RGBColor(0x00, 0x00, 0x00));
        assert_eq!(rrc.color_code(20), RGBColor(0x66, 0xCC, 0x66));
        assert_eq!(rrc.color_code(100), RGBColor(0x99, 0xCC, 0xFF));

        // 5 and 15 are not defined level codes.
        assert_eq!(rrc.color_code(5), FALLBACK_GRAY);
        assert_eq!(rrc.color_code(15), FALLBACK_GRAY);
        // Nor is anything past the last level.
        assert_eq!(rrc.color_code(110), FALLBACK_GRAY);

        let table = rrc.color_table().unwrap();
        assert_eq!(
            table.level_codes().collect::<Vec<_>>(),
            vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
        );
    }

    /// VAD Wind Profile barb colours start at level code 1, matching the Wind
    /// Barb Data Packet's 1-to-5 value range (Figure 3-13), so level 0 is not
    /// a table entry.
    #[test]
    fn vad_wind_barb_level_codes_start_at_one() {
        let vad = MessageCode::VADWindProfile;
        assert_eq!(vad.color_code(0), FALLBACK_GRAY);
        assert_eq!(vad.color_code(1), RGBColor(0x00, 0xFF, 0x00));
        assert_eq!(vad.color_code(5), RGBColor(0xFF, 0x70, 0xFF));
        assert_eq!(vad.color_code(6), FALLBACK_GRAY);

        let table = vad.color_table().unwrap();
        assert_eq!(table.level_codes().collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
    }

    /// Spot checks against values read directly out of the Product
    /// Specification, so a bad transcription is caught.
    #[test]
    fn transcribed_values_match_the_specification() {
        // 8.2.2: levels 0 and 1 are both black ("No Data" and "kft<5").
        assert_eq!(MessageCode::EchoTops.color_code(0), RGBColor(0x00, 0x00, 0x00));
        assert_eq!(MessageCode::EchoTops.color_code(1), RGBColor(0x00, 0x00, 0x00));
        assert_eq!(MessageCode::EchoTops.color_code(0x0A), RGBColor(0xFE, 0xBF, 0x00));
        assert_eq!(MessageCode::EchoTops.color_code(0x0F), RGBColor(0xE7, 0x00, 0xFF));

        // 16.2.2
        let srm = MessageCode::StormRelativeMeanRadialVelocity;
        assert_eq!(srm.color_code(7), RGBColor(0xCD, 0xC0, 0x9F));
        assert_eq!(srm.color_code(0x0F), RGBColor(0x77, 0x00, 0x7D));

        // 53.2.2
        let oha = MessageCode::OneHourAccumulation;
        assert_eq!(oha.color_code(8), RGBColor(0xAF, 0x32, 0x7D));
        assert_eq!(oha.color_code(0x0F), RGBColor(0xFF, 0xFF, 0xFF));

        // 23.2.2 (8 level)
        let lrm = MessageCode::LayerCompositeReflectivityLayer1Max;
        assert_eq!(lrm.color_code(0), RGBColor(0x00, 0x00, 0x00));
        assert_eq!(lrm.color_code(7), RGBColor(0xFF, 0xFF, 0xFF));

        // 3.2.2 (8 level)
        assert_eq!(
            MessageCode::BaseSpectrumWidth.color_code(5),
            RGBColor(0xD0, 0x70, 0x00)
        );
    }

    /// Section 17.2.2 lists VIL level 3 as `FA AA AA`, whereas the otherwise
    /// identical table in 40.2.2 lists `FF AA AA` — both named "light pink".
    /// This is a discrepancy in the document itself; both are transcribed as
    /// printed rather than silently harmonised, and this test pins that down
    /// so the difference is not mistaken for a typo on our side.
    #[test]
    fn vil_and_ulr_differ_at_level_three_as_the_document_does() {
        assert_eq!(
            MessageCode::VerticallyIntegratedLiquid.color_code(3),
            RGBColor(0xFA, 0xAA, 0xAA)
        );
        assert_eq!(
            MessageCode::UserSelectableLayerCompositeReflectivity.color_code(3),
            RGBColor(0xFF, 0xAA, 0xAA)
        );
    }
}