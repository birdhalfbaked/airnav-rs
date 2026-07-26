use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError, is_primary_record,
};
pub(super) struct HeliportTerminalWaypointRecords;
impl HeliportTerminalWaypointRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::HeliportTerminalWaypointPrimary(
                HeliportTerminalWaypointPrimaryRecord::parse(input)?,
            ))
        } else {
            if input[Self::CONTINUATION_APPLICATION_COLUMN - 1].is_ascii_digit() {
                Ok(
                    ARINCRecord::HeliportTerminalWaypointChangedDataContinuation(
                        HeliportTerminalWaypointChangedDataContinuationRecord::parse(input)?,
                    ),
                )
            } else {
                match ContinuationRecordApplicationType::from_bytes(
                    &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                        ..Self::CONTINUATION_APPLICATION_COLUMN],
                )? {
                    Some(ContinuationRecordApplicationType::StandardContinuation) => {
                        Ok(ARINCRecord::HeliportTerminalWaypointContinuation(
                            HeliportTerminalWaypointContinuationRecord::parse(input)?,
                        ))
                    }
                    Some(ContinuationRecordApplicationType::FlightPlanningContinuation) => Ok(
                        ARINCRecord::HeliportTerminalWaypointFlightPlanningContinuation(
                            HeliportTerminalWaypointFlightPlanningContinuationRecord::parse(input)?,
                        ),
                    ),
                    _ => Err(RecordParseError::new(
                        "Invalid continuation record application type".to_string(),
                        Some(String::from_utf8_lossy(input).into_owned()),
                    )),
                }
            }
        }
    }
}

/// 4.2.2.1 Heliport Terminal Waypoint Primary Record
#[derive(Debug)]
pub struct HeliportTerminalWaypointPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, HeliportSubsection>,
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

impl<'a> Arinc424RecordSpec<'a> for HeliportTerminalWaypointPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportTerminalWaypointPrimaryRecord"
    }
    
    #[rustfmt::skip]
    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                    RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:             RecordField::from_bytes(input, 2, 3)?,
            section:                        RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:            RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:             RecordField::from_bytes(input, 11, 2)?,
            subsection:                     RecordField::from_bytes(input, 13, 1)?,
            waypoint_identifier:            RecordField::from_bytes(input, 14, 5)?,
            waypoint_icao_code:             RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:     RecordField::from_bytes(input, 22, 1)?,
            waypoint_type:                  RecordField::from_bytes(input, 27, 3)?,
            waypoint_usage:                 RecordField::from_bytes(input, 30, 2)?,
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
            self.heliport_identifier.raw_bytes,
            self.waypoint_identifier.raw_bytes,
        ])
    }
}

/// 4.2.2.2 Heliport Terminal Waypoint Continuation Record
#[derive(Debug)]
pub struct HeliportTerminalWaypointContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, NavaidSubsection>,
    pub waypoint_identifier: RecordField<'a, FixIdentifier>,
    pub waypoint_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for HeliportTerminalWaypointContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportTerminalWaypointContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                    RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:             RecordField::from_bytes(input, 2, 3)?,
            section:                        RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:            RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:             RecordField::from_bytes(input, 11, 2)?,
            subsection:                     RecordField::from_bytes(input, 13, 1)?,
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
            self.heliport_identifier.raw_bytes,
            self.waypoint_identifier.raw_bytes,
        ])
    }
}

/// 4.2.2.3 Heliport Terminal Waypoint Flight Planning Continuation Record
#[derive(Debug)]
pub struct HeliportTerminalWaypointFlightPlanningContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, NavaidSubsection>,
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
impl<'a> Arinc424RecordSpec<'a> for HeliportTerminalWaypointFlightPlanningContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportTerminalWaypointFlightPlanningContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                 RecordField::from_bytes(input, 11, 2)?,
            subsection:                         RecordField::from_bytes(input, 13, 1)?,
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
            self.heliport_identifier.raw_bytes,
            self.waypoint_identifier.raw_bytes,
        ])
    }
}

/// 4.2.2.4 Heliport Terminal Waypoint Changed Data Continuation Record
pub type HeliportTerminalWaypointChangedDataContinuationRecord<'a> =
    HeliportTerminalWaypointPrimaryRecord<'a>;
