//! # ARINC 424 Field Parsers
//! This module contains the parsers for the ARINC 424 field types.
//!
//! ## Field Types
//! - Alpha
//! - Alphanumeric
//! - Numeric
//!
//! ## Field Parsers
//! - FieldRaw
//! - FieldParseError
//!
//! ## Raw Fields
//! We define raw field types that do a minimum data validation for first pass data loading.
//! Since Generative LLMs were used to help generate the field names, there may be errors in what the field is supposed to be.
//! To ensure quality, human-verified raw fields are denoted with ✅
//!
//! ### Variants
//!
//! Variants are denoted with (A), (B), (C), (D), etc.
//! They are used ONLY when the length between records is necessarily different.
//! If there are conditionally Numeric AND Alpha fields, it is preferred to use the Alphanumeric field type.
//! and validate on record level later on.

#[derive(Debug, PartialEq, Eq)]
pub struct FieldParseError {
    pub message: String,
}

pub type DType = u8;

const DTYPE_ALPHA: DType = 0;
const DTYPE_ALPHANUMERIC: DType = 1;
const DTYPE_NUMERIC: DType = 2;

#[derive(Debug, PartialEq, Eq)]
pub struct FieldRaw<const DTYPE: DType, const START: usize, const LEN: usize> {
    pub bytes: [u8; LEN],
}
impl<const DTYPE: DType, const START: usize, const LEN: usize> FieldRaw<DTYPE, START, LEN> {
    pub fn new(input: &[u8]) -> Self {
        let mut bytes = [0u8; LEN];
        bytes.copy_from_slice(&input[START..START + LEN]);
        Self { bytes }
    }
}
impl<const START: usize, const LEN: usize> FieldRaw<DTYPE_ALPHA, START, LEN> {
    pub fn as_value(&self) -> Result<&str, FieldParseError> {
        if let Ok(s) = std::str::from_utf8(&self.bytes.trim_ascii_end()) {
            if s.chars().any(|c| {
                !(c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_ascii_whitespace())
                    || c.is_ascii_digit()
            }) {
                return Err(FieldParseError {
                    message: format!("Invalid alpha data: {s}"),
                });
            }
            Ok(s)
        } else {
            Err(FieldParseError {
                message: "Unexpected character encountered".to_string(),
            })
        }
    }
}
impl<const START: usize, const LEN: usize> FieldRaw<DTYPE_ALPHANUMERIC, START, LEN> {
    pub fn as_value(&self) -> Result<&str, FieldParseError> {
        if let Ok(s) = std::str::from_utf8(&self.bytes.trim_ascii_end()) {
            if s.chars().any(|c| {
                !(c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_ascii_whitespace())
            }) {
                return Err(FieldParseError {
                    message: format!("Invalid alphanumeric data: {s}"),
                });
            }
            Ok(s)
        } else {
            Err(FieldParseError {
                message: "Unexpected character encountered".to_string(),
            })
        }
    }
}
impl<const START: usize, const LEN: usize> FieldRaw<DTYPE_NUMERIC, START, LEN> {
    pub fn as_value(&self) -> Result<u64, FieldParseError> {
        // needs to handle left padding with zeros
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            if s.chars().any(|c| !c.is_ascii_digit()) {
                return Err(FieldParseError {
                    message: format!("Invalid numeric data: {s}"),
                });
            }
            s.parse::<u64>().map_err(|e| FieldParseError {
                message: format!("Invalid numeric data: {s}: {e}"),
            })
        } else {
            Err(FieldParseError {
                message: "Unexpected character encountered".to_string(),
            })
        }
    }
}

#[test]
pub fn test_as_alpha() {
    // should trim trailing blanks
    let r: FieldRaw<DTYPE_ALPHA, 0, 3> = FieldRaw::new(&[b'D', b'-', b' ']);
    assert_eq!(r.as_value(), Ok("D-"));
    // should keep leading blanks
    let r: FieldRaw<DTYPE_ALPHA, 0, 3> = FieldRaw::new(&[b' ', b'@', b'D']);
    assert_eq!(r.as_value(), Ok(" @D"));
    // should error on non-alpha characters
    let r: FieldRaw<DTYPE_ALPHA, 0, 3> = FieldRaw::new(&[b'0', b'0', b'S']);
    assert_eq!(
        r.as_value(),
        Err(FieldParseError {
            message: "Invalid alpha data: 00S".to_string(),
        })
    );
}

#[test]
pub fn test_as_alphanumeric() {
    // similar behavior to as_alpha
    let r: FieldRaw<DTYPE_ALPHANUMERIC, 0, 3> = FieldRaw::new(&[b'D', b'-', b' ']);
    assert_eq!(r.as_value(), Ok("D-"));
    let r: FieldRaw<DTYPE_ALPHANUMERIC, 0, 3> = FieldRaw::new(&[b' ', b'@', b'D']);
    assert_eq!(r.as_value(), Ok(" @D"));
    // except now this should be ok
    let r: FieldRaw<DTYPE_ALPHANUMERIC, 0, 3> = FieldRaw::new(&[b'0', b'0', b'S']);
    assert_eq!(r.as_value(), Ok("00S"));
}

#[test]
pub fn test_as_numeric() {
    let r: FieldRaw<DTYPE_NUMERIC, 0, 3> = FieldRaw::new(&[b'0', b'0', b'1']);
    assert_eq!(r.as_value(), Ok(1));
    let r: FieldRaw<DTYPE_NUMERIC, 0, 3> = FieldRaw::new(&[b'0', b'0', b' ']);
    assert!(
        r.as_value()
            .unwrap_err()
            .message
            .contains("Invalid numeric data: 00")
    );
}

// --- ARINC 424 Chapter 5 navigation field raw types (Section 5.0 field definitions) ---
/// 5.2 – Record Type (S/T) ✅
pub type RecordTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.3(A) – Customer/Area Code (CUST/AREA), Area ✅
pub type CustomerAreaCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 3>;
/// 5.4 – Section Code (SEC CODE) ✅
pub type SectionCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.5 – Subsection Code (SUB CODE) ✅
pub type SubsectionCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.6 – Airport/Heliport Identifier (ARPT/HELI IDENT) ✅
pub type AirportHeliportIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.7(A) – Route Type (RT TYPE), Enroute Airway ✅
pub type RouteTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 1>;
/// 5.8(A) – Route Identifier (ROUTE IDENT), Enroute Airway ✅
pub type EnrouteRouteIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 5>;
/// 5.8(B) – Route Identifier (ROUTE IDENT), Preferred Route ✅
pub type PreferredRouteIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 10>;
/// 5.9 – SID/STAR Route Identifier (SID/STAR IDENT) ✅
pub type SidStarRouteIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 6>;
/// 5.10 – Approach Route Identifier (APPROACH IDENT) ✅
pub type ApproachRouteIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 6>;
/// 5.11 – Transition Identifier (TRANS IDENT) ✅
pub type TransitionIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.12(A) – Sequence Number (SEQ NR), 4 characters ✅
pub type SequenceNumber4CharacterRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.12(B) – Sequence Number (SEQ NR), 3 characters ✅
pub type SequenceNumber3CharacterRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.12(C) – Sequence Number (SEQ NR), 2 characters ✅
pub type SequenceNumber2CharacterRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 2>;
/// 5.12(D) – Sequence Number (SEQ NR), 1 character ✅
pub type SequenceNumber1CharacterRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 1>;
/// 5.13 – Fix Identifier (FIX IDENT) ✅
pub type FixIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.14 – ICAO Code (ICAO CODE) ✅
pub type IcaoCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 2>;
/// 5.15 – Inbound Course Theta (holding pattern) ✅
pub type InboundCourseThetaRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 3>;
/// 5.16 – Continuation Record Number (CONT NR) ✅
pub type ContinuationRecordNumberRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 1>;
/// 5.17 – Waypoint Description Code (DESC CODE) ✅
pub type WaypointDescriptionCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 4>;
/// 5.18 – Boundary Code (BDY CODE) ✅
pub type BoundaryCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 1>;
/// 5.19 – Level (LEVEL) ✅
pub type LevelRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.20 – Turn Direction (TURN DIR) ✅
pub type TurnDirectionRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.21 – Path and Termination (PATH TERM) ✅
pub type PathAndTerminationRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 2>;
/// 5.22 – Turn Direction Valid (TDV) ✅
pub type TurnDirectionValidRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.23 – Recommended NAVAID (RECD NAV) ✅
pub type RecommendedNavaidRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.24 – Theta (THETA) ✅
pub type ThetaRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.25 – Rho (RHO) ✅
pub type RhoRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.26 – Outbound Course (OB CRS) ✅
pub type OutboundCourseRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.27(A) – Route Distance From, Holding Distance/Time ✅
pub type RouteDistanceFromHoldingDistanceTimeRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.28 – Inbound Course (IB CRS) ✅
pub type InboundCourseRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.29 – Altitude Description (ALT DESC) ✅
pub type AltitudeDescriptionRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.30 – Altitude / Minimum Altitude ✅
pub type AltitudeMinimumAltitudeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.31 – File Record Number (FRN) ✅
pub type FileRecordNumberRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 5>;
/// 5.32 – Cycle Date (CYCLE) ✅
pub type CycleDateRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.33 – VOR/NDB Identifier (VOR IDENT/NDB IDENT) ✅
pub type VorNdbIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.34 – VOR/NDB Frequency (VOR/NDB FREQ) ✅
pub type VorNdbFrequencyRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 5>;
/// 5.35 – NAVAID Class (CLASS) ✅
pub type NavaidClassRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 5>;
/// 5.36 – Latitude (LATITUDE) ✅
pub type LatitudeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 9>;
/// 5.37 – Longitude (LONGITUDE) ✅
pub type LongitudeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 10>;
/// 5.38 – DME Identifier (DME IDENT) ✅
pub type DmeIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.39 – Magnetic Variation (MAG VAR, D MAG VAR) ✅
pub type MagneticVariationRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.40 – DME Elevation (DME ELEV) ✅
pub type DmeElevationRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.41 – Region Code (REGN CODE) ✅
pub type RegionCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.42 – Waypoint Type (TYPE) ✅
pub type WaypointTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 3>;
/// 5.43 – Waypoint Name/Description (NAME/DESC) ✅
pub type WaypointNameDescriptionRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 25>;
/// 5.44 – Localizer/MLS/GLS Identifier (LOC, MLS, GLS IDENT) ✅
pub type LocalizerMlsGlsIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.45 – Localizer Frequency (FREQ) ✅
pub type LocalizerFrequencyRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 5>;
/// 5.46 – Runway Identifier (RUNWAY ID) ✅
pub type RunwayIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.47 – Localizer Bearing (LOC BRG) ✅
pub type LocalizerBearingRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.48 – Localizer/Azimuth Position (LOC FR RW END / AZ/BAZ FR RW END) ✅
pub type LocalizerAzimuthPositionRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.49 – Localizer/Azimuth Position Reference (@, +, -) ✅
pub type LocalizerAzimuthPositionReferenceRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.50 – Glideslope/Elevation Position (GS FR RW THRES / EL FR RW THRES) ✅
pub type GlideslopeElevationPositionRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.51 – Localizer Width (LOC WIDTH) ✅
pub type LocalizerWidthRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.52 – Glideslope Angle / Minimum Elevation Angle (GS ANGLE / MIN ELEV ANGLE) ✅
pub type GlideslopeAngleRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.53 – Transition Altitude/Level (TRANS ALTITUDE/LEVEL) ✅
pub type TransitionAltitudeLevelRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 5>;
/// 5.54 – Longest Runway (LONGEST RWY) ✅
pub type LongestRunwayRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.55 – Airport/Heliport Elevation (ELEV) ✅
pub type AirportHeliportElevationRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.56 – Gate Identifier (GATE IDENT) ✅
pub type GateIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.57 – Runway Length (RUNWAY LENGTH) ✅
pub type RunwayLengthRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 5>;
/// 5.58 – Runway Bearing (RWY BRG) ✅
pub type RunwayBearingRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.59 – Runway Description (RUNWAY DESCRIPTION) ✅
pub type RunwayDescriptionRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 22>;
/// 5.60 – Name (NAME), Gate and Holding Pattern records ✅
pub type GateHoldingPatternNameRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 25>;
/// 5.61 – Notes, continuation records (NOTES) ✅
pub type ContinuationNotesRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 102>;
/// 5.62 – Inbound Holding Course (IB HOLD CRS) ✅
pub type InboundHoldingCourseRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.63 – Turn (TURN), Holding Pattern records ✅
pub type HoldingPatternTurnRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.64 – Leg Length (LEG LENGTH) ✅
pub type HoldingLegLengthRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.65 – Leg Time (LEG TIME) ✅
pub type HoldingLegTimeRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 2>;
/// 5.66 – Station Declination (STN DEC) ✅
pub type StationDeclinationRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.67 – Threshold Crossing Height (TCH) ✅
pub type ThresholdCrossingHeightRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.68 – Landing Threshold Elevation (LANDING THRES ELEV) ✅
pub type LandingThresholdElevationRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.69 – Threshold Displacement Distance (DSPLCD THR) ✅
pub type ThresholdDisplacementDistanceRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.70 – Vertical Angle (VERT ANGLE) ✅
pub type VerticalAngleRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.71 – Name Field, Navaid/Airport/Heliport/Enroute Marker records ✅
pub type FacilityNameFieldRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 30>;
/// 5.72 – Speed Limit (SPEED LIMIT) ✅
pub type SpeedLimitRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 3>;
/// 5.73 – Speed Limit Altitude ✅
pub type SpeedLimitAltitudeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.74 – Component Elevation (GS ELEV, EL ELEV, AZ ELEV, BAZ ELEV, GLS ELEV) ✅
pub type ComponentElevationRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.75 – From/To Airport/Heliport/Fix ✅
pub type FromToAirportHeliportFixRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.76 – Company Route Ident ✅
pub type CompanyRouteIdentRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 10>;
/// 5.77 – VIA Code ✅
pub type ViaCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 3>;
/// 5.78 – SID/STAR/App/AWY (S/S/A/AWY), SID/STAR/AWY (S/S/AWY) ✅
pub type SidStarApproachAirwayRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 6>;
/// 5.79 – Stopway ✅
pub type StopwayRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.80 – ILS/MLS/GLS Category (CAT) ✅
pub type IlsMlsGlsCategoryRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 1>;
/// 5.81 – ATC Indicator (ATC) ✅
pub type AtcIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.82 – Waypoint Usage ✅
pub type WaypointUsageRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.83 – To Fix, Company Route / Helicopter Operations Company Route (6 characters max) ✅
pub type CompanyRouteToFixRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 6>;
/// 5.83 – To Fix, Preferred Route (5 characters max) ✅
pub type PreferredRouteToFixRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.84 – Runway Transition (RUNWAY TRANS) ✅
pub type RunwayTransitionRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.85 – Enroute Transition (ENRT TRANS) ✅
pub type EnrouteTransitionRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.86 – Cruise Altitude ✅
pub type CruiseAltitudeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.87 – Terminal/Alternate Airport (TERM/ALT ARPT) ✅
pub type TerminalAlternateAirportRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.88 – Alternate Distance (ALT DIST) ✅
pub type AlternateDistanceRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.89 – Cost Index ✅
pub type CostIndexRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.90 – ILS/DME Bias ✅
pub type IlsDmeBiasRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 2>;
/// 5.91 – Continuation Record Application Type (APPL) ✅
pub type ContinuationRecordApplicationTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.92 – Facility Elevation (FAC ELEV) ✅
pub type FacilityElevationRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.93 – Facility Characteristics (FAC CHAR) ✅
pub type FacilityCharacteristicsRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.94 – True Bearing (TRUE BRG) ✅
pub type TrueBearingRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 5>;
/// 5.95 – Government Source (SOURCE) ✅
pub type GovernmentSourceRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.96 – Glideslope Beam Width (GS BEAM WIDTH) ✅
pub type GlideslopeBeamWidthRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.97 – Touchdown Zone Elevation (TDZE) ✅
pub type TouchdownZoneElevationRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.98 – Elevation Type ✅
pub type ElevationTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.99 – Marker Type (MKR TYPE) ✅
pub type MarkerTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 3>;
/// 5.100 – Minor Axis True Bearing (MINOR AXIS TRUE BRG) ✅
pub type MinorAxisTrueBearingRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.101 – Communications Type (COMM TYPE) ✅
pub type CommunicationsTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 3>;
/// 5.102 – Radar ✅
pub type RadarRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.103 – Communications Frequency ✅
pub type CommunicationsFrequencyRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 7>;
/// 5.104 – Frequency Units ✅
pub type FrequencyUnitsRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.105 – Call Sign ✅
pub type CallSignRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 25>;
/// 5.106 – Service Indicator (SERV IND) ✅
pub type ServiceIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 3>;
/// 5.107 – ATA/IATA Designator ✅
pub type AtaIataDesignatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 3>;
/// 5.108 – IFR Capability ✅
pub type IfrCapabilityRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.109 – Runway Width ✅
pub type RunwayWidthRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.110 – Marker Identifier (Enroute Marker) (IDENT) ✅
pub type EnrouteMarkerIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.111 – Marker Code (Morse); spec lists Alpha, Morse encoding often uses dot/dash ✅
pub type EnrouteMarkerMorseCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.112 – Marker Shape ✅
pub type MarkerShapeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.113 – High/Low (Enroute Marker) ✅
pub type EnrouteMarkerHighLowRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.114 – Duplicate Indicator ✅
pub type DuplicateIndicatorRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 2>;
/// 5.115 – Direction Restriction ✅
pub type DirectionRestrictionRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.116 – FIR/UIR Identifier; spec lists Alpha, examples include digits ✅
pub type FirUirIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.117 – FIR/UIR Indicator (IND) ✅
pub type FirUirIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.118 – Boundary Via ✅
pub type BoundaryViaRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 2>;
/// 5.119 – Arc Distance ✅
pub type ArcDistanceRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.120 – Arc Bearing ✅
pub type ArcBearingRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.121 – Lower/Upper Limit ✅
pub type FirUirLowerUpperLimitRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.122 – FIR/UIR Reporting Units Speed ✅
pub type FirUirRUSRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 1>;
/// 5.123 – FIR/UIR Reporting Units Altitude ✅
pub type FirUirRUARaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 1>;
/// 5.124 – FIR/UIR Entry Report (ENTRY) ✅
pub type FirUirEntryReportRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.125 – FIR/UIR Name ✅
pub type FirUirNameRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 25>;
/// 5.126 – Restrictive Airspace Name ✅
pub type RestrictiveAirspaceNameRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 30>;
/// 5.127 – Maximum Altitude ✅
pub type MaximumAltitudeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.128 – Restrictive Airspace Type ✅
pub type RestrictiveAirspaceTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.129 – Restrictive Airspace Designation ✅
pub type RestrictiveAirspaceDesignationRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 10>;
/// 5.130 – Multiple Code (MULTI CD) ✅
pub type MultipleCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 1>;
/// 5.131 – Time Code (TIME CD) ✅
pub type TimeCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.132 – NOTAM ✅
pub type NotamRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.133 – Unit Indicator (UNIT IND) ✅
pub type UnitIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.134 – Cruise Table Identifier (CRSE TBL IDENT) ✅
pub type CruiseTableIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 2>;
/// 5.135 – Course From/To (Cruise Table) ✅
pub type CruiseTableCourseFromToRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.136 – Cruise Level From/To ✅
pub type CruiseLevelFromToRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.137 – Vertical Separation ✅
pub type VerticalSeparationRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.138 – Time Indicator (TIME IND) ✅
pub type TimeIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.139 – Procedure Name ✅
pub type ProcedureNameRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 78>;
/// 5.140 – Controlling Agency ✅
pub type ControllingAgencyRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 25>;
/// 5.141 – Starting Latitude (Grid MORA) ✅
pub type GridMoraStartingLatitudeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 3>;
/// 5.142 – Starting Longitude (Grid MORA) ✅
pub type GridMoraStartingLongitudeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.143 – Grid MORA ✅
pub type GridMoraRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 3>;
/// 5.144 – Center Fix (CENTER FIX) (5 characters max) ✅
pub type CenterFixRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.145 – Radius Limit (MSA) ✅
pub type MsaRadiusLimitRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 2>;
/// 5.146 – Sector Bearing (SEC BRG) ✅
pub type SectorBearingRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 6>;
/// 5.147 – Sector Altitude (SEC ALT) ✅
pub type SectorAltitudeRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.148 – Enroute Alternate Airport/Heliport (EAA) ✅
pub type EnrouteAlternateAirportHeliportRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.149 – Navaid Usable Range (Figure of Merit) ✅
pub type NavaidUsableRangeRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 1>;
/// 5.150 – Frequency Protection Distance (FREQ PRD) ✅
pub type FrequencyProtectionDistanceRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 3>;
/// 5.151 – FIR/UIR Address (ADDRESS) ✅
pub type FirUirAddressRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 4>;
/// 5.154 – Restriction Identifier (REST IDENT) ✅
pub type AirwayRestrictionIdentifierRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.155 – BARO-VNAV Not Authorized ✅
pub type BaroVnavNotAuthorizedRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 1>;
/// 5.157 – Airway Restriction Start/End Date (START/END DATE) ✅
pub type AirwayRestrictionStartEndDateRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 7>;
/// 5.158 – VFR Checkpoint Flag ✅ (Possible error in specification)
pub type VfrCheckpointFlagRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.159 – ATC Assigned Only ✅ (Possible error in specification)
pub type AtcAssignedOnlyRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.160 – Units of Altitude for airway restriction (UNIT IND); distinct from 5.133 ✅
pub type AirwayRestrictionAltitudeUnitRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.161 – Restriction Altitude (RSTR ALT) ✅
pub type AirwayRestrictionAltitudeRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.162 – Step Climb Indicator (STEP) ✅
pub type StepClimbIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.163 – Restriction Notes ✅
pub type AirwayRestrictionNotesRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 104>;
/// 5.164 – EU Indicator (EU IND) ✅
pub type EuIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.165 – Magnetic/True Indicator (M/T IND) ✅
pub type MagneticTrueIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.166 – Channel (MLS) ✅
pub type MLSChannelRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.167 – MLS Azimuth Bearing (MLS AZ BRG) ✅
pub type MLSAzimuthBearingRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.168 – MLS Azimuth / Back Azimuth proportional angle (AZ PRO / BAZ PRO RIGHT/LEFT) ✅
pub type MLSAzimuthProportionalAngleRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.169 – Elevation Angle Span (EL ANGLE SPAN) ✅
pub type MLSElevationAngleSpanRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.172 – Azimuth / Back Azimuth coverage sector (AZ COV / BAZ COV RIGHT/LEFT) ✅
pub type MLSAzimuthCoverageSectorRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.173 – Nominal Elevation Angle (NOM ELEV ANGLE) ✅
pub type MLSNominalElevationAngleRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.174 – Restrictive Airspace Link Continuation (LC) ✅
pub type RestrictiveAirspaceLinkContinuationRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.175 – Holding Speed (HOLD SPEED) ✅
pub type HoldingSpeedRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.176 – Pad Dimensions ✅ (possible error in specification)
pub type PadDimensionsRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 8>;
/// 5.177 – Public/Military Indicator (PUB/MIL) ✅
pub type PublicMilitaryIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.178 – Time Zone ✅
pub type TimeZoneRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 3>;
/// 5.179 – Daylight Time Indicator (DAY TIME) ✅
pub type DaylightTimeIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.180 – Pad Identifier (PAD IDENT) (5 characters max) ✅
pub type PadIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.181 – H24 Indicator (H24) ✅
pub type H24IndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.183 – Sectorization (SECTOR)
pub type CommunicationsSectorizationRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 6>;
/// 5.184 – Communications Altitude (COMM ALTITUDE); Altitude 1 or 2 column ✅
pub type CommunicationsAltitudeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 3>;
/// 5.185 – Sector Facility (SEC FAC) ✅
pub type CommunicationsSectorFacilityRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.186 – Sectorization Narrative ✅
pub type CommunicationsSectorizationNarrativeRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 60>;
/// 5.187 – Distance Description (DIST DESC) ✅
pub type CommunicationsDistanceDescriptionRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.188 – Communications Distance (COMM DIST)✅
pub type CommunicationsDistanceRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 2>;
/// 5.189 – Position Narrative ✅
pub type CommunicationsPositionNarrativeRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 25>;
/// 5.190 – FIR/RDO Identifier (FIR/RDO) ✅
pub type FirRdoIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.194 – Initial / Terminus Fix or Airport  ✅
pub type InitialTerminusFixOrAirportRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.195 – Time of Operation ✅
pub type TimeOfOperationRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 10>;
/// 5.196 – Name Format Indicator (NAME IND) ✅
pub type WaypointNameFormatIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 3>;
/// 5.197 – Modulation (MODULN) ✅
pub type CommunicationsModulationRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.198 – Datum Code (DATUM) ✅
pub type DatumCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 3>;
/// 5.199 – Signal Emission (SIG EM) ✅
pub type SignalEmissionRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 1>;
/// 5.200 – Remote Facility (REM FAC) ✅
pub type CommunicationsRemoteFacilityRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.201 – Restriction Record Type (REST TYPE) ✅
pub type AirwayRestrictionRecordTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 2>;
/// 5.202 – Exclusion Indicator (EXC IND) ✅
pub type AirwayRestrictionExclusionIndicatorRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.203 – Block Indicator (BLOCK IND) ✅
pub type AirwayRestrictionBlockIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.204 – ARC Radius (ARC RAD) ✅
pub type ArcRadiusRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 6>;
/// 5.205 – Navaid Limitation Code (NLC) ✅
pub type NavaidLimitationCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.206 – Component Affected Indicator (COMP AFFTD IND) ✅
pub type NavaidLimitationComponentAffectedRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.207 – Sector From / Sector To (SECTR) ✅
pub type NavaidLimitationSectorFromToRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 2>;
/// 5.208 – Distance Limitation (DIST LIMIT) ✅
pub type NavaidLimitationDistanceLimitRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 6>;
/// 5.209 – Altitude Limitation (ALT LIMIT) ✅
pub type NavaidLimitationAltitudeLimitRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 6>;
/// 5.210 – Sequence End Indicator (SEQ END) ✅
pub type NavaidLimitationSequenceEndRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.211 – Required Navigation Performance (RNP) ✅
pub type RequiredNavigationPerformanceRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.212 – Runway Gradient (RWY GRAD) ✅
pub type RunwayGradientRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 6>;
/// 5.213 – Controlled Airspace Type (ARSP TYPE) ✅
pub type ControlledAirspaceTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.214 – Controlled Airspace Center (ARSP CNTR) ✅
pub type ControlledAirspaceCenterRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.215 – Controlled Airspace Classification (ARSP CLASS) ✅
pub type ControlledAirspaceClassificationRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.216 – Controlled Airspace Name (ARSP NAME) ✅
pub type ControlledAirspaceNameRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 30>;
/// 5.217 – Controlled Airspace Indicator (CTLD ARSP IND) ✅
pub type ControlledAirspaceIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.218 – Geographical Reference Table Identifier (GEO REF TBL ID) ✅
pub type GeographicalReferenceTableIdRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 2>;
/// 5.219 – Geographical Entity (GEO ENT) ✅
pub type GeographicalEntityRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 29>;
/// 5.220 – Preferred Route Use Indicator (ET IND) ✅
pub type PreferredRouteUseIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 2>;
/// 5.221 – Aircraft Use Group (ACFT USE GP) ✅
pub type AircraftUseGroupRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 2>;
/// 5.222 – GNSS/FMS Indicator (GNSS/FMS IND) ✅
pub type GnssFmsIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 1>;
/// 5.223 – Operation Type (OPS TYPE) ✅
pub type OperationTypeRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 2>;
/// 5.224 – Route Indicator (RTE IND) ✅
pub type FinalApproachRouteIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.225 – Ellipsoidal Height ✅
pub type EllipsoidalHeightRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 6>;
/// 5.226 – Glide Path Angle (GPA) ✅
pub type GlidePathAngleRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.227 – Orthometric Height (ORTH HGT) ✅
pub type OrthometricHeightRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 6>;
/// 5.228 – Course Width At Threshold (CRS WDTH) ✅
pub type CourseWidthAtThresholdRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 5>;
/// 5.229 – Final Approach Segment Data CRC Remainder (FAS CRC) ✅
pub type FinalApproachSegmentDataCrcRemainderRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 8>;
/// 5.230 – Procedure Type (PROC TYPE) ✅
pub type FlightPlanningProcedureTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.231 – Along Track Distance (ATD) ✅
pub type FlightPlanningAlongTrackDistanceRaw<const START: usize> =
    FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.232 – Number of Engines Restriction (NOE) ✅
pub type FlightPlanningEnginesRestrictionRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 4>;
/// 5.233 – Turboprop/Jet Indicator (TURBO) ✅
pub type FlightPlanningTurbopropJetIndicatorRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.234 – RNAV Flag (RNAV) ✅
pub type FlightPlanningRnavFlagRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.235 – ATC Weight Category (ATC WC) ✅
pub type FlightPlanningAtcWeightCategoryRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.236 – ATC Identifier (ATC ID) ✅
pub type FlightPlanningAtcIdentifierRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 7>;
/// 5.237 – Procedure Description (PROC DESC) ✅
pub type FlightPlanningProcedureDescriptionRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 15>;
/// 5.238 – Leg Type Code (LTC) ✅
pub type FlightPlanningLegTypeCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 2>;
/// 5.239 – Reporting Code (RPT) ✅
pub type FlightPlanningReportingCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.240 – Altitude (ALT) ✅
pub type FlightPlanningAltitudeRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.241 – Fix Related Transition Code (FRT Code) ✅
pub type FlightPlanningFixRelatedTransitionCodeRaw<const START: usize> =
    FieldRaw<DTYPE_NUMERIC, START, 1>;
/// 5.242 – Procedure Category (PROC CAT) ✅
pub type SidStarApproachProcedureCategoryRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 4>;
/// 5.243 – GLS Station Identifier (4 characters max) ✅
pub type GlsStationIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.244 – SBAS/GBAS Channel ✅
pub type SbasGbasChannelRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 5>;
/// 5.245 – Service Volume Radius ✅
pub type GlsServiceVolumeRadiusRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 2>;
/// 5.246 – TDMA Slots ✅
pub type GlsTdmaSlotsRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 2>;
/// 5.247 – Station Type ✅
pub type GlsStationTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 3>;
/// 5.248 – Station Elevation WGS84 ✅
pub type GlsStationElevationWgs84Raw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.249 – Surface Code (SC) ✅
pub type SurfaceCodeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.250 – Alternate Record Type (ART) ✅
pub type AlternateRecordTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 2>;
/// 5.251 – Distance To Alternate (DTA) (3 characters max) ✅
pub type DistanceToAlternateRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.252 – Alternate Type (ALT TYPE) ✅
pub type AlternateTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.253 – Primary and Additional Alternate Identifier (ALT IDENT) (10 characters max) ✅
pub type AlternateIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 10>;
/// 5.254 – Fixed Radius Transition Indicator (FIXED RAD IND) ✅
pub type FixedRadiusTransitionIndicatorRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.255 – SBAS Service Provider Identifier (SBAS ID) ✅
pub type SbasServiceProviderIdentifierRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 2>;
/// 5.256 – Reference Path Data Selector (REF PDS) ✅
pub type ReferencePathDataSelectorRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 2>;
/// 5.257 – Reference Path Identifier (REF ID) ✅
pub type ReferencePathIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.258 – Approach Performance Designator (APD) ✅
pub type ApproachPerformanceDesignatorRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 1>;
/// 5.259 – Length Offset (OFFSET) ✅
pub type PathPointLengthOffsetRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.260 – Terminal Procedure Flight Planning Leg Distance (LEG DIST) ✅
pub type TerminalProcedureFlightPlanningLegDistanceRaw<const START: usize> =
    FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.261 – Speed Limit Description (SLD) ✅
pub type SpeedLimitDescriptionRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.262 – Approach Type Identifier (ATI) ✅
pub type ApproachTypeIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 10>;
/// 5.263 – Horizontal Alert Limit (HAL) / Lateral Alert Limit (LAL) ✅
pub type HorizontalOrLateralAlertLimitRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 3>;
/// 5.264 – Vertical Alert Limit (VAL) ✅
pub type VerticalAlertLimitRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 3>;
/// 5.265 – Path Point TCH ✅
pub type PathPointTchRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 6>;
/// 5.266 – TCH Units Indicator ✅
pub type TchUnitsIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.267 – High Precision Latitude (HPLAT) ✅
pub type HighPrecisionLatitudeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 11>;
/// 5.268 – High Precision Longitude (HPLONG) ✅
pub type HighPrecisionLongitudeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 12>;
/// 5.269 – Helicopter Procedure Course (HPC) ✅
pub type HelicopterProcedureCourseRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.270 – TCH Value Indicator (TCHVI) ✅
pub type TchValueIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.271 – Procedure Turn (PROC TURN) ✅
pub type TaaProcedureTurnRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.272 – TAA Sector Identifier ✅
pub type TaaSectorIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.273 – TAA Waypoint (5-character max) ✅
pub type TaaWaypointRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.274 – TAA Sector Radius ✅
pub type TaaSectorRadiusRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.275 – Level of Service Name (LSN) ✅
pub type LevelOfServiceNameRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 10>;
/// 5.276 – Level of Service Authorized ✅
pub type LevelOfServiceAuthorizedRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 1>;
/// 5.277 – DME Operational Service Volume (D-OSV) ✅
pub type DmeOperationalServiceVolumeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.278 – Activity Type ✅
pub type SpecialActivityAreaTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.279 – Activity Identifier ✅
pub type SpecialActivityAreaIdentifierRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 6>;
/// 5.280 – Special Activity Area Size ✅
pub type SpecialActivityAreaSizeRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.281 – Special Activity Area Volume ✅
pub type SpecialActivityAreaVolumeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 1>;
/// 5.282 – Special Activity Area Operating Times ✅
pub type SpecialActivityAreaOperatingTimesRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 3>;
/// 5.283 – Communications Class (Comm Class) ✅
pub type CommunicationsClassRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 4>;
/// 5.284 – Assigned Sector Name (ASN) (25 characters max) ✅
pub type AssignedSectorNameRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 25>;
/// 5.285 – Time Narrative (100 characters max per record) ✅
pub type CommunicationsTimeNarrativeRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 100>;
/// 5.286 – Multi-Sector Indicator (MSEC IND) ✅
pub type MultiSectorIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 1>;
/// 5.287 – Type Recognized By (TRB) ✅
pub type CommunicationsTypeRecognizedByRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.288 – Translation (80 characters max) ✅
pub type CommunicationsTypeTranslationRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 80>;
/// 5.289 – Used On ✅
pub type CommunicationsTypeTranslationTableUsedOnRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.290 – Procedure Design Mag Var (PDMV) ✅
pub type ProcedureDesignMagneticVariationRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.291 – Procedure Design Mag Var Indicator (PDMVI) ✅
pub type ProcedureDesignMagneticVariationIndicatorRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.292 – Category Distance ✅
pub type CirclingCategoryDistanceRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 2>;
/// 5.293 – Vertical Scale Factor (VSF) ✅
pub type VerticalScaleFactorRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.294 – RVSM Minimum Level ✅
pub type RvsmMinimumLevelRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.295 – RVSM Maximum Level ✅
pub type RvsmMaximumLevelRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.296 – RNP Level of Service (LSN) ✅
pub type RnpApproachLevelOfServiceNameRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 3>;
/// 5.297 – Route Inappropriate Navaid Indicator ✅
pub type RouteInappropriateNavaidIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.298 – Holding Pattern/Race Track Course Reversal Leg Inbound/Outbound Indicator ✅
pub type HoldingLegInboundOutboundIndicatorRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.299 – Procedure Referenced Fix Identifier ✅
pub type ProcedureReferencedFixIdentifierRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.300 – Final Approach Course as Runway ✅
pub type FinalApproachCourseAsRunwayRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 5>;
/// 5.301 – Procedure Design Aircraft Category or Type ✅
pub type ProcedureDesignAircraftCategoryOrTypeRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.302 – Surface Type ✅
pub type RunwaySurfaceTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 4>;
/// 5.303 – Helipad Shape ✅
pub type HelipadShapeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.304 – Sector Bearing Reference Waypoint (5 characters max) ✅
pub type TaaSectorBearingReferenceWaypointRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.305 – Heliport Type ✅
pub type HeliportTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.306 – Preferred Multiple Approach Indicator ✅
pub type PreferredMultipleApproachIndicatorRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.307 – Special Indicator ✅
pub type TerminalProcedureSpecialIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.308 – Remote Altimeter Flag ✅
pub type RemoteAltimeterFlagRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.309 – Maximum Allowable Helicopter Weight ✅
pub type MaximumAllowableHelicopterWeightRaw<const START: usize> =
    FieldRaw<DTYPE_NUMERIC, START, 3>;
/// 5.310 – Helicopter Performance Requirement (M/S/U; length 1 per code table) ✅
pub type HelipadPerformanceRequirementRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.311 – FIR/FRA Transition ✅
pub type FirFraTransitionRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 1>;
/// 5.312 – Starter Extension ✅
pub type RunwayStarterExtensionRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.313 – TORA ✅
pub type RunwayToraRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 5>;
/// 5.314 – TODA ✅
pub type RunwayTodaRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 5>;
/// 5.315 – ASDA ✅
pub type RunwayAsdaRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 5>;
/// 5.316 – LDA ✅
pub type RunwayLdaRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 5>;
/// 5.317 – Runway Usage Indicator ✅
pub type RunwayUsageIndicatorRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.318 – Runway Accuracy Compliance Flag ✅
pub type RunwayAccuracyComplianceFlagRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.319 – Landing Threshold Elevation Accuracy Compliance Flag ✅
pub type LandingThresholdElevationAccuracyComplianceFlagRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.320 – SBAS Final Approach Course ✅
pub type SbasFinalApproachCourseRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.321 – Helipad Maximum Rotor Diameter ✅
pub type HelipadMaximumRotorDiameterRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 3>;
/// 5.322 – Helipad Type ✅
pub type HelipadTypeRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 1>;
/// 5.323 – Helipad Orientation ✅
pub type HelipadOrientationRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.324 – Helipad Identifier Orientation ✅
pub type HelipadIdentifierOrientationRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
/// 5.325 – Preferred Approach Bearing ✅
pub type PreferredApproachBearingRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.326 – Ground Facility Identifier ✅
pub type AtnGroundFacilityIdentifierRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 8>;
/// 5.327 – Authority Format Identifier (AFI) ✅
pub type AtnAuthorityFormatIdentifierRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 2>;
/// 5.328 – Initial Domain Identifier ✅
pub type AtnInitialDomainIdentifierRaw<const START: usize> = FieldRaw<DTYPE_NUMERIC, START, 4>;
/// 5.329 – Version (VER) ✅
pub type AtnVersionRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 2>;
/// 5.330 – Administration (ADM) ✅
pub type AtnAdministrationRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 6>;
/// 5.331 – Routing Domain Format (RDF) ✅
pub type AtnRoutingDomainFormatRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 2>;
/// 5.332 – Administrative Region Selector (ARS) ✅
pub type AtnAdministrativeRegionSelectorRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 6>;
/// 5.333 – Location (LOC) (NSAP routing location subfield) ✅
pub type AtnRoutingLocationRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.334 – System Identifier (SYS) ✅
pub type AtnSystemIdentifierRaw<const START: usize> = FieldRaw<DTYPE_ALPHANUMERIC, START, 12>;
/// 5.335 – Network Service Access Point Selector (NSEL) ✅
pub type AtnNetworkServiceAccessPointSelectorRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 2>;
/// 5.336 – Context Management Transport Selector (CM TSEL) ✅
pub type AtnContextManagementTransportSelectorRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 4>;
/// 5.337 – Use Indicator (ATN ATSU ground facility) ✅
pub type AtnGroundFacilityUseIndicatorRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 1>;
/// 5.338 – VOR Range/Power (VORPWR) ✅
pub type VhfNavaidVorRangePowerRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.339 – DME Expanded Service Volume (DESV) ✅
pub type VhfNavaidDmeExpandedServiceVolumeRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.340 – Unmanned Aerial Vehicle (UAV) Only ✅ (possibly error in specification)
pub type UnmannedAerialVehicleOperationsOnlyRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.341 – Military Indicator (terminal SID/STAR/APP) ✅
pub type TerminalProcedureMilitaryIndicatorRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.342 – Source of LAL/VAL ✅
pub type SbasApproachMinimaLalValSourceRaw<const START: usize> = FieldRaw<DTYPE_ALPHA, START, 1>;
/// 5.343 – Holding Pattern Magnetic Variation (HPMV) ✅
pub type HoldingPatternMagneticVariationRaw<const START: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, START, 5>;
