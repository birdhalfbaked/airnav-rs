use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct RestrictiveAirspaceRecords;
impl RestrictiveAirspaceRecords {
    const CONTINUATION_COLUMN: usize = 25;
    const CONTINUATION_APPLICATION_COLUMN: usize = 26;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::RestrictiveAirspacePrimary(
                RestrictiveAirspacePrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::CombinedControllingAgencyFormattedTimeOfOperationsContinuation) => Ok(
                    ARINCRecord::RestrictiveAirspaceTimeControllingAgencyContinuation(
                        RestrictiveAirspaceTimeControllingAgencyContinuationRecord::parse(input)?,
                    ),
                ),
                Some(ContinuationRecordApplicationType::FlightPlanningContinuation) => {
                    Ok(ARINCRecord::RestrictiveAirspaceFlightPlanningContinuation(
                        RestrictiveAirspaceFlightPlanningContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::ControllingAgencyContinuation) => {
                    Ok(ARINCRecord::RestrictiveAirspaceControllingAgencyContinuation(
                        RestrictiveAirspaceControllingAgencyContinuationRecord::parse(input)?,
                    ))
                }
                _ => Err(RecordParseError::new("Invalid continuation record application type".to_string(), Some(String::from_utf8_lossy(input).into_owned()))),
            }
        }
    }
}

/// 4.1.18.1 Restrictive Airspace Primary Record
#[derive(Debug)]
pub struct RestrictiveAirspacePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, AirspaceSubsection>,
    pub airspace_icao_code: RecordField<'a, IcaoCode>,
    pub restrictive_type: RecordField<'a, RestrictiveAirspaceType>,
    pub restrictive_airspace_designation: RecordField<'a, RestrictiveAirspaceDesignation>,
    pub multiple_code: RecordField<'a, MultipleCode>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub level: RecordField<'a, Level>,
    pub time_code: RecordField<'a, StandardPrimaryRecordTimeCode>,
    pub notam: RecordField<'a, NotamFlag>,
    pub boundary_via: RecordField<'a, BoundaryVia>,
    pub latitude: RecordField<'a, Latitude>,
    pub longitude: RecordField<'a, Longitude>,
    pub arc_origin_latitude: RecordField<'a, Latitude>,
    pub arc_origin_longitude: RecordField<'a, Longitude>,
    pub arc_distance: RecordField<'a, ArcDistance>,
    pub arc_bearing: RecordField<'a, ArcBearing>,
    pub lower_limit: RecordField<'a, LowerUpperLimit>,
    pub lower_limit_unit: RecordField<'a, AirspaceLimitUnitIndicator>,
    pub upper_limit: RecordField<'a, LowerUpperLimit>,
    pub upper_limit_unit: RecordField<'a, AirspaceLimitUnitIndicator>,
    pub airspace_name: RecordField<'a, RestrictiveAirspaceName>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for RestrictiveAirspacePrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "RestrictiveAirspacePrimaryRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            subsection:                         RecordField::from_bytes(input, 6, 1)?,
            airspace_icao_code:                 RecordField::from_bytes(input, 7, 2)?,
            restrictive_type:                   RecordField::from_bytes(input, 9, 1)?,
            restrictive_airspace_designation:   RecordField::from_bytes(input, 10, 10)?,
            multiple_code:                      RecordField::from_bytes(input, 20, 1)?,
            sequence_number:                    RecordField::from_bytes(input, 21, 4)?,
            continuation_record_number:         RecordField::from_bytes(input, 25, 1)?,
            level:                              RecordField::from_bytes(input, 26, 1)?,
            time_code:                          RecordField::from_bytes(input, 27, 1)?,
            notam:                              RecordField::from_bytes(input, 28, 1)?,
            boundary_via:                       RecordField::from_bytes(input, 31, 2)?,
            latitude:                           RecordField::from_bytes(input, 33, 9)?,
            longitude:                          RecordField::from_bytes(input, 42, 10)?,
            arc_origin_latitude:                RecordField::from_bytes(input, 52, 9)?,
            arc_origin_longitude:               RecordField::from_bytes(input, 61, 10)?,
            arc_distance:                       RecordField::from_bytes(input, 71, 4)?,
            arc_bearing:                        RecordField::from_bytes(input, 75, 4)?,
            lower_limit:                        RecordField::from_bytes(input, 82, 5)?,
            lower_limit_unit:                   RecordField::from_bytes(input, 87, 1)?,
            upper_limit:                        RecordField::from_bytes(input, 88, 5)?,
            upper_limit_unit:                   RecordField::from_bytes(input, 93, 1)?,
            airspace_name:                      RecordField::from_bytes(input, 94, 30)?,
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
            self.restrictive_airspace_designation.raw_bytes,
            self.multiple_code.raw_bytes,
        ])
    }
}

/// 4.1.18.2 Restrictive Airspace Time/Controlling Agency Continuation Record
#[derive(Debug)]
pub struct RestrictiveAirspaceTimeControllingAgencyContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, AirspaceSubsection>,
    pub airspace_icao_code: RecordField<'a, IcaoCode>,
    pub restrictive_type: RecordField<'a, RestrictiveAirspaceType>,
    pub restrictive_airspace_designation: RecordField<'a, RestrictiveAirspaceDesignation>,
    pub multiple_code: RecordField<'a, MultipleCode>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub time_code: RecordField<'a, StandardContinuationRecordTimeCode>,
    pub notam: RecordField<'a, NotamFlag>,
    pub time_indicator: RecordField<'a, TimeIndicator>,
    pub time_of_operation_1: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_2: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_3: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_4: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_5: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_6: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_7: RecordField<'a, TimeOfOperation>,
    pub controlling_agency: RecordField<'a, ControllingAgency>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}
#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for RestrictiveAirspaceTimeControllingAgencyContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "RestrictiveAirspaceTimeControllingAgencyContinuationRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            subsection:                         RecordField::from_bytes(input, 6, 1)?,
            airspace_icao_code:                 RecordField::from_bytes(input, 7, 2)?,
            restrictive_type:                   RecordField::from_bytes(input, 9, 1)?,
            restrictive_airspace_designation:   RecordField::from_bytes(input, 10, 10)?,
            multiple_code:                      RecordField::from_bytes(input, 20, 1)?,
            sequence_number:                    RecordField::from_bytes(input, 21, 4)?,
            continuation_record_number:         RecordField::from_bytes(input, 25, 1)?,
            application_type:                   RecordField::from_bytes(input, 26, 1)?,
            time_code:                          RecordField::from_bytes(input, 27, 1)?,
            notam:                              RecordField::from_bytes(input, 28, 1)?,
            time_indicator:                     RecordField::from_bytes(input, 29, 1)?,
            time_of_operation_1:                RecordField::from_bytes(input, 30, 10)?,
            time_of_operation_2:                RecordField::from_bytes(input, 40, 10)?,
            time_of_operation_3:                RecordField::from_bytes(input, 50, 10)?,
            time_of_operation_4:                RecordField::from_bytes(input, 60, 10)?,
            time_of_operation_5:                RecordField::from_bytes(input, 70, 10)?,
            time_of_operation_6:                RecordField::from_bytes(input, 80, 10)?,
            time_of_operation_7:                RecordField::from_bytes(input, 90, 10)?,
            controlling_agency:                 RecordField::from_bytes(input, 100, 24)?,
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
            self.restrictive_airspace_designation.raw_bytes,
            self.multiple_code.raw_bytes,
        ])
    }
}

/// 4.1.18.2 Restrictive Airspace Controlling Agency Continuation Record
#[derive(Debug)]
pub struct RestrictiveAirspaceControllingAgencyContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, AirspaceSubsection>,
    pub airspace_icao_code: RecordField<'a, IcaoCode>,
    pub restrictive_type: RecordField<'a, RestrictiveAirspaceType>,
    pub restrictive_airspace_designation: RecordField<'a, RestrictiveAirspaceDesignation>,
    pub multiple_code: RecordField<'a, MultipleCode>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub controlling_agency: RecordField<'a, ControllingAgency>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}
#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for RestrictiveAirspaceControllingAgencyContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "RestrictiveAirspaceControllingAgencyContinuationRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            subsection:                         RecordField::from_bytes(input, 6, 1)?,
            airspace_icao_code:                 RecordField::from_bytes(input, 7, 2)?,
            restrictive_type:                   RecordField::from_bytes(input, 9, 1)?,
            restrictive_airspace_designation:   RecordField::from_bytes(input, 10, 10)?,
            multiple_code:                      RecordField::from_bytes(input, 20, 1)?,
            sequence_number:                    RecordField::from_bytes(input, 21, 4)?,
            continuation_record_number:         RecordField::from_bytes(input, 25, 1)?,
            application_type:                   RecordField::from_bytes(input, 26, 1)?,
            controlling_agency:                 RecordField::from_bytes(input, 100, 24)?,
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
            self.restrictive_airspace_designation.raw_bytes,
            self.multiple_code.raw_bytes,
        ])
    }
}

/// 4.1.18.3 Restrictive Airspace Flight Planning Continuation Record
#[derive(Debug)]
pub struct RestrictiveAirspaceFlightPlanningContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, AirspaceSubsection>,
    pub airspace_icao_code: RecordField<'a, IcaoCode>,
    pub restrictive_type: RecordField<'a, RestrictiveAirspaceType>,
    pub restrictive_airspace_designation: RecordField<'a, RestrictiveAirspaceDesignation>,
    pub multiple_code: RecordField<'a, MultipleCode>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub start_end_indicator: RecordField<'a, StartEndIndicator>,
    pub start_end_date: RecordField<'a, StartEndDate>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for RestrictiveAirspaceFlightPlanningContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "RestrictiveAirspaceFlightPlanningContinuationRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            subsection:                         RecordField::from_bytes(input, 6, 1)?,
            airspace_icao_code:                 RecordField::from_bytes(input, 7, 2)?,
            restrictive_type:                   RecordField::from_bytes(input, 9, 1)?,
            restrictive_airspace_designation:   RecordField::from_bytes(input, 10, 10)?,
            multiple_code:                      RecordField::from_bytes(input, 20, 1)?,
            sequence_number:                    RecordField::from_bytes(input, 21, 4)?,
            continuation_record_number:         RecordField::from_bytes(input, 25, 1)?,
            application_type:                   RecordField::from_bytes(input, 26, 1)?,
            start_end_indicator:                RecordField::from_bytes(input, 30, 1)?,
            start_end_date:                     RecordField::from_bytes(input, 31, 11)?,
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
            self.restrictive_airspace_designation.raw_bytes,
            self.multiple_code.raw_bytes,
        ])
    }
}
