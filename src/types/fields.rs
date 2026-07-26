//! # ARINC 424 Field Parsers
//! This module contains the core type converters for the ARINC 424 field types.
//!
//! Not all types can be used commonly, so we allow version specifications to derive their own in that
//! revision package.
//!
//! Later improvements could be to find more commonality but it won't be prio'd now since it generally
//! will get things forward in a stable manner without worry about breakage across revisions.
//!
#[cfg(test)]
use crate::test_util::assert_within_epsilon;
use std::convert::Into;
use std::fmt::Debug;
use std::str::FromStr;

pub trait ParseableField: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError>;
}

pub const BLANK: u8 = b' ';
#[derive(Debug, PartialEq, Eq)]
pub struct FieldParseError {
    pub message: String,
    pub column: Option<usize>,
    pub column_type: Option<Box<str>>,
}

impl FieldParseError {
    pub fn new(message: String) -> Self {
        Self {
            message,
            column: None,
            column_type: None,
        }
    }
    pub fn with_context(self, field_type: &str, column: usize) -> Self {
        Self {
            message: self.message,
            column: Some(column),
            column_type: Some(Box::from(field_type)),
        }
    }
}

pub fn coalesce_into_number<T: FromStr>(bytes: &[u8]) -> Result<T, FieldParseError> where {
    if let Ok(utf8_str) = std::str::from_utf8(bytes) {
        if let Ok(value) = T::from_str(utf8_str) {
            return Ok(value);
        }
    }
    return Err(FieldParseError::new(format!(
        "Numeric is not a valid {}: {}",
        std::any::type_name::<T>(),
        std::str::from_utf8(bytes).unwrap_or("unknown error")
    )));
}

#[test]
pub fn test_coalesce_into_number() {
    let r = coalesce_into_number::<u16>(b"1234");
    if let Ok(value) = r {
        assert_eq!(value, 1234);
    } else {
        panic!("Failed to coalesce into number: {:?}", r);
    }
    let r = coalesce_into_number::<u16>(b"1234a");
    if let Err(e) = r {
        assert_eq!(e.message, "Numeric is not a valid u16: 1234a");
    } else {
        panic!("Expected error, got {:?}", r);
    }
    let r: Result<u64, FieldParseError> = coalesce_into_number(b"12345678901234567890");
    if let Ok(value) = r {
        assert_eq!(value, 12345678901234567890);
    } else {
        panic!("Failed to coalesce into number: {:?}", r);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UintNumeric(pub u64);
impl ParseableField for UintNumeric {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let trimmed_bytes = bytes.trim_ascii();
        if trimmed_bytes.is_empty() {
            return Ok(None);
        }
        let value = coalesce_into_number::<u64>(trimmed_bytes)?;
        Ok(Some(UintNumeric(value)))
    }
}

impl Into<u64> for UintNumeric {
    fn into(self: UintNumeric) -> u64 {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntNumeric(pub i64);
impl ParseableField for IntNumeric {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let trimmed_bytes = bytes.trim_ascii();
        if trimmed_bytes.is_empty() {
            return Ok(None);
        }
        let value = coalesce_into_number::<i64>(trimmed_bytes)?;
        Ok(Some(IntNumeric(value)))
    }
}

impl Into<i64> for IntNumeric {
    fn into(self: IntNumeric) -> i64 {
        self.0
    }
}

#[test]
pub fn test_int_numeric() {
    let r = IntNumeric::from_bytes(b"001");
    if let Ok(Some(IntNumeric(value))) = r {
        assert_eq!(value, 1);
    } else {
        panic!("Failed to parse int numeric");
    }
    let r = IntNumeric::from_bytes(b"+001");
    if let Ok(Some(IntNumeric(value))) = r {
        assert_eq!(value, 1);
    } else {
        panic!("Failed to parse int numeric");
    }
    let r = IntNumeric::from_bytes(b" -001");
    if let Ok(Some(IntNumeric(value))) = r {
        assert_eq!(value, -1);
    } else {
        panic!("Failed to parse int numeric");
    }
}

#[derive(Debug, PartialEq)]
pub struct FloatNumeric<const RADIX_SHIFT: i32 = 0>(pub f64);
impl<const RADIX_SHIFT: i32> ParseableField for FloatNumeric<RADIX_SHIFT> {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let trimmed_bytes = bytes.trim_ascii();
        if trimmed_bytes.is_empty() {
            return Ok(None);
        }
        let int_val = i64::from_str_radix(
            std::str::from_utf8(trimmed_bytes)
                .map_err(|e| FieldParseError::new(format!("Numeric is not valid UTF-8: {}", e)))?,
            10,
        )
        .map_err(|e| FieldParseError::new(format!("Numeric is not a valid u64: {}", e)))?;
        Ok(Some(FloatNumeric(
            int_val as f64 * 10_f64.powi(RADIX_SHIFT),
        )))
    }
}

impl<const RADIX_SHIFT: i32> Into<f64> for FloatNumeric<RADIX_SHIFT> {
    fn into(self: FloatNumeric<RADIX_SHIFT>) -> f64 {
        self.0
    }
}

#[test]
pub fn test_float_numeric() {
    let r = FloatNumeric::<0>::from_bytes(b"001");
    if let Ok(Some(f)) = r {
        let val: f64 = f.into();
        assert_within_epsilon(val, 1.0);
    } else {
        panic!("Failed to parse float numeric");
    }

    let r = FloatNumeric::<-1>::from_bytes(b"+2761");
    if let Ok(Some(f)) = r {
        let val: f64 = f.into();
        assert_within_epsilon(val, 276.1);
    } else {
        panic!("Failed to parse float numeric");
    }
    let r = FloatNumeric::<-1>::from_bytes(b" -2761");
    if let Ok(Some(f)) = r {
        let val: f64 = f.into();
        assert_within_epsilon(val, -276.1);
    } else {
        panic!("Failed to parse float numeric");
    }
}

#[derive(Debug, PartialEq)]
pub struct VariableFloatNumeric<const RADIX_DIRECTION: i32 = 1>(pub f64);
impl<const RADIX_DIRECTION: i32> VariableFloatNumeric<RADIX_DIRECTION> {
    const _VALID_RADIX_DIRECTIONS: () = assert!(
        RADIX_DIRECTION == 1 || RADIX_DIRECTION == -1,
        "Invalid RADIX_DIRECTION"
    );
}
impl<const RADIX_DIRECTION: i32> ParseableField for VariableFloatNumeric<RADIX_DIRECTION> {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let _ = Self::_VALID_RADIX_DIRECTIONS;
        let trimmed_bytes = bytes.trim_ascii();
        if trimmed_bytes.is_empty() {
            return Ok(None);
        }
        // get last byte in the slice and use that as the RADIX_SHIFT
        let radix_shift = bytes[bytes.len() - 1] - b'0';
        let numeric_value = coalesce_into_number::<i64>(&trimmed_bytes[..trimmed_bytes.len() - 1])?;
        let value = numeric_value as f64 * 10_f64.powi(RADIX_DIRECTION * radix_shift as i32);
        Ok(Some(VariableFloatNumeric(value)))
    }
}

impl<const RADIX_DIRECTION: i32> Into<f64> for VariableFloatNumeric<RADIX_DIRECTION> {
    fn into(self: VariableFloatNumeric<RADIX_DIRECTION>) -> f64 {
        self.0
    }
}

#[test]

pub fn test_variable_float_numeric() {
    let r = VariableFloatNumeric::<-1>::from_bytes(b"013");
    if let Ok(Some(f)) = r {
        let val: f64 = f.into();
        assert_within_epsilon(val, 0.001);
    }
    let r = VariableFloatNumeric::<1>::from_bytes(b" 013");
    if let Ok(Some(f)) = r {
        let val: f64 = f.into();
        assert_within_epsilon(val, 1000.0);
    }
}

// This is special since it has two values in one field because of the way
// some fields are handled.
#[derive(Debug, PartialEq)]
pub enum TimeDistanceNumeric<const RADIX_SHIFT: i32 = 0> {
    Time(FloatNumeric<RADIX_SHIFT>),
    Distance(FloatNumeric<RADIX_SHIFT>),
}

impl<const RADIX_SHIFT: i32> ParseableField for TimeDistanceNumeric<RADIX_SHIFT> {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let trimmed_bytes = bytes.trim_ascii();
        if trimmed_bytes.is_empty() {
            return Ok(None);
        }
        // if starts with T, it is a time
        if trimmed_bytes.starts_with(b"T") {
            let time = FloatNumeric::<RADIX_SHIFT>::from_bytes(&trimmed_bytes[1..])?;
            if let Some(time) = time {
                return Ok(Some(TimeDistanceNumeric::Time(time)));
            }
            return Ok(None);
        }
        // otherwise it is a distance
        let distance = FloatNumeric::<RADIX_SHIFT>::from_bytes(&trimmed_bytes)?;
        if let Some(distance) = distance {
            return Ok(Some(TimeDistanceNumeric::Distance(distance)));
        }
        return Ok(None);
    }
}

impl<const RADIX_SHIFT: i32> Into<f64> for TimeDistanceNumeric<RADIX_SHIFT> {
    fn into(self: TimeDistanceNumeric<RADIX_SHIFT>) -> f64 {
        match self {
            TimeDistanceNumeric::Time(t) => t.into(),
            TimeDistanceNumeric::Distance(d) => d.into(),
        }
    }
}
#[test]

pub fn test_time_distance_numeric() {
    let r = TimeDistanceNumeric::<-1>::from_bytes(&[b'T', b'0', b'0', b'1']);
    if let Ok(Some(TimeDistanceNumeric::Time(t))) = r {
        let val: f64 = t.into();
        assert_within_epsilon(val, 0.1);
    } else {
        panic!("Failed to parse time distance numeric");
    }
    let r = TimeDistanceNumeric::<-1>::from_bytes(&[b'1', b'0', b'0', b'1']);
    if let Ok(Some(TimeDistanceNumeric::Distance(d))) = r {
        let val: f64 = d.into();
        assert_within_epsilon(val, 100.1);
    } else {
        panic!("Failed to parse time distance numeric");
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AltitudeNumeric(pub i32);
impl ParseableField for AltitudeNumeric {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let trimmed_bytes = bytes.trim_ascii();
        if trimmed_bytes.is_empty() {
            return Ok(None);
        }
        // if starts with FL, it is a flight level and we convert to altitude
        if trimmed_bytes.starts_with(b"FL") {
            let value = coalesce_into_number::<i32>(&trimmed_bytes[2..])?;
            return Ok(Some(AltitudeNumeric(value * 100)));
        }
        // otherwise it is an altitude
        let value = coalesce_into_number::<i32>(trimmed_bytes)?;
        Ok(Some(AltitudeNumeric(value as i32)))
    }
}

impl Into<i32> for AltitudeNumeric {
    fn into(self: AltitudeNumeric) -> i32 {
        self.0
    }
}

#[test]
pub fn test_altitude() {
    let r = AltitudeNumeric::from_bytes(&[b'1', b'0', b'0', b'0']);
    if let Ok(Some(AltitudeNumeric(altitude))) = r {
        assert_eq!(altitude, 1000);
    } else {
        panic!("Failed to parse altitude");
    }
    let r = AltitudeNumeric::from_bytes(&[b'F', b'L', b'1', b'0', b'0']);
    if let Ok(Some(AltitudeNumeric(altitude))) = r {
        assert_eq!(altitude, 10000);
    } else {
        panic!("Failed to parse altitude");
    }
    let r = AltitudeNumeric::from_bytes(&[b'-', b'1', b'1', b'0', b'0']);
    if let Ok(Some(AltitudeNumeric(altitude))) = r {
        assert_eq!(altitude, -1100);
    } else {
        panic!("Failed to parse altitude");
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MultiUnitAltitudeNumeric {
    Meters(AltitudeNumeric),
    Feet(AltitudeNumeric),
}

impl ParseableField for MultiUnitAltitudeNumeric {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let trimmed_bytes = bytes.trim_ascii();
        if trimmed_bytes.is_empty() {
            return Ok(None);
        }
        match bytes {
            [b'M', rest @ ..] => {
                let value = AltitudeNumeric::from_bytes(rest)?;
                if let Some(value) = value {
                    Ok(Some(MultiUnitAltitudeNumeric::Meters(value)))
                } else {
                    Ok(None)
                }
            }
            _ => {
                let value = AltitudeNumeric::from_bytes(bytes)?;
                if let Some(value) = value {
                    Ok(Some(MultiUnitAltitudeNumeric::Feet(value)))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

impl Into<i32> for MultiUnitAltitudeNumeric {
    fn into(self: MultiUnitAltitudeNumeric) -> i32 {
        match self {
            MultiUnitAltitudeNumeric::Meters(m) => m.into(),
            MultiUnitAltitudeNumeric::Feet(f) => f.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MinimumAltitudeNumeric {
    Established(AltitudeNumeric),
    Unknown,
    NotEstablished,
}

impl ParseableField for MinimumAltitudeNumeric {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let trimmed_bytes = bytes.trim_ascii();
        if trimmed_bytes.is_empty() {
            return Ok(None);
        }
        match trimmed_bytes {
            b"UNKNN" => Ok(Some(MinimumAltitudeNumeric::Unknown)),
            b"NESTB" => Ok(Some(MinimumAltitudeNumeric::NotEstablished)),
            _ => {
                let altitude = AltitudeNumeric::from_bytes(&bytes)?;
                if let Some(altitude) = altitude {
                    Ok(Some(MinimumAltitudeNumeric::Established(altitude)))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

#[test]
pub fn test_altitude_minimum_altitude() {
    let r = MinimumAltitudeNumeric::from_bytes(&[b'F', b'L', b'1', b'0', b'0']);
    if let Ok(Some(MinimumAltitudeNumeric::Established(altitude))) = r {
        assert_eq!(altitude, AltitudeNumeric(10000));
    } else {
        panic!("Failed to parse altitude minimum altitude");
    }
    let r = MinimumAltitudeNumeric::from_bytes(&[b'-', b'1', b'1', b'0', b'0']);
    if let Ok(Some(MinimumAltitudeNumeric::Established(altitude))) = r {
        assert_eq!(altitude, AltitudeNumeric(-1100));
    } else {
        panic!("Failed to parse altitude minimum altitude");
    }
}

#[derive(Debug, PartialEq)]
pub struct LatitudeNumeric(pub f64);
impl ParseableField for LatitudeNumeric {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let trimmed_bytes = bytes.trim_ascii();
        if trimmed_bytes.is_empty() {
            return Ok(None);
        }
        let sign = if trimmed_bytes[0] == b'S' { -1.0 } else { 1.0 };
        let degrees = coalesce_into_number::<u16>(&trimmed_bytes[1..3])?;
        let minutes = coalesce_into_number::<u16>(&trimmed_bytes[3..5])?;
        let seconds = coalesce_into_number::<u16>(&trimmed_bytes[5..7])?;
        // rest of the bytes are on the other side of the decimal point
        let decimal = coalesce_into_number::<u64>(&trimmed_bytes[7..])?;
        let decimal_fraction = decimal as f64 / 10_f64.powi((trimmed_bytes.len() - 7) as i32);
        let value = sign
            * (degrees as f64
                + minutes as f64 / 60.0
                + (seconds as f64 + decimal_fraction) / 3600.0);
        Ok(Some(LatitudeNumeric(value)))
    }
}

impl Into<f64> for LatitudeNumeric {
    fn into(self: LatitudeNumeric) -> f64 {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct LongitudeNumeric(pub f64);
impl ParseableField for LongitudeNumeric {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let trimmed_bytes = bytes.trim_ascii();
        if trimmed_bytes.is_empty() {
            return Ok(None);
        }
        let sign = if trimmed_bytes[0] == b'W' { -1.0 } else { 1.0 };
        let degrees = coalesce_into_number::<u16>(&trimmed_bytes[1..4])?;
        let minutes = coalesce_into_number::<u16>(&trimmed_bytes[4..6])?;
        let seconds = coalesce_into_number::<u16>(&trimmed_bytes[6..8])?;
        // rest of the bytes are on the other side of the decimal point
        let decimal = coalesce_into_number::<u64>(&trimmed_bytes[8..])?;
        let decimal_fraction = decimal as f64 / 10_f64.powi((trimmed_bytes.len() - 8) as i32);
        let value = sign
            * (degrees as f64
                + minutes as f64 / 60.0
                + (seconds as f64 + decimal_fraction) / 3600.0);
        Ok(Some(LongitudeNumeric(value)))
    }
}

impl Into<f64> for LongitudeNumeric {
    fn into(self: LongitudeNumeric) -> f64 {
        self.0
    }
}

#[test]
pub fn test_longitude() {
    let r = LongitudeNumeric::from_bytes(b" W039513881");
    if let Ok(Some(LongitudeNumeric(longitude))) = r {
        assert_within_epsilon(longitude, -39.860780556);
    }
    let r = LongitudeNumeric::from_bytes(b"W039513881123 ");
    if let Ok(Some(LongitudeNumeric(longitude))) = r {
        assert_within_epsilon(longitude, -39.860780556123);
    }
}

#[derive(Debug, PartialEq)]
pub struct MagneticVariationNumeric(pub f64);
impl ParseableField for MagneticVariationNumeric {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let trimmed_bytes = bytes.trim_ascii();
        if trimmed_bytes.is_empty() {
            return Ok(None);
        }
        let sign = match trimmed_bytes[0] {
            b'E' => 1.0,
            b'W' => -1.0,
            b'T' => 0.0,
            b'G' => 0.0,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid magnetic variation".to_string(),
                ));
            }
        };
        let value = 0.1 * sign * coalesce_into_number::<u32>(&trimmed_bytes[1..])? as f64;
        Ok(Some(MagneticVariationNumeric(value)))
    }
}
impl Into<f64> for MagneticVariationNumeric {
    fn into(self: MagneticVariationNumeric) -> f64 {
        self.0
    }
}

#[test]
pub fn test_magnetic_variation() {
    let r = MagneticVariationNumeric::from_bytes(b"W0010");
    if let Ok(Some(MagneticVariationNumeric(magnetic_variation))) = r {
        assert_within_epsilon(magnetic_variation, -1.0);
    }
}

/// Declination Numeric at first glance seems similar to Magnetic Variation, but is actually used in
/// much more different contexts that need different semantic handling of values.
#[derive(Debug, PartialEq)]
pub enum DeclinationNumeric {
    StandardDeclination(f64),
    TrueNorth(f64),
    GridNorth(f64),
}

impl ParseableField for DeclinationNumeric {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let trimmed_bytes = bytes.trim_ascii();
        if trimmed_bytes.is_empty() {
            return Ok(None);
        }
        let sign = match trimmed_bytes[0] {
            b'E' => 1.0,
            b'W' => -1.0,
            b'T' => 0.0,
            b'G' => 0.0,
            _ => {
                return Err(FieldParseError::new("Invalid declination".to_string()));
            }
        };
        let value = sign * 0.1 * coalesce_into_number::<u32>(&trimmed_bytes[1..])? as f64;
        Ok(Some(match trimmed_bytes[0] {
            b'E' => DeclinationNumeric::StandardDeclination(value),
            b'W' => DeclinationNumeric::StandardDeclination(value),
            b'T' => DeclinationNumeric::TrueNorth(value),
            b'G' => DeclinationNumeric::GridNorth(value),
            _ => {
                return Err(FieldParseError::new("Invalid declination".to_string()));
            }
        }))
    }
}

#[test]
pub fn test_declination() {
    let r = DeclinationNumeric::from_bytes(b"E0010");
    if let Ok(Some(DeclinationNumeric::StandardDeclination(declination))) = r {
        assert_within_epsilon(declination, 1.0);
    }
    let r = DeclinationNumeric::from_bytes(b"W0010");
    if let Ok(Some(DeclinationNumeric::StandardDeclination(declination))) = r {
        assert_within_epsilon(declination, -1.0);
    }
    let r = DeclinationNumeric::from_bytes(b"T0010");
    if let Ok(Some(DeclinationNumeric::TrueNorth(declination))) = r {
        assert_within_epsilon(declination, 0.0);
    }
    let r = DeclinationNumeric::from_bytes(b"G0010");
    if let Ok(Some(DeclinationNumeric::GridNorth(declination))) = r {
        assert_within_epsilon(declination, 0.0);
    }
}

/// Bearing Numeric at first glance seems similar to Declination Numeric, but in this case the indicator for True is the only one
/// to look for and is at the end of the byte array passed in.
#[derive(Debug, PartialEq)]
pub enum BearingNumeric {
    Magnetic(f64),
    True(f64),
}

impl ParseableField for BearingNumeric {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let trimmed_bytes = bytes.trim_ascii();
        if trimmed_bytes.is_empty() {
            return Ok(None);
        }
        if trimmed_bytes.ends_with(b"T") {
            let value =
                (coalesce_into_number::<u32>(&trimmed_bytes[..trimmed_bytes.len() - 1])?) as f64;
            return Ok(Some(BearingNumeric::True(value as f64)));
        } else {
            let value = 0.1 * coalesce_into_number::<u32>(&trimmed_bytes)? as f64;
            return Ok(Some(BearingNumeric::Magnetic(value)));
        }
    }
}

#[test]
pub fn test_bearing() {
    let r = BearingNumeric::from_bytes(b"0010");
    if let Ok(Some(BearingNumeric::Magnetic(bearing))) = r {
        assert_within_epsilon(bearing, 1.0);
    }
    let r = BearingNumeric::from_bytes(b"0010");
    if let Ok(Some(BearingNumeric::Magnetic(bearing))) = r {
        assert_within_epsilon(bearing, 1.0);
    }
    let r = BearingNumeric::from_bytes(b"347T");
    if let Ok(Some(BearingNumeric::True(bearing))) = r {
        assert_within_epsilon(bearing, 347.0);
    }
}

// Identifiers

fn validate_alphanumeric(bytes: &[u8]) -> Result<(), FieldParseError> {
    if !bytes
        .iter()
        .all(|&b| b.is_ascii_alphanumeric() || b.is_ascii_punctuation() || b == b' ')
    {
        return Err(FieldParseError::new(
            "Alphanumeric identifier is not alphanumeric".to_string(),
        ));
    }
    Ok(())
}

#[test]
fn test_validate_alphanumeric() {
    assert!(validate_alphanumeric(b"ABC123").is_ok());
    assert!(validate_alphanumeric(b"ABC 123").is_ok());
    assert!(validate_alphanumeric(b"ABC\x00123").is_err());
}

#[derive(Debug, PartialEq, Eq)]
pub struct LengthLimitedIdentifier<const LEN: usize, const EXACT: bool>(Box<str>);
impl<const LEN: usize, const EXACT: bool> ParseableField for LengthLimitedIdentifier<LEN, EXACT> {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        validate_alphanumeric(bytes)?;
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        if EXACT && bytes.len() != LEN {
            return Err(FieldParseError::new(
                "Identifier is not the exact length".to_string(),
            ));
        } else if !EXACT && bytes.len() > LEN {
            return Err(FieldParseError::new(format!(
                "Identifier is too long, expected {} characters, got {} value: {}",
                LEN,
                bytes.len(),
                String::from_utf8_lossy(bytes)
            )));
        }
        Ok(Some(LengthLimitedIdentifier::<LEN, EXACT>(Box::from(
            std::str::from_utf8(bytes)
                .map_err(|e| FieldParseError::new(format!("Identifier is not valid UTF-8: {}", e)))?
                .trim_end(),
        ))))
    }
}

impl<const LEN: usize, const EXACT: bool> AsRef<str> for LengthLimitedIdentifier<LEN, EXACT> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<const LEN: usize, const EXACT: bool> Into<Box<str>> for LengthLimitedIdentifier<LEN, EXACT> {
    fn into(self: LengthLimitedIdentifier<LEN, EXACT>) -> Box<str> {
        self.0
    }
}
