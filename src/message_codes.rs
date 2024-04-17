use parse_display::{Display, FromStr};


/// TABLE II NEXRAD MESSAGE CODE DEFINITIONS
/// TABLE III MESSAGE CODES FOR PRODUCTS
#[derive(Display, FromStr, PartialEq, Debug, Copy, Clone, Default)]
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
    CompositeReflectivityEditedforAP0p54Nmi = 97,
    #[display("Composite Reflectivity Edited for AP 2.2 Nmi")]
    CompositeReflectivityEditedforAP2p2Nmi = 98,
    #[display("Base Velocity Data Array")]
    BaseVelocityDataArray = 99,
    #[display("Site Adaptable parameters for VAD Wind Profile (Product 48)")]
    SiteAdaptableparametersforVADWindProfile = 100,
    #[display("Storm Track Alphanumeric Block")]
    StormTrackAlphanumericBlock = 101,
    #[display("Hail Index Alphanumeric Block")]
    HailIndexAlphanumericBlock = 102,
    #[display("TVS Alphanumeric Block")]
    TVSAlphanumericBlock = 104,
    #[display("Site Adaptable Parameters for Combined Shear")]
    SiteAdaptableParametersforCombinedShear = 105,
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