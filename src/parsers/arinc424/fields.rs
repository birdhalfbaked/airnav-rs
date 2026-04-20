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
#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq)]
pub struct FieldParseError {
    pub message: String,
}

pub type DType = u8;

const DTYPE_ALPHA: DType = 0;
const DTYPE_ALPHANUMERIC: DType = 1;
const DTYPE_NUMERIC: DType = 2;

#[derive(Debug, PartialEq, Eq)]
pub struct FieldRaw<const DTYPE: DType, const STARTCOL: usize, const LEN: usize> {
    pub bytes: [u8; LEN],
}
impl<const DTYPE: DType, const STARTCOL: usize, const LEN: usize> FieldRaw<DTYPE, STARTCOL, LEN> {
    pub fn new(input: &[u8]) -> Self {
        assert!(STARTCOL >= 1, "STARTCOL must be >= 1");
        let mut bytes = [0u8; LEN];
        bytes.copy_from_slice(&input[STARTCOL - 1..STARTCOL + LEN - 1]);
        Self { bytes }
    }
}
impl<const STARTCOL: usize, const LEN: usize> FieldRaw<DTYPE_ALPHA, STARTCOL, LEN> {
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
impl<const STARTCOL: usize, const LEN: usize> FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, LEN> {
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
impl<const STARTCOL: usize, const LEN: usize> FieldRaw<DTYPE_NUMERIC, STARTCOL, LEN> {
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
    let r: FieldRaw<DTYPE_ALPHA, 1, 3> = FieldRaw::new(&[b'D', b'-', b' ']);
    assert_eq!(r.as_value(), Ok("D-"));
    // should keep leading blanks
    let r: FieldRaw<DTYPE_ALPHA, 1, 3> = FieldRaw::new(&[b' ', b'@', b'D']);
    assert_eq!(r.as_value(), Ok(" @D"));
    // should error on non-alpha characters
    let r: FieldRaw<DTYPE_ALPHA, 1, 3> = FieldRaw::new(&[b'0', b'0', b'S']);
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
    let r: FieldRaw<DTYPE_ALPHANUMERIC, 1, 3> = FieldRaw::new(&[b'D', b'-', b' ']);
    assert_eq!(r.as_value(), Ok("D-"));
    let r: FieldRaw<DTYPE_ALPHANUMERIC, 1, 3> = FieldRaw::new(&[b' ', b'@', b'D']);
    assert_eq!(r.as_value(), Ok(" @D"));
    // except now this should be ok
    let r: FieldRaw<DTYPE_ALPHANUMERIC, 1, 3> = FieldRaw::new(&[b'0', b'0', b'S']);
    assert_eq!(r.as_value(), Ok("00S"));
}

#[test]
pub fn test_as_numeric() {
    let r: FieldRaw<DTYPE_NUMERIC, 1, 3> = FieldRaw::new(&[b'0', b'0', b'1']);
    assert_eq!(r.as_value(), Ok(1));
    let r: FieldRaw<DTYPE_NUMERIC, 1, 3> = FieldRaw::new(&[b'0', b'0', b' ']);
    assert!(
        r.as_value()
            .unwrap_err()
            .message
            .contains("Invalid numeric data: 00")
    );
}

#[test]
#[should_panic(expected = "STARTCOL must be >= 1")]
pub fn test_field_raw_startcol_must_be_greater_than_zero() {
    let _: FieldRaw<DTYPE_NUMERIC, 0, 3> = FieldRaw::new(&[b'0', b'0', b'1']);
}

/// Helper field for spacing to keep record definitions easier to maintain
pub type BlankSpacingRaw<const STARTCOL: usize, const LENGTH: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, LENGTH>;

// --- ARINC 424 Chapter 5 navigation field raw types (Section 5.0 field definitions) ---

/// 5.2 – Record Type (S/T) ✅
pub type _5_2_RecordTypeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.3 – Customer/Area Code (CUST/AREA), Area ✅
pub type _5_3_CustomerAreaCodeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.4 – Section Code (SEC CODE) ✅
pub type _5_4_SectionCodeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.5 – Subsection Code (SUB CODE) ✅
pub type _5_5_SubsectionCodeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.6 – Airport/Heliport Identifier (ARPT/HELI IDENT) ✅
pub type _5_6_AirportHeliportIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.7 – Route Type (RT TYPE), Enroute Airway ✅
pub type _5_7_RouteTypeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.8(A) – Route Identifier (ROUTE IDENT), Enroute Airway ✅
pub type _5_8_A_EnrouteRouteIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 5>;
/// 5.8(B) – Route Identifier (ROUTE IDENT), Preferred Route ✅
pub type _5_8_B_PreferredRouteIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 10>;
/// 5.9 – SID/STAR Route Identifier (SID/STAR IDENT) ✅
pub type _5_9_SidStarRouteIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.10 – Approach Route Identifier (APPROACH IDENT) ✅
pub type _5_10_ApproachRouteIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.11 – Transition Identifier (TRANS IDENT) ✅
pub type _5_11_TransitionIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.12(A) – Sequence Number (SEQ NR), 4 characters ✅
pub type _5_12_A_SequenceNumber4CharacterRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.12(B) – Sequence Number (SEQ NR), 3 characters ✅
pub type _5_12_B_SequenceNumber3CharacterRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.12(C) – Sequence Number (SEQ NR), 2 characters ✅
pub type _5_12_C_SequenceNumber2CharacterRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.12(D) – Sequence Number (SEQ NR), 1 character ✅
pub type _5_12_D_SequenceNumber1CharacterRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 1>;
/// 5.13 – Fix Identifier (FIX IDENT) ✅
pub type _5_13_FixIdentifierRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.14 – ICAO Code (ICAO CODE) ✅
pub type _5_14_IcaoCodeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.15 – Inbound Course Theta (holding pattern) ✅
pub type _5_15_InboundCourseThetaRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.16 – Continuation Record Number (CONT NR) ✅
pub type _5_16_ContinuationRecordNumberRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.17 – Waypoint Description Code (DESC CODE) ✅
pub type _5_17_WaypointDescriptionCodeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 4>;
/// 5.18 – Boundary Code (BDY CODE) ✅
pub type _5_18_BoundaryCodeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.19 – Level (LEVEL) ✅
pub type _5_19_LevelRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.20 – Turn Direction (TURN DIR) ✅
pub type _5_20_TurnDirectionRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.21 – Path and Termination (PATH TERM) ✅
pub type _5_21_PathAndTerminationRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 2>;
/// 5.22 – Turn Direction Valid (TDV) ✅
pub type _5_22_TurnDirectionValidRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.23 – Recommended NAVAID (RECD NAV) ✅
pub type _5_23_RecommendedNavaidRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.24 – Theta (THETA) ✅
pub type _5_24_ThetaRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.25 – Rho (RHO) ✅
pub type _5_25_RhoRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.26 – Outbound Course (OB CRS) ✅
pub type _5_26_OutboundCourseRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.27 – Route Distance From, Holding Distance/Time ✅
pub type _5_27_RouteDistanceFromHoldingDistanceTimeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.28 – Inbound Course (IB CRS) ✅
pub type _5_28_InboundCourseRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.29 – Altitude Description (ALT DESC) ✅
pub type _5_29_AltitudeDescriptionRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.30 – Altitude / Minimum Altitude ✅
pub type _5_30_AltitudeMinimumAltitudeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.31 – File Record Number (FRN) ✅
pub type _5_31_FileRecordNumberRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.32 – Cycle Date (CYCLE) ✅
pub type _5_32_CycleDateRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.33 – VOR/NDB Identifier (VOR IDENT/NDB IDENT) ✅
pub type _5_33_VorNdbIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.34 – VOR/NDB Frequency (VOR/NDB FREQ) ✅
pub type _5_34_VorNdbFrequencyRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.35 – NAVAID Class (CLASS) ✅
pub type _5_35_NavaidClassRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 5>;
/// 5.36 – Latitude (LATITUDE) ✅
pub type _5_36_LatitudeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 9>;
/// 5.37 – Longitude (LONGITUDE) ✅
pub type _5_37_LongitudeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 10>;
/// 5.38 – DME Identifier (DME IDENT) ✅
pub type _5_38_DMEIdentifierRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.39 – Magnetic Variation (MAG VAR, D MAG VAR) ✅
pub type _5_39_MagneticVariationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.40 – DME Elevation (DME ELEV) ✅
pub type _5_40_DmeElevationRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.41 – Region Code (REGN CODE) ✅
pub type _5_41_RegionCodeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.42 – Waypoint Type (TYPE) ✅
pub type _5_42_WaypointTypeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 3>;
/// 5.43 – Waypoint Name/Description (NAME/DESC) ✅
pub type _5_43_WaypointNameDescriptionRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 25>;
/// 5.44 – Localizer/MLS/GLS Identifier (LOC, MLS, GLS IDENT) ✅
pub type _5_44_LocalizerMlsGlsIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.45 – Localizer Frequency (FREQ) ✅
pub type _5_45_LocalizerFrequencyRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.46 – Runway Identifier (RUNWAY ID) ✅
pub type _5_46_RunwayIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.47 – Localizer Bearing (LOC BRG) ✅
pub type _5_47_LocalizerBearingRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.48 – Localizer/Azimuth Position (LOC FR RW END / AZ/BAZ FR RW END) ✅
pub type _5_48_LocalizerAzimuthPositionRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.49 – Localizer/Azimuth Position Reference (@, +, -) ✅
pub type _5_49_LocalizerAzimuthPositionReferenceRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.50 – Glideslope/Elevation Position (GS FR RW THRES / EL FR RW THRES) ✅
pub type _5_50_GlideslopeElevationPositionRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.51 – Localizer Width (LOC WIDTH) ✅
pub type _5_51_LocalizerWidthRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.52 – Glideslope Angle / Minimum Elevation Angle (GS ANGLE / MIN ELEV ANGLE) ✅
pub type _5_52_GlideslopeAngleRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.53 – Transition Altitude/Level (TRANS ALTITUDE/LEVEL) ✅
pub type _5_53_TransitionAltitudeLevelRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.54 – Longest Runway (LONGEST RWY) ✅
pub type _5_54_LongestRunwayRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.55 – Airport/Heliport Elevation (ELEV) ✅
pub type _5_55_AirportHeliportElevationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.56 – Gate Identifier (GATE IDENT) ✅
pub type _5_56_GateIdentifierRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.57 – Runway Length (RUNWAY LENGTH) ✅
pub type _5_57_RunwayLengthRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.58 – Runway Bearing (RWY BRG) ✅
pub type _5_58_RunwayBearingRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.59 – Runway Description (RUNWAY DESCRIPTION) ✅
pub type _5_59_RunwayDescriptionRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 22>;
/// 5.60 – Name (NAME), Gate and Holding Pattern records ✅
pub type _5_60_GateHoldingPatternNameRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 25>;
/// 5.61 – Notes, continuation records (NOTES) ✅
pub type _5_61_ContinuationNotesRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 102>;
/// 5.62 – Inbound Holding Course (IB HOLD CRS) ✅
pub type _5_62_InboundHoldingCourseRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.63 – Turn (TURN), Holding Pattern records ✅
pub type _5_63_HoldingPatternTurnRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.64 – Leg Length (LEG LENGTH) ✅
pub type _5_64_HoldingLegLengthRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.65 – Leg Time (LEG TIME) ✅
pub type _5_65_HoldingLegTimeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.66 – Station Declination (STN DEC) ✅
pub type _5_66_StationDeclinationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.67 – Threshold Crossing Height (TCH) ✅
pub type _5_67_ThresholdCrossingHeightRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.68 – Landing Threshold Elevation (LANDING THRES ELEV) ✅
pub type _5_68_LandingThresholdElevationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.69 – Threshold Displacement Distance (DSPLCD THR) ✅
pub type _5_69_ThresholdDisplacementDistanceRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.70 – Vertical Angle (VERT ANGLE) ✅
pub type _5_70_VerticalAngleRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.71 – Name Field, Navaid/Airport/Heliport/Enroute Marker records ✅
pub type _5_71_FacilityNameFieldRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 30>;
/// 5.72 – Speed Limit (SPEED LIMIT) ✅
pub type _5_72_SpeedLimitRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.73 – Speed Limit Altitude ✅
pub type _5_73_SpeedLimitAltitudeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.74 – Component Elevation (GS ELEV, EL ELEV, AZ ELEV, BAZ ELEV, GLS ELEV) ✅
pub type _5_74_ComponentElevationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.75 – From/To Airport/Heliport/Fix ✅
pub type _5_75_FromToAirportHeliportFixRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.76 – Company Route Ident ✅
pub type _5_76_CompanyRouteIdentRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 10>;
/// 5.77 – VIA Code ✅
pub type _5_77_ViaCodeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.78 – SID/STAR/App/AWY (S/S/A/AWY), SID/STAR/AWY (S/S/AWY) ✅
pub type _5_78_SidStarApproachAirwayRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.79 – Stopway ✅
pub type _5_79_StopwayRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.80 – ILS/MLS/GLS Category (CAT) ✅
pub type _5_80_IlsMlsGlsCategoryRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.81 – ATC Indicator (ATC) ✅
pub type _5_81_AtcIndicatorRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.82 – Waypoint Usage ✅
pub type _5_82_WaypointUsageRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.83 – To Fix, Company Route / Helicopter Operations Company Route (6 characters max) ✅
pub type _5_83_CompanyRouteToFixRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.83 – To Fix, Preferred Route (5 characters max) ✅
pub type _5_83_PreferredRouteToFixRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.84 – Runway Transition (RUNWAY TRANS) ✅
pub type _5_84_RunwayTransitionRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.85 – Enroute Transition (ENRT TRANS) ✅
pub type _5_85_EnrouteTransitionRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.86 – Cruise Altitude ✅
pub type _5_86_CruiseAltitudeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.87 – Terminal/Alternate Airport (TERM/ALT ARPT) ✅
pub type _5_87_TerminalAlternateAirportRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.88 – Alternate Distance (ALT DIST) ✅
pub type _5_88_AlternateDistanceRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.89 – Cost Index ✅
pub type _5_89_CostIndexRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.90 – ILS/DME Bias ✅
pub type _5_90_IlsDmeBiasRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.91 – Continuation Record Application Type (APPL) ✅
pub type _5_91_ContinuationRecordApplicationTypeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.92 – Facility Elevation (FAC ELEV) ✅
pub type _5_92_FacilityElevationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.93 – Facility Characteristics (FAC CHAR) ✅
pub type _5_93_FacilityCharacteristicsRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.94 – True Bearing (TRUE BRG) ✅
pub type _5_94_TrueBearingRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.95 – Government Source (SOURCE) ✅
pub type _5_95_GovernmentSourceRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.96 – Glideslope Beam Width (GS BEAM WIDTH) ✅
pub type _5_96_GlideslopeBeamWidthRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.97 – Touchdown Zone Elevation (TDZE) ✅
pub type _5_97_TouchdownZoneElevationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.98 – Elevation Type ✅
pub type _5_98_ElevationTypeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.99 – Marker Type (MKR TYPE) ✅
pub type _5_99_MarkerTypeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 3>;
/// 5.100 – Minor Axis True Bearing (MINOR AXIS TRUE BRG) ✅
pub type _5_100_MinorAxisTrueBearingRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.101 – Communications Type (COMM TYPE) ✅
pub type _5_101_CommunicationsTypeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 3>;
/// 5.102 – Radar ✅
pub type _5_102_RadarRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.103 – Communications Frequency ✅
pub type _5_103_CommunicationsFrequencyRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 7>;
/// 5.104 – Frequency Units ✅
pub type _5_104_FrequencyUnitsRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.105 – Call Sign ✅
pub type _5_105_CallSignRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 25>;
/// 5.106 – Service Indicator (SERV IND) ✅
pub type _5_106_ServiceIndicatorRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 3>;
/// 5.107 – ATA/IATA Designator ✅
pub type _5_107_AtaIataDesignatorRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 3>;
/// 5.108 – IFR Capability ✅
pub type _5_108_IfrCapabilityRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.109 – Runway Width ✅
pub type _5_109_RunwayWidthRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.110 – Marker Identifier (Enroute Marker) (IDENT) ✅
pub type _5_110_EnrouteMarkerIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.111 – Marker Code (Morse); spec lists Alpha, Morse encoding often uses dot/dash ✅
pub type _5_111_EnrouteMarkerMorseCodeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.112 – Marker Shape ✅
pub type _5_112_MarkerShapeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.113 – High/Low (Enroute Marker) ✅
pub type _5_113_EnrouteMarkerHighLowRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.114 – Duplicate Indicator ✅
pub type _5_114_DuplicateIndicatorRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.115 – Direction Restriction ✅
pub type _5_115_DirectionRestrictionRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.116 – FIR/UIR Identifier; spec lists Alpha, examples include digits ✅
pub type _5_116_FirUirIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.117 – FIR/UIR Indicator (IND) ✅
pub type _5_117_FirUirIndicatorRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.118 – Boundary Via ✅
pub type _5_118_BoundaryViaRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 2>;
/// 5.119 – Arc Distance ✅
pub type _5_119_ArcDistanceRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.120 – Arc Bearing ✅
pub type _5_120_ArcBearingRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.121 – Lower/Upper Limit ✅
pub type _5_121_FirUirLowerUpperLimitRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.122 – FIR/UIR Reporting Units Speed ✅
pub type _5_122_FirUirRUSRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 1>;
/// 5.123 – FIR/UIR Reporting Units Altitude ✅
pub type _5_123_FirUirRUARaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 1>;
/// 5.124 – FIR/UIR Entry Report (ENTRY) ✅
pub type _5_124_FirUirEntryReportRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.125 – FIR/UIR Name ✅
pub type _5_125_FirUirNameRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 25>;
/// 5.126 – Restrictive Airspace Name ✅
pub type _5_126_RestrictiveAirspaceNameRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 30>;
/// 5.127 – Maximum Altitude ✅
pub type _5_127_MaximumAltitudeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.128 – Restrictive Airspace Type ✅
pub type _5_128_RestrictiveAirspaceTypeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.129 – Restrictive Airspace Designation ✅
pub type _5_129_RestrictiveAirspaceDesignationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 10>;
/// 5.130 – Multiple Code (MULTI CD) ✅
pub type _5_130_MultipleCodeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.131 – Time Code (TIME CD) ✅
pub type _5_131_TimeCodeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.132 – NOTAM ✅
pub type _5_132_NotamRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.133 – Unit Indicator (UNIT IND) ✅
pub type _5_133_UnitIndicatorRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.134 – Cruise Table Identifier (CRSE TBL IDENT) ✅
pub type _5_134_CruiseTableIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.135 – Course From/To (Cruise Table) ✅
pub type _5_135_CruiseTableCourseFromToRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.136 – Cruise Level From/To ✅
pub type _5_136_CruiseLevelFromToRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.137 – Vertical Separation ✅
pub type _5_137_VerticalSeparationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.138 – Time Indicator (TIME IND) ✅
pub type _5_138_TimeIndicatorRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.139 – Procedure Name ✅
pub type _5_139_ProcedureNameRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 78>;
/// 5.140 – Controlling Agency ✅
pub type _5_140_ControllingAgencyRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 25>;
/// 5.141 – Starting Latitude (Grid MORA) ✅
pub type _5_141_GridMoraStartingLatitudeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.142 – Starting Longitude (Grid MORA) ✅
pub type _5_142_GridMoraStartingLongitudeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.143 – Grid MORA ✅
pub type _5_143_GridMoraRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.144 – Center Fix (CENTER FIX) (5 characters max) ✅
pub type _5_144_CenterFixRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.145 – Radius Limit (MSA) ✅
pub type _5_145_MsaRadiusLimitRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.146 – Sector Bearing (SEC BRG) ✅
pub type _5_146_SectorBearingRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 6>;
/// 5.147 – Sector Altitude (SEC ALT) ✅
pub type _5_147_SectorAltitudeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.148 – Enroute Alternate Airport/Heliport (EAA) ✅
pub type _5_148_EnrouteAlternateAirportHeliportRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.149 – Navaid Usable Range (Figure of Merit) ✅
pub type _5_149_NavaidUsableRangeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 1>;
/// 5.150 – Frequency Protection Distance (FREQ PRD) ✅
pub type _5_150_FrequencyProtectionDistanceRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.151 – FIR/UIR Address (ADDRESS) ✅
pub type _5_151_FirUirAddressRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 4>;
/// 5.154 – Restriction Identifier (REST IDENT) ✅
pub type _5_154_AirwayRestrictionIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.155 – BARO-VNAV Not Authorized ✅
pub type _5_155_BaroVnavNotAuthorizedRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.157 – Airway Restriction StartCOL/End Date (STARTCOL/END DATE) ✅
pub type _5_157_AirwayRestrictionStartCOLEndDateRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 7>;
/// 5.158 – VFR Checkpoint Flag ✅ (Possible error in specification)
pub type _5_158_VfrCheckpointFlagRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.159 – ATC Assigned Only ✅ (Possible error in specification)
pub type _5_159_AtcAssignedOnlyRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.160 – Units of Altitude for airway restriction (UNIT IND); distinct from 5.133 ✅
pub type _5_160_AirwayRestrictionAltitudeUnitRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.161 – Restriction Altitude (RSTR ALT) ✅
pub type _5_161_AirwayRestrictionAltitudeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.162 – Step Climb Indicator (STEP) ✅
pub type _5_162_StepClimbIndicatorRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.163 – Restriction Notes ✅
pub type _5_163_AirwayRestrictionNotesRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 104>;
/// 5.164 – EU Indicator (EU IND) ✅
pub type _5_164_EuIndicatorRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.165 – Magnetic/True Indicator (M/T IND) ✅
pub type _5_165_MagneticTrueIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.166 – Channel (MLS) ✅
pub type _5_166_MLSChannelRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.167 – MLS Azimuth Bearing (MLS AZ BRG) ✅
pub type _5_167_MLSAzimuthBearingRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.168 – MLS Azimuth / Back Azimuth proportional angle (AZ PRO / BAZ PRO RIGHT/LEFT) ✅
pub type _5_168_MLSAzimuthProportionalAngleRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.169 – Elevation Angle Span (EL ANGLE SPAN) ✅
pub type _5_169_MLSElevationAngleSpanRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.172 – Azimuth / Back Azimuth coverage sector (AZ COV / BAZ COV RIGHT/LEFT) ✅
pub type _5_172_MLSAzimuthCoverageSectorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.173 – Nominal Elevation Angle (NOM ELEV ANGLE) ✅
pub type _5_173_MLSNominalElevationAngleRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.174 – Restrictive Airspace Link Continuation (LC) ✅
pub type _5_174_RestrictiveAirspaceLinkContinuationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.175 – Holding Speed (HOLD SPEED) ✅
pub type _5_175_HoldingSpeedRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.176 – Pad Dimensions ✅ (possible error in specification)
pub type _5_176_PadDimensionsRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 8>;
/// 5.177 – Public/Military Indicator (PUB/MIL) ✅
pub type _5_177_PublicMilitaryIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.178 – Time Zone ✅
pub type _5_178_TimeZoneRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.179 – Daylight Time Indicator (DAY TIME) ✅
pub type _5_179_DaylightTimeIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.180 – Pad Identifier (PAD IDENT) (5 characters max) ✅
pub type _5_180_PadIdentifierRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.181 – H24 Indicator (H24) ✅
pub type _5_181_H24IndicatorRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.183 – Sectorization (SECTOR)
pub type _5_183_CommunicationsSectorizationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.184 – Communications Altitude (COMM ALTITUDE); Altitude 1 or 2 column ✅
pub type _5_184_CommunicationsAltitudeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.185 – Sector Facility (SEC FAC) ✅
pub type _5_185_CommunicationsSectorFacilityRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.186 – Sectorization Narrative ✅
pub type _5_186_CommunicationsSectorizationNarrativeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 60>;
/// 5.187 – Distance Description (DIST DESC) ✅
pub type _5_187_CommunicationsDistanceDescriptionRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.188 – Communications Distance (COMM DIST)✅
pub type _5_188_CommunicationsDistanceRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.189 – Position Narrative ✅
pub type _5_189_CommunicationsPositionNarrativeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 25>;
/// 5.190 – FIR/RDO Identifier (FIR/RDO) ✅
pub type _5_190_FirRdoIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.194 – Initial / Terminus Fix or Airport  ✅
pub type _5_194_InitialTerminusFixOrAirportRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.195 – Time of Operation ✅
pub type _5_195_TimeOfOperationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 10>;
/// 5.196 – Name Format Indicator (NAME IND) ✅
pub type _5_196_WaypointNameFormatIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 3>;
/// 5.197 – Modulation (MODULN) ✅
pub type _5_197_CommunicationsModulationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.198 – Datum Code (DATUM) ✅
pub type _5_198_DatumCodeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 3>;
/// 5.199 – Signal Emission (SIG EM) ✅
pub type _5_199_SignalEmissionRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.200 – Remote Facility (REM FAC) ✅
pub type _5_200_CommunicationsRemoteFacilityRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.201 – Restriction Record Type (REST TYPE) ✅
pub type _5_201_AirwayRestrictionRecordTypeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 2>;
/// 5.202 – Exclusion Indicator (EXC IND) ✅
pub type _5_202_AirwayRestrictionExclusionIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.203 – Block Indicator (BLOCK IND) ✅
pub type _5_203_AirwayRestrictionBlockIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.204 – ARC Radius (ARC RAD) ✅
pub type _5_204_ArcRadiusRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 6>;
/// 5.205 – Navaid Limitation Code (NLC) ✅
pub type _5_205_NavaidLimitationCodeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.206 – Component Affected Indicator (COMP AFFTD IND) ✅
pub type _5_206_NavaidLimitationComponentAffectedRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.207 – Sector From / Sector To (SECTR) ✅
pub type _5_207_NavaidLimitationSectorFromToRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 2>;
/// 5.208 – Distance Limitation (DIST LIMIT) ✅
pub type _5_208_NavaidLimitationDistanceLimitRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.209 – Altitude Limitation (ALT LIMIT) ✅
pub type _5_209_NavaidLimitationAltitudeLimitRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.210 – Sequence End Indicator (SEQ END) ✅
pub type _5_210_NavaidLimitationSequenceEndRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.211 – Required Navigation Performance (RNP) ✅
pub type _5_211_RequiredNavigationPerformanceRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.212 – Runway Gradient (RWY GRAD) ✅
pub type _5_212_RunwayGradientRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.213 – Controlled Airspace Type (ARSP TYPE) ✅
pub type _5_213_ControlledAirspaceTypeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.214 – Controlled Airspace Center (ARSP CNTR) ✅
pub type _5_214_ControlledAirspaceCenterRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.215 – Controlled Airspace Classification (ARSP CLASS) ✅
pub type _5_215_ControlledAirspaceClassificationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.216 – Controlled Airspace Name (ARSP NAME) ✅
pub type _5_216_ControlledAirspaceNameRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 30>;
/// 5.217 – Controlled Airspace Indicator (CTLD ARSP IND) ✅
pub type _5_217_ControlledAirspaceIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.218 – Geographical Reference Table Identifier (GEO REF TBL ID) ✅
pub type _5_218_GeographicalReferenceTableIdRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.219 – Geographical Entity (GEO ENT) ✅
pub type _5_219_GeographicalEntityRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 29>;
/// 5.220 – Preferred Route Use Indicator (ET IND) ✅
pub type _5_220_PreferredRouteUseIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 2>;
/// 5.221 – Aircraft Use Group (ACFT USE GP) ✅
pub type _5_221_AircraftUseGroupRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 2>;
/// 5.222 – GNSS/FMS Indicator (GNSS/FMS IND) ✅
pub type _5_222_GnssFmsIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.223 – Operation Type (OPS TYPE) ✅
pub type _5_223_OperationTypeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.224 – Route Indicator (RTE IND) ✅
pub type _5_224_FinalApproachRouteIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.225 – Ellipsoidal Height ✅
pub type _5_225_EllipsoidalHeightRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.226 – Glide Path Angle (GPA) ✅
pub type _5_226_GlidePathAngleRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.227 – Orthometric Height (ORTH HGT) ✅
pub type _5_227_OrthometricHeightRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.228 – Course Width At Threshold (CRS WDTH) ✅
pub type _5_228_CourseWidthAtThresholdRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.229 – Final Approach Segment Data CRC Remainder (FAS CRC) ✅
pub type _5_229_FinalApproachSegmentDataCrcRemainderRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 8>;
/// 5.230 – Procedure Type (PROC TYPE) ✅
pub type _5_230_FlightPlanningProcedureTypeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.231 – Along Track Distance (ATD) ✅
pub type _5_231_FlightPlanningAlongTrackDistanceRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.232 – Number of Engines Restriction (NOE) ✅
pub type _5_232_FlightPlanningEnginesRestrictionRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 4>;
/// 5.233 – Turboprop/Jet Indicator (TURBO) ✅
pub type _5_233_FlightPlanningTurbopropJetIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.234 – RNAV Flag (RNAV) ✅
pub type _5_234_FlightPlanningRnavFlagRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.235 – ATC Weight Category (ATC WC) ✅
pub type _5_235_FlightPlanningAtcWeightCategoryRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.236 – ATC Identifier (ATC ID) ✅
pub type _5_236_FlightPlanningAtcIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 7>;
/// 5.237 – Procedure Description (PROC DESC) ✅
pub type _5_237_FlightPlanningProcedureDescriptionRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 15>;
/// 5.238 – Leg Type Code (LTC) ✅
pub type _5_238_FlightPlanningLegTypeCodeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 2>;
/// 5.239 – Reporting Code (RPT) ✅
pub type _5_239_FlightPlanningReportingCodeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.240 – Altitude (ALT) ✅
pub type _5_240_FlightPlanningAltitudeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.241 – Fix Related Transition Code (FRT Code) ✅
pub type _5_241_FlightPlanningFixRelatedTransitionCodeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 1>;
/// 5.242 – Procedure Category (PROC CAT) ✅
pub type _5_242_SidStarApproachProcedureCategoryRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 4>;
/// 5.243 – GLS Station Identifier (4 characters max) ✅
pub type _5_243_GlsStationIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.244 – SBAS/GBAS Channel ✅
pub type _5_244_SbasGbasChannelRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.245 – Service Volume Radius ✅
pub type _5_245_GlsServiceVolumeRadiusRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.246 – TDMA Slots ✅
pub type _5_246_GlsTdmaSlotsRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.247 – Station Type ✅
pub type _5_247_GlsStationTypeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.248 – Station Elevation WGS84 ✅
pub type _5_248_GlsStationElevationWgs84Raw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.249 – Surface Code (SC) ✅
pub type _5_249_SurfaceCodeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.250 – Alternate Record Type (ART) ✅
pub type _5_250_AlternateRecordTypeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 2>;
/// 5.251 – Distance To Alternate (DTA) (3 characters max) ✅
pub type _5_251_DistanceToAlternateRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.252 – Alternate Type (ALT TYPE) ✅
pub type _5_252_AlternateTypeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.253 – Primary and Additional Alternate Identifier (ALT IDENT) (10 characters max) ✅
pub type _5_253_AlternateIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 10>;
/// 5.254 – Fixed Radius Transition Indicator (FIXED RAD IND) ✅
pub type _5_254_FixedRadiusTransitionIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.255 – SBAS Service Provider Identifier (SBAS ID) ✅
pub type _5_255_SbasServiceProviderIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.256 – Reference Path Data Selector (REF PDS) ✅
pub type _5_256_ReferencePathDataSelectorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.257 – Reference Path Identifier (REF ID) ✅
pub type _5_257_ReferencePathIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.258 – Approach Performance Designator (APD) ✅
pub type _5_258_ApproachPerformanceDesignatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 1>;
/// 5.259 – Length Offset (OFFSET) ✅
pub type _5_259_PathPointLengthOffsetRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.260 – Terminal Procedure Flight Planning Leg Distance (LEG DIST) ✅
pub type _5_260_TerminalProcedureFlightPlanningLegDistanceRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.261 – Speed Limit Description (SLD) ✅
pub type _5_261_SpeedLimitDescriptionRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.262 – Approach Type Identifier (ATI) ✅
pub type _5_262_ApproachTypeIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 10>;
/// 5.263 – Horizontal Alert Limit (HAL) / Lateral Alert Limit (LAL) ✅
pub type _5_263_HorizontalOrLateralAlertLimitRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.264 – Vertical Alert Limit (VAL) ✅
pub type _5_264_VerticalAlertLimitRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.265 – Path Point TCH ✅
pub type _5_265_PathPointTchRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 6>;
/// 5.266 – TCH Units Indicator ✅
pub type _5_266_TchUnitsIndicatorRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.267 – High Precision Latitude (HPLAT) ✅
pub type _5_267_HighPrecisionLatitudeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 11>;
/// 5.268 – High Precision Longitude (HPLONG) ✅
pub type _5_268_HighPrecisionLongitudeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 12>;
/// 5.269 – Helicopter Procedure Course (HPC) ✅
pub type _5_269_HelicopterProcedureCourseRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.270 – TCH Value Indicator (TCHVI) ✅
pub type _5_270_TchValueIndicatorRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.271 – Procedure Turn (PROC TURN) ✅
pub type _5_271_TaaProcedureTurnRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.272 – TAA Sector Identifier ✅
pub type _5_272_TaaSectorIdentifierRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.273 – TAA Waypoint (5-character max) ✅
pub type _5_273_TaaWaypointRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.274 – TAA Sector Radius ✅
pub type _5_274_TaaSectorRadiusRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.275 – Level of Service Name (LSN) ✅
pub type _5_275_LevelOfServiceNameRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 10>;
/// 5.276 – Level of Service Authorized ✅
pub type _5_276_LevelOfServiceAuthorizedRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.277 – DME Operational Service Volume (D-OSV) ✅
pub type _5_277_DmeOperationalServiceVolumeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.278 – Activity Type ✅
pub type _5_278_SpecialActivityAreaTypeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.279 – Activity Identifier ✅
pub type _5_279_SpecialActivityAreaIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.280 – Special Activity Area Size ✅
pub type _5_280_SpecialActivityAreaSizeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.281 – Special Activity Area Volume ✅
pub type _5_281_SpecialActivityAreaVolumeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.282 – Special Activity Area Operating Times ✅
pub type _5_282_SpecialActivityAreaOperatingTimesRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 3>;
/// 5.283 – Communications Class (Comm Class) ✅
pub type _5_283_CommunicationsClassRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 4>;
/// 5.284 – Assigned Sector Name (ASN) (25 characters max) ✅
pub type _5_284_AssignedSectorNameRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 25>;
/// 5.285 – Time Narrative (100 characters max per record) ✅
pub type _5_285_CommunicationsTimeNarrativeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 100>;
/// 5.286 – Multi-Sector Indicator (MSEC IND) ✅
pub type _5_286_MultiSectorIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.287 – Type Recognized By (TRB) ✅
pub type _5_287_CommunicationsTypeRecognizedByRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.288 – Translation (80 characters max) ✅
pub type _5_288_CommunicationsTypeTranslationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 80>;
/// 5.289 – Used On ✅
pub type _5_289_CommunicationsTypeTranslationTableUsedOnRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.290 – Procedure Design Mag Var (PDMV) ✅
pub type _5_290_ProcedureDesignMagneticVariationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.291 – Procedure Design Mag Var Indicator (PDMVI) ✅
pub type _5_291_ProcedureDesignMagneticVariationIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.292 – Category Distance ✅
pub type _5_292_CirclingCategoryDistanceRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.293 – Vertical Scale Factor (VSF) ✅
pub type _5_293_VerticalScaleFactorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.294 – RVSM Minimum Level ✅
pub type _5_294_RvsmMinimumLevelRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.295 – RVSM Maximum Level ✅
pub type _5_295_RvsmMaximumLevelRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.296 – RNP Level of Service (LSN) ✅
pub type _5_296_RnpApproachLevelOfServiceNameRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.297 – Route Inappropriate Navaid Indicator ✅
pub type _5_297_RouteInappropriateNavaidIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.298 – Holding Pattern/Race Track Course Reversal Leg Inbound/Outbound Indicator ✅
pub type _5_298_HoldingLegInboundOutboundIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.299 – Procedure Referenced Fix Identifier ✅
pub type _5_299_ProcedureReferencedFixIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.300 – Final Approach Course as Runway ✅
pub type _5_300_FinalApproachCourseAsRunwayRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.301 – Procedure Design Aircraft Category or Type ✅
pub type _5_301_ProcedureDesignAircraftCategoryOrTypeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.302 – Surface Type ✅
pub type _5_302_RunwaySurfaceTypeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 4>;
/// 5.303 – Helipad Shape ✅
pub type _5_303_HelipadShapeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.304 – Sector Bearing Reference Waypoint (5 characters max) ✅
pub type _5_304_TaaSectorBearingReferenceWaypointRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.305 – Heliport Type ✅
pub type _5_305_HeliportTypeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.306 – Preferred Multiple Approach Indicator ✅
pub type _5_306_PreferredMultipleApproachIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.307 – Special Indicator ✅
pub type _5_307_TerminalProcedureSpecialIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.308 – Remote Altimeter Flag ✅
pub type _5_308_RemoteAltimeterFlagRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.309 – Maximum Allowable Helicopter Weight ✅
pub type _5_309_MaximumAllowableHelicopterWeightRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.310 – Helicopter Performance Requirement (M/S/U; length 1 per code table) ✅
pub type _5_310_HelipadPerformanceRequirementRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.311 – FIR/FRA Transition ✅
pub type _5_311_FirFraTransitionRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.312 – StartCOLer Extension ✅
pub type _5_312_RunwayStartCOLerExtensionRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.313 – TORA ✅
pub type _5_313_RunwayToraRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.314 – TODA ✅
pub type _5_314_RunwayTodaRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.315 – ASDA ✅
pub type _5_315_RunwayAsdaRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.316 – LDA ✅
pub type _5_316_RunwayLdaRaw<const STARTCOL: usize> = FieldRaw<DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.317 – Runway Usage Indicator ✅
pub type _5_317_RunwayUsageIndicatorRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.318 – Runway Accuracy Compliance Flag ✅
pub type _5_318_RunwayAccuracyComplianceFlagRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.319 – Landing Threshold Elevation Accuracy Compliance Flag ✅
pub type _5_319_LandingThresholdElevationAccuracyComplianceFlagRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.320 – SBAS Final Approach Course ✅
pub type _5_320_SbasFinalApproachCourseRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.321 – Helipad Maximum Rotor Diameter ✅
pub type _5_321_HelipadMaximumRotorDiameterRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.322 – Helipad Type ✅
pub type _5_322_HelipadTypeRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.323 – Helipad Orientation ✅
pub type _5_323_HelipadOrientationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.324 – Helipad Identifier Orientation ✅
pub type _5_324_HelipadIdentifierOrientationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.325 – Preferred Approach Bearing ✅
pub type _5_325_PreferredApproachBearingRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.326 – Ground Facility Identifier ✅
pub type _5_326_AtnGroundFacilityIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 8>;
/// 5.327 – Authority Format Identifier (AFI) ✅
pub type _5_327_AtnAuthorityFormatIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.328 – Initial Domain Identifier ✅
pub type _5_328_AtnInitialDomainIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.329 – Version (VER) ✅
pub type _5_329_AtnVersionRaw<const STARTCOL: usize> = FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.330 – Administration (ADM) ✅
pub type _5_330_AtnAdministrationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.331 – Routing Domain Format (RDF) ✅
pub type _5_331_AtnRoutingDomainFormatRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.332 – Administrative Region Selector (ARS) ✅
pub type _5_332_AtnAdministrativeRegionSelectorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.333 – Location (LOC) (NSAP routing location subfield) ✅
pub type _5_333_AtnRoutingLocationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.334 – System Identifier (SYS) ✅
pub type _5_334_AtnSystemIdentifierRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 12>;
/// 5.335 – Network Service Access Point Selector (NSEL) ✅
pub type _5_335_AtnNetworkServiceAccessPointSelectorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.336 – Context Management Transport Selector (CM TSEL) ✅
pub type _5_336_AtnContextManagementTransportSelectorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.337 – Use Indicator (ATN ATSU ground facility) ✅
pub type _5_337_AtnGroundFacilityUseIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.338 – VOR Range/Power (VORPWR) ✅
pub type _5_338_VhfNavaidVorRangePowerRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.339 – DME Expanded Service Volume (DESV) ✅
pub type _5_339_VhfNavaidDmeExpandedServiceVolumeRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.340 – Unmanned Aerial Vehicle (UAV) Only ✅ (possibly error in specification)
pub type _5_340_UnmannedAerialVehicleOperationsOnlyRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.341 – Military Indicator (terminal SID/STAR/APP) ✅
pub type _5_341_TerminalProcedureMilitaryIndicatorRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.342 – Source of LAL/VAL ✅
pub type _5_342_SbasApproachMinimaLalValSourceRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHA, STARTCOL, 1>;
/// 5.343 – Holding Pattern Magnetic Variation (HPMV) ✅
pub type _5_343_HoldingPatternMagneticVariationRaw<const STARTCOL: usize> =
    FieldRaw<DTYPE_ALPHANUMERIC, STARTCOL, 5>;
