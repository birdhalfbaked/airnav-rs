use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct PreferredRouteRecords;
impl PreferredRouteRecords {
    const CONTINUATION_COLUMN: usize = 39;
    const CONTINUATION_APPLICATION_COLUMN: usize = 40;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::PreferredRoutePrimary(
                PreferredRoutePrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    Ok(ARINCRecord::PreferredRouteTimeContinuation(
                        PreferredRouteTimeContinuationRecord::parse(input)?,
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

/// 4.1.24.1 Preferred Route Primary Record
#[derive(Debug)]
pub struct PreferredRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub route_id: RecordField<'a, PreferredRouteIdentifier>,
    pub route_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub to_fix_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section_code: RecordField<'a, Section>,
    pub to_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub sid_identifier: RecordField<'a, SidStarApproachAirwayIdentifier>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub level: RecordField<'a, Level>,
    pub route_type: RecordField<'a, PreferredRouteRouteType>,
    pub initial_point: RecordField<'a, InitialTerminusFixOrAirport>,
    pub initial_point_icao_code: RecordField<'a, IcaoCode>,
    pub initial_point_section_code: RecordField<'a, Section>,
    pub initial_point_subsection_code: RecordField<'a, GenericSubsection>,
    pub terminus_point: RecordField<'a, InitialTerminusFixOrAirport>,
    pub terminus_point_icao_code: RecordField<'a, IcaoCode>,
    pub terminus_point_section_code: RecordField<'a, Section>,
    pub terminus_point_subsection_code: RecordField<'a, GenericSubsection>,
    pub minimum_altitude: RecordField<'a, MinimumAltitude>,
    pub maximum_altitude: RecordField<'a, MaximumAltitude>,
    pub time_code: RecordField<'a, StandardPrimaryRecordTimeCode>,
    pub aircraft_use_group: RecordField<'a, AircraftUseGroupIndicator>,
    pub direction_restriction: RecordField<'a, PreferredRouteDirectionalRestriction>,
    pub altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub altitude_1: RecordField<'a, MinimumAltitude>,
    pub altitude_2: RecordField<'a, MinimumAltitude>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

impl<'a> Arinc424RecordSpec<'a> for PreferredRoutePrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "PreferredRoutePrimaryRecord"
    }
    
    #[rustfmt::skip]
    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(PreferredRoutePrimaryRecord {
            record_type:                      RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:               RecordField::from_bytes(input, 2, 3)?,
            section:                          RecordField::from_bytes(input, 5, 1)?,
            subsection:                       RecordField::from_bytes(input, 6, 1)?,
            route_id:                         RecordField::from_bytes(input, 14, 10)?,
            route_use_indicator:              RecordField::from_bytes(input, 24, 2)?,
            sequence_number:                  RecordField::from_bytes(input, 26, 4)?,
            continuation_record_number:       RecordField::from_bytes(input, 39, 1)?,
            to_fix_identifier:                RecordField::from_bytes(input, 40, 5)?,
            to_fix_icao_code:                 RecordField::from_bytes(input, 45, 2)?,
            to_fix_section_code:              RecordField::from_bytes(input, 47, 1)?,
            to_fix_subsection_code:           RecordField::from_bytes(input, 48, 1)?,
            via_code:                         RecordField::from_bytes(input, 49, 3)?,
            sid_identifier:                   RecordField::from_bytes(input, 52, 6)?,
            area:                             RecordField::from_bytes(input, 58, 3)?,
            level:                            RecordField::from_bytes(input, 61, 1)?,
            route_type:                       RecordField::from_bytes(input, 62, 1)?,
            initial_point:                    RecordField::from_bytes(input, 63, 5)?,
            initial_point_icao_code:          RecordField::from_bytes(input, 68, 2)?,
            initial_point_section_code:       RecordField::from_bytes(input, 70, 1)?,
            initial_point_subsection_code:    RecordField::from_bytes(input, 71, 1)?,
            terminus_point:                   RecordField::from_bytes(input, 72, 5)?,
            terminus_point_icao_code:         RecordField::from_bytes(input, 77, 2)?,
            terminus_point_section_code:      RecordField::from_bytes(input, 79, 1)?,
            terminus_point_subsection_code:   RecordField::from_bytes(input, 80, 1)?,
            minimum_altitude:                 RecordField::from_bytes(input, 81, 5)?,
            maximum_altitude:                 RecordField::from_bytes(input, 86, 5)?,
            time_code:                        RecordField::from_bytes(input, 91, 1)?,
            aircraft_use_group:               RecordField::from_bytes(input, 92, 2)?,
            direction_restriction:            RecordField::from_bytes(input, 94, 1)?,
            altitude_description:             RecordField::from_bytes(input, 95, 1)?,
            altitude_1:                       RecordField::from_bytes(input, 96, 5)?,
            altitude_2:                       RecordField::from_bytes(input, 101, 5)?,
            file_record_number:               RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                       RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.to_fix_identifier.value.is_none() {
            validation_result.extend_messages(
                "to fix identifier reference",
                is_valid_reference(
                    &self.to_fix_identifier,
                    &self.to_fix_section_code,
                    &self.to_fix_subsection_code,
                ),
            );
        }
        if !self.initial_point.value.is_none() {
            validation_result.extend_messages(
                "initial point reference",
                is_valid_reference(
                    &self.initial_point,
                    &self.initial_point_section_code,
                    &self.initial_point_subsection_code,
                ),
            );
        }
        if !self.terminus_point.value.is_none() {
            validation_result.extend_messages(
                "terminus point reference",
                is_valid_reference(
                    &self.terminus_point,
                    &self.terminus_point_section_code,
                    &self.terminus_point_subsection_code,
                ),
            );
        }
        validation_result.extend_messages(
            "altitude description",
            is_valid_altitude_description(
                &self.section,
                &self.subsection,
                &self.altitude_description,
            ),
        );
        validation_result.as_result()
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.route_id.raw_bytes,
        ])
    }
}

/// 4.1.24.2 Preferred Route Time Continuation Record
#[derive(Debug)]
pub struct PreferredRouteTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub route_id: RecordField<'a, PreferredRouteIdentifier>,
    pub route_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub time_code: RecordField<'a, StandardContinuationRecordTimeCode>,
    pub time_indicator: RecordField<'a, TimeIndicator>,
    pub time_of_operation_1: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_2: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_3: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_4: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_5: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_6: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_7: RecordField<'a, TimeOfOperation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for PreferredRouteTimeContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "PreferredRouteTimeContinuationRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(PreferredRouteTimeContinuationRecord {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            route_id:                     RecordField::from_bytes(input, 14, 10)?,
            route_use_indicator:          RecordField::from_bytes(input, 24, 2)?,
            sequence_number:              RecordField::from_bytes(input, 26, 4)?,
            continuation_record_number:   RecordField::from_bytes(input, 39, 1)?,
            application_type:             RecordField::from_bytes(input, 40, 1)?,
            time_code:                    RecordField::from_bytes(input, 41, 1)?,
            time_indicator:               RecordField::from_bytes(input, 42, 1)?,
            time_of_operation_1:          RecordField::from_bytes(input, 43, 10)?,
            time_of_operation_2:          RecordField::from_bytes(input, 53, 10)?,
            time_of_operation_3:          RecordField::from_bytes(input, 63, 10)?,
            time_of_operation_4:          RecordField::from_bytes(input, 73, 10)?,
            time_of_operation_5:          RecordField::from_bytes(input, 83, 10)?,
            time_of_operation_6:          RecordField::from_bytes(input, 93, 10)?,
            time_of_operation_7:          RecordField::from_bytes(input, 103, 10)?,
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
            self.route_id.raw_bytes,
        ])
    }
}

/// 4.1.24.3 Preferred Route Continuation Record
#[derive(Debug)]
pub struct PreferredRouteContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub route_id: RecordField<'a, PreferredRouteIdentifier>,
    pub route_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for PreferredRouteContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "PreferredRouteContinuationRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(PreferredRouteContinuationRecord {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            route_id:                     RecordField::from_bytes(input, 14, 10)?,
            route_use_indicator:          RecordField::from_bytes(input, 24, 2)?,
            sequence_number:              RecordField::from_bytes(input, 26, 4)?,
            continuation_record_number:   RecordField::from_bytes(input, 39, 1)?,
            application_type:             RecordField::from_bytes(input, 40, 1)?,
            notes:                        RecordField::from_bytes(input, 41, 69)?,
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
            self.route_id.raw_bytes,
        ])
    }
}
