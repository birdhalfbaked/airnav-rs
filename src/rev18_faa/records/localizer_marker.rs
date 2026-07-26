use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct LocalizerMarkerRecords;
impl LocalizerMarkerRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::LocalizerMarkerPrimary(
                LocalizerMarkerPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    Ok(ARINCRecord::LocalizerMarkerContinuation(
                        LocalizerMarkerContinuationRecord::parse(input)?,
                    ))
                }
                _ => Err(RecordParseError::new(
                    "Invalid continuation record application type".to_string(),
                    Some(String::from_utf8_lossy(input).into_owned()),
                )),
            }
        }
    }
}

/// 4.1.13.1 Localizer Marker Primary Record
#[derive(Debug)]
pub struct LocalizerMarkerPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub localizer_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub marker_type: RecordField<'a, MarkerType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub locator_frequency: RecordField<'a, VORFrequency>,
    pub runway_identifier: RecordField<'a, RunwayIdentifier>,
    pub marker_latitude: RecordField<'a, Latitude>,
    pub marker_longitude: RecordField<'a, Longitude>,
    pub minor_axis_bearing: RecordField<'a, MinorAxisBearing>,
    pub locator_latitude: RecordField<'a, Latitude>,
    pub locator_longitude: RecordField<'a, Longitude>,
    pub locator_class: RecordField<'a, MarkerLocatorNavaidClass>,
    pub locator_facility_characteristics: RecordField<'a, FacilityCharacteristics>,
    pub locator_identifier: RecordField<'a, VORNDBIdentifier>,
    pub magnetic_variation: RecordField<'a, MagneticVariation>,
    pub facility_elevation: RecordField<'a, FacilityElevation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for LocalizerMarkerPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "LocalizerMarkerPrimaryRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                 RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                  RecordField::from_bytes(input, 11, 2)?,
            subsection:                         RecordField::from_bytes(input, 13, 1)?,
            localizer_identifier:               RecordField::from_bytes(input, 14, 4)?,
            marker_type:                        RecordField::from_bytes(input, 18, 3)?,
            continuation_record_number:         RecordField::from_bytes(input, 22, 1)?,
            locator_frequency:                  RecordField::from_bytes(input, 23, 5)?,
            runway_identifier:                  RecordField::from_bytes(input, 28, 5)?,
            marker_latitude:                    RecordField::from_bytes(input, 33, 9)?,
            marker_longitude:                   RecordField::from_bytes(input, 42, 10)?,
            minor_axis_bearing:                 RecordField::from_bytes(input, 52, 4)?,
            locator_latitude:                   RecordField::from_bytes(input, 56, 9)?,
            locator_longitude:                  RecordField::from_bytes(input, 65, 10)?,
            locator_class:                      RecordField::from_bytes(input, 75, 5)?,
            locator_facility_characteristics:   RecordField::from_bytes(input, 80, 5)?,
            locator_identifier:                 RecordField::from_bytes(input, 85, 4)?,
            magnetic_variation:                 RecordField::from_bytes(input, 91, 5)?,
            facility_elevation:                 RecordField::from_bytes(input, 98, 5)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.airport_identifier.raw_bytes,
            self.localizer_identifier.raw_bytes,
        ])
    }
}

/// 4.1.13.2 Localizer Marker Continuation Record
/// Note: this is essentially a reserved record format for future use
#[derive(Debug)]
pub struct LocalizerMarkerContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub localizer_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub marker_type: RecordField<'a, MarkerType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for LocalizerMarkerContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "LocalizerMarkerContinuationRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                 RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                  RecordField::from_bytes(input, 11, 2)?,
            subsection:                         RecordField::from_bytes(input, 13, 1)?,
            localizer_identifier:               RecordField::from_bytes(input, 14, 4)?,
            marker_type:                        RecordField::from_bytes(input, 18, 3)?,
            continuation_record_number:         RecordField::from_bytes(input, 22, 1)?,
            application_type:                   RecordField::from_bytes(input, 23, 1)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.airport_identifier.raw_bytes,
            self.localizer_identifier.raw_bytes,
        ])
    }
}
