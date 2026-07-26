//! # ARINC 424 Definitions - Complexes
//! This module contains the complex types for the ARINC 424 data. This is not a formally defined thing, but
//! rather a term I have used to describe the encoding of multiple annotations onto one field definition in the spec
//! The first example of which is given in 5.35 - Navaid Class which describes a 5-letter code that adds annotations onto a Navaid record.
//!
//! The way we encode these complexes are as tuples or nested enums. In the above example, the Navaid Class is encoded as a tuple of 5 enum fields. Thus,
//! when parsing a VHF Navaid record in particular we represent this as the combination of:
//!
//! - VHFNavaidType1
//! - VHFNavaidType2
//! - VHFRangePower
//! - VHFAdditionalInfo
//! - Collocation
//!
use crate::types::fields::{
    AltitudeNumeric, BLANK, FieldParseError, MultiUnitAltitudeNumeric, ParseableField, UintNumeric,
};

// VHF Navaid Class
#[derive(Debug, PartialEq, Eq)]
pub enum VHFNavaidType1 {
    VOR,
    Other,
}
impl ParseableField for VHFNavaidType1 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"V" => VHFNavaidType1::VOR,
            [BLANK] => VHFNavaidType1::Other,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid VHF Navaid Type 1: '{}'",
                    bytes[0] as char
                )));
            }
        }))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum VHFNavaidType2 {
    DME,
    TACAN,
    MILTACAN,
    ILSDMETACAN,
    MLSDMEN,
    MLSDMEP,
    NotApplicable,
}
impl ParseableField for VHFNavaidType2 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(Some(VHFNavaidType2::NotApplicable));
        }
        Ok(Some(match bytes {
            b"D" => VHFNavaidType2::DME,
            b"T" => VHFNavaidType2::TACAN,
            b"M" => VHFNavaidType2::MILTACAN,
            b"I" => VHFNavaidType2::ILSDMETACAN,
            b"N" => VHFNavaidType2::MLSDMEN,
            b"P" => VHFNavaidType2::MLSDMEP,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid VHF Navaid Type 2".to_string(),
                ));
            }
        }))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum VHFRangePower {
    Terminal,
    LowAltitude,
    HighAltitude,
    Undefined,
    ILSTACAN,
}
impl VHFRangePower {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"T" => VHFRangePower::Terminal,
            b"L" => VHFRangePower::LowAltitude,
            b"H" => VHFRangePower::HighAltitude,
            b"U" => VHFRangePower::Undefined,
            b"C" => VHFRangePower::ILSTACAN,
            _ => {
                return Err(FieldParseError::new("Invalid Range Power".to_string()));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum VHFAdditionalInfo {
    BiasedILSDMEOrILSTACAN,
    AutomaticTranscribedWeatherBroadcast,
    ScheduledWeatherBroadcast,
    NoVoiceOnFrequency,
    VoiceOnFrequency,
}

impl VHFAdditionalInfo {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"D" => VHFAdditionalInfo::BiasedILSDMEOrILSTACAN,
            b"A" => VHFAdditionalInfo::AutomaticTranscribedWeatherBroadcast,
            b"B" => VHFAdditionalInfo::ScheduledWeatherBroadcast,
            b"W" => VHFAdditionalInfo::NoVoiceOnFrequency,
            [BLANK] => VHFAdditionalInfo::VoiceOnFrequency,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid VHF Additional Info".to_string(),
                ));
            }
        }))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum Collocation {
    Collocated,
    NonCollocated,
}

impl ParseableField for Collocation {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => Collocation::Collocated,
            b"N" => Collocation::NonCollocated,
            _ => {
                return Err(FieldParseError::new("Invalid Collocation".to_string()));
            }
        }))
    }
}

/// 5.35(A) NAVAID Class
#[derive(Debug, PartialEq, Eq)]
pub struct VHFNavaidClass(
    VHFNavaidType1,
    VHFNavaidType2,
    VHFRangePower,
    VHFAdditionalInfo,
    Collocation,
);
impl ParseableField for VHFNavaidClass {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(VHFNavaidClass(
            VHFNavaidType1::from_bytes(&bytes[0..1])?
                .ok_or(FieldParseError::new("Invalid Navaid Type 1".to_string()))?,
            VHFNavaidType2::from_bytes(&bytes[1..2])?
                .ok_or(FieldParseError::new("Invalid Navaid Type 2".to_string()))?,
            VHFRangePower::from_bytes(&bytes[2..3])?
                .ok_or(FieldParseError::new("Invalid Range Power".to_string()))?,
            VHFAdditionalInfo::from_bytes(&bytes[3..4])?
                .ok_or(FieldParseError::new("Invalid Additional Info".to_string()))?,
            Collocation::from_bytes(&bytes[4..5])?
                .ok_or(FieldParseError::new("Invalid Collocation".to_string()))?,
        )))
    }
}

// NDB Navaid Class
#[derive(Debug, PartialEq, Eq)]
pub enum NDBNavaidType1 {
    NDB,
    SABH,
    MarineBeacon,
}
impl ParseableField for NDBNavaidType1 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"H" => NDBNavaidType1::NDB,
            b"S" => NDBNavaidType1::SABH,
            b"M" => NDBNavaidType1::MarineBeacon,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid NDB Navaid Type 1".to_string(),
                ));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NDBNavaidType2 {
    InnerMarker,
    MiddleMarker,
    OuterMarker,
    BackMarker,
    NotApplicable,
}
impl ParseableField for NDBNavaidType2 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(Some(NDBNavaidType2::NotApplicable));
        }
        Ok(Some(match bytes {
            b"I" => NDBNavaidType2::InnerMarker,
            b"M" => NDBNavaidType2::MiddleMarker,
            b"O" => NDBNavaidType2::OuterMarker,
            b"C" => NDBNavaidType2::BackMarker,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid NDB Navaid Type 2: '{}'",
                    bytes[0] as char
                )));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NDBRangePower {
    HighPoweredNDB,
    NDB,
    LowPoweredNDB,
    Locator,
}

impl ParseableField for NDBRangePower {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"H" => NDBRangePower::HighPoweredNDB,
            [BLANK] => NDBRangePower::NDB,
            b"M" => NDBRangePower::LowPoweredNDB,
            b"L" => NDBRangePower::Locator,
            _ => {
                return Err(FieldParseError::new("Invalid NDB Range Power".to_string()));
            }
        }))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum NDBAdditionalInfo {
    AutomaticTranscribedWeatherBroadcast,
    ScheduledWeatherBroadcast,
    NoVoiceOnFrequency,
    VoiceOnFrequency,
}

impl ParseableField for NDBAdditionalInfo {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => NDBAdditionalInfo::AutomaticTranscribedWeatherBroadcast,
            b"B" => NDBAdditionalInfo::ScheduledWeatherBroadcast,
            b"W" => NDBAdditionalInfo::NoVoiceOnFrequency,
            [BLANK] => NDBAdditionalInfo::VoiceOnFrequency,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid NDB Additional Info".to_string(),
                ));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NDBCollocation {
    BFOOperation,
    NonCollocated,
}

impl ParseableField for NDBCollocation {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"B" => NDBCollocation::BFOOperation,
            [BLANK] => NDBCollocation::NonCollocated,
            _ => {
                return Err(FieldParseError::new("Invalid NDB Collocation".to_string()));
            }
        }))
    }
}

/// 5.35(B) NDB Navaid Class
#[derive(Debug, PartialEq, Eq)]
pub struct NDBNavaidClass(
    NDBNavaidType1,
    NDBNavaidType2,
    NDBRangePower,
    NDBAdditionalInfo,
    NDBCollocation,
);
impl ParseableField for NDBNavaidClass {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(NDBNavaidClass(
            NDBNavaidType1::from_bytes(&bytes[0..1])?
                .ok_or(FieldParseError::new("Invalid Navaid Type 1".to_string()))?,
            NDBNavaidType2::from_bytes(&bytes[1..2])?
                .ok_or(FieldParseError::new("Invalid Navaid Type 2".to_string()))?,
            NDBRangePower::from_bytes(&bytes[2..3])?
                .ok_or(FieldParseError::new("Invalid Range Power".to_string()))?,
            NDBAdditionalInfo::from_bytes(&bytes[3..4])?
                .ok_or(FieldParseError::new("Invalid Additional Info".to_string()))?,
            NDBCollocation::from_bytes(&bytes[4..5])?
                .ok_or(FieldParseError::new("Invalid Collocation".to_string()))?,
        )))
    }
}

// Localizer Marker/Locator Navaid Class
// we reuse all of the NDB fields except for Collocation

#[derive(Debug, PartialEq, Eq)]
pub enum MarkerCollocation {
    BFOOperation,
    LocatorMarkerCollocated,
    LocatorMarkerNotCollocated,
}
impl ParseableField for MarkerCollocation {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"B" => MarkerCollocation::BFOOperation,
            b"A" => MarkerCollocation::LocatorMarkerCollocated,
            b"N" => MarkerCollocation::LocatorMarkerNotCollocated,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Marker Collocation".to_string(),
                ));
            }
        }))
    }
}

/// 5.35(C) Localizer Marker/Locator Navaid Class
#[derive(Debug, PartialEq, Eq)]
pub struct MarkerLocatorNavaidClass(
    NDBNavaidType1,
    NDBNavaidType2,
    NDBRangePower,
    NDBAdditionalInfo,
    MarkerCollocation,
);
impl ParseableField for MarkerLocatorNavaidClass {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(MarkerLocatorNavaidClass(
            NDBNavaidType1::from_bytes(&bytes[0..1])?
                .ok_or(FieldParseError::new("Invalid Navaid Type 1".to_string()))?,
            NDBNavaidType2::from_bytes(&bytes[1..2])?
                .ok_or(FieldParseError::new("Invalid Navaid Type 2".to_string()))?,
            NDBRangePower::from_bytes(&bytes[2..3])?
                .ok_or(FieldParseError::new("Invalid Range Power".to_string()))?,
            NDBAdditionalInfo::from_bytes(&bytes[3..4])?
                .ok_or(FieldParseError::new("Invalid Additional Info".to_string()))?,
            MarkerCollocation::from_bytes(&bytes[4..5])?
                .ok_or(FieldParseError::new("Invalid Collocation".to_string()))?,
        )))
    }
}

// Waypoint Type

#[derive(Debug, PartialEq, Eq)]
pub enum PrimaryWaypointType {
    ArcCenterFix,
    CombinedNamedIntersectionAndRNAV,
    UnnamedChartedIntersection,
    MiddleMarkerAsWaypoint,
    NDBAsWaypoint,
    OuterMarkerAsWaypoint,
    NamedIntersection,
    UnchartedAirwayIntersection,
    VFRWaypoint,
    RNAVWaypoint,
    NotApplicable,
}
impl ParseableField for PrimaryWaypointType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(Some(PrimaryWaypointType::NotApplicable));
        }
        Ok(Some(match bytes {
            b"A" => PrimaryWaypointType::ArcCenterFix,
            b"C" => PrimaryWaypointType::CombinedNamedIntersectionAndRNAV,
            b"I" => PrimaryWaypointType::UnnamedChartedIntersection,
            b"M" => PrimaryWaypointType::MiddleMarkerAsWaypoint,
            b"N" => PrimaryWaypointType::NDBAsWaypoint,
            b"O" => PrimaryWaypointType::OuterMarkerAsWaypoint,
            b"R" => PrimaryWaypointType::NamedIntersection,
            b"U" => PrimaryWaypointType::UnchartedAirwayIntersection,
            b"V" => PrimaryWaypointType::VFRWaypoint,
            b"W" => PrimaryWaypointType::RNAVWaypoint,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Primary Waypoint Type".to_string(),
                ));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SecondaryWaypointType {
    FinalApproachFix,
    InitialAndFinalApproachFix,
    FinalApproachCourseFix,
    IntermediateApproachFix,
    FAAOffRouteWaypointIntersection,
    OffRouteWaypointIntersection,
    InitialApproachFix,
    InitialAndFinalApproachCourseFix,
    IntermediateAndFinalApproachCourseFix,
    MissedApproachFix,
    InitialAndMissedApproachFix,
    OceanicGatewayFix,
    FAAPitchAndCatchPoint,
    AACAAOrSUAWaypoint,
    FIRUIRControlledAirspaceFix,
    FullDegreeLatLongFix,
    HalfDegreeLatLongFix,
    NotApplicable,
}
impl ParseableField for SecondaryWaypointType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(Some(SecondaryWaypointType::NotApplicable));
        }
        Ok(Some(match bytes {
            b"A" => SecondaryWaypointType::FinalApproachFix,
            b"B" => SecondaryWaypointType::InitialAndFinalApproachFix,
            b"C" => SecondaryWaypointType::FinalApproachCourseFix,
            b"D" => SecondaryWaypointType::IntermediateApproachFix,
            b"E" => SecondaryWaypointType::FAAOffRouteWaypointIntersection,
            b"F" => SecondaryWaypointType::OffRouteWaypointIntersection,
            b"I" => SecondaryWaypointType::InitialApproachFix,
            b"K" => SecondaryWaypointType::InitialAndFinalApproachCourseFix,
            b"L" => SecondaryWaypointType::IntermediateAndFinalApproachCourseFix,
            b"M" => SecondaryWaypointType::MissedApproachFix,
            b"N" => SecondaryWaypointType::InitialAndMissedApproachFix,
            b"O" => SecondaryWaypointType::OceanicGatewayFix,
            b"P" => SecondaryWaypointType::FAAPitchAndCatchPoint,
            b"S" => SecondaryWaypointType::AACAAOrSUAWaypoint,
            b"U" => SecondaryWaypointType::FIRUIRControlledAirspaceFix,
            b"V" => SecondaryWaypointType::FullDegreeLatLongFix,
            b"W" => SecondaryWaypointType::HalfDegreeLatLongFix,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid Secondary Waypoint Type: '{}'",
                    bytes[0] as char
                )));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PublishedUse {
    PublishedForUseInSID,
    PublishedForUseInSTAR,
    PublishedForUseInApproachProcedure,
    PublishedForUseInMultipleTerminalProcedures,
    NotApplicable,
}

impl ParseableField for PublishedUse {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(Some(PublishedUse::NotApplicable));
        }
        Ok(Some(match bytes {
            b"D" => PublishedUse::PublishedForUseInSID,
            b"E" => PublishedUse::PublishedForUseInSTAR,
            b"F" => PublishedUse::PublishedForUseInApproachProcedure,
            b"Z" => PublishedUse::PublishedForUseInMultipleTerminalProcedures,
            _ => {
                return Err(FieldParseError::new("Invalid Published Use".to_string()));
            }
        }))
    }
}

/// 5.42 Waypoint Type (TYPE)
#[derive(Debug, PartialEq, Eq)]
pub struct WaypointType(PrimaryWaypointType, SecondaryWaypointType, PublishedUse);
impl ParseableField for WaypointType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(WaypointType(
            PrimaryWaypointType::from_bytes(&bytes[0..1])?.ok_or(FieldParseError::new(
                "Invalid Primary Waypoint Type".to_string(),
            ))?,
            SecondaryWaypointType::from_bytes(&bytes[1..2])?.ok_or(FieldParseError::new(
                "Invalid Secondary Waypoint Type".to_string(),
            ))?,
            PublishedUse::from_bytes(&bytes[2..3])?
                .ok_or(FieldParseError::new("Invalid Published Use".to_string()))?,
        )))
    }
}

// Waypoint Usage
#[derive(Debug, PartialEq, Eq)]
pub enum WaypointUsageAltitude {
    HIAndLo,
    Hi,
    Lo,
    TerminalOnly,
}

impl ParseableField for WaypointUsageAltitude {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"B" => WaypointUsageAltitude::HIAndLo,
            b"H" => WaypointUsageAltitude::Hi,
            b"L" => WaypointUsageAltitude::Lo,
            [BLANK] => WaypointUsageAltitude::TerminalOnly,
            _ => {
                return Err(FieldParseError::new("Invalid waypoint usage".to_string()));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum WaypointUsageRNAVIndicator {
    RNAV,
    NonRNAV,
}

impl ParseableField for WaypointUsageRNAVIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"R" => WaypointUsageRNAVIndicator::RNAV,
            [BLANK] => WaypointUsageRNAVIndicator::NonRNAV,
            _ => {
                return Err(FieldParseError::new("Invalid waypoint usage".to_string()));
            }
        }))
    }
}
/// 5.82 Waypoint Usage
#[derive(Debug, PartialEq, Eq)]
pub struct WaypointUsage(WaypointUsageAltitude, WaypointUsageRNAVIndicator);
impl ParseableField for WaypointUsage {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(WaypointUsage(
            WaypointUsageAltitude::from_bytes(&bytes[0..1])?.ok_or(FieldParseError::new(
                "Invalid Waypoint Usage Altitude".to_string(),
            ))?,
            WaypointUsageRNAVIndicator::from_bytes(&bytes[1..2])?.ok_or(FieldParseError::new(
                "Invalid Waypoint Usage RNAV Indicator".to_string(),
            ))?,
        )))
    }
}

// 5.93 Facility Characteristics (FAC CHAR)
// Note: This will be a best guess at layout since the table is unclear in its capture of the logic required

#[derive(Debug, PartialEq, Eq)]
pub enum FacilityCharacteristicsSynchronicity {
    Synchronous,
    Asynchronous,
    Unknown,
}
impl ParseableField for FacilityCharacteristicsSynchronicity {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"S" => FacilityCharacteristicsSynchronicity::Synchronous,
            b"A" => FacilityCharacteristicsSynchronicity::Asynchronous,
            b"U" => FacilityCharacteristicsSynchronicity::Unknown,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid VHFNavaid Facility Characteristics 1".to_string(),
                ));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FacilityCharacteristicsVoiceIdent {
    VoiceIdent,
    NoVoiceIdent,
    Undefined,
    NotApplicable,
}

impl ParseableField for FacilityCharacteristicsVoiceIdent {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => FacilityCharacteristicsVoiceIdent::VoiceIdent,
            b"N" => FacilityCharacteristicsVoiceIdent::NoVoiceIdent,
            b"U" => FacilityCharacteristicsVoiceIdent::Undefined,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid VHFNavaid Facility Characteristics 2".to_string(),
                ));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FacilityCharacteristicsEmissionType {
    UnmodulatedCarrier,
    CarrierKeyed,
    ToneKeyed,
    NotApplicable,
}

impl ParseableField for FacilityCharacteristicsEmissionType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(Some(FacilityCharacteristicsEmissionType::NotApplicable));
        }
        Ok(Some(match bytes {
            b"0" => FacilityCharacteristicsEmissionType::UnmodulatedCarrier,
            b"1" => FacilityCharacteristicsEmissionType::CarrierKeyed,
            b"2" => FacilityCharacteristicsEmissionType::ToneKeyed,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Facility Characteristics Emission Type".to_string(),
                ));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FacilityCharacteristicsFreqAndBackcourse {
    Freq400Hz,
    Freq1020Hz,
    BackcourseUsable,
    BackcourseUnusable,
    BackcourseRestricted,
    BackcourseUndefined,
    MLSHighRateAzimuthGuidance,
    NotApplicable,
}
impl ParseableField for FacilityCharacteristicsFreqAndBackcourse {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(Some(
                FacilityCharacteristicsFreqAndBackcourse::NotApplicable,
            ));
        }
        Ok(Some(match bytes {
            b"4" => FacilityCharacteristicsFreqAndBackcourse::Freq400Hz,
            b"1" => FacilityCharacteristicsFreqAndBackcourse::Freq1020Hz,
            b"Y" => FacilityCharacteristicsFreqAndBackcourse::BackcourseUsable,
            b"N" => FacilityCharacteristicsFreqAndBackcourse::BackcourseUnusable,
            b"R" => FacilityCharacteristicsFreqAndBackcourse::BackcourseRestricted,
            b"U" => FacilityCharacteristicsFreqAndBackcourse::BackcourseUndefined,
            b"H" => FacilityCharacteristicsFreqAndBackcourse::MLSHighRateAzimuthGuidance,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Facility Characteristics Freq And Backcourse".to_string(),
                ));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FacilityCharacteristicsRepetitionAndCollocation {
    CollocatedWithLocalizer,
    CollocatedWithGlideslope,
    CollocatedWithAzimuth,
    CollocatedWithElevation,
    NotCollocatedWithAzOrElev,
    NotCollocated,
    KnownRepetition(u8),
}
impl ParseableField for FacilityCharacteristicsRepetitionAndCollocation {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(Some(
                FacilityCharacteristicsRepetitionAndCollocation::NotCollocated,
            ));
        }
        Ok(Some(match bytes {
            b"L" => FacilityCharacteristicsRepetitionAndCollocation::CollocatedWithLocalizer,
            b"G" => FacilityCharacteristicsRepetitionAndCollocation::CollocatedWithGlideslope,
            b"A" => FacilityCharacteristicsRepetitionAndCollocation::CollocatedWithAzimuth,
            b"E" => FacilityCharacteristicsRepetitionAndCollocation::CollocatedWithElevation,
            b"N" => FacilityCharacteristicsRepetitionAndCollocation::NotCollocatedWithAzOrElev,
            _ => {
                if bytes[0].is_ascii_digit() {
                    return Ok(Some(
                        FacilityCharacteristicsRepetitionAndCollocation::KnownRepetition(
                            bytes[0] - b'0',
                        ),
                    ));
                } else {
                    return Err(FieldParseError::new(
                        "Invalid Facility Characteristics Repetition And Collocation".to_string(),
                    ));
                }
            }
        }))
    }
}

/// 5.93 Facility Characteristics (FAC CHAR)
#[derive(Debug, PartialEq, Eq)]
pub struct FacilityCharacteristics(
    FacilityCharacteristicsSynchronicity,
    FacilityCharacteristicsVoiceIdent,
    FacilityCharacteristicsEmissionType,
    FacilityCharacteristicsFreqAndBackcourse,
    FacilityCharacteristicsRepetitionAndCollocation,
);
impl ParseableField for FacilityCharacteristics {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(FacilityCharacteristics(
            FacilityCharacteristicsSynchronicity::from_bytes(&bytes[0..1])?.ok_or(
                FieldParseError::new("Invalid Facility Characteristics Synchronicity".to_string()),
            )?,
            FacilityCharacteristicsVoiceIdent::from_bytes(&bytes[1..2])?.ok_or(
                FieldParseError::new("Invalid Facility Characteristics Voice Ident".to_string()),
            )?,
            FacilityCharacteristicsEmissionType::from_bytes(&bytes[2..3])?.ok_or(
                FieldParseError::new("Invalid Facility Characteristics Emission Type".to_string()),
            )?,
            FacilityCharacteristicsFreqAndBackcourse::from_bytes(&bytes[3..4])?.ok_or(
                FieldParseError::new(
                    "Invalid Facility Characteristics Freq And Backcourse".to_string(),
                ),
            )?,
            FacilityCharacteristicsRepetitionAndCollocation::from_bytes(&bytes[4..5])?.ok_or(
                FieldParseError::new(
                    "Invalid Facility Characteristics Repetition And Collocation".to_string(),
                ),
            )?,
        )))
    }
}

#[test]
fn test_facility_characteristics() {
    let bytes = b"SY049";
    let facility_characteristics = FacilityCharacteristics::from_bytes(bytes).unwrap().unwrap();
    assert_eq!(
        facility_characteristics.0,
        FacilityCharacteristicsSynchronicity::Synchronous
    );
    assert_eq!(
        facility_characteristics.1,
        FacilityCharacteristicsVoiceIdent::VoiceIdent
    );
    assert_eq!(
        facility_characteristics.2,
        FacilityCharacteristicsEmissionType::UnmodulatedCarrier
    );
    assert_eq!(
        facility_characteristics.3,
        FacilityCharacteristicsFreqAndBackcourse::Freq400Hz
    );
    assert_eq!(
        facility_characteristics.4,
        FacilityCharacteristicsRepetitionAndCollocation::KnownRepetition(9)
    );
}

// Marker Type

#[derive(Debug, PartialEq, Eq)]
pub enum MarkerTypeLocator {
    Locator,
    NotApplicable,
}
impl ParseableField for MarkerTypeLocator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(Some(MarkerTypeLocator::NotApplicable));
        }
        Ok(Some(match bytes {
            b"L" => MarkerTypeLocator::Locator,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Marker Type Locator".to_string(),
                ));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MarkerTypeLocation {
    Inner,
    Middle,
    Outer,
    Back,
    NotApplicable,
}
impl ParseableField for MarkerTypeLocation {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(Some(MarkerTypeLocation::NotApplicable));
        }
        Ok(Some(match bytes {
            b"I" => MarkerTypeLocation::Inner,
            b"M" => MarkerTypeLocation::Middle,
            b"O" => MarkerTypeLocation::Outer,
            b"B" => MarkerTypeLocation::Back,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Marker Type Location".to_string(),
                ));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MarkerTypeMarker {
    Marker,
    NotApplicable,
}
impl ParseableField for MarkerTypeMarker {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(Some(MarkerTypeMarker::NotApplicable));
        }
        Ok(Some(match bytes {
            b"M" => MarkerTypeMarker::Marker,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Marker Type Marker".to_string(),
                ));
            }
        }))
    }
}

/// 5.99 Marker Type
#[derive(Debug, PartialEq, Eq)]
pub struct MarkerType(MarkerTypeLocator, MarkerTypeLocation, MarkerTypeMarker);
impl ParseableField for MarkerType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(MarkerType(
            MarkerTypeLocator::from_bytes(&bytes[0..1])?.ok_or(FieldParseError::new(
                "Invalid Marker Type Locator".to_string(),
            ))?,
            MarkerTypeLocation::from_bytes(&bytes[1..2])?.ok_or(FieldParseError::new(
                "Invalid Marker Type Location".to_string(),
            ))?,
            MarkerTypeMarker::from_bytes(&bytes[2..3])?.ok_or(FieldParseError::new(
                "Invalid Marker Type Marker".to_string(),
            ))?,
        )))
    }
}

// Service Indicator

// Airport Heliport Communications Records
#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportCommunicationsServiceIndicator1 {
    AirportAdvisoryService,
    CommunityAerodromeRadioStation,
    DepartureServiceOtherThanDepartureControl,
    FlightInformationService,
    InitialContact,
    ArrivalServiceOtherThanArrivalControl,
    AerodromeFlightInformationService,
    TerminalAreaControlOtherThanDedicatedTerminalControl,
    NotApplicable,
}

impl ParseableField for AirportHeliportCommunicationsServiceIndicator1 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => AirportHeliportCommunicationsServiceIndicator1::NotApplicable,
            b"A" => AirportHeliportCommunicationsServiceIndicator1::AirportAdvisoryService,
            b"C" => AirportHeliportCommunicationsServiceIndicator1::CommunityAerodromeRadioStation,
            b"D" => AirportHeliportCommunicationsServiceIndicator1::DepartureServiceOtherThanDepartureControl,
            b"F" => AirportHeliportCommunicationsServiceIndicator1::FlightInformationService,
            b"I" => AirportHeliportCommunicationsServiceIndicator1::InitialContact,
            b"L" => AirportHeliportCommunicationsServiceIndicator1::ArrivalServiceOtherThanArrivalControl,
            b"S" => AirportHeliportCommunicationsServiceIndicator1::AerodromeFlightInformationService,
            b"T" => AirportHeliportCommunicationsServiceIndicator1::TerminalAreaControlOtherThanDedicatedTerminalControl,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Airport Heliport Communications Service Indicator 1".to_string(),
                ));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportCommunicationsServiceIndicator2 {
    AerodromeTrafficFrequency,
    CommonTrafficAdvisoryFrequency,
    MandatoryFrequency,
    AirToAirFrequency,
    SecondaryFrequency,
    NotApplicable,
}
impl ParseableField for AirportHeliportCommunicationsServiceIndicator2 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => AirportHeliportCommunicationsServiceIndicator2::NotApplicable,
            b"A" => AirportHeliportCommunicationsServiceIndicator2::AerodromeTrafficFrequency,
            b"C" => AirportHeliportCommunicationsServiceIndicator2::CommonTrafficAdvisoryFrequency,
            b"M" => AirportHeliportCommunicationsServiceIndicator2::MandatoryFrequency,
            b"R" => AirportHeliportCommunicationsServiceIndicator2::AirToAirFrequency,
            b"S" => AirportHeliportCommunicationsServiceIndicator2::SecondaryFrequency,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Airport Heliport Communications Service Indicator 2".to_string(),
                ));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportCommunicationsServiceIndicator3 {
    AirToGround,
    VHFDirectionFindingService,
    RemoteCommunicationsAirToGround,
    LanguageOtherThanEnglish,
    MilitaryUseFrequency,
    PilotControlledLight,
    RemoteCommunicationsOutlet,
    NotApplicable,
}
impl ParseableField for AirportHeliportCommunicationsServiceIndicator3 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => AirportHeliportCommunicationsServiceIndicator3::NotApplicable,
            b"A" => AirportHeliportCommunicationsServiceIndicator3::AirToGround,
            b"D" => AirportHeliportCommunicationsServiceIndicator3::VHFDirectionFindingService,
            b"G" => AirportHeliportCommunicationsServiceIndicator3::RemoteCommunicationsAirToGround,
            b"L" => AirportHeliportCommunicationsServiceIndicator3::LanguageOtherThanEnglish,
            b"M" => AirportHeliportCommunicationsServiceIndicator3::MilitaryUseFrequency,
            b"P" => AirportHeliportCommunicationsServiceIndicator3::PilotControlledLight,
            b"R" => AirportHeliportCommunicationsServiceIndicator3::RemoteCommunicationsOutlet,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Airport Heliport Communications Service Indicator 3".to_string(),
                ));
            }
        }))
    }
}

/// 5.106(A) Airport Heliport Communications Service Indicator
#[derive(Debug, PartialEq, Eq)]
pub struct AirportHeliportCommunicationsServiceIndicator(
    AirportHeliportCommunicationsServiceIndicator1,
    AirportHeliportCommunicationsServiceIndicator2,
    AirportHeliportCommunicationsServiceIndicator3,
);
impl ParseableField for AirportHeliportCommunicationsServiceIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(AirportHeliportCommunicationsServiceIndicator(
            AirportHeliportCommunicationsServiceIndicator1::from_bytes(&bytes[0..1])?.ok_or(
                FieldParseError::new(
                    "Invalid Airport Heliport Communications Service Indicator 1".to_string(),
                ),
            )?,
            AirportHeliportCommunicationsServiceIndicator2::from_bytes(&bytes[1..2])?.ok_or(
                FieldParseError::new(
                    "Invalid Airport Heliport Communications Service Indicator 2".to_string(),
                ),
            )?,
            AirportHeliportCommunicationsServiceIndicator3::from_bytes(&bytes[2..3])?.ok_or(
                FieldParseError::new(
                    "Invalid Airport Heliport Communications Service Indicator 3".to_string(),
                ),
            )?,
        )))
    }
}

// Enroute Communications Records
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteCommunicationsServiceIndicator1 {
    AeronauticalEnrouteInformationService,
    FlightInformationService,
    NotApplicable,
}
impl ParseableField for EnrouteCommunicationsServiceIndicator1 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => EnrouteCommunicationsServiceIndicator1::NotApplicable,
            b"A" => EnrouteCommunicationsServiceIndicator1::AeronauticalEnrouteInformationService,
            b"F" => EnrouteCommunicationsServiceIndicator1::FlightInformationService,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Enroute Communications Service Indicator 1".to_string(),
                ));
            }
        }))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteCommunicationsServiceIndicator2 {
    AirToGround,
    DiscreteFrequency,
    AirToAir,
    MandatoryFrequency,
    SecondaryFrequency,
    NotApplicable,
}
impl ParseableField for EnrouteCommunicationsServiceIndicator2 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => EnrouteCommunicationsServiceIndicator2::NotApplicable,
            b"A" => EnrouteCommunicationsServiceIndicator2::AirToGround,
            b"D" => EnrouteCommunicationsServiceIndicator2::DiscreteFrequency,
            b"R" => EnrouteCommunicationsServiceIndicator2::AirToAir,
            b"M" => EnrouteCommunicationsServiceIndicator2::MandatoryFrequency,
            b"S" => EnrouteCommunicationsServiceIndicator2::SecondaryFrequency,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Enroute Communications Service Indicator 2".to_string(),
                ));
            }
        }))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteCommunicationsServiceIndicator3 {
    VHFDirectionFindingService,
    RemoteCommunicationsAirToGround,
    LanguageOtherThanEnglish,
    MilitaryUseFrequency,
    RemoteCommunicationsOutlet,
    NotApplicable,
}
impl ParseableField for EnrouteCommunicationsServiceIndicator3 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => EnrouteCommunicationsServiceIndicator3::NotApplicable,
            b"D" => EnrouteCommunicationsServiceIndicator3::VHFDirectionFindingService,
            b"G" => EnrouteCommunicationsServiceIndicator3::RemoteCommunicationsAirToGround,
            b"L" => EnrouteCommunicationsServiceIndicator3::LanguageOtherThanEnglish,
            b"M" => EnrouteCommunicationsServiceIndicator3::MilitaryUseFrequency,
            b"R" => EnrouteCommunicationsServiceIndicator3::RemoteCommunicationsOutlet,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Enroute Communications Service Indicator 3".to_string(),
                ));
            }
        }))
    }
}

/// 5.106(B) Enroute Communications Service Indicator
#[derive(Debug, PartialEq, Eq)]
pub struct EnrouteCommunicationsServiceIndicator(
    EnrouteCommunicationsServiceIndicator1,
    EnrouteCommunicationsServiceIndicator2,
    EnrouteCommunicationsServiceIndicator3,
);
impl ParseableField for EnrouteCommunicationsServiceIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(EnrouteCommunicationsServiceIndicator(
            EnrouteCommunicationsServiceIndicator1::from_bytes(&bytes[0..1])?.ok_or(
                FieldParseError::new(
                    "Invalid Enroute Communications Service Indicator 1".to_string(),
                ),
            )?,
            EnrouteCommunicationsServiceIndicator2::from_bytes(&bytes[2..3])?.ok_or(
                FieldParseError::new(
                    "Invalid Enroute Communications Service Indicator 2".to_string(),
                ),
            )?,
            EnrouteCommunicationsServiceIndicator3::from_bytes(&bytes[3..4])?.ok_or(
                FieldParseError::new(
                    "Invalid Enroute Communications Service Indicator 3".to_string(),
                ),
            )?,
        )))
    }
}

// Duplicate Indicator

#[derive(Debug, PartialEq, Eq)]
pub enum DuplicateIndicatorAirspace {
    Undefined,
    HighAltitude,
    LowAltitude,
    SID,
    STAR,
    Approach,
    MissedApproach,
    UndefinedWithOther,
    AllAltitude,
}

impl ParseableField for DuplicateIndicatorAirspace {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"0" => DuplicateIndicatorAirspace::Undefined,
            b"1" => DuplicateIndicatorAirspace::HighAltitude,
            b"2" => DuplicateIndicatorAirspace::LowAltitude,
            b"3" => DuplicateIndicatorAirspace::SID,
            b"4" => DuplicateIndicatorAirspace::STAR,
            b"5" => DuplicateIndicatorAirspace::Approach,
            b"6" => DuplicateIndicatorAirspace::MissedApproach,
            b"7" => DuplicateIndicatorAirspace::UndefinedWithOther,
            b"8" => DuplicateIndicatorAirspace::AllAltitude,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid duplicate indicator airspace".to_string(),
                ));
            }
        }))
    }
}

pub type DuplicateIndicatorDuplicates = u8;

/// 5.114 Duplicate Indicator
#[derive(Debug, PartialEq, Eq)]
pub struct DuplicateIndicator(DuplicateIndicatorAirspace, DuplicateIndicatorDuplicates);
impl ParseableField for DuplicateIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(DuplicateIndicator(
            DuplicateIndicatorAirspace::from_bytes(&bytes[0..1])?.ok_or(FieldParseError::new(
                "Invalid duplicate indicator airspace".to_string(),
            ))?,
            bytes[1] - b'0' as u8,
        )))
    }
}

// Boundary Via

#[derive(Debug, PartialEq, Eq)]
pub enum BoundaryViaPathType {
    Circle,
    GreatCircle,
    RhumbLine,
    CounterclockwiseArc,
    ClockwiseArc,
}
impl BoundaryViaPathType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"C" => BoundaryViaPathType::Circle,
            b"G" => BoundaryViaPathType::GreatCircle,
            b"H" => BoundaryViaPathType::RhumbLine,
            b"L" => BoundaryViaPathType::CounterclockwiseArc,
            b"R" => BoundaryViaPathType::ClockwiseArc,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid boundary via path type".to_string(),
                ));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoundaryViaEndPoint {
    EndOfDescription,
    Other,
}
impl BoundaryViaEndPoint {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"E" => BoundaryViaEndPoint::EndOfDescription,
            [BLANK] => BoundaryViaEndPoint::Other,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid boundary via end point".to_string(),
                ));
            }
        }))
    }
}
/// 5.118 Boundary Via
#[derive(Debug, PartialEq, Eq)]
pub struct BoundaryVia(BoundaryViaPathType, BoundaryViaEndPoint);
impl ParseableField for BoundaryVia {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(BoundaryVia(
            BoundaryViaPathType::from_bytes(&bytes[0..1])?.ok_or(FieldParseError::new(
                "Invalid boundary via path type".to_string(),
            ))?,
            BoundaryViaEndPoint::from_bytes(&bytes[1..2])?.ok_or(FieldParseError::new(
                "Invalid boundary via end point value".to_string(),
            ))?,
        )))
    }
}

// Lower/Upper Limit

#[derive(Debug, PartialEq, Eq)]
pub enum DescriptiveAltitudeLimits {
    NotSpecified,
    Unlimited,
    Ground,
    MeanSeaLevel,
    SeeNOTAM,
}

impl ParseableField for DescriptiveAltitudeLimits {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"NOTSP" => DescriptiveAltitudeLimits::NotSpecified,
            b"UNLTD" => DescriptiveAltitudeLimits::Unlimited,
            b"GND  " => DescriptiveAltitudeLimits::Ground,
            b"MSL  " => DescriptiveAltitudeLimits::MeanSeaLevel,
            b"NOTAM" => DescriptiveAltitudeLimits::SeeNOTAM,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid descriptive altitude limits".to_string(),
                ));
            }
        }))
    }
}

/// 5.121 Lower/Upper Limit
/// Note: maybe belongs in the common numeric field section? unsure
#[derive(Debug, PartialEq, Eq)]
pub enum LowerUpperLimit {
    Descriptive(DescriptiveAltitudeLimits),
    Numeric(AltitudeNumeric),
}
impl ParseableField for LowerUpperLimit {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        // try to parse as descriptive altitude limit first, on error handle as numeric altitude
        let descriptive = DescriptiveAltitudeLimits::from_bytes(bytes);
        if let Ok(Some(descriptive)) = descriptive {
            return Ok(Some(LowerUpperLimit::Descriptive(descriptive)));
        }
        match AltitudeNumeric::from_bytes(bytes) {
            Ok(Some(numeric)) => Ok(Some(LowerUpperLimit::Numeric(numeric))),
            Ok(None) => Ok(None),
            _ => {
                return Err(FieldParseError::new(
                    "Invalid lower/upper limit".to_string(),
                ));
            }
        }
    }
}

#[test]
pub fn test_lower_upper_limit() {
    let r = LowerUpperLimit::from_bytes(b"10000");
    if let Ok(Some(LowerUpperLimit::Numeric(numeric))) = r {
        let val: i32 = numeric.into();
        assert_eq!(val, 10000);
    } else {
        panic!("Failed to parse lower/upper limit");
    }
    let r = LowerUpperLimit::from_bytes(b"NOTSP");
    if let Ok(Some(LowerUpperLimit::Descriptive(descriptive))) = r {
        assert_eq!(descriptive, DescriptiveAltitudeLimits::NotSpecified);
    } else {
        panic!("Failed to parse lower/upper limit");
    }
    let r = LowerUpperLimit::from_bytes(b"FAILS");
    if let Ok(_) = r {
        panic!("Should have failed to parse lower/upper limit");
    }
    let r = LowerUpperLimit::from_bytes(b"     ");
    if let Ok(optional) = r {
        if let Some(_) = optional {
            panic!("Should have parsed None for blank lower/upper limit");
        }
    }
}

/// 5.127 Maximum Altitude
/// Note: see 5.121
#[derive(Debug, PartialEq, Eq)]
pub enum MaximumAltitude {
    Descriptive(DescriptiveAltitudeLimits),
    Numeric(AltitudeNumeric),
}
impl ParseableField for MaximumAltitude {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let descriptive = DescriptiveAltitudeLimits::from_bytes(bytes);
        if let Ok(Some(descriptive)) = descriptive {
            // we only allow unlimited as descriptive altitude limit
            match descriptive {
                DescriptiveAltitudeLimits::Unlimited => {
                    return Ok(Some(MaximumAltitude::Descriptive(descriptive)));
                }
                _ => {
                    return Err(FieldParseError::new("Invalid maximum altitude".to_string()));
                }
            }
        }
        match AltitudeNumeric::from_bytes(bytes) {
            Ok(Some(numeric)) => Ok(Some(MaximumAltitude::Numeric(numeric))),
            Ok(None) => Ok(None),
            _ => {
                return Err(FieldParseError::new(
                    "Invalid lower/upper limit".to_string(),
                ));
            }
        }
    }
}

/// 5.136 Cruise Level From/To
/// Note: see 5.121
#[derive(Debug, PartialEq, Eq)]
pub enum CruiseLevelFromTo {
    Descriptive(DescriptiveAltitudeLimits),
    Numeric(MultiUnitAltitudeNumeric),
}
impl ParseableField for CruiseLevelFromTo {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let descriptive = DescriptiveAltitudeLimits::from_bytes(bytes);
        if let Ok(Some(descriptive)) = descriptive {
            // we only allow unlimited as descriptive altitude limit
            match descriptive {
                DescriptiveAltitudeLimits::Unlimited => {
                    return Ok(Some(CruiseLevelFromTo::Descriptive(descriptive)));
                }
                _ => {
                    return Err(FieldParseError::new("Invalid maximum altitude".to_string()));
                }
            }
        }
        match MultiUnitAltitudeNumeric::from_bytes(bytes) {
            Ok(Some(numeric)) => Ok(Some(CruiseLevelFromTo::Numeric(numeric))),
            Ok(None) => Ok(None),
            _ => {
                return Err(FieldParseError::new(
                    "Invalid cruise level from to".to_string(),
                ));
            }
        }
    }
}

#[test]
pub fn test_cruise_level_from_to() {
    let r = CruiseLevelFromTo::from_bytes(b"M1000");
    if let Ok(Some(CruiseLevelFromTo::Numeric(MultiUnitAltitudeNumeric::Meters(altitude)))) = r {
        let val: i32 = altitude.into();
        assert_eq!(val, 1000);
    } else {
        panic!("Failed to parse cruise level from to");
    }
    let r = CruiseLevelFromTo::from_bytes(b"10000");
    if let Ok(Some(CruiseLevelFromTo::Numeric(MultiUnitAltitudeNumeric::Feet(altitude)))) = r {
        let val: i32 = altitude.into();
        assert_eq!(val, 10000);
    } else {
        panic!("Failed to parse cruise level from to");
    }
}

/// 5.146 MSA Sector Bearing
#[derive(Debug, PartialEq, Eq)]
pub struct SectorBearing {
    pub start_bearing: UintNumeric,
    pub end_bearing: UintNumeric,
}
impl ParseableField for SectorBearing {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let start_bearing = UintNumeric::from_bytes(&bytes[0..3])?
            .ok_or(FieldParseError::new("Invalid sector bearing".to_string()))?;
        let end_bearing = UintNumeric::from_bytes(&bytes[3..6])?
            .ok_or(FieldParseError::new("Invalid sector bearing".to_string()))?;
        Ok(Some(SectorBearing {
            start_bearing,
            end_bearing,
        }))
    }
}

// Pad Dimensions
#[derive(Debug, PartialEq, Eq)]
pub struct RectangularPadDimensions {
    pub width: UintNumeric,
    pub length: UintNumeric,
}
impl ParseableField for RectangularPadDimensions {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let width = UintNumeric::from_bytes(&bytes[0..3])?.ok_or(FieldParseError::new(
            "Invalid rectangular pad dimensions".to_string(),
        ))?;
        let length = UintNumeric::from_bytes(&bytes[3..6])?.ok_or(FieldParseError::new(
            "Invalid rectangular pad dimensions".to_string(),
        ))?;
        Ok(Some(RectangularPadDimensions { width, length }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CircularPadDimensions {
    pub diameter: UintNumeric,
}
impl ParseableField for CircularPadDimensions {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let diameter = UintNumeric::from_bytes(&bytes[0..3])?.ok_or(FieldParseError::new(
            "Invalid circular pad dimensions".to_string(),
        ))?;
        Ok(Some(CircularPadDimensions { diameter }))
    }
}

/// 5.176 Pad Dimensions
///
/// Note: This is a complex type that can be either rectangular or circular. Construction is manual
/// based on the indicator that tells us which it is
#[derive(Debug, PartialEq, Eq)]
pub enum PadDimensions {
    Rectangular(RectangularPadDimensions),
    Circular(CircularPadDimensions),
}

impl ParseableField for PadDimensions {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        // assume if 000 on the last three bytes then it's a circular pad
        if bytes.ends_with(b"000") {
            Ok(Some(PadDimensions::Circular(
                CircularPadDimensions::from_bytes(bytes)?.ok_or(FieldParseError::new(
                    "Invalid circular pad dimensions".to_string(),
                ))?,
            )))
        } else {
            Ok(Some(PadDimensions::Rectangular(
                RectangularPadDimensions::from_bytes(bytes)?.ok_or(FieldParseError::new(
                    "Invalid rectangular pad dimensions".to_string(),
                ))?,
            )))
        }
    }
}

// Timezone (ugh)

#[derive(Debug, PartialEq, Eq)]
pub enum TimezoneZone {
    UTCMinus1,
    UTCMinus2,
    UTCMinus3,
    UTCMinus4,
    UTCMinus5,
    UTCMinus6,
    UTCMinus7,
    UTCMinus8,
    UTCMinus9,
    UTCMinus10,
    UTCMinus11,
    UTCMinus12,
    UTCMinus13,
    UTCMinus14,
    UTC,
    UTCPlus1,
    UTCPlus2,
    UTCPlus3,
    UTCPlus4,
    UTCPlus5,
    UTCPlus6,
    UTCPlus7,
    UTCPlus8,
    UTCPlus9,
    UTCPlus10,
    UTCPlus11,
    UTCPlus12,
}

impl ParseableField for TimezoneZone {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => TimezoneZone::UTCMinus1,
            b"B" => TimezoneZone::UTCMinus2,
            b"C" => TimezoneZone::UTCMinus3,
            b"D" => TimezoneZone::UTCMinus4,
            b"E" => TimezoneZone::UTCMinus5,
            b"F" => TimezoneZone::UTCMinus6,
            b"G" => TimezoneZone::UTCMinus7,
            b"H" => TimezoneZone::UTCMinus8,
            b"I" => TimezoneZone::UTCMinus9,
            b"K" => TimezoneZone::UTCMinus10,
            b"L" => TimezoneZone::UTCMinus11,
            b"M" => TimezoneZone::UTCMinus12,
            b"1" => TimezoneZone::UTCMinus13,
            b"2" => TimezoneZone::UTCMinus14,
            b"Z" => TimezoneZone::UTC,
            b"N" => TimezoneZone::UTCPlus1,
            b"O" => TimezoneZone::UTCPlus2,
            b"P" => TimezoneZone::UTCPlus3,
            b"Q" => TimezoneZone::UTCPlus4,
            b"R" => TimezoneZone::UTCPlus5,
            b"S" => TimezoneZone::UTCPlus6,
            b"T" => TimezoneZone::UTCPlus7,
            b"U" => TimezoneZone::UTCPlus8,
            b"V" => TimezoneZone::UTCPlus9,
            b"W" => TimezoneZone::UTCPlus10,
            b"X" => TimezoneZone::UTCPlus11,
            b"Y" => TimezoneZone::UTCPlus12,
            _ => {
                return Err(FieldParseError::new("Invalid timezone zone".to_string()));
            }
        }))
    }
}

/// 5.178 Timezone
#[derive(Debug, PartialEq, Eq)]
pub struct Timezone {
    pub zone: TimezoneZone,
    pub minutes_offset: Option<UintNumeric>,
}
impl ParseableField for Timezone {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let zone = TimezoneZone::from_bytes(&bytes[0..1])?;
        let minutes_offset = UintNumeric::from_bytes(&bytes[1..3])?;
        if let Some(zone) = zone {
            Ok(Some(Timezone {
                zone: zone,
                minutes_offset: minutes_offset,
            }))
        } else {
            Ok(None)
        }
    }
}

#[test]
pub fn test_timezone() {
    let r = Timezone::from_bytes(b"A30");
    if let Ok(Some(Timezone {
        zone,
        minutes_offset: Some(minutes_offset),
    })) = r
    {
        let minutes_offset: u64 = minutes_offset.into();
        assert_eq!(zone, TimezoneZone::UTCMinus1);
        assert_eq!(minutes_offset, 30);
    } else {
        panic!("Failed to parse timezone");
    }
    let r = Timezone::from_bytes(b"A  ");
    if let Ok(Some(Timezone {
        zone,
        minutes_offset,
    })) = r
    {
        assert_eq!(zone, TimezoneZone::UTCMinus1);
        assert!(minutes_offset.is_none());
    } else {
        panic!("Failed to parse timezone");
    }
}

/// 5.183 Comms Sectorization
pub type CommunicationsSectorization = SectorBearing;

// Name Format Indicator

#[derive(Debug, PartialEq, Eq)]
pub enum NameFormatType1 {
    AbeamFix,
    BearingDistanceFix,
    AirportNameAsFix,
    FIRFix,
    PhoneticLetterNameFix,
    AirportIdentAsFix,
    LatLongFix,
    MultipleWordNameFix,
    NavadIdentAsFix,
    PublishedFiveLetterNameFix,
    PublishedLessThanFiveLetterNameFix,
    PublishedMoreThanFiveLetterNameFix,
    AirportRunwayRelatedFix,
    UIRFix,
    NotApplicable,
}

impl ParseableField for NameFormatType1 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => NameFormatType1::AbeamFix,
            b"B" => NameFormatType1::BearingDistanceFix,
            b"D" => NameFormatType1::AirportNameAsFix,
            b"F" => NameFormatType1::FIRFix,
            b"H" => NameFormatType1::PhoneticLetterNameFix,
            b"I" => NameFormatType1::AirportIdentAsFix,
            b"L" => NameFormatType1::LatLongFix,
            b"M" => NameFormatType1::MultipleWordNameFix,
            b"N" => NameFormatType1::NavadIdentAsFix,
            b"P" => NameFormatType1::PublishedFiveLetterNameFix,
            b"Q" => NameFormatType1::PublishedLessThanFiveLetterNameFix,
            b"R" => NameFormatType1::PublishedMoreThanFiveLetterNameFix,
            b"T" => NameFormatType1::AirportRunwayRelatedFix,
            b"U" => NameFormatType1::UIRFix,
            [BLANK] => NameFormatType1::NotApplicable,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid name format indicator".to_string(),
                ));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NameFormatType2 {
    LocalizerMarkerWithPublishedIdentifier,
    LocalizerMarkerWithoutPublishedIdentifier,
    NotApplicable,
}
impl NameFormatType2 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"O" => NameFormatType2::LocalizerMarkerWithPublishedIdentifier,
            b"M" => NameFormatType2::LocalizerMarkerWithoutPublishedIdentifier,
            [BLANK] => NameFormatType2::NotApplicable,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid name format indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.196 Name Format Indicator
#[derive(Debug, PartialEq, Eq)]
pub struct NameFormat(NameFormatType1, NameFormatType2);
impl ParseableField for NameFormat {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let type1 = NameFormatType1::from_bytes(&bytes[0..1])?.ok_or(FieldParseError::new(
            "Invalid name format indicator".to_string(),
        ))?;
        let type2 = NameFormatType2::from_bytes(&bytes[1..2])?.ok_or(FieldParseError::new(
            "Invalid name format indicator".to_string(),
        ))?;
        Ok(Some(NameFormat(type1, type2)))
    }
}

/// 5.207 Sector From / Sector To
/// Note: This should resolve to degrees, but for now we will keep it as is
#[derive(Debug, PartialEq, Eq)]
pub struct SectorFromTo {
    pub from: char,
    pub to: char,
}
impl ParseableField for SectorFromTo {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        if !(matches!(bytes[0], b'A'..=b'X') && matches!(bytes[1], b'A'..=b'X')) {
            return Err(FieldParseError::new(
                "Invalid sector in 'from' or 'to' position".to_string(),
            ));
        }

        Ok(Some(SectorFromTo {
            from: bytes[0] as char,
            to: bytes[1] as char,
        }))
    }
}

impl Into<[char; 2]> for SectorFromTo {
    fn into(self) -> [char; 2] {
        [self.from, self.to]
    }
}

/// 5.208 Navaid Distance Limitation
#[derive(Debug, PartialEq, Eq)]
pub struct NavaidDistanceLimitation {
    pub first_limit: UintNumeric,
    pub second_limit: UintNumeric,
}

impl ParseableField for NavaidDistanceLimitation {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let first_limit = UintNumeric::from_bytes(&bytes[0..3])?;
        let second_limit = UintNumeric::from_bytes(&bytes[3..6])?;
        if let Some(first_limit) = first_limit
            && let Some(second_limit) = second_limit
        {
            Ok(Some(NavaidDistanceLimitation {
                first_limit: first_limit,
                second_limit: second_limit,
            }))
        } else {
            return Err(FieldParseError::new(
                "Invalid first or second limit".to_string(),
            ));
        }
    }
}

impl Into<[u64; 2]> for NavaidDistanceLimitation {
    fn into(self) -> [u64; 2] {
        [self.first_limit.into(), self.second_limit.into()]
    }
}

/// 5.209 Navaid Altitude Limitation
#[derive(Debug, PartialEq, Eq)]
pub struct NavaidAltitudeLimitation {
    pub first_limit: UintNumeric,
    pub second_limit: UintNumeric,
}

impl ParseableField for NavaidAltitudeLimitation {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let first_limit = UintNumeric::from_bytes(&bytes[0..3])?;
        let second_limit = UintNumeric::from_bytes(&bytes[3..6])?;
        if let Some(first_limit) = first_limit
            && let Some(second_limit) = second_limit
        {
            Ok(Some(NavaidAltitudeLimitation {
                first_limit: first_limit,
                second_limit: second_limit,
            }))
        } else {
            return Err(FieldParseError::new(
                "Invalid first or second limit".to_string(),
            ));
        }
    }
}

impl Into<[u64; 2]> for NavaidAltitudeLimitation {
    fn into(self) -> [u64; 2] {
        [self.first_limit.into(), self.second_limit.into()]
    }
}

// Preferred Route Use
#[derive(Debug, PartialEq, Eq)]
pub enum PreferredRouteType {
    PointToPoint,
    AreaToArea,
}
impl PreferredRouteType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"P" => PreferredRouteType::PointToPoint,
            b"A" => PreferredRouteType::AreaToArea,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid preferred route type".to_string(),
                ));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PreferredRouteRNAVRequirement {
    RNAVRequired,
    RNAVNotRequired,
}

impl PreferredRouteRNAVRequirement {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"R" => PreferredRouteRNAVRequirement::RNAVRequired,
            b"N" => PreferredRouteRNAVRequirement::RNAVNotRequired,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid preferred route RNAV requirement".to_string(),
                ));
            }
        }))
    }
}

/// 5.220 Preferred Route Use Indicator
#[derive(Debug, PartialEq, Eq)]
pub struct PreferredRouteUseIndicator(PreferredRouteType, PreferredRouteRNAVRequirement);
impl ParseableField for PreferredRouteUseIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let route_type = PreferredRouteType::from_bytes(&bytes[0..1])?;
        let rnav_requirement = PreferredRouteRNAVRequirement::from_bytes(&bytes[1..2])?;
        if let Some(route_type) = route_type
            && let Some(rnav_requirement) = rnav_requirement
        {
            Ok(Some(PreferredRouteUseIndicator(
                route_type,
                rnav_requirement,
            )))
        } else {
            return Err(FieldParseError::new(
                "Invalid preferred route type or RNAV requirement".to_string(),
            ));
        }
    }
}

// Aircraft Use Group
#[derive(Debug, PartialEq, Eq)]
pub enum AircraftUseGroup {
    AllAircraft,
    AllAircraftLessThan250Kts,
    NonJetAndTurboprop,
    MultiEnginePropsOnly,
    JetsAndTurboPropsGreaterThan190Kts,
    HelicopterOnly,
    JetPower,
    TurbopropGreaterThan190Kts,
    NonJetNonTurboprop,
    NonJetGreaterThan190Kts,
    NonJetLessThan190Kts,
    AsDefinedInNotes,
    SingleEngine,
    TwinEngine,
}

impl ParseableField for AircraftUseGroup {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => AircraftUseGroup::AllAircraft,
            b"C" => AircraftUseGroup::AllAircraftLessThan250Kts,
            b"D" => AircraftUseGroup::NonJetAndTurboprop,
            b"E" => AircraftUseGroup::MultiEnginePropsOnly,
            b"F" => AircraftUseGroup::JetsAndTurboPropsGreaterThan190Kts,
            b"H" => AircraftUseGroup::HelicopterOnly,
            b"J" => AircraftUseGroup::JetPower,
            b"M" => AircraftUseGroup::TurbopropGreaterThan190Kts,
            b"N" => AircraftUseGroup::NonJetNonTurboprop,
            b"P" => AircraftUseGroup::NonJetGreaterThan190Kts,
            b"Q" => AircraftUseGroup::NonJetLessThan190Kts,
            b"R" => AircraftUseGroup::AsDefinedInNotes,
            b"S" => AircraftUseGroup::SingleEngine,
            b"T" => AircraftUseGroup::TwinEngine,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid aircraft use group".to_string(),
                ));
            }
        }))
    }
}

/// 5.221 Aircraft Use Group Indicator
#[derive(Debug, PartialEq, Eq)]
pub struct AircraftUseGroupIndicator(AircraftUseGroup, Option<AircraftUseGroup>);
impl ParseableField for AircraftUseGroupIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let aircraft_use_primary_group = AircraftUseGroup::from_bytes(&bytes[0..1])?;
        let aircraft_use_alternate_group = AircraftUseGroup::from_bytes(&bytes[1..2])?;
        if let Some(aircraft_use_primary_group) = aircraft_use_primary_group {
            // second group is optional
            Ok(Some(AircraftUseGroupIndicator(
                aircraft_use_primary_group,
                aircraft_use_alternate_group,
            )))
        } else {
            return Err(FieldParseError::new(
                "Invalid aircraft use group indicator".to_string(),
            ));
        }
    }
}

// Number of Engines Restriction
// Side note, this is the least efficient field use I have found yet.. I would like to know more
#[derive(Debug, PartialEq, Eq)]
pub enum NumberOfEnginesRestrictionIndicator {
    Restricted,
    NotRestricted,
}
impl ParseableField for NumberOfEnginesRestrictionIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => NumberOfEnginesRestrictionIndicator::Restricted,
            b"N" => NumberOfEnginesRestrictionIndicator::NotRestricted,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid number of engines restriction indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.232 Number of Engines Restriction
#[derive(Debug, PartialEq, Eq)]
pub struct NumberOfEnginesRestriction(
    NumberOfEnginesRestrictionIndicator,
    NumberOfEnginesRestrictionIndicator,
    NumberOfEnginesRestrictionIndicator,
    NumberOfEnginesRestrictionIndicator,
);
impl ParseableField for NumberOfEnginesRestriction {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        // all must have values
        let indicator1 = NumberOfEnginesRestrictionIndicator::from_bytes(&bytes[0..1])?;
        let indicator2 = NumberOfEnginesRestrictionIndicator::from_bytes(&bytes[1..2])?;
        let indicator3 = NumberOfEnginesRestrictionIndicator::from_bytes(&bytes[2..3])?;
        let indicator4 = NumberOfEnginesRestrictionIndicator::from_bytes(&bytes[3..4])?;
        if let Some(indicator1) = indicator1
            && let Some(indicator2) = indicator2
            && let Some(indicator3) = indicator3
            && let Some(indicator4) = indicator4
        {
            Ok(Some(NumberOfEnginesRestriction(
                indicator1, indicator2, indicator3, indicator4,
            )))
        } else {
            return Err(FieldParseError::new(
                "Invalid number of engines restriction".to_string(),
            ));
        }
    }
}

// Leg Type Code
#[derive(Debug, PartialEq, Eq)]
pub enum LegTypePath {
    PointToPoint,
    CurvedLine,
}

impl ParseableField for LegTypePath {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"S" => LegTypePath::PointToPoint,
            b"C" => LegTypePath::CurvedLine,
            _ => {
                return Err(FieldParseError::new("Invalid leg type path".to_string()));
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LegTypeTurnIndication {
    Left,
    Right,
}

impl ParseableField for LegTypeTurnIndication {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"L" => LegTypeTurnIndication::Left,
            b"R" => LegTypeTurnIndication::Right,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid leg type turn indication".to_string(),
                ));
            }
        }))
    }
}

/// 5.238 Leg Type Code
#[derive(Debug, PartialEq, Eq)]
pub struct LegTypeCode(LegTypePath, Option<LegTypeTurnIndication>);
impl ParseableField for LegTypeCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let leg_type_path = LegTypePath::from_bytes(&bytes[0..1])?;
        let leg_type_turn_indication = LegTypeTurnIndication::from_bytes(&bytes[1..2])?;
        // only the path is required
        if let Some(leg_type_path) = leg_type_path {
            Ok(Some(LegTypeCode(leg_type_path, leg_type_turn_indication)))
        } else {
            return Err(FieldParseError::new("Invalid leg type code".to_string()));
        }
    }
}

// GLS Station Type
#[derive(Debug, PartialEq, Eq)]
pub enum GLSStationType1 {
    LAASOrGLSGroundStation,
    SCAT1Station,
}

impl ParseableField for GLSStationType1 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"L" => GLSStationType1::LAASOrGLSGroundStation,
            b"C" => GLSStationType1::SCAT1Station,
            _ => {
                return Err(FieldParseError::new("Invalid GLS station type".to_string()));
            }
        }))
    }
}

// reserved for future use
#[derive(Debug, PartialEq, Eq)]
pub enum GLSStationType2 {}
// reserved for future use
#[derive(Debug, PartialEq, Eq)]
pub enum GLSStationType3 {}

/// 5.247 GLS Station Type
#[derive(Debug, PartialEq, Eq)]
pub struct GLSStationType(
    GLSStationType1,
    Option<GLSStationType2>,
    Option<GLSStationType3>,
);
impl ParseableField for GLSStationType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let gls_station_type1 = GLSStationType1::from_bytes(&bytes[0..1])?;
        if let Some(gls_station_type1) = gls_station_type1 {
            Ok(Some(GLSStationType(gls_station_type1, None, None)))
        } else {
            return Err(FieldParseError::new("Invalid GLS station type".to_string()));
        }
    }
}

/// 5.274 TAA Sector Radius
#[derive(Debug, PartialEq, Eq)]
pub struct TaaSectorRadius {
    pub start_radius: UintNumeric,
    pub end_radius: UintNumeric,
}
impl ParseableField for TaaSectorRadius {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let start_radius = UintNumeric::from_bytes(&bytes[0..2])?.ok_or(FieldParseError::new(
            "Invalid TAA sector radius".to_string(),
        ))?;
        let end_radius = UintNumeric::from_bytes(&bytes[2..4])?.ok_or(FieldParseError::new(
            "Invalid TAA sector radius".to_string(),
        ))?;
        Ok(Some(TaaSectorRadius {
            start_radius,
            end_radius,
        }))
    }
}
