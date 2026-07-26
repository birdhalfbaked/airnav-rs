//! # ARINC 424 Definitions - Enums
//! This module contains the enums for the ARINC 424 data.
//! Enums are not a formally defined data type in the spec, but are obviously represented as such given
//! the character mapping tables.
//!
//! Example is 5.4 - Section Code which describes the major section of the record.
use crate::types::fields::{BLANK, FieldParseError, ParseableField};

/// 5.2 Record Type Code
#[derive(Debug, PartialEq, Eq)]
pub enum RecordType {
    /// S
    Standard,
    /// T
    Tailored,
}

impl ParseableField for RecordType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"S" => RecordType::Standard,
            b"T" => RecordType::Tailored,
            _ => {
                return Err(FieldParseError::new("Invalid record type".to_string()));
            }
        }))
    }
}

/// 5.3 Customer/Area Code
#[derive(Debug, PartialEq, Eq)]
pub enum CustomerAreaCode {
    /// AFR
    Africa,
    /// CAN
    Canada,
    /// EEU
    EasternEurope,
    /// EUR
    Europe,
    /// LAM
    LatinAmerica,
    /// MES
    MiddleEast,
    /// PAC
    Pacific,
    /// SAM
    SouthAmerica,
    /// SPA
    SouthPacific,
    /// USA
    USA,
    /// (other 3-char code)
    Customer(String),
}

impl ParseableField for CustomerAreaCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.len() != 3 {
            return Err(FieldParseError::new(
                "Customer area code must be 3 characters long".to_string(),
            ));
        }
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"EEU" => CustomerAreaCode::EasternEurope,
            b"EUR" => CustomerAreaCode::Europe,
            b"USA" => CustomerAreaCode::USA,
            b"CAN" => CustomerAreaCode::Canada,
            b"PAC" => CustomerAreaCode::Pacific,
            b"AFR" => CustomerAreaCode::Africa,
            b"LAM" => CustomerAreaCode::LatinAmerica,
            b"MES" => CustomerAreaCode::MiddleEast,
            b"SAM" => CustomerAreaCode::SouthAmerica,
            b"SPA" => CustomerAreaCode::SouthPacific,
            _ => CustomerAreaCode::Customer(
                String::from_utf8(bytes.to_vec())
                    .map_err(|e| FieldParseError::new(e.to_string()))?,
            ),
        }))
    }
}

/// 5.4 Section Code
#[derive(Debug, PartialEq, Eq)]
pub enum Section {
    /// A
    MORA,
    /// D
    Navaid,
    /// E
    Enroute,
    /// H
    Heliport,
    /// P
    Airport,
    /// R
    CompanyRoutes,
    /// T
    Tables,
    /// U
    Airspace,
}

impl ParseableField for Section {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => Section::MORA,
            b"D" => Section::Navaid,
            b"E" => Section::Enroute,
            b"H" => Section::Heliport,
            b"P" => Section::Airport,
            b"R" => Section::CompanyRoutes,
            b"T" => Section::Tables,
            b"U" => Section::Airspace,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid major section code".to_string(),
                ));
            }
        }))
    }
}

/// 5.5 Any Subsection
/// Note: This field must have the section in the first byte and the subsection in
/// the second byte of the slice
#[derive(Debug)]
pub enum CombinedSectionSubsection {
    GridMORA(MORASubsection),
    Navaid(NavaidSubsection),
    Enroute(EnrouteSubsection),
    Heliport(HeliportSubsection),
    Airport(AirportSubsection),
    CompanyRoutes(CompanyRoutesSubsection),
    Tables(TablesSubsection),
    Airspace(AirspaceSubsection),
    Unresolved(char),
}

impl ParseableField for CombinedSectionSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let fallback_code = Self::Unresolved(bytes[1] as char);
        let section = Section::from_bytes(&bytes[0..1])?;
        let subsection_bytes = &bytes[1..2];
        Ok(Some(match section {
            Some(Section::MORA) => {
                if let Ok(Some(subsection)) = MORASubsection::from_bytes(&subsection_bytes) {
                    Self::GridMORA(subsection)
                } else {
                    fallback_code
                }
            }
            Some(Section::Navaid) => {
                if let Ok(Some(subsection)) = NavaidSubsection::from_bytes(&subsection_bytes) {
                    Self::Navaid(subsection)
                } else {
                    fallback_code
                }
            }
            Some(Section::Enroute) => {
                if let Ok(Some(subsection)) = EnrouteSubsection::from_bytes(&subsection_bytes) {
                    Self::Enroute(subsection)
                } else {
                    fallback_code
                }
            }
            Some(Section::Heliport) => {
                if let Ok(Some(subsection)) = HeliportSubsection::from_bytes(&subsection_bytes) {
                    Self::Heliport(subsection)
                } else {
                    fallback_code
                }
            }
            Some(Section::Airport) => {
                if let Ok(Some(subsection)) = AirportSubsection::from_bytes(&subsection_bytes) {
                    Self::Airport(subsection)
                } else {
                    fallback_code
                }
            }
            Some(Section::CompanyRoutes) => {
                if let Ok(Some(subsection)) = CompanyRoutesSubsection::from_bytes(&subsection_bytes)
                {
                    Self::CompanyRoutes(subsection)
                } else {
                    fallback_code
                }
            }
            Some(Section::Tables) => {
                if let Ok(Some(subsection)) = TablesSubsection::from_bytes(&subsection_bytes) {
                    Self::Tables(subsection)
                } else {
                    fallback_code
                }
            }
            Some(Section::Airspace) => {
                if let Ok(Some(subsection)) = AirspaceSubsection::from_bytes(&subsection_bytes) {
                    Self::Airspace(subsection)
                } else {
                    fallback_code
                }
            }
            _ => return Err(FieldParseError::new("Invalid section code".to_string())),
        }))
    }
}

/// 5.5(A) MORA Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum MORASubsection {
    /// S
    GridMORA,
}

impl ParseableField for MORASubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"S" => MORASubsection::GridMORA,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid MORA subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.5(B) Navaid Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum NavaidSubsection {
    /// (blank)
    VHFNavaid,
    /// B
    NDBNavaid,
}

impl ParseableField for NavaidSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => NavaidSubsection::VHFNavaid,
            b"B" => NavaidSubsection::NDBNavaid,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Navaid subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.6(A) Enroute Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteSubsection {
    /// A
    Waypoints,
    /// M
    AirwayMarkers,
    /// P
    HoldingPatterns,
    /// R
    AirwaysAndRoutes,
    /// T
    PreferredRoutes,
    /// U
    AirwayRestrictions,
    /// V
    Communications,
}

impl ParseableField for EnrouteSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => EnrouteSubsection::Waypoints,
            b"M" => EnrouteSubsection::AirwayMarkers,
            b"P" => EnrouteSubsection::HoldingPatterns,
            b"R" => EnrouteSubsection::AirwaysAndRoutes,
            b"T" => EnrouteSubsection::PreferredRoutes,
            b"U" => EnrouteSubsection::AirwayRestrictions,
            b"V" => EnrouteSubsection::Communications,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Enroute subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.6(B) Heliport Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum HeliportSubsection {
    /// A
    Pads,
    /// C
    TerminalWaypoints,
    /// D
    SIDS,
    /// E
    STARS,
    /// F
    ApproachProcedures,
    /// K
    TAA,
    /// S
    MSA,
    /// V
    Communications,
}

impl ParseableField for HeliportSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => HeliportSubsection::Pads,
            b"C" => HeliportSubsection::TerminalWaypoints,
            b"D" => HeliportSubsection::SIDS,
            b"E" => HeliportSubsection::STARS,
            b"F" => HeliportSubsection::ApproachProcedures,
            b"K" => HeliportSubsection::TAA,
            b"S" => HeliportSubsection::MSA,
            b"V" => HeliportSubsection::Communications,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Heliport subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.6(C) Airport Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum AirportSubsection {
    /// A
    ReferencePoints,
    /// B
    Gates,
    /// C
    TerminalWaypoints,
    /// D
    SIDS,
    /// E
    STARS,
    /// F
    ApproachProcedures,
    /// G
    Runways,
    /// I
    LocalizerGlideslope,
    /// K
    TAA,
    /// L
    MLS,
    /// M
    LocalizerMarker,
    /// N
    TerminalNDB,
    /// P
    PathPoint,
    /// R
    FlightPlanningARRDEP,
    /// S
    MSA,
    /// T
    GLSStation,
    /// V
    Communications,
}

impl ParseableField for AirportSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => AirportSubsection::ReferencePoints,
            b"B" => AirportSubsection::Gates,
            b"C" => AirportSubsection::TerminalWaypoints,
            b"D" => AirportSubsection::SIDS,
            b"E" => AirportSubsection::STARS,
            b"F" => AirportSubsection::ApproachProcedures,
            b"G" => AirportSubsection::Runways,
            b"I" => AirportSubsection::LocalizerGlideslope,
            b"K" => AirportSubsection::TAA,
            b"L" => AirportSubsection::MLS,
            b"M" => AirportSubsection::LocalizerMarker,
            b"N" => AirportSubsection::TerminalNDB,
            b"P" => AirportSubsection::PathPoint,
            b"R" => AirportSubsection::FlightPlanningARRDEP,
            b"S" => AirportSubsection::MSA,
            b"T" => AirportSubsection::GLSStation,
            b"V" => AirportSubsection::Communications,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Airport subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.6(D) Company Routes Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum CompanyRoutesSubsection {
    /// (blank)
    CompanyRoutes,
    /// A
    AlternateRecords,
}

impl ParseableField for CompanyRoutesSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => CompanyRoutesSubsection::CompanyRoutes,
            b"A" => CompanyRoutesSubsection::AlternateRecords,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Company Routes subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.6(E) Tables Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum TablesSubsection {
    /// C
    CruisingTables,
    /// G
    GeographicalReference,
    /// N
    RNAVNameTable,
}

impl ParseableField for TablesSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"C" => TablesSubsection::CruisingTables,
            b"G" => TablesSubsection::GeographicalReference,
            b"N" => TablesSubsection::RNAVNameTable,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Tables subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.6(F) Airspace Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum AirspaceSubsection {
    /// C
    ControlledAirspace,
    /// F
    FIRUIR,
    /// R
    RestrictiveAirspace,
}

impl ParseableField for AirspaceSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"C" => AirspaceSubsection::ControlledAirspace,
            b"F" => AirspaceSubsection::FIRUIR,
            b"R" => AirspaceSubsection::RestrictiveAirspace,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Airspace subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.7.1(A) Enroute Airway Route Type
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteAirwayRouteType {
    /// A
    AirlineAirway,
    /// C
    Control,
    /// D
    DirectRoute,
    /// H
    HelicopterAirway,
    /// O
    ConventionalDesignatedAirway,
    /// R
    RNAVAirway,
    /// S
    UndesignatedATSRoute,
}

impl ParseableField for EnrouteAirwayRouteType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => EnrouteAirwayRouteType::AirlineAirway,
            b"C" => EnrouteAirwayRouteType::Control,
            b"D" => EnrouteAirwayRouteType::DirectRoute,
            b"H" => EnrouteAirwayRouteType::HelicopterAirway,
            b"O" => EnrouteAirwayRouteType::ConventionalDesignatedAirway,
            b"R" => EnrouteAirwayRouteType::RNAVAirway,
            b"S" => EnrouteAirwayRouteType::UndesignatedATSRoute,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid enroute airway route type".to_string(),
                ));
            }
        }))
    }
}

/// 5.7.1(B) Preferred Route Route Type
#[derive(Debug, PartialEq, Eq)]
pub enum PreferredRouteRouteType {
    /// C
    NACommonRoute,
    /// D
    PreferentialRoute,
    /// J
    PACOTSRoute,
    /// M
    TACANAustraliaRoute,
    /// N
    NANonCommonRoute,
    /// O
    PreferredOverflightRoute,
    /// P
    PreferredRoute,
    /// S
    TOSRoute,
    /// T
    TECRoute,
}

impl ParseableField for PreferredRouteRouteType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => PreferredRouteRouteType::NACommonRoute,
            b"D" => PreferredRouteRouteType::PreferentialRoute,
            b"J" => PreferredRouteRouteType::PACOTSRoute,
            b"M" => PreferredRouteRouteType::TACANAustraliaRoute,
            b"N" => PreferredRouteRouteType::NANonCommonRoute,
            b"O" => PreferredRouteRouteType::PreferredOverflightRoute,
            b"P" => PreferredRouteRouteType::PreferredRoute,
            b"S" => PreferredRouteRouteType::TOSRoute,
            b"T" => PreferredRouteRouteType::TECRoute,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid preferred route route type".to_string(),
                ));
            }
        }))
    }
}

/// 5.7.1(C) SID Route Type
#[derive(Debug, PartialEq, Eq)]
pub enum SIDRouteType {
    /// 0
    EngineOut,
    /// 1
    RunwayTransition,
    /// 2
    CommonRoute,
    /// 3
    EnrouteTransition,
    /// 4
    RNAVRunwayTransition,
    /// 5
    RNAVCommonRoute,
    /// 6
    RNAVEnrouteTransition,
    /// F
    FMSRunwayTransition,
    /// M
    FMSCommonRoute,
    /// S
    FMSEnrouteTransition,
    /// T
    VectorRunwayTransition,
    /// V
    VectorEnrouteTransition,
}

impl ParseableField for SIDRouteType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => SIDRouteType::EngineOut,
            b"1" => SIDRouteType::RunwayTransition,
            b"2" => SIDRouteType::CommonRoute,
            b"3" => SIDRouteType::EnrouteTransition,
            b"4" => SIDRouteType::RNAVEnrouteTransition,
            b"5" => SIDRouteType::RNAVRunwayTransition,
            b"6" => SIDRouteType::RNAVEnrouteTransition,
            b"F" => SIDRouteType::FMSEnrouteTransition,
            b"M" => SIDRouteType::FMSEnrouteTransition,
            b"S" => SIDRouteType::FMSEnrouteTransition,
            b"T" => SIDRouteType::VectorRunwayTransition,
            b"V" => SIDRouteType::VectorEnrouteTransition,
            _ => {
                return Err(FieldParseError::new("Invalid SID route type".to_string()));
            }
        }))
    }
}

/// 5.7.1(D) STAR Route Type
#[derive(Debug, PartialEq, Eq)]
pub enum STARRouteType {
    /// 1
    EnrouteTransition,
    /// 2
    CommonRoute,
    /// 3
    RunwayTransition,
    /// 4
    RNAVEnrouteTransition,
    /// 5
    RNAVCommonRoute,
    /// 6
    RNAVRunwayTransition,
    /// 7
    ProfileDescentEnrouteTransition,
    /// 8
    ProfileDescentCommonRoute,
    /// 9
    ProfileDescentRunwayTransition,
    /// F
    FMSEnrouteTransition,
    /// M
    FMSCommonRoute,
    /// S
    FMSRunwayTransition,
}

impl ParseableField for STARRouteType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"1" => STARRouteType::EnrouteTransition,
            b"2" => STARRouteType::CommonRoute,
            b"3" => STARRouteType::RunwayTransition,
            b"4" => STARRouteType::RNAVEnrouteTransition,
            b"5" => STARRouteType::RNAVCommonRoute,
            b"6" => STARRouteType::RNAVRunwayTransition,
            b"7" => STARRouteType::ProfileDescentEnrouteTransition,
            b"8" => STARRouteType::ProfileDescentCommonRoute,
            b"9" => STARRouteType::ProfileDescentRunwayTransition,
            b"F" => STARRouteType::FMSEnrouteTransition,
            b"M" => STARRouteType::FMSCommonRoute,
            b"S" => STARRouteType::FMSRunwayTransition,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid STAR route type: {}",
                    bytes[0] as char
                )));
            }
        }))
    }
}

/// 5.7.1(E) Airport Heliport Approach Route Type
#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportApproachRouteType {
    /// A
    ApproachTransition,
    /// B
    LocalizerBackcourseApproach,
    /// D
    VORDMEApproach,
    /// F
    FMSApproach,
    /// G
    IGSApproach,
    /// H
    RNAVRNP,
    /// I
    ILSApproach,
    /// J
    GLSApproach,
    /// L
    LOCApproach,
    /// M
    MLSApproach,
    /// N
    NDBApproach,
    /// P
    GPSApproach,
    /// Q
    NDBDMEApproach,
    /// R
    RNAVApproach,
    /// S
    VORTACApproach,
    /// T
    TACANApproach,
    /// U
    SDFApproach,
    /// V
    VORApproach,
    /// W
    MLSTypeAApproach,
    /// X
    LDAApproach,
    /// Y
    MLSTypeBCApproach,
    /// Z
    MissedApproach,
}

impl ParseableField for AirportHeliportApproachRouteType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => AirportHeliportApproachRouteType::ApproachTransition,
            b"B" => AirportHeliportApproachRouteType::LocalizerBackcourseApproach,
            b"D" => AirportHeliportApproachRouteType::VORDMEApproach,
            b"F" => AirportHeliportApproachRouteType::FMSApproach,
            b"G" => AirportHeliportApproachRouteType::IGSApproach,
            b"H" => AirportHeliportApproachRouteType::RNAVRNP,
            b"I" => AirportHeliportApproachRouteType::ILSApproach,
            b"J" => AirportHeliportApproachRouteType::GLSApproach,
            b"L" => AirportHeliportApproachRouteType::LOCApproach,
            b"M" => AirportHeliportApproachRouteType::MLSApproach,
            b"N" => AirportHeliportApproachRouteType::NDBApproach,
            b"P" => AirportHeliportApproachRouteType::GPSApproach,
            b"Q" => AirportHeliportApproachRouteType::NDBDMEApproach,
            b"R" => AirportHeliportApproachRouteType::RNAVApproach,
            b"S" => AirportHeliportApproachRouteType::VORTACApproach,
            b"T" => AirportHeliportApproachRouteType::TACANApproach,
            b"U" => AirportHeliportApproachRouteType::SDFApproach,
            b"V" => AirportHeliportApproachRouteType::VORApproach,
            b"W" => AirportHeliportApproachRouteType::MLSTypeAApproach,
            b"X" => AirportHeliportApproachRouteType::LDAApproach,
            b"Y" => AirportHeliportApproachRouteType::MLSTypeBCApproach,
            b"Z" => AirportHeliportApproachRouteType::MissedApproach,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid airport heliport approach route type".to_string(),
                ));
            }
        }))
    }
}
/// 5.7.2 Route Type Qualifier 1
#[derive(Debug, PartialEq, Eq)]
pub enum RouteTypeQualifier1 {
    /// A
    ARNPRequired,
    /// D
    DMERequired,
    /// F
    RNAVRNPRequired,
    /// J
    GPSRequiredNoDMEDME,
    /// L
    GBASProcedure,
    /// N
    DMENotRequired,
    /// P
    GNSSRequired,
    /// R
    GPSOrDMEDMERequired,
    /// T
    DMEDMERequired,
    /// U
    GeneralRNAV,
    /// V
    VORDMERNAV,
    /// W
    RNAVWithFAS,
}

impl ParseableField for RouteTypeQualifier1 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => RouteTypeQualifier1::ARNPRequired,
            b"D" => RouteTypeQualifier1::DMERequired,
            b"F" => RouteTypeQualifier1::RNAVRNPRequired,
            b"J" => RouteTypeQualifier1::GPSRequiredNoDMEDME,
            b"L" => RouteTypeQualifier1::GBASProcedure,
            b"N" => RouteTypeQualifier1::DMENotRequired,
            b"P" => RouteTypeQualifier1::GNSSRequired,
            b"R" => RouteTypeQualifier1::GPSOrDMEDMERequired,
            b"T" => RouteTypeQualifier1::DMEDMERequired,
            b"U" => RouteTypeQualifier1::GeneralRNAV,
            b"V" => RouteTypeQualifier1::VORDMERNAV,
            b"W" => RouteTypeQualifier1::RNAVWithFAS,
            _ => {
                return Err(FieldParseError::new(
                    format!("Invalid route qualifier 1: {}", bytes[0] as char).to_string(),
                ));
            }
        }))
    }
}
/// 5.7.3 Route Type Qualifier 2
#[derive(Debug, PartialEq, Eq)]
pub enum RouteTypeQualifier2 {
    /// A
    PrimaryMissedApproach,
    /// B
    SecondaryMissedApproach,
    /// E
    EngineOutMissedApproach,
    /// C
    ProcedureWithoutStraightInMinimums,
    /// S
    ProcedureWithStraightInMinimums,
    /// H
    HelicopterHelipadApproach,
}

impl ParseableField for RouteTypeQualifier2 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => RouteTypeQualifier2::PrimaryMissedApproach,
            b"B" => RouteTypeQualifier2::SecondaryMissedApproach,
            b"E" => RouteTypeQualifier2::EngineOutMissedApproach,
            b"C" => RouteTypeQualifier2::ProcedureWithoutStraightInMinimums,
            b"S" => RouteTypeQualifier2::ProcedureWithStraightInMinimums,
            b"H" => RouteTypeQualifier2::HelicopterHelipadApproach,
            _ => {
                return Err(FieldParseError::new(
                    format!("Invalid route qualifier 2: {}", bytes[0] as char).to_string(),
                ));
            }
        }))
    }
}

/// 5.18 Boundary Code
#[derive(Debug, PartialEq, Eq)]
pub enum BoundaryCode {
    /// U
    USA,
    /// C
    CanadaAlaska,
    /// P
    Pacific,
    /// L
    LatinAmerica,
    /// S
    SouthAmerica,
    /// 1
    SouthPacific,
    /// E
    Europe,
    /// 2
    EasternEurope,
    /// M
    MiddleEastSouthAsia,
    /// A
    Africa,
}

impl ParseableField for BoundaryCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"U" => BoundaryCode::USA,
            b"C" => BoundaryCode::CanadaAlaska,
            b"P" => BoundaryCode::Pacific,
            b"L" => BoundaryCode::LatinAmerica,
            b"S" => BoundaryCode::SouthAmerica,
            b"1" => BoundaryCode::SouthPacific,
            b"E" => BoundaryCode::Europe,
            b"2" => BoundaryCode::EasternEurope,
            b"M" => BoundaryCode::MiddleEastSouthAsia,
            b"A" => BoundaryCode::Africa,
            _ => {
                return Err(FieldParseError::new("Invalid boundary code".to_string()));
            }
        }))
    }
}

/// 5.19 Level
#[derive(Debug, PartialEq, Eq)]
pub enum Level {
    /// B
    AllAltitudes,
    /// H
    HighLevelAirwaysAltitudes,
    /// L
    LowLevelAirwaysAltitudes,
}

impl ParseableField for Level {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"B" => Level::AllAltitudes,
            b"H" => Level::HighLevelAirwaysAltitudes,
            b"L" => Level::LowLevelAirwaysAltitudes,
            _ => {
                return Err(FieldParseError::new("Invalid level".to_string()));
            }
        }))
    }
}

/// 5.20 Turn Direction
#[derive(Debug, PartialEq, Eq)]
pub enum TurnDirection {
    /// L
    Left,
    /// R
    Right,
    /// E
    Either,
}

impl ParseableField for TurnDirection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"L" => TurnDirection::Left,
            b"R" => TurnDirection::Right,
            b"E" => TurnDirection::Either,
            _ => {
                return Err(FieldParseError::new(
                    format!("Invalid turn direction: {}", bytes[0] as char).to_string(),
                ));
            }
        }))
    }
}

/// 5.22 Turn Direction Valid
#[derive(Debug, PartialEq, Eq)]
pub enum TurnDirectionValid {
    /// Y
    Yes,
    /// (blank)
    No,
}

impl ParseableField for TurnDirectionValid {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => TurnDirectionValid::Yes,
            [BLANK] => TurnDirectionValid::No,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid turn direction valid".to_string(),
                ));
            }
        }))
    }
}

/// 5.29 Crossing Altitude Description
#[derive(Debug, PartialEq, Eq)]
pub enum CrossingAltitudeDescription {
    /// \+
    AtOrAbove,
    /// \-
    AtOrBelow,
    /// @
    At,
    /// B
    Between,
    /// C
    AtOrAboveSecondAltitude,
    /// G
    GlideslopeWithAtAltitude,
    /// H
    GlideslopeWithAtOrAboveAltitude,
    /// I
    GlideslopeInterceptWithAtAltitude,
    /// J
    GlideslopeInterceptWithAtOrAboveAltitude,
    AtUntilInbound,
    /// V
    AtOrAboveStepDown,
    /// X
    AtStepDown,
    /// Y
    AtOrBelowStepDown,
}

impl ParseableField for CrossingAltitudeDescription {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"+" => CrossingAltitudeDescription::AtOrAbove,
            b"-" => CrossingAltitudeDescription::AtOrBelow,
            b"@" => CrossingAltitudeDescription::At,
            b"B" => CrossingAltitudeDescription::Between,
            b"C" => CrossingAltitudeDescription::AtOrAboveSecondAltitude,
            b"G" => CrossingAltitudeDescription::GlideslopeWithAtAltitude,
            b"H" => CrossingAltitudeDescription::GlideslopeWithAtOrAboveAltitude,
            b"I" => CrossingAltitudeDescription::GlideslopeInterceptWithAtAltitude,
            b"J" => CrossingAltitudeDescription::GlideslopeInterceptWithAtOrAboveAltitude,
            b"V" => CrossingAltitudeDescription::AtOrAboveStepDown,
            b"X" => CrossingAltitudeDescription::AtStepDown,
            b"Y" => CrossingAltitudeDescription::AtOrBelowStepDown,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid crossing altitude description: {}",
                    bytes[0] as char
                )));
            }
        }))
    }
}

/// 5.49 Localizer Azimuth Position Reference (@, +, -)
#[derive(Debug, PartialEq, Eq)]
pub enum LocalizerAzimuthPositionReference {
    /// @
    AheadOfApproachEnd,
    /// +
    BeyondStopEnd,
    /// -
    Aside,
}

impl ParseableField for LocalizerAzimuthPositionReference {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"@" => LocalizerAzimuthPositionReference::AheadOfApproachEnd,
            b"+" => LocalizerAzimuthPositionReference::BeyondStopEnd,
            b"-" => LocalizerAzimuthPositionReference::Aside,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid localizer azimuth position reference".to_string(),
                ));
            }
        }))
    }
}

/// 5.63 Turn (TURN) for Holding Pattern records (TODO: investigate deduplication with 5.20 Turn Direction)
#[derive(Debug, PartialEq, Eq)]
pub enum Turn {
    /// L
    Left,
    /// R
    Right,
}

impl ParseableField for Turn {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"L" => Turn::Left,
            b"R" => Turn::Right,
            _ => {
                return Err(FieldParseError::new("Invalid turn".to_string()));
            }
        }))
    }
}

/// 5.77(A) Company Route VIA Code
#[derive(Debug, PartialEq, Eq)]
pub enum CompanyRouteVIACode {
    /// ALT
    AlternateAirport,
    /// APP
    ApproachRoute,
    /// AWY
    DesignatedAirway,
    /// DIR
    DirectToFix,
    /// INT
    InitialFix,
    /// PRE
    PreferredRoute,
    /// SID
    SID,
    /// SDE
    SIDEnrouteTransition,
    /// SDY
    SIDRunwayTransition,
    /// STR
    STARProfileDescent,
    /// STE
    STARProfileDescentEnrouteTransition,
    /// STY
    STARProfileDescentRunwayTransition,
}

impl ParseableField for CompanyRouteVIACode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"ALT" => CompanyRouteVIACode::AlternateAirport,
            b"APP" => CompanyRouteVIACode::ApproachRoute,
            b"AWY" => CompanyRouteVIACode::DesignatedAirway,
            b"DIR" => CompanyRouteVIACode::DirectToFix,
            b"INT" => CompanyRouteVIACode::InitialFix,
            b"PRE" => CompanyRouteVIACode::PreferredRoute,
            b"SID" => CompanyRouteVIACode::SID,
            b"SDE" => CompanyRouteVIACode::SIDEnrouteTransition,
            b"SDY" => CompanyRouteVIACode::SIDRunwayTransition,
            b"STR" => CompanyRouteVIACode::STARProfileDescent,
            b"STE" => CompanyRouteVIACode::STARProfileDescentEnrouteTransition,
            b"STY" => CompanyRouteVIACode::STARProfileDescentRunwayTransition,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid company route VIA code".to_string(),
                ));
            }
        }))
    }
}

/// 5.77(B) Preferred Route VIA Code
pub enum PreferredRouteVIACode {
    /// AWY
    DesignatedAirway,
    /// DIR
    DirectToFix,
    /// INT
    InitialFix,
    /// RVF
    RouteViaFix,
    /// RNF
    RouteViaFixNotPermitted,
    /// SID
    SID,
    /// STR
    STARProfileDescent,
}

impl ParseableField for PreferredRouteVIACode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"AWY" => PreferredRouteVIACode::DesignatedAirway,
            b"DIR" => PreferredRouteVIACode::DirectToFix,
            b"INT" => PreferredRouteVIACode::InitialFix,
            b"RVF" => PreferredRouteVIACode::RouteViaFix,
            b"RNF" => PreferredRouteVIACode::RouteViaFixNotPermitted,
            b"SID" => PreferredRouteVIACode::SID,
            b"STR" => PreferredRouteVIACode::STARProfileDescent,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid preferred route VIA code".to_string(),
                ));
            }
        }))
    }
}

/// 5.80 ILS/MLS/GLS Category (CAT)
#[derive(Debug, PartialEq, Eq)]
pub enum IlsMlsGlsCategory {
    /// 0
    LocalizerOnlyNoGS,
    /// 1
    ILSMLSGLSCatI,
    /// 2
    ILSMLSGLSCatII,
    /// 3
    ILSMLSGLSCatIII,
    /// I
    IGSFacility,
    /// L
    LDAWithGS,
    /// A
    LDANoGS,
    /// S
    SDFWithGS,
    /// F
    SDFNoGS,
}

impl ParseableField for IlsMlsGlsCategory {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => IlsMlsGlsCategory::LocalizerOnlyNoGS,
            b"1" => IlsMlsGlsCategory::ILSMLSGLSCatI,
            b"2" => IlsMlsGlsCategory::ILSMLSGLSCatII,
            b"3" => IlsMlsGlsCategory::ILSMLSGLSCatIII,
            b"I" => IlsMlsGlsCategory::IGSFacility,
            b"L" => IlsMlsGlsCategory::LDAWithGS,
            b"A" => IlsMlsGlsCategory::LDANoGS,
            b"S" => IlsMlsGlsCategory::SDFWithGS,
            b"F" => IlsMlsGlsCategory::SDFNoGS,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid ILS/MLS/GLS category".to_string(),
                ));
            }
        }))
    }
}

/// 5.81 ATC Indicator (ATC)
///
/// Note: This is confusing as written in the spec, but I have encoded it such that it is more properly understood.
#[derive(Debug, PartialEq, Eq)]
pub enum AtcIndicator {
    /// A
    ATCAssignmentOptional,
    /// S
    ATCAssignmentRequired,
}

impl ParseableField for AtcIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => AtcIndicator::ATCAssignmentOptional,
            b"S" => AtcIndicator::ATCAssignmentRequired,
            _ => {
                return Err(FieldParseError::new("Invalid ATC indicator".to_string()));
            }
        }))
    }
}

/// 5.91 Continuation Record Application Type (APPL)
#[derive(Debug, PartialEq, Eq)]
pub enum ContinuationRecordApplicationType {
    /// A
    StandardContinuation,
    /// B
    CombinedControllingAgencyFormattedTimeOfOperationsContinuation,
    /// C
    ControllingAgencyContinuation,
    /// E
    PrimaryRecordExtension,
    /// L
    VHFNavaidLimitationContinuation,
    /// N
    SectorNarrativeContinuation,
    /// T
    FormattedTimeOfOperationsContinuation,
    /// U
    NarrativeTimeOfOperationsContinuation,
    /// V
    StartEndDateTimeOfOperationsContinuation,
    /// P
    FlightPlanningContinuation,
    /// Q
    FlightPlanningPrimaryDataContinuation,
    /// S
    SimulationContinuation,
    /// W
    AirportHeliportProcedureDataContinuation,
}

impl ParseableField for ContinuationRecordApplicationType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => ContinuationRecordApplicationType::StandardContinuation,
            b"B" => ContinuationRecordApplicationType::CombinedControllingAgencyFormattedTimeOfOperationsContinuation,
            b"C" => ContinuationRecordApplicationType::ControllingAgencyContinuation,
            b"E" => ContinuationRecordApplicationType::PrimaryRecordExtension,
            b"L" => {
                ContinuationRecordApplicationType::VHFNavaidLimitationContinuation
            }
            b"N" => ContinuationRecordApplicationType::SectorNarrativeContinuation,
            b"T" => ContinuationRecordApplicationType::FormattedTimeOfOperationsContinuation,
            b"U" => ContinuationRecordApplicationType::NarrativeTimeOfOperationsContinuation,
            b"V" => ContinuationRecordApplicationType::StartEndDateTimeOfOperationsContinuation,
            b"P" => ContinuationRecordApplicationType::FlightPlanningContinuation,
            b"Q" => ContinuationRecordApplicationType::FlightPlanningPrimaryDataContinuation,
            b"S" => ContinuationRecordApplicationType::SimulationContinuation,
            b"W" => ContinuationRecordApplicationType::AirportHeliportProcedureDataContinuation,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid continuation record application type".to_string(),
                ));
            }
        }))
    }
}

/// 5.95 Government Source
#[derive(Debug, PartialEq, Eq)]
pub enum GovernmentSource {
    /// Y
    OfficialGovernment,
    /// N
    OtherSource,
}

impl ParseableField for GovernmentSource {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => GovernmentSource::OfficialGovernment,
            b"N" => GovernmentSource::OtherSource,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid government source".to_string(),
                ));
            }
        }))
    }
}

/// 5.98 Elevation Type
#[derive(Debug, PartialEq, Eq)]
pub enum ElevationType {
    /// A
    AirportHeliportElevation,
    /// L
    LandingThresholdElevation,
    /// G
    GovernmentSourced,
}

impl ParseableField for ElevationType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => ElevationType::AirportHeliportElevation,
            b"L" => ElevationType::LandingThresholdElevation,
            b"G" => ElevationType::GovernmentSourced,
            _ => {
                return Err(FieldParseError::new("Invalid elevation type".to_string()));
            }
        }))
    }
}

/// 5.101 Communications Type
/// Note: Technically there are only some valid values depending on Enroute vs Airport/Heliport comms
///     for now we skip this
#[derive(Debug, PartialEq, Eq)]
pub enum CommunicationsType {
    /// ACC
    AreaControlCenter,
    /// ACP
    AirliftCommandPost,
    /// AIR
    AirToAir,
    /// APP
    ApproachControl,
    /// ARR
    ArrivalControl,
    /// ASO
    ASOS,
    /// ATI
    ATIS,
    /// AWI
    AWIB,
    /// AWO
    AWOS,
    /// AWS
    AWIS,
    /// CLD
    ClearanceDelivery,
    /// CPT
    ClearancePreTaxi,
    /// CTA
    TerminalControlArea,
    /// CTL
    Control,
    /// DEP
    DepartureControl,
    /// DIR
    ApproachControlRadarDirector,
    /// EFS
    EFAS,
    /// EMR
    Emergency,
    /// FSS
    FSS,
    /// GCO
    GroundCommOutlet,
    /// GND
    GroundControl,
    /// GTE
    GateControl,
    /// HEL
    HelicopterFrequency,
    /// INF
    Information,
    /// MIL
    MilitaryFrequency,
    /// MUL
    Multicom,
    /// OPS
    Operations,
    /// PAL
    PilotActivatedLighting,
    /// RDO
    Radio,
    /// RDR
    Radar,
    /// RFS
    RFSS,
    /// RMP
    RampControl,
    /// RSA
    ARSA,
    /// TCA
    TCA,
    /// TMA
    TMA,
    /// TML
    Terminal,
    /// TRS
    TRSA,
    /// TWE
    TWEB,
    /// TWR
    TowerControl,
    /// UAC
    UpperAreaControl,
    /// UNI
    Unicom,
    /// VOL
    Volmet,
}

impl ParseableField for CommunicationsType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"ACC" => CommunicationsType::AreaControlCenter,
            b"ACP" => CommunicationsType::AirliftCommandPost,
            b"AIR" => CommunicationsType::AirToAir,
            b"APP" => CommunicationsType::ApproachControl,
            b"ARR" => CommunicationsType::ArrivalControl,
            b"ASO" => CommunicationsType::ASOS,
            b"ATI" => CommunicationsType::ATIS,
            b"AWI" => CommunicationsType::AWIB,
            b"AWO" => CommunicationsType::AWOS,
            b"AWS" => CommunicationsType::AWIS,
            b"CLD" => CommunicationsType::ClearanceDelivery,
            b"CPT" => CommunicationsType::ClearancePreTaxi,
            b"CTA" => CommunicationsType::TerminalControlArea,
            b"CTL" => CommunicationsType::Control,
            b"DEP" => CommunicationsType::DepartureControl,
            b"DIR" => CommunicationsType::ApproachControlRadarDirector,
            b"EFS" => CommunicationsType::EFAS,
            b"EMR" => CommunicationsType::Emergency,
            b"FSS" => CommunicationsType::FSS,
            b"GCO" => CommunicationsType::GroundCommOutlet,
            b"GND" => CommunicationsType::GroundControl,
            b"GTE" => CommunicationsType::GateControl,
            b"HEL" => CommunicationsType::HelicopterFrequency,
            b"INF" => CommunicationsType::Information,
            b"MIL" => CommunicationsType::MilitaryFrequency,
            b"MUL" => CommunicationsType::Multicom,
            b"OPS" => CommunicationsType::Operations,
            b"PAL" => CommunicationsType::PilotActivatedLighting,
            b"RDO" => CommunicationsType::Radio,
            b"RDR" => CommunicationsType::Radar,
            b"RFS" => CommunicationsType::RFSS,
            b"RMP" => CommunicationsType::RampControl,
            b"RSA" => CommunicationsType::ARSA,
            b"TCA" => CommunicationsType::TCA,
            b"TMA" => CommunicationsType::TMA,
            b"TML" => CommunicationsType::Terminal,
            b"TRS" => CommunicationsType::TRSA,
            b"TWE" => CommunicationsType::TWEB,
            b"TWR" => CommunicationsType::TowerControl,
            b"UAC" => CommunicationsType::UpperAreaControl,
            b"UNI" => CommunicationsType::Unicom,
            b"VOL" => CommunicationsType::Volmet,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid communications type".to_string(),
                ));
            }
        }))
    }
}

/// 5.102 Radar
#[derive(Debug, PartialEq, Eq)]
pub enum Radar {
    /// R
    Radar,
    /// N
    NonRadar,
}

impl ParseableField for Radar {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"R" => Radar::Radar,
            b"N" => Radar::NonRadar,
            _ => {
                return Err(FieldParseError::new("Invalid radar".to_string()));
            }
        }))
    }
}

/// 5.104 Frequency Units
#[derive(Debug, PartialEq, Eq)]
pub enum FrequencyUnits {
    /// H
    HF,
    /// V
    VHF,
    /// U
    UHF,
    /// C
    VHF8_33KHzSpacing,
}

impl ParseableField for FrequencyUnits {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"H" => FrequencyUnits::HF,
            b"V" => FrequencyUnits::VHF,
            b"U" => FrequencyUnits::UHF,
            b"C" => FrequencyUnits::VHF8_33KHzSpacing,
            _ => {
                return Err(FieldParseError::new("Invalid frequency units".to_string()));
            }
        }))
    }
}

/// 5.108 IFR Capability
#[derive(Debug, PartialEq, Eq)]
pub enum IfrCapability {
    /// Y
    HasApproach,
    /// N
    NoApproach,
}

impl ParseableField for IfrCapability {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => IfrCapability::HasApproach,
            b"N" => IfrCapability::NoApproach,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid IFR capability: {}",
                    bytes[0] as char
                )));
            }
        }))
    }
}

/// 5.112 Marker Radiation Shape
#[derive(Debug, PartialEq, Eq)]
pub enum MarkerRadiationShape {
    /// B
    Bone,
    /// E
    EllipseOrUnknown,
}

impl ParseableField for MarkerRadiationShape {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"B" => MarkerRadiationShape::Bone,
            b"E" => MarkerRadiationShape::EllipseOrUnknown,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid marker radiation shape".to_string(),
                ));
            }
        }))
    }
}

/// 5.113 High/Low (Enroute Marker)
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteMarkerPower {
    /// H
    High,
    /// L
    Low,
}

impl ParseableField for EnrouteMarkerPower {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"H" => EnrouteMarkerPower::High,
            b"L" => EnrouteMarkerPower::Low,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid enroute marker power".to_string(),
                ));
            }
        }))
    }
}

/// 5.115(A) Enroute Directional Restriction
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteDirectionalRestriction {
    /// F
    ForwardOnly,
    /// B
    BackwardOnly,
    /// (blank)
    NoRestriction,
}

impl ParseableField for EnrouteDirectionalRestriction {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"F" => EnrouteDirectionalRestriction::ForwardOnly,
            b"B" => EnrouteDirectionalRestriction::BackwardOnly,
            [BLANK] => EnrouteDirectionalRestriction::NoRestriction,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid enroute directional restriction".to_string(),
                ));
            }
        }))
    }
}

/// 5.115(B) Preferred Route Directional Restriction
#[derive(Debug, PartialEq, Eq)]
pub enum PreferredRouteDirectionalRestriction {
    /// F
    ForwardOnly,
    /// B
    Bidirectional,
}

impl ParseableField for PreferredRouteDirectionalRestriction {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"F" => PreferredRouteDirectionalRestriction::ForwardOnly,
            b"B" => PreferredRouteDirectionalRestriction::Bidirectional,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid preferred route directional restriction".to_string(),
                ));
            }
        }))
    }
}

/// 5.117 FIR/UIR Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum FirUirIndicator {
    /// F
    FIR,
    /// U
    UIR,
    /// B
    Combined,
}

impl ParseableField for FirUirIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"F" => FirUirIndicator::FIR,
            b"U" => FirUirIndicator::UIR,
            b"B" => FirUirIndicator::Combined,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid FIR/UIR indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.122 FIR/UIR Reporting Units Speed
#[derive(Debug, PartialEq, Eq)]
pub enum FirUirReportingUnitsSpeed {
    /// 0
    NotSpecified,
    /// 1
    Knots,
    /// 2
    Mach,
    /// 3
    KPH,
}

impl ParseableField for FirUirReportingUnitsSpeed {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => FirUirReportingUnitsSpeed::NotSpecified,
            b"1" => FirUirReportingUnitsSpeed::Knots,
            b"2" => FirUirReportingUnitsSpeed::Mach,
            b"3" => FirUirReportingUnitsSpeed::KPH,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid FIR/UIR reporting units speed".to_string(),
                ));
            }
        }))
    }
}

/// 5.123 FIR/UIR Reporting Units Altitude
#[derive(Debug, PartialEq, Eq)]
pub enum FirUirReportingUnitsAltitude {
    /// 0
    NotSpecified,
    /// 1
    FlightLevel,
    /// 2
    Meters,
    /// 3
    Feet,
}

impl ParseableField for FirUirReportingUnitsAltitude {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => FirUirReportingUnitsAltitude::NotSpecified,
            b"1" => FirUirReportingUnitsAltitude::FlightLevel,
            b"2" => FirUirReportingUnitsAltitude::Meters,
            b"3" => FirUirReportingUnitsAltitude::Feet,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid FIR/UIR reporting units altitude".to_string(),
                ));
            }
        }))
    }
}

/// 5.124 FIR/UIR Entry Report
#[derive(Debug, PartialEq, Eq)]
pub enum FirUirEntryReport {
    /// Y
    Required,
    /// N
    NotRequired,
}

impl ParseableField for FirUirEntryReport {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => FirUirEntryReport::Required,
            b"N" => FirUirEntryReport::NotRequired,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid FIR/UIR entry report".to_string(),
                ));
            }
        }))
    }
}

/// 5.128 Restrictive Airspace Type
#[derive(Debug, PartialEq, Eq)]
pub enum RestrictiveAirspaceType {
    /// A
    Alert,
    /// C
    Caution,
    /// D
    Danger,
    /// M
    MilitaryOperationsArea,
    /// N
    NationalSecurityArea,
    /// P
    Prohibited,
    /// R
    Restricted,
    /// T
    Training,
    /// W
    Warning,
    /// U
    SFRAOrNSAOrUnspecified,
}

impl ParseableField for RestrictiveAirspaceType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => RestrictiveAirspaceType::Alert,
            b"C" => RestrictiveAirspaceType::Caution,
            b"D" => RestrictiveAirspaceType::Danger,
            b"M" => RestrictiveAirspaceType::MilitaryOperationsArea,
            b"N" => RestrictiveAirspaceType::NationalSecurityArea,
            b"P" => RestrictiveAirspaceType::Prohibited,
            b"R" => RestrictiveAirspaceType::Restricted,
            b"T" => RestrictiveAirspaceType::Training,
            b"W" => RestrictiveAirspaceType::Warning,
            b"U" => RestrictiveAirspaceType::SFRAOrNSAOrUnspecified,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid restrictive airspace type".to_string(),
                ));
            }
        }))
    }
}

/// 5.131(A).1 Standard Primary Record Time Code
#[derive(Debug, PartialEq, Eq)]
pub enum StandardPrimaryRecordTimeCode {
    /// C
    ActiveContinuouslyIncludingHolidays,
    /// H
    ActiveContinuouslyExcludingHolidays,
    /// N
    ActiveNonContinuously,
    /// (blank)
    ActiveDuringNOTAM,
}

impl ParseableField for StandardPrimaryRecordTimeCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => StandardPrimaryRecordTimeCode::ActiveContinuouslyIncludingHolidays,
            b"H" => StandardPrimaryRecordTimeCode::ActiveContinuouslyExcludingHolidays,
            b"N" => StandardPrimaryRecordTimeCode::ActiveNonContinuously,
            [BLANK] => StandardPrimaryRecordTimeCode::ActiveDuringNOTAM,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid primary record time code".to_string(),
                ));
            }
        }))
    }
}

/// 5.131(A).2 Standard Continuation Record Time Code
#[derive(Debug, PartialEq, Eq)]
pub enum StandardContinuationRecordTimeCode {
    /// H
    ExcludingHolidays,
    /// T
    IncludingHolidays,
    /// N
    NoteForm,
}

impl ParseableField for StandardContinuationRecordTimeCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"H" => StandardContinuationRecordTimeCode::ExcludingHolidays,
            b"T" => StandardContinuationRecordTimeCode::IncludingHolidays,
            b"N" => StandardContinuationRecordTimeCode::NoteForm,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid continuation record time code".to_string(),
                ));
            }
        }))
    }
}

/// 5.131(B) Enroute Airway Restriction Record Time Code
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteAirwayRestrictionTimeCode {
    /// C
    ActiveContinuouslyIncludingHolidays,
    /// H
    ActiveContinuouslyExcludingHolidays,
    /// S
    ActiveNonContinuouslyExcludingHolidays,
    /// T
    ActiveNonContinuouslyIncludingHolidays,
}

impl ParseableField for EnrouteAirwayRestrictionTimeCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => EnrouteAirwayRestrictionTimeCode::ActiveContinuouslyIncludingHolidays,
            b"H" => EnrouteAirwayRestrictionTimeCode::ActiveContinuouslyExcludingHolidays,
            b"S" => EnrouteAirwayRestrictionTimeCode::ActiveNonContinuouslyExcludingHolidays,
            b"T" => EnrouteAirwayRestrictionTimeCode::ActiveNonContinuouslyIncludingHolidays,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid primary record time code".to_string(),
                ));
            }
        }))
    }
}

/// 5.132 NOTAM Flag
#[derive(Debug, PartialEq, Eq)]
pub enum NotamFlag {
    /// N
    ActiveByNotam,
    /// (blank)
    NotActiveByNotam,
}

impl ParseableField for NotamFlag {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"N" => NotamFlag::ActiveByNotam,
            [BLANK] => NotamFlag::NotActiveByNotam,
            _ => {
                return Err(FieldParseError::new("Invalid NOTAM flag".to_string()));
            }
        }))
    }
}

/// 5.133 Unit Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum AirspaceLimitUnitIndicator {
    /// A
    AboveGroundLevel,
    /// M
    MeanSeaLevel,
}

impl ParseableField for AirspaceLimitUnitIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => AirspaceLimitUnitIndicator::AboveGroundLevel,
            b"M" => AirspaceLimitUnitIndicator::MeanSeaLevel,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid airspace limit unit indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.134 Cruise Table Identifier
#[derive(Debug, PartialEq, Eq)]
pub enum CruiseTableIdentifier {
    /// AA
    ICAOCruiseTable,
    /// AO
    ExceptionToICAOCruiseTable,
    /// BB-ZZ (same letter pair, B-Z)
    ModifiedCruiseTable(Box<str>),
    /// BO-ZO (letter + O, B-Z)
    ExceptionToModifiedCruiseTable(Box<str>),
}

impl ParseableField for CruiseTableIdentifier {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let string =
            Box::from(std::str::from_utf8(bytes).map_err(|e| FieldParseError::new(e.to_string()))?);
        Ok(Some(match bytes {
            b"AA" => CruiseTableIdentifier::ICAOCruiseTable,
            b"AO" => CruiseTableIdentifier::ExceptionToICAOCruiseTable,
            [l1, l2] => match (*l1, *l2) {
                (l1, l2) if l1 == l2 && matches!(l1, b'B'..=b'Z') => {
                    CruiseTableIdentifier::ModifiedCruiseTable(string)
                }
                (l, b'O') if matches!(l, b'B'..=b'Z') => {
                    CruiseTableIdentifier::ExceptionToModifiedCruiseTable(string)
                }
                _ => {
                    return Err(FieldParseError::new(
                        "Invalid cruise table identifier".to_string(),
                    ));
                }
            },
            _ => {
                return Err(FieldParseError::new(
                    "Invalid cruise table identifier".to_string(),
                ));
            }
        }))
    }
}

/// 5.138 Time Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum TimeIndicator {
    /// S
    LocalTimeWithDST,
    /// T
    LocalTimeWithoutDST,
    /// (blank)
    UTC,
}

impl ParseableField for TimeIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"T" => TimeIndicator::LocalTimeWithoutDST,
            b"S" => TimeIndicator::LocalTimeWithDST,
            [BLANK] => TimeIndicator::UTC,
            _ => {
                return Err(FieldParseError::new("Invalid time indicator".to_string()));
            }
        }))
    }
}

/// 5.149 Figure of Merit
#[derive(Debug, PartialEq, Eq)]
pub enum FigureOfMerit {
    /// 0
    Terminal,
    /// 1
    LowAltitude,
    /// 2
    HighAltitude,
    /// 3
    ExtendedHighAltitude,
    /// 7
    NavaidNotCivil,
    /// 9
    NavaidOutOfService,
}

impl ParseableField for FigureOfMerit {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => FigureOfMerit::Terminal,
            b"1" => FigureOfMerit::LowAltitude,
            b"2" => FigureOfMerit::HighAltitude,
            b"3" => FigureOfMerit::ExtendedHighAltitude,
            b"7" => FigureOfMerit::NavaidNotCivil,
            b"9" => FigureOfMerit::NavaidOutOfService,
            _ => {
                return Err(FieldParseError::new("Invalid figure of merit".to_string()));
            }
        }))
    }
}

/// 5.152 Start End Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum StartEndIndicator {
    /// C
    ChangeDate,
    /// E
    EndDate,
    /// S
    StartDate,
}

impl ParseableField for StartEndIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => StartEndIndicator::ChangeDate,
            b"E" => StartEndIndicator::EndDate,
            b"S" => StartEndIndicator::StartDate,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid start end indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.160 Units of Altitude for airway restriction
#[derive(Debug, PartialEq, Eq)]
pub enum AirwayRestrictionAltitudeUnit {
    /// F
    HundredsOfFeet,
    /// K
    MetricFlightLevels,
    /// L
    FeetFlightLevels,
    /// M
    TensOfMeters,
}

impl ParseableField for AirwayRestrictionAltitudeUnit {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"F" => AirwayRestrictionAltitudeUnit::HundredsOfFeet,
            b"K" => AirwayRestrictionAltitudeUnit::MetricFlightLevels,
            b"L" => AirwayRestrictionAltitudeUnit::FeetFlightLevels,
            b"M" => AirwayRestrictionAltitudeUnit::TensOfMeters,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid airway restriction altitude unit".to_string(),
                ));
            }
        }))
    }
}

/// 5.162 Step Climb Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum StepClimbIndicator {
    /// B
    StepClimbUpDown,
    /// D
    StepClimbDown,
    /// N
    NoStepClimbPermitted,
    /// U
    StepClimbUp,
}

impl ParseableField for StepClimbIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"B" => StepClimbIndicator::StepClimbUpDown,
            b"D" => StepClimbIndicator::StepClimbDown,
            b"N" => StepClimbIndicator::NoStepClimbPermitted,
            b"U" => StepClimbIndicator::StepClimbUp,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid step climb indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.164 Enroute Airway Restriction Flag
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteAirwayRestrictionFlag {
    /// Y
    Yes,
    /// (blank)
    No,
}

impl ParseableField for EnrouteAirwayRestrictionFlag {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => EnrouteAirwayRestrictionFlag::Yes,
            [BLANK] => EnrouteAirwayRestrictionFlag::No,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid enroute airway restriction flag".to_string(),
                ));
            }
        }))
    }
}

/// 5.165 Magnetic/True Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum MagneticTrueIndicator {
    /// M
    Magnetic,
    /// T
    True,
    /// (blank)
    Both,
}

impl ParseableField for MagneticTrueIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"M" => MagneticTrueIndicator::Magnetic,
            b"T" => MagneticTrueIndicator::True,
            [BLANK] => MagneticTrueIndicator::Both,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid magnetic/true indicator: {}",
                    bytes[0] as char
                )));
            }
        }))
    }
}

/// 5.174 Restrictive Airspace Link Continuation
#[derive(Debug, PartialEq, Eq)]
pub enum RestrictiveAirspaceLinkContinuation {
    /// Y
    Yes,
    /// (blank)
    No,
}

impl ParseableField for RestrictiveAirspaceLinkContinuation {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => RestrictiveAirspaceLinkContinuation::Yes,
            [BLANK] => RestrictiveAirspaceLinkContinuation::No,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid restrictive airspace link continuation flag".to_string(),
                ));
            }
        }))
    }
}

/// 5.177 Public/Military Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum PublicMilitaryIndicator {
    /// C
    Public,
    /// M
    Military,
    /// P
    Private,
}

impl ParseableField for PublicMilitaryIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => PublicMilitaryIndicator::Public,
            b"M" => PublicMilitaryIndicator::Military,
            b"P" => PublicMilitaryIndicator::Private,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid public/military indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.179 Daylight Time Observed Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum DaylightTimeObservedIndicator {
    /// Y
    Yes,
    /// N
    NoOrUnknown,
}

impl ParseableField for DaylightTimeObservedIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => DaylightTimeObservedIndicator::Yes,
            b"N" => DaylightTimeObservedIndicator::NoOrUnknown,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid daylight time observed indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.181 H24 Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum H24Indicator {
    /// Y
    Yes,
    /// N
    No,
}

impl ParseableField for H24Indicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => H24Indicator::Yes,
            b"N" => H24Indicator::No,
            _ => {
                return Err(FieldParseError::new("Invalid H24 indicator".to_string()));
            }
        }))
    }
}

/// 5.182 Guard/Transmit Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum GuardTransmitIndicator {
    /// G
    Guard,
    /// T
    Transmit,
}

impl ParseableField for GuardTransmitIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"G" => GuardTransmitIndicator::Guard,
            b"T" => GuardTransmitIndicator::Transmit,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid guard/transmit indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.187 Distance Description
#[derive(Debug, PartialEq, Eq)]
pub enum DistanceDescription {
    /// -
    AppliedUpToDistance,
    /// +
    AppliedFromDistance,
    /// (blank)
    NotAppliedOrAtSpecifiedDistance,
}

impl ParseableField for DistanceDescription {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"-" => DistanceDescription::AppliedUpToDistance,
            b"+" => DistanceDescription::AppliedFromDistance,
            [BLANK] => DistanceDescription::NotAppliedOrAtSpecifiedDistance,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid distance description".to_string(),
                ));
            }
        }))
    }
}

/// 5.198 Modulation
#[derive(Debug, PartialEq, Eq)]
pub enum Modulation {
    /// A
    AM,
    /// F
    FM,
}

impl ParseableField for Modulation {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => Modulation::AM,
            b"F" => Modulation::FM,
            _ => {
                return Err(FieldParseError::new("Invalid modulation".to_string()));
            }
        }))
    }
}

/// 5.199 Signal Emission
#[derive(Debug, PartialEq, Eq)]
pub enum SignalEmission {
    /// 3
    DoubleSideband,
    /// A
    SingleSidebandReducedCarrier,
    /// B
    DualIndependentSidebands,
    /// H
    SingleSidebandFullCarrier,
    /// J
    SingleSidebandSuppressedCarrier,
    /// L
    LowerSidebandUnknownCarrier,
    /// U
    UpperSidebandUnknownCarrier,
}

impl ParseableField for SignalEmission {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"3" => SignalEmission::DoubleSideband,
            b"A" => SignalEmission::SingleSidebandReducedCarrier,
            b"B" => SignalEmission::DualIndependentSidebands,
            b"H" => SignalEmission::SingleSidebandFullCarrier,
            b"J" => SignalEmission::SingleSidebandSuppressedCarrier,
            b"L" => SignalEmission::LowerSidebandUnknownCarrier,
            b"U" => SignalEmission::UpperSidebandUnknownCarrier,
            _ => {
                return Err(FieldParseError::new("Invalid signal emission".to_string()));
            }
        }))
    }
}

/// 5.201 Restriction Record Type
#[derive(Debug, PartialEq, Eq)]
pub enum RestrictionRecordType {
    /// AE
    AltitudeExclusion,
    /// TC
    CruisingTableReplacement,
    /// SC
    SeasonalRestriction,
    /// NR
    NoteRestriction,
}

impl ParseableField for RestrictionRecordType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"AE" => RestrictionRecordType::AltitudeExclusion,
            b"TC" => RestrictionRecordType::CruisingTableReplacement,
            b"SC" => RestrictionRecordType::SeasonalRestriction,
            b"NR" => RestrictionRecordType::NoteRestriction,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid restriction record type".to_string(),
                ));
            }
        }))
    }
}

/// 5.202 Exclusion Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum AltitudeExclusionIndicator {
    /// A
    AllAltitudesBothDirections,
    /// B
    AllAltitudesBackwardDirection,
    /// F
    AllAltitudesForwardDirection,
    /// (blank)
    NoAltitudeExclusion,
}

impl ParseableField for AltitudeExclusionIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => AltitudeExclusionIndicator::AllAltitudesBothDirections,
            b"B" => AltitudeExclusionIndicator::AllAltitudesBackwardDirection,
            b"F" => AltitudeExclusionIndicator::AllAltitudesForwardDirection,
            [BLANK] => AltitudeExclusionIndicator::NoAltitudeExclusion,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid altitude exclusion indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.203 Block Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum BlockAltitudeIndicator {
    /// B
    BlockAltitude,
    /// I
    IndividualAltitude,
}

impl ParseableField for BlockAltitudeIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"B" => BlockAltitudeIndicator::BlockAltitude,
            b"I" => BlockAltitudeIndicator::IndividualAltitude,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid block altitude indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.205 Navaid Limitation
#[derive(Debug, PartialEq, Eq)]
pub enum NavaidLimitationCode {
    /// C
    Coverage,
    /// F
    Fluctuations,
    /// G
    SignalRoughness,
    /// N
    UnreliableAtLimitation,
    /// R
    RestrictedAtLimitation,
    /// T
    UnusableAtLimitation,
    /// U
    OutOfToleranceAtLimitation,
}

impl ParseableField for NavaidLimitationCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => NavaidLimitationCode::Coverage,
            b"F" => NavaidLimitationCode::Fluctuations,
            b"G" => NavaidLimitationCode::SignalRoughness,
            b"N" => NavaidLimitationCode::UnreliableAtLimitation,
            b"R" => NavaidLimitationCode::RestrictedAtLimitation,
            b"T" => NavaidLimitationCode::UnusableAtLimitation,
            b"U" => NavaidLimitationCode::OutOfToleranceAtLimitation,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid navaid limitation code".to_string(),
                ));
            }
        }))
    }
}

/// 5.206 Component Affected Indicator
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ComponentAffectedIndicator {
    /// A
    TACANAzimuthOnly,
    /// B
    VORDMEAzimuthAndDistance,
    /// D
    VORDMEDistanceOnly,
    /// M
    TACANAzimuthAndDistance,
    /// T
    TACANDistanceOnly,
    /// V
    VORAzimuthOnly,
    /// Z
    VORDMETACANAZimuthAndDistance,
}

impl ParseableField for ComponentAffectedIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => ComponentAffectedIndicator::TACANAzimuthOnly,
            b"B" => ComponentAffectedIndicator::VORDMEAzimuthAndDistance,
            b"D" => ComponentAffectedIndicator::VORDMEDistanceOnly,
            b"M" => ComponentAffectedIndicator::TACANAzimuthAndDistance,
            b"T" => ComponentAffectedIndicator::TACANDistanceOnly,
            b"V" => ComponentAffectedIndicator::VORAzimuthOnly,
            b"Z" => ComponentAffectedIndicator::VORDMETACANAZimuthAndDistance,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid component affected indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.210 Sequence End Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum SequenceEndIndicator {
    /// E
    EndOfSequence,
    /// (blank)
    NotEndOfSequence,
}

impl ParseableField for SequenceEndIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"E" => SequenceEndIndicator::EndOfSequence,
            [BLANK] => SequenceEndIndicator::NotEndOfSequence,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid sequence end indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.213 Controlled Airspace Type
#[derive(Debug, PartialEq, Eq)]
pub enum ControlledAirspaceType {
    /// A, C
    ClassC,
    ControlArea,
    /// M
    TerminalControlArea,
    /// R
    RadarArea,
    /// T
    ClassB,
    /// Z
    ClassD,
}

impl ParseableField for ControlledAirspaceType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => ControlledAirspaceType::ClassC,
            b"C" => ControlledAirspaceType::ClassC,
            b"M" => ControlledAirspaceType::TerminalControlArea,
            b"R" => ControlledAirspaceType::RadarArea,
            b"T" => ControlledAirspaceType::ClassB,
            b"Z" => ControlledAirspaceType::ClassD,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid controlled airspace type".to_string(),
                ));
            }
        }))
    }
}

/// 5.217 Controlled Airspace Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum ControlledAirspaceIndicator {
    /// A
    WithinClassC,
    /// C
    WithinControlArea,
    /// M
    WithinTerminalControlArea,
    /// R
    WithinRadarArea,
    /// T
    WithinClassB,
}

impl ParseableField for ControlledAirspaceIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => ControlledAirspaceIndicator::WithinClassC,
            b"C" => ControlledAirspaceIndicator::WithinControlArea,
            b"M" => ControlledAirspaceIndicator::WithinTerminalControlArea,
            b"R" => ControlledAirspaceIndicator::WithinRadarArea,
            b"T" => ControlledAirspaceIndicator::WithinClassB,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid controlled airspace indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.222 GNSS/FMS Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum GNSSFMSIndicator {
    /// 0
    NotAuthorizedForGNSSOrFMSOverlay,
    /// 1
    GNSSOverlayWithNavaidMonitoring,
    /// 2
    GNSSOverlayWithNavaid,
    /// 3
    GNSSOverlay,
    /// 4
    FMSOverlay,
    /// 5
    FMSAndOrGNSSOverlay,
    /// A
    RNAVSBASAllowed,
    /// B
    RNAVNoSBAS,
    /// C
    RNAVSBASNotSpecified,
    /// P
    StandaloneGNSS,
    /// U
    OverlayAuthorizationNotPublished,
}

impl ParseableField for GNSSFMSIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => GNSSFMSIndicator::NotAuthorizedForGNSSOrFMSOverlay,
            b"1" => GNSSFMSIndicator::GNSSOverlayWithNavaidMonitoring,
            b"2" => GNSSFMSIndicator::GNSSOverlayWithNavaid,
            b"3" => GNSSFMSIndicator::GNSSOverlay,
            b"4" => GNSSFMSIndicator::FMSOverlay,
            b"5" => GNSSFMSIndicator::FMSAndOrGNSSOverlay,
            b"A" => GNSSFMSIndicator::RNAVSBASAllowed,
            b"B" => GNSSFMSIndicator::RNAVNoSBAS,
            b"C" => GNSSFMSIndicator::RNAVSBASNotSpecified,
            b"P" => GNSSFMSIndicator::StandaloneGNSS,
            b"U" => GNSSFMSIndicator::OverlayAuthorizationNotPublished,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid GNSS/FMS indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.223(A) Path Point Operation Type
#[derive(Debug, PartialEq, Eq)]
pub enum SBASOperationType {
    /// 0
    StraightIn,
    /// 1-15
    Reserved,
}

impl ParseableField for SBASOperationType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }

        let numeric_value = u8::from_str_radix(
            std::str::from_utf8(bytes)
                .map_err(|e| FieldParseError::new(format!("Numeric is not valid UTF-8: {}", e)))?,
            10,
        )
        .map_err(|e| FieldParseError::new(format!("Numeric is not a valid u8: {}", e)))?;

        Ok(Some(match numeric_value {
            0 => SBASOperationType::StraightIn,
            1..=15 => SBASOperationType::Reserved,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid SBAS operation type".to_string(),
                ));
            }
        }))
    }
}

/// 5.230 Procedure Type
#[derive(Debug, PartialEq, Eq)]
pub enum ProcedureType {
    /// A
    ArrivalProcedureInDatabase,
    /// B
    ArrivalProcedureNotInDatabase,
    /// C
    DepartureProcedureInDatabase,
    /// D
    DepartureProcedureNotInDatabase,
    /// E
    STARInDatabase,
    /// F
    STARNotInDatabase,
    /// G
    SIDInDatabase,
    /// H
    SIDNotInDatabase,
    /// I
    VectorSIDInDatabase,
    /// J
    VectorSIDNotInDatabase,
    /// K
    ApproachProcedureInDatabase,
    /// L
    ApproachProcedureNotInDatabase,
}

impl ParseableField for ProcedureType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => ProcedureType::ArrivalProcedureInDatabase,
            b"B" => ProcedureType::ArrivalProcedureNotInDatabase,
            b"C" => ProcedureType::DepartureProcedureInDatabase,
            b"D" => ProcedureType::DepartureProcedureNotInDatabase,
            b"E" => ProcedureType::STARInDatabase,
            b"F" => ProcedureType::STARNotInDatabase,
            b"G" => ProcedureType::SIDInDatabase,
            b"H" => ProcedureType::SIDNotInDatabase,
            b"I" => ProcedureType::VectorSIDInDatabase,
            b"J" => ProcedureType::VectorSIDNotInDatabase,
            b"K" => ProcedureType::ApproachProcedureInDatabase,
            b"L" => ProcedureType::ApproachProcedureNotInDatabase,
            _ => {
                return Err(FieldParseError::new("Invalid procedure type".to_string()));
            }
        }))
    }
}

/// 5.233 Turboprop/Jet Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum TurbopropJetIndicator {
    /// A
    AllAircraft,
    /// B
    JetsAndTurboprops,
    /// C
    AllAircraftLessThan250Kts,
    /// D
    NonJetAndTurboprop,
    /// E
    MultiEngineProps,
    /// J
    Jets,
    /// N
    NonJetNonTurboprop,
    /// P
    Turboprops,
}

impl ParseableField for TurbopropJetIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => TurbopropJetIndicator::AllAircraft,
            b"B" => TurbopropJetIndicator::JetsAndTurboprops,
            b"C" => TurbopropJetIndicator::AllAircraftLessThan250Kts,
            b"D" => TurbopropJetIndicator::NonJetAndTurboprop,
            b"E" => TurbopropJetIndicator::MultiEngineProps,
            b"J" => TurbopropJetIndicator::Jets,
            b"N" => TurbopropJetIndicator::NonJetNonTurboprop,
            b"P" => TurbopropJetIndicator::Turboprops,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid turboprop/jet indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.234 RNAV Flag
#[derive(Debug, PartialEq, Eq)]
pub enum RNAVFlag {
    /// Y
    RNAV,
    /// N
    NotRNAV,
}

impl ParseableField for RNAVFlag {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => RNAVFlag::RNAV,
            b"N" => RNAVFlag::NotRNAV,
            _ => {
                return Err(FieldParseError::new("Invalid RNAV flag".to_string()));
            }
        }))
    }
}

/// 5.235 ATC Weight Category
#[derive(Debug, PartialEq, Eq)]
pub enum ATCWeightCategory {
    /// H
    Heavy,
    /// M
    Medium,
    /// L
    Light,
}

impl ParseableField for ATCWeightCategory {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"H" => ATCWeightCategory::Heavy,
            b"M" => ATCWeightCategory::Medium,
            b"L" => ATCWeightCategory::Light,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid ATC weight category".to_string(),
                ));
            }
        }))
    }
}

/// 5.239 Reporting Code
#[derive(Debug, PartialEq, Eq)]
pub enum ReportingCode {
    /// C
    ReportingRequired,
    /// X
    ReportingNotRequired,
}

impl ParseableField for ReportingCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => ReportingCode::ReportingRequired,
            b"X" => ReportingCode::ReportingNotRequired,
            _ => {
                return Err(FieldParseError::new("Invalid reporting code".to_string()));
            }
        }))
    }
}

/// 5.241 Fix Related Transition Code
#[derive(Debug, PartialEq, Eq)]
pub enum FixRelatedTransitionCode {
    /// 1
    SIDRunwayTransition,
    /// 2
    SIDCommonPortion,
    /// 3
    SIDEnrouteTransition,
    /// 4
    STAREnrouteTransition,
    /// 5
    STARCommonPortion,
    /// 6
    STARRunwayTransition,
}

impl ParseableField for FixRelatedTransitionCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"1" => FixRelatedTransitionCode::SIDRunwayTransition,
            b"2" => FixRelatedTransitionCode::SIDCommonPortion,
            b"3" => FixRelatedTransitionCode::SIDEnrouteTransition,
            b"4" => FixRelatedTransitionCode::STAREnrouteTransition,
            b"5" => FixRelatedTransitionCode::STARCommonPortion,
            b"6" => FixRelatedTransitionCode::STARRunwayTransition,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid fix related transition code".to_string(),
                ));
            }
        }))
    }
}

/// 5.242 Procedure Category
#[derive(Debug, PartialEq, Eq)]
pub enum ProcedureCategory {
    /// LAAS
    LAAS,
    /// WAAS
    WAAS,
    /// FMS
    FMS,
    /// GPS
    GPS,
    /// VDME
    VORDMEVORTAC,
    /// CIRC
    CircleToLand,
}

impl ParseableField for ProcedureCategory {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"LAAS" => ProcedureCategory::LAAS,
            b"WAAS" => ProcedureCategory::WAAS,
            b"FMS " => ProcedureCategory::FMS,
            b"GPS " => ProcedureCategory::GPS,
            b"VDME" => ProcedureCategory::VORDMEVORTAC,
            b"CIRC" => ProcedureCategory::CircleToLand,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid procedure category".to_string(),
                ));
            }
        }))
    }
}

/// 5.249 Runway Surface Code
#[derive(Debug, PartialEq, Eq)]
pub enum RunwaySurfaceCode {
    /// H
    HardSurface,
    /// S
    SoftSurface,
    /// W
    WaterRunway,
    /// U
    Undefined,
}

impl ParseableField for RunwaySurfaceCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"H" => RunwaySurfaceCode::HardSurface,
            b"S" => RunwaySurfaceCode::SoftSurface,
            b"W" => RunwaySurfaceCode::WaterRunway,
            b"U" => RunwaySurfaceCode::Undefined,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid runway surface code".to_string(),
                ));
            }
        }))
    }
}

/// 5.250 Alternate Record Type
#[derive(Debug, PartialEq, Eq)]
pub enum AlternateRecordType {
    /// AA
    ArrivalAirport,
    /// DA
    DepartureAirport,
    /// EA
    EndFix,
}

impl ParseableField for AlternateRecordType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"AA" => AlternateRecordType::ArrivalAirport,
            b"DA" => AlternateRecordType::DepartureAirport,
            b"EA" => AlternateRecordType::EndFix,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid alternate record type".to_string(),
                ));
            }
        }))
    }
}

/// 5.252 Alternate Type
#[derive(Debug, PartialEq, Eq)]
pub enum AlternateType {
    /// A
    Airport,
    /// C
    CompanyRoute,
}

impl ParseableField for AlternateType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => AlternateType::Airport,
            b"C" => AlternateType::CompanyRoute,
            _ => {
                return Err(FieldParseError::new("Invalid alternate type".to_string()));
            }
        }))
    }
}

/// 5.255 SBAS Service Provider Identifier
/// Note: This field as of this version is not developed and thus no enumeration even from future revisions should be assigned
#[derive(Debug, PartialEq, Eq)]
pub enum SbasServiceProviderIdentifier {
    /// 0-15
    InDevelopment,
}

impl ParseableField for SbasServiceProviderIdentifier {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let numeric_value: u8 = u8::from_str_radix(
            std::str::from_utf8(bytes).map_err(|e| {
                FieldParseError::new(format!("Invalid SBAS service provider identifier: {}", e))
            })?,
            10,
        )
        .map_err(|e| {
            FieldParseError::new(format!("Invalid SBAS service provider identifier: {}", e))
        })?;
        Ok(Some(match numeric_value {
            0..=15 => SbasServiceProviderIdentifier::InDevelopment,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid SBAS service provider identifier: {}",
                    numeric_value
                )));
            }
        }))
    }
}

/// 5.258 Approach Performance Designator
/// Note: This field as of this version is not developed and thus no enumeration even from future revisions should be assigned
#[derive(Debug, PartialEq, Eq)]
pub enum ApproachPerformanceDesignator {
    /// 0-7
    InDevelopment,
}

impl ParseableField for ApproachPerformanceDesignator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let numeric_value = u8::from_str_radix(
            std::str::from_utf8(bytes).map_err(|e| {
                FieldParseError::new(format!(
                    "Invalid GBAS approach performance designator: {}",
                    e
                ))
            })?,
            10,
        )
        .map_err(|e| {
            FieldParseError::new(format!(
                "Invalid GBAS approach performance designator: {}",
                e
            ))
        })?;
        Ok(Some(match numeric_value {
            0..=7 => ApproachPerformanceDesignator::InDevelopment,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid GBAS approach performance designator".to_string(),
                ));
            }
        }))
    }
}

/// 5.261 Speed Limit Description
#[derive(Debug, PartialEq, Eq)]
pub enum SpeedLimitDescription {
    /// @
    AtSpeed,
    /// +
    AtOrAboveSpeed,
    /// -
    AtOrBelowSpeed,
}

impl ParseableField for SpeedLimitDescription {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"@" => SpeedLimitDescription::AtSpeed,
            b"+" => SpeedLimitDescription::AtOrAboveSpeed,
            b"-" => SpeedLimitDescription::AtOrBelowSpeed,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid speed limit description".to_string(),
                ));
            }
        }))
    }
}

/// 5.266 TCH Units Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum TCHUnitsIndicator {
    /// F
    Feet,
    /// M
    Meters,
}

impl ParseableField for TCHUnitsIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"F" => TCHUnitsIndicator::Feet,
            b"M" => TCHUnitsIndicator::Meters,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid TCH units indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.270 TCH Value Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum TCHValueIndicator {
    /// I
    ElectronicGlideslope,
    /// R
    RNAVProcedureToRunway,
    /// V
    VisualGlideslope,
    /// D
    DefaultValue,
}

impl ParseableField for TCHValueIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"I" => TCHValueIndicator::ElectronicGlideslope,
            b"R" => TCHValueIndicator::RNAVProcedureToRunway,
            b"V" => TCHValueIndicator::VisualGlideslope,
            b"D" => TCHValueIndicator::DefaultValue,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid TCH value indicator: {}",
                    bytes[0] as char
                )));
            }
        }))
    }
}

/// 5.271 Procedure Turn
#[derive(Debug, PartialEq, Eq)]
pub enum ProcedureTurn {
    Required,
    /// NOPT
    NoProcedureTurn,
}

impl ParseableField for ProcedureTurn {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(Some(ProcedureTurn::NoProcedureTurn));
        }
        Ok(Some(match bytes {
            b"NOPT" => ProcedureTurn::NoProcedureTurn,
            _ => {
                return Err(FieldParseError::new("Invalid procedure turn".to_string()));
            }
        }))
    }
}

/// 5.272 TAA Sector Identifier
#[derive(Debug, PartialEq, Eq)]
pub enum TaaSectorIdentifier {
    /// C
    StraightInOrCenterFix,
    /// L
    LeftBaseArea,
    /// T
    RightBaseArea,
}

impl ParseableField for TaaSectorIdentifier {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => TaaSectorIdentifier::StraightInOrCenterFix,
            b"L" => TaaSectorIdentifier::LeftBaseArea,
            b"T" => TaaSectorIdentifier::RightBaseArea,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid TAA sector identifier".to_string(),
                ));
            }
        }))
    }
}

/// 5.276 Level of Service Authorized
#[derive(Debug, PartialEq, Eq)]
pub enum LevelOfServiceAuthorized {
    /// A
    Authorized,
    /// N
    NotAuthorized,
}

impl ParseableField for LevelOfServiceAuthorized {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => LevelOfServiceAuthorized::Authorized,
            b"N" => LevelOfServiceAuthorized::NotAuthorized,
            _ => {
                return Err(FieldParseError::new(
                    format!("Invalid level of service authorized: {}", bytes[0] as char)
                        .to_string(),
                ));
            }
        }))
    }
}
