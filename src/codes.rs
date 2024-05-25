use parse_display::{Display, FromStr};
use plotters::style::RGBColor;
use serde::{Deserialize, Serialize};
use tracing::error;

/// TABLE II NEXRAD MESSAGE CODE DEFINITIONS
/// TABLE III MESSAGE CODES FOR PRODUCTS
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
    pub fn is_supported_product(&self) -> bool {
        let supported_products: [u32;33] = [19, 20, 25, 27, 28, 30, 32, 34, 56, 78, 79, 80, 94, 99, 134, 135, 138, 159, 161, 163, 165, 169, 170, 171, 172, 173, 174, 175, 176, 177, 181, 182, 186];
        supported_products.contains(&(*self as u32))
    }

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

    pub fn color_code(&self, code: u8) -> RGBColor {
        match self {
            
            MessageCode::StormTotalSnowDepth => {
                // Code	SSW_Display_Inches	SSW_Range_Inches SSD_Range_Inches	SSD_Display_Inches	Code	Color
                // 0	ND	in=0.0	ND	in=0.0	(00 00 00)	black
                // 1	>0.00	0.0<in<0.05	>0.00	0.0<in<0.5	(AA AA AA)	gray
                // 2	0.05	0.05<in<0.10	0.5	0.5<in<1.0	(76 76 76)	dark gray
                // 3	0.10	0.10<in<0.15	1.0	1.0<in<2.0	(00 FF FF)	cyan
                // 4	0.15	0.15<in<0.20	2.0	2.0<in<3.0	(00 AF AF)	dark cyan
                // 5	0.20	0.20<in<0.25	3.0	3.0<in<4.0	(00 FF 00)	green
                // 6	0.25	0.25<in<0.30	4.0	4.0<in<5.0	(00 8F 00)	dark green
                // 7	0.30	0.30<in<0.40	5.0	5.0<in<6.0	(FF 00 FF)	magenta
                // 8	0.40	0.40<in<0.50	6.0	6.0<in<8.0	(AF 32 7D)	dark magenta
                // 9	0.50	0.50<in<0.75	8.0	8.0<in<10.0	(00 00 FF)	blue
                // A	0.75	0.75<in<1.00	10.0	10.0<in<12.0	(32 00 96)	dark blue
                // B	1.00	1.00<in<1.25	12.0	12.0<in<15.0	(FF FF 00)	yellow
                // C	1.25	1.25<in<1.50	15.0	15.0<in<20.0	(FF AA 00)	orange
                // D	1.50	1.50<in<2.00	20.0	20.0<in<25.0	(FF 00 00)	bright red
                // E	2.00	2.00<in<2.50	25.0	25.0<in<30.0	(AE 00 00)	dark red
                // F	2.50	2.50>in	30	30.0>in	(FF FF FF)	white
                match code {
                    0 => RGBColor(0x00, 0x00, 0x00),
                    1 => RGBColor(0xAA, 0xAA, 0xAA),
                    2 => RGBColor(0x76, 0x76, 0x76),
                    3 => RGBColor(0x00, 0xFF, 0xFF),
                    4 => RGBColor(0x00, 0xAF, 0xAF),
                    5 => RGBColor(0x00, 0xFF, 0x00),
                    6 => RGBColor(0x00, 0x8F, 0x00),
                    7 => RGBColor(0xFF, 0x00, 0xFF),
                    8 => RGBColor(0xAF, 0x32, 0x7D),
                    9 => RGBColor(0x00, 0x00, 0xFF),
                    0x0A => RGBColor(0x32, 0x00, 0x96),
                    0x0B => RGBColor(0xFF, 0xFF, 0x00),
                    0x0C => RGBColor(0xFF, 0xAA, 0x00),
                    0x0D => RGBColor(0xFF, 0x00, 0x00),
                    0x0E => RGBColor(0xAE, 0x00, 0x00),
                    0x0F => RGBColor(0xFF, 0xFF, 0xFF),
                    _ => RGBColor(0x88, 0x88, 0x88),
                }
            }
            MessageCode::MeltingLayer => {
                // 16-Level Code	Display Category Code	Display Condition	Color Levels
                // 0	TE	Melting Layer Top Edge	(9C 9C 9C)	medium gray
                // 1	TC	Melting Layer Top Center	(F5 F5 F5)	near white
                // 2	BC	Melting Layer Bottom Center	(F5 F5 F5)	near white
                // 3	BE	Melting Layer Bottom Edge	(9C 9C 9C)	medium gray
                match code {
                    0 => RGBColor(0x9C, 0x9C, 0x9C),
                    1 => RGBColor(0xF5, 0xF5, 0xF5),
                    2 => RGBColor(0xF5, 0xF5, 0xF5),
                    3 => RGBColor(0x9C, 0x9C, 0x9C),
                    _ => RGBColor(0x88, 0x88, 0x88),
                }
            }
            MessageCode::OneHourAccumulation => {
                // 16-Level Code	Display Inches	Range Inches	Code	Color
                // 0	ND	in = 0.0	(00 00 00)	black
                // 1	>0.00	0.0 < in < 0.1	(AA AA AA)	gray
                // 2	0.10	0.1 ≤ in < 0.25	(76 76 76)	dark gray
                // 3	0.25	0.25 ≤ in < 0.5	(00 FF FF)	cyan
                // 4	0.50	0.5 ≤ in < 0.75	(00 AF AF)	dark cyan
                // 5	0.75	0.75 ≤ in < 1.0	(00 FF 00)	green
                // 6	1.00	1.0 ≤ in < 1.25	(00 8F 00)	dark green
                // 7	1.25	1.25 ≤ in < 1.5	(FF 00 FF)	magenta
                // 8	1.50	1.5 ≤ in < 1.75	(AF 32 7D)	dark magenta
                // 9	1.75	1.75 ≤ in < 2.0	(00 00 FF)	blue
                // A	2.00	2.0 ≤ in < 2.5	(32 00 96)	dark blue
                // B	2.50	2.5 ≤ in < 3.0	(FF FF 00)	yellow
                // C	3.00	3.0 ≤ in < 4.0	(FF AA 00)	orange
                // D	4.00	4.0 ≤ in < 6.0	(FF 00 00)	bright red
                // E	6.00	6.0 ≤ in < 8.0	(AE 00 00)	dark red
                // F	8.00	8.0 ≤ in	(FF FF FF)	white

                match code {
                    0 => RGBColor(0x00, 0x00, 0x00),
                    1 => RGBColor(0xAA, 0xAA, 0xAA),
                    2 => RGBColor(0x76, 0x76, 0x76),
                    3 => RGBColor(0x00, 0xFF, 0xFF),
                    4 => RGBColor(0x00, 0xAF, 0xAF),
                    5 => RGBColor(0x00, 0xFF, 0x00),
                    6 => RGBColor(0x00, 0x8F, 0x00),
                    7 => RGBColor(0xFF, 0x00, 0xFF),
                    8 => RGBColor(0xAF, 0x32, 0x7D),
                    9 => RGBColor(0x00, 0x00, 0xFF),
                    0x0A => RGBColor(0x32, 0x00, 0x96),
                    0x0B => RGBColor(0xFF, 0xFF, 0x00),
                    0x0C => RGBColor(0xFF, 0xAA, 0x00),
                    0x0D => RGBColor(0xFF, 0x00, 0x00),
                    0x0E => RGBColor(0xAE, 0x00, 0x00),
                    0x0F => RGBColor(0xFF, 0xFF, 0xFF),
                    _ => RGBColor(0x88, 0x88, 0x88),
                }
            }
            MessageCode::RainRateClassification => {
                // Level Code	Display	Meaning	                    Code	    Color
                // 0	        NP	    No Precip (Biota or NoEcho)	(00 00 00)	black
                // 10	        UF	    Unfilled	                (66 66 66)	gray
                // 20	        CZ	    Convective R(Z,ZDR)	        (66 CC 66)	light green
                // 30	        TZ	    Tropical R(Z,ZDR)	        (C9 70 70)	medium green
                // 40	        SA	    Specific Attenuation	    (00 BB 00)	dark green
                // 50	        KL	    R(KDP) 25 coeff.	        (FF FF 70)	yellow
                // 60	        KH	    R(KDP) 44 coeff.	        (DA 00 00)	red
                // 70	        Z1	    R(Z)	                    (00 00 FF)	dark blue
                // 80	        Z6	    R(Z) * 0.6	                (CC 99 FF)	lavender
                // 90	        Z8	    R(Z) * 0.8	                (33 99 FF)	medium blue
                // 100	        SI	    R(Z) * multiplier	        (99 CC FF)	light blue

                match code {
                    0 => RGBColor(0x00, 0x00, 0x00),
                    10 => RGBColor(0x66, 0x66, 0x66),
                    20 => RGBColor(0x66, 0xCC, 0x66),
                    30 => RGBColor(0xC9, 0x70, 0x70),
                    40 => RGBColor(0x00, 0xBB, 0x00),
                    50 => RGBColor(0xFF, 0xFF, 0x70),
                    60 => RGBColor(0xDA, 0x00, 0x00),
                    70 => RGBColor(0x00, 0x00, 0xFF),
                    80 => RGBColor(0xCC, 0x99, 0xFF),
                    90 => RGBColor(0x33, 0x99, 0xFF),
                    100 => RGBColor(0x99, 0xCC, 0xFF),
                    _ => RGBColor(0x88, 0x88, 0x88),
                }
            }
            MessageCode::StormRelativeMeanRadialVelocity => {
                match code {
                     0 => RGBColor(0x00, 0x00, 0x00),
                     1 => RGBColor(0x00, 0xE0, 0xFF),
                     2 => RGBColor(0x00, 0x80, 0xFF),
                     3 => RGBColor(0x32, 0x00, 0x96),
                     4 => RGBColor(0x00, 0xFB, 0x90),
                     5 => RGBColor(0x00, 0xBB, 0x00),
                     6 => RGBColor(0x00, 0x8F, 0x00),
                     7 => RGBColor(0xCD, 0xC0, 0x9F),
                     8 => RGBColor(0x76, 0x76, 0x76),
                    9 => RGBColor(0xF8, 0x87, 0x00),
                    0x0A => RGBColor(0xFF, 0xCF, 0x00),
                    0x0B => RGBColor(0xFF, 0xFF, 0x00),
                    0x0C => RGBColor(0xAE, 0x00, 0x00),
                    0x0D => RGBColor(0xD0, 0x70, 0x00),
                    0x0E => RGBColor(0xFF, 0x00, 0x00),
                    0x0F => RGBColor(0x77, 0x00, 0x7D),
                     _ => RGBColor(0x88, 0x88, 0x88),
                }
            }
            _ => {
                error!("This message code type does not have a color table defined.  It needs to be added to codes.rs.  Tables of color levels for each product type are found in document `2620003AB`");
                todo!();
            }
        }
    }
}



/// Figure 3-7 through 3-14
/// 
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


impl PacketCode {
    // pub fn is_supported_product(&self) -> bool {
    //     let supported_products: [i32;3] = [-20705, 16, 28];
    //     supported_products.contains(&(*self as i32))
    // }


}