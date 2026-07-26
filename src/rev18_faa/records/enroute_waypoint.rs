use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct EnrouteWaypointRecords;
impl EnrouteWaypointRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::EnrouteWaypointPrimary(
                EnrouteWaypointPrimaryRecord::parse(input)?,
            ))
        } else {
            if let Ok(Some(application_type)) = ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            ) {
                match application_type {
                    ContinuationRecordApplicationType::StandardContinuation => {
                        Ok(ARINCRecord::EnrouteWaypointContinuation(
                            EnrouteWaypointContinuationRecord::parse(input)?,
                        ))
                    }
                    ContinuationRecordApplicationType::FlightPlanningContinuation => {
                        Ok(ARINCRecord::EnrouteWaypointFlightPlanningContinuation(
                            EnrouteWaypointFlightPlanningContinuationRecord::parse(input)?,
                        ))
                    }
                    _ => Err(RecordParseError::new(
                        "Invalid continuation record application type".to_string(),
                        Some(String::from_utf8_lossy(input).into_owned()),
                    )),
                }
            } else {
                Ok(ARINCRecord::EnrouteWaypointChangedDataContinuation(
                    EnrouteWaypointChangedDataContinuationRecord::parse(input)?,
                ))
            }
        }
    }
}

/// 4.1.4.1(A) Enroute Waypoint Primary Record
#[derive(Debug)]
pub struct EnrouteWaypointPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub region_code: RecordField<'a, RegionCode>,
    pub region_icao_code: RecordField<'a, IcaoCode>,
    pub waypoint_identifier: RecordField<'a, FixIdentifier>,
    pub waypoint_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub waypoint_type: RecordField<'a, WaypointType>,
    pub waypoint_usage: RecordField<'a, WaypointUsage>,
    pub waypoint_latitude: RecordField<'a, Latitude>,
    pub waypoint_longitude: RecordField<'a, Longitude>,
    pub dynamic_magnetic_variation: RecordField<'a, MagneticVariation>,
    pub datum_code: RecordField<'a, DatumCode>,
    pub name_format_indicator: RecordField<'a, NameFormat>,
    pub waypoint_name_description: RecordField<'a, WaypointNameDescription>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteWaypointPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteWaypointPrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                    RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:             RecordField::from_bytes(input, 2, 3)?,
            section:                        RecordField::from_bytes(input, 5, 1)?,
            subsection:                     RecordField::from_bytes(input, 6, 1)?,
            region_code:                    RecordField::from_bytes(input, 7, 4)?,
            region_icao_code:               RecordField::from_bytes(input, 11, 2)?,
            waypoint_identifier:            RecordField::from_bytes(input, 14, 5)?,
            waypoint_icao_code:             RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:     RecordField::from_bytes(input, 22, 1)?,
            waypoint_type:                  RecordField::from_bytes(input, 27, 3)?,
            waypoint_usage:                 RecordField::from_bytes(input, 31, 2)?,
            waypoint_latitude:              RecordField::from_bytes(input, 33, 9)?,
            waypoint_longitude:             RecordField::from_bytes(input, 42, 10)?,
            dynamic_magnetic_variation:     RecordField::from_bytes(input, 75, 5)?,
            datum_code:                     RecordField::from_bytes(input, 85, 3)?,
            name_format_indicator:          RecordField::from_bytes(input, 96, 3)?,
            waypoint_name_description:      RecordField::from_bytes(input, 99, 25)?,
            file_record_number:             RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                     RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
    
    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.region_code.raw_bytes,
            self.waypoint_identifier.raw_bytes,
        ])
    }
}

/// 4.1.4.2(A) Enroute Waypoint Continuation Record
#[derive(Debug)]
pub struct EnrouteWaypointContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub region_code: RecordField<'a, RegionCode>,
    pub region_icao_code: RecordField<'a, IcaoCode>,
    pub waypoint_identifier: RecordField<'a, FixIdentifier>,
    pub waypoint_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteWaypointContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteWaypointContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                    RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:             RecordField::from_bytes(input, 2, 3)?,
            section:                        RecordField::from_bytes(input, 5, 1)?,
            subsection:                     RecordField::from_bytes(input, 6, 1)?,
            region_code:                    RecordField::from_bytes(input, 7, 4)?,
            region_icao_code:               RecordField::from_bytes(input, 11, 2)?,
            waypoint_identifier:            RecordField::from_bytes(input, 14, 5)?,
            waypoint_icao_code:             RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:     RecordField::from_bytes(input, 22, 1)?,
            application_type:               RecordField::from_bytes(input, 23, 1)?,
            notes:                          RecordField::from_bytes(input, 24, 69)?,
            file_record_number:             RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                     RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.region_code.raw_bytes,
            self.waypoint_identifier.raw_bytes,
        ])
    }
}

/// 4.1.4.3(A) Enroute Waypoint Flight Planning Continuation Record
#[derive(Debug)]
pub struct EnrouteWaypointFlightPlanningContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub region_code: RecordField<'a, RegionCode>,
    pub region_icao_code: RecordField<'a, IcaoCode>,
    pub waypoint_identifier: RecordField<'a, FixIdentifier>,
    pub waypoint_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub fir_identifier: RecordField<'a, FirUirIdentifier>,
    pub uir_identifier: RecordField<'a, FirUirIdentifier>,
    pub start_end_indicator: RecordField<'a, StartEndIndicator>,
    pub start_end_date: RecordField<'a, StartEndDate>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteWaypointFlightPlanningContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteWaypointFlightPlanningContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            subsection:                         RecordField::from_bytes(input, 6, 1)?,
            region_code:                        RecordField::from_bytes(input, 7, 4)?,
            region_icao_code:                   RecordField::from_bytes(input, 11, 2)?,
            waypoint_identifier:                RecordField::from_bytes(input, 14, 5)?,
            waypoint_icao_code:                 RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:         RecordField::from_bytes(input, 22, 1)?,
            application_type:                   RecordField::from_bytes(input, 23, 1)?,
            fir_identifier:                     RecordField::from_bytes(input, 24, 4)?,
            uir_identifier:                     RecordField::from_bytes(input, 28, 4)?,
            start_end_indicator:                RecordField::from_bytes(input, 32, 1)?,
            start_end_date:                     RecordField::from_bytes(input, 33, 11)?,
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
            self.region_code.raw_bytes,
            self.waypoint_identifier.raw_bytes,
        ])
    }
}

/// 4.1.4.4(A) Enroute Waypoint Changed Data Continuation Record
pub type EnrouteWaypointChangedDataContinuationRecord<'a> = EnrouteWaypointPrimaryRecord<'a>;
