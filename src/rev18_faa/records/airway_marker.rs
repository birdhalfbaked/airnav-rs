use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct AirwaysMarkerRecords;
impl AirwaysMarkerRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::AirwaysMarkerPrimary(
                AirwaysMarkerPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN
                    ..Self::CONTINUATION_APPLICATION_COLUMN + 1],
            )? {
                Some(ContinuationRecordApplicationType::PrimaryRecordExtension) => Ok(
                    ARINCRecord::AirwaysMarkerPrimary(AirwaysMarkerPrimaryRecord::parse(input)?),
                ),
                _ => Err(RecordParseError::new(
                    "Invalid continuation record application type".to_string(),
                    Some(String::from_utf8_lossy(input).into_owned()),
                )),
            }
        }
    }
}

/// 4.1.15.1 Airways Marker Primary Record
#[derive(Debug)]
pub struct AirwaysMarkerPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub marker_identifier: RecordField<'a, MarkerIdentifier>,
    pub marker_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub marker_code: RecordField<'a, MarkerCode>,
    pub marker_shape: RecordField<'a, MarkerRadiationShape>,
    pub marker_power: RecordField<'a, EnrouteMarkerPower>,
    pub marker_latitude: RecordField<'a, Latitude>,
    pub marker_longitude: RecordField<'a, Longitude>,
    pub minor_axis_bearing: RecordField<'a, MinorAxisBearing>,
    pub magnetic_variation: RecordField<'a, MagneticVariation>,
    pub facility_elevation: RecordField<'a, FacilityElevation>,
    pub datum_code: RecordField<'a, DatumCode>,
    pub marker_name: RecordField<'a, NameOfFacility>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for AirwaysMarkerPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "AirwaysMarkerPrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            marker_identifier:            RecordField::from_bytes(input, 14, 4)?,
            marker_icao_code:             RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:   RecordField::from_bytes(input, 22, 1)?,
            marker_code:                  RecordField::from_bytes(input, 23, 4)?,
            marker_shape:                 RecordField::from_bytes(input, 28, 1)?,
            marker_power:                 RecordField::from_bytes(input, 29, 1)?,
            marker_latitude:              RecordField::from_bytes(input, 33, 9)?,
            marker_longitude:             RecordField::from_bytes(input, 42, 10)?,
            minor_axis_bearing:           RecordField::from_bytes(input, 52, 4)?,
            magnetic_variation:           RecordField::from_bytes(input, 75, 5)?,
            facility_elevation:           RecordField::from_bytes(input, 80, 5)?,
            datum_code:                   RecordField::from_bytes(input, 85, 3)?,
            marker_name:                  RecordField::from_bytes(input, 94, 30)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.marker_identifier.raw_bytes,
        ])
    }
}

/// 4.1.15.2 Airways Marker Continuation Record
#[derive(Debug)]
pub struct AirwaysMarkerContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub marker_identifier: RecordField<'a, MarkerIdentifier>,
    pub marker_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for AirwaysMarkerContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "AirwaysMarkerContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            marker_identifier:            RecordField::from_bytes(input, 14, 4)?,
            marker_icao_code:             RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:   RecordField::from_bytes(input, 22, 1)?,
            application_type:             RecordField::from_bytes(input, 23, 1)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.marker_identifier.raw_bytes,
        ])
    }
}
