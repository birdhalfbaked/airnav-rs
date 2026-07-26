use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct FlightPlanningDataRecords;
impl FlightPlanningDataRecords {
    const CONTINUATION_COLUMN: usize = 70;
    const CONTINUATION_APPLICATION_COLUMN: usize = 71;
    const PROCEDURE_TYPE_COLUMN: usize = 20;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        match ProcedureType::from_bytes(
            &input[Self::PROCEDURE_TYPE_COLUMN - 1..Self::PROCEDURE_TYPE_COLUMN],
        )? {
            Some(ProcedureType::DepartureProcedureInDatabase)
            | Some(ProcedureType::DepartureProcedureNotInDatabase)
            | Some(ProcedureType::SIDInDatabase)
            | Some(ProcedureType::SIDNotInDatabase)
            | Some(ProcedureType::VectorSIDInDatabase)
            | Some(ProcedureType::VectorSIDNotInDatabase)
            | Some(ProcedureType::STARInDatabase)
            | Some(ProcedureType::STARNotInDatabase)
            | Some(ProcedureType::ArrivalProcedureInDatabase)
            | Some(ProcedureType::ArrivalProcedureNotInDatabase) => {
                if is_primary_record(input, Self::CONTINUATION_COLUMN) {
                    Ok(ARINCRecord::FlightPlanningSIDSTARPrimary(
                        FlightPlanningSIDSTARPrimaryRecord::parse(input)?,
                    ))
                } else {
                    match ContinuationRecordApplicationType::from_bytes(
                        &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                            ..Self::CONTINUATION_APPLICATION_COLUMN],
                    )? {
                        Some(ContinuationRecordApplicationType::PrimaryRecordExtension) => {
                            Ok(ARINCRecord::FlightPlanningSIDSTARContinuation(
                                FlightPlanningSIDSTARContinuationRecord::parse(input)?,
                            ))
                        }
                        Some(ContinuationRecordApplicationType::FormattedTimeOfOperationsContinuation) => {
                            Ok(ARINCRecord::FlightPlanningSIDSTARTimeContinuation(
                                FlightPlanningSIDSTARTimeContinuationRecord::parse(input)?,
                            ))
                        }
                        _ => Err(RecordParseError::new("Invalid continuation record application type".to_string(), Some(String::from_utf8_lossy(input).into_owned()))),
                    }
                }
            }
            Some(ProcedureType::ApproachProcedureInDatabase)
            | Some(ProcedureType::ApproachProcedureNotInDatabase) => {
                if is_primary_record(input, Self::CONTINUATION_COLUMN) {
                    Ok(ARINCRecord::FlightPlanningApproachPrimary(
                        FlightPlanningApproachPrimaryRecord::parse(input)?,
                    ))
                } else {
                    match ContinuationRecordApplicationType::from_bytes(
                        &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                            ..Self::CONTINUATION_APPLICATION_COLUMN],
                    )? {
                        Some(ContinuationRecordApplicationType::PrimaryRecordExtension) => {
                            Ok(ARINCRecord::FlightPlanningApproachContinuation(
                                FlightPlanningApproachContinuationRecord::parse(input)?,
                            ))
                        }
                        Some(ContinuationRecordApplicationType::FormattedTimeOfOperationsContinuation) => {
                            Ok(ARINCRecord::FlightPlanningApproachTimeContinuation(
                                FlightPlanningApproachTimeContinuationRecord::parse(input)?,
                            ))
                        }
                        _ => Err(RecordParseError::new("Invalid continuation record application type".to_string(), Some(String::from_utf8_lossy(input).into_owned()))),
                    }
                }
            }
            _ => Err(RecordParseError::new(
                "Invalid procedure type".to_string(),
                Some(String::from_utf8_lossy(input).into_owned()),
            )),
        }
    }
}

/// 4.1.27.1(A) Flight Planning SID/STAR Data Primary Record
#[derive(Debug)]
pub struct FlightPlanningSIDSTARPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub sid_star_identifier: RecordField<'a, SidStarRouteIdentifier>,
    pub procedure_type: RecordField<'a, ProcedureType>,
    pub runway_transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub runway_transition_fix: RecordField<'a, FixIdentifier>,
    pub runway_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub runway_transition_fix_section_code: RecordField<'a, Section>,
    pub runway_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub runway_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub common_segment_transition_fix: RecordField<'a, FixIdentifier>,
    pub common_segment_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub common_segment_transition_fix_section_code: RecordField<'a, Section>,
    pub common_segment_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub common_segment_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub enroute_transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub enroute_transition_fix: RecordField<'a, FixIdentifier>,
    pub enroute_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub enroute_transition_fix_section_code: RecordField<'a, Section>,
    pub enroute_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub enroute_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub number_of_engines: RecordField<'a, NumberOfEnginesRestriction>,
    pub turboprop_jet_indicator: RecordField<'a, TurbopropJetIndicator>,
    pub rnav: RecordField<'a, RNAVFlag>,
    pub atc_weight_category: RecordField<'a, ATCWeightCategory>,
    pub atc_identifier: RecordField<'a, AtcIdentifier>,
    pub time_code: RecordField<'a, StandardPrimaryRecordTimeCode>,
    pub procedure_description: RecordField<'a, ProcedureDescription>,
    pub leg_type: RecordField<'a, LegTypeCode>,
    pub reporting_code: RecordField<'a, ReportingCode>,
    pub initial_departure_course: RecordField<'a, OutboundCourse>,
    pub altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub altitude_1: RecordField<'a, Altitude>,
    pub altitude_2: RecordField<'a, Altitude>,
    pub speed_limit: RecordField<'a, SpeedLimit>,
    pub initial_cruise_table: RecordField<'a, CruiseTableIdentifier>,
    pub speed_limit_description: RecordField<'a, SpeedLimitDescription>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for FlightPlanningSIDSTARPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "FlightPlanningSIDSTARPrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                                          RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                                   RecordField::from_bytes(input, 2, 3)?,
            section:                                              RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                                   RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                                    RecordField::from_bytes(input, 11, 2)?,
            subsection:                                           RecordField::from_bytes(input, 13, 1)?,
            sid_star_identifier:                                  RecordField::from_bytes(input, 14, 6)?,
            procedure_type:                                       RecordField::from_bytes(input, 20, 1)?,
            runway_transition_identifier:                         RecordField::from_bytes(input, 21, 5)?,
            runway_transition_fix:                                RecordField::from_bytes(input, 26, 5)?,
            runway_transition_fix_icao_code:                      RecordField::from_bytes(input, 31, 2)?,
            runway_transition_fix_section_code:                   RecordField::from_bytes(input, 33, 1)?,
            runway_transition_fix_subsection_code:                RecordField::from_bytes(input, 34, 1)?,
            runway_transition_fix_along_track_distance:           RecordField::from_bytes(input, 35, 3)?,
            common_segment_transition_fix:                        RecordField::from_bytes(input, 38, 5)?,
            common_segment_transition_fix_icao_code:              RecordField::from_bytes(input, 43, 2)?,
            common_segment_transition_fix_section_code:           RecordField::from_bytes(input, 45, 1)?,
            common_segment_transition_fix_subsection_code:        RecordField::from_bytes(input, 46, 1)?,
            common_segment_transition_fix_along_track_distance:   RecordField::from_bytes(input, 47, 3)?,
            enroute_transition_identifier:                        RecordField::from_bytes(input, 50, 5)?,
            enroute_transition_fix:                               RecordField::from_bytes(input, 55, 5)?,
            enroute_transition_fix_icao_code:                     RecordField::from_bytes(input, 60, 2)?,
            enroute_transition_fix_section_code:                  RecordField::from_bytes(input, 62, 1)?,
            enroute_transition_fix_subsection_code:               RecordField::from_bytes(input, 63, 1)?,
            enroute_transition_fix_along_track_distance:          RecordField::from_bytes(input, 64, 3)?,
            sequence_number:                                      RecordField::from_bytes(input, 67, 3)?,
            continuation_record_number:                           RecordField::from_bytes(input, 70, 1)?,
            number_of_engines:                                    RecordField::from_bytes(input, 71, 4)?,
            turboprop_jet_indicator:                              RecordField::from_bytes(input, 75, 1)?,
            rnav:                                                 RecordField::from_bytes(input, 76, 1)?,
            atc_weight_category:                                  RecordField::from_bytes(input, 77, 1)?,
            atc_identifier:                                       RecordField::from_bytes(input, 78, 7)?,
            time_code:                                            RecordField::from_bytes(input, 85, 1)?,
            procedure_description:                                RecordField::from_bytes(input, 86, 15)?,
            leg_type:                                             RecordField::from_bytes(input, 101, 2)?,
            reporting_code:                                       RecordField::from_bytes(input, 103, 1)?,
            initial_departure_course:                             RecordField::from_bytes(input, 104, 4)?,
            altitude_description:                                 RecordField::from_bytes(input, 108, 1)?,
            altitude_1:                                           RecordField::from_bytes(input, 109, 3)?,
            altitude_2:                                           RecordField::from_bytes(input, 112, 3)?,
            speed_limit:                                          RecordField::from_bytes(input, 115, 3)?,
            initial_cruise_table:                                 RecordField::from_bytes(input, 118, 2)?,
            speed_limit_description:                              RecordField::from_bytes(input, 120, 1)?,
            file_record_number:                                   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                                           RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.runway_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "runway transition fix reference",
                is_valid_reference(
                    &self.runway_transition_fix,
                    &self.runway_transition_fix_section_code,
                    &self.runway_transition_fix_subsection_code,
                ),
            );
        }
        if !self.common_segment_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "common segment transition fix reference",
                is_valid_reference(
                    &self.common_segment_transition_fix,
                    &self.common_segment_transition_fix_section_code,
                    &self.common_segment_transition_fix_subsection_code,
                ),
            );
        }
        if !self.enroute_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "enroute transition fix reference",
                is_valid_reference(
                    &self.enroute_transition_fix,
                    &self.enroute_transition_fix_section_code,
                    &self.enroute_transition_fix_subsection_code,
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
            self.airport_identifier.raw_bytes,
            self.sid_star_identifier.raw_bytes,
        ])
    }
}

/// 4.1.27.1(B) Flight Planning Approach Data Primary Record
#[derive(Debug)]
pub struct FlightPlanningApproachPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub approach_identifier: RecordField<'a, ApproachRouteIdentifier>,
    pub procedure_type: RecordField<'a, ProcedureType>,
    pub runway_transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub runway_transition_fix: RecordField<'a, FixIdentifier>,
    pub runway_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub runway_transition_fix_section_code: RecordField<'a, Section>,
    pub runway_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub runway_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub common_segment_transition_fix: RecordField<'a, FixIdentifier>,
    pub common_segment_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub common_segment_transition_fix_section_code: RecordField<'a, Section>,
    pub common_segment_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub common_segment_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub enroute_transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub enroute_transition_fix: RecordField<'a, FixIdentifier>,
    pub enroute_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub enroute_transition_fix_section_code: RecordField<'a, Section>,
    pub enroute_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub enroute_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub number_of_engines: RecordField<'a, NumberOfEnginesRestriction>,
    pub turboprop_jet_indicator: RecordField<'a, TurbopropJetIndicator>,
    pub rnav: RecordField<'a, RNAVFlag>,
    pub atc_weight_category: RecordField<'a, ATCWeightCategory>,
    pub atc_identifier: RecordField<'a, AtcIdentifier>,
    pub time_code: RecordField<'a, StandardPrimaryRecordTimeCode>,
    pub procedure_description: RecordField<'a, ProcedureDescription>,
    pub leg_type: RecordField<'a, LegTypeCode>,
    pub reporting_code: RecordField<'a, ReportingCode>,
    pub initial_departure_course: RecordField<'a, OutboundCourse>,
    pub altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub altitude_1: RecordField<'a, Altitude>,
    pub altitude_2: RecordField<'a, Altitude>,
    pub speed_limit: RecordField<'a, SpeedLimit>,
    pub initial_cruise_table: RecordField<'a, CruiseTableIdentifier>,
    pub speed_limit_description: RecordField<'a, SpeedLimitDescription>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for FlightPlanningApproachPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "FlightPlanningApproachPrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                                          RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                                   RecordField::from_bytes(input, 2, 3)?,
            section:                                              RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                                   RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                                    RecordField::from_bytes(input, 11, 2)?,
            subsection:                                           RecordField::from_bytes(input, 13, 1)?,
            approach_identifier:                                  RecordField::from_bytes(input, 14, 6)?,
            procedure_type:                                       RecordField::from_bytes(input, 20, 1)?,
            runway_transition_identifier:                         RecordField::from_bytes(input, 21, 5)?,
            runway_transition_fix:                                RecordField::from_bytes(input, 26, 5)?,
            runway_transition_fix_icao_code:                      RecordField::from_bytes(input, 31, 2)?,
            runway_transition_fix_section_code:                   RecordField::from_bytes(input, 33, 1)?,
            runway_transition_fix_subsection_code:                RecordField::from_bytes(input, 34, 1)?,
            runway_transition_fix_along_track_distance:           RecordField::from_bytes(input, 35, 3)?,
            common_segment_transition_fix:                        RecordField::from_bytes(input, 38, 5)?,
            common_segment_transition_fix_icao_code:              RecordField::from_bytes(input, 43, 2)?,
            common_segment_transition_fix_section_code:           RecordField::from_bytes(input, 45, 1)?,
            common_segment_transition_fix_subsection_code:        RecordField::from_bytes(input, 46, 1)?,
            common_segment_transition_fix_along_track_distance:   RecordField::from_bytes(input, 47, 3)?,
            enroute_transition_identifier:                        RecordField::from_bytes(input, 50, 5)?,
            enroute_transition_fix:                               RecordField::from_bytes(input, 55, 5)?,
            enroute_transition_fix_icao_code:                     RecordField::from_bytes(input, 60, 2)?,
            enroute_transition_fix_section_code:                  RecordField::from_bytes(input, 62, 1)?,
            enroute_transition_fix_subsection_code:               RecordField::from_bytes(input, 63, 1)?,
            enroute_transition_fix_along_track_distance:          RecordField::from_bytes(input, 64, 3)?,
            sequence_number:                                      RecordField::from_bytes(input, 67, 3)?,
            continuation_record_number:                           RecordField::from_bytes(input, 70, 1)?,
            number_of_engines:                                    RecordField::from_bytes(input, 71, 4)?,
            turboprop_jet_indicator:                              RecordField::from_bytes(input, 75, 1)?,
            rnav:                                                 RecordField::from_bytes(input, 76, 1)?,
            atc_weight_category:                                  RecordField::from_bytes(input, 77, 1)?,
            atc_identifier:                                       RecordField::from_bytes(input, 78, 7)?,
            time_code:                                            RecordField::from_bytes(input, 85, 1)?,
            procedure_description:                                RecordField::from_bytes(input, 86, 15)?,
            leg_type:                                             RecordField::from_bytes(input, 101, 2)?,
            reporting_code:                                       RecordField::from_bytes(input, 103, 1)?,
            initial_departure_course:                             RecordField::from_bytes(input, 104, 4)?,
            altitude_description:                                 RecordField::from_bytes(input, 108, 1)?,
            altitude_1:                                           RecordField::from_bytes(input, 109, 3)?,
            altitude_2:                                           RecordField::from_bytes(input, 112, 3)?,
            speed_limit:                                          RecordField::from_bytes(input, 115, 3)?,
            initial_cruise_table:                                 RecordField::from_bytes(input, 118, 2)?,
            speed_limit_description:                              RecordField::from_bytes(input, 120, 1)?,
            file_record_number:                                   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                                           RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.runway_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "runway transition fix reference",
                is_valid_reference(
                    &self.runway_transition_fix,
                    &self.runway_transition_fix_section_code,
                    &self.runway_transition_fix_subsection_code,
                ),
            );
        }
        if !self.common_segment_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "common segment transition fix reference",
                is_valid_reference(
                    &self.common_segment_transition_fix,
                    &self.common_segment_transition_fix_section_code,
                    &self.common_segment_transition_fix_subsection_code,
                ),
            );
        }
        if !self.enroute_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "enroute transition fix reference",
                is_valid_reference(
                    &self.enroute_transition_fix,
                    &self.enroute_transition_fix_section_code,
                    &self.enroute_transition_fix_subsection_code,
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
            self.airport_identifier.raw_bytes,
            self.approach_identifier.raw_bytes,
        ])
    }
}

/// 4.1.27.2(A) Flight Planning SID/STAR Continuation Record
#[derive(Debug)]
pub struct FlightPlanningSIDSTARContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub sid_star_identifier: RecordField<'a, SidStarRouteIdentifier>,
    pub procedure_type: RecordField<'a, ProcedureType>,
    pub runway_transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub runway_transition_fix: RecordField<'a, FixIdentifier>,
    pub runway_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub runway_transition_fix_section_code: RecordField<'a, Section>,
    pub runway_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub runway_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub common_segment_transition_fix: RecordField<'a, FixIdentifier>,
    pub common_segment_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub common_segment_transition_fix_section_code: RecordField<'a, Section>,
    pub common_segment_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub common_segment_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub enroute_transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub enroute_transition_fix: RecordField<'a, FixIdentifier>,
    pub enroute_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub enroute_transition_fix_section_code: RecordField<'a, Section>,
    pub enroute_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub enroute_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub intermediate_fix_1_identifier: RecordField<'a, FixIdentifier>,
    pub intermediate_fix_1_icao_code: RecordField<'a, IcaoCode>,
    pub intermediate_fix_1_section_code: RecordField<'a, Section>,
    pub intermediate_fix_1_subsection_code: RecordField<'a, GenericSubsection>,
    pub intermediate_fix_1_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub intermediate_fix_1_related_transition_code: RecordField<'a, FixRelatedTransitionCode>,
    pub intermediate_fix_2_identifier: RecordField<'a, FixIdentifier>,
    pub intermediate_fix_2_icao_code: RecordField<'a, IcaoCode>,
    pub intermediate_fix_2_section_code: RecordField<'a, Section>,
    pub intermediate_fix_2_subsection_code: RecordField<'a, GenericSubsection>,
    pub intermediate_fix_2_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub intermediate_fix_2_related_transition_code: RecordField<'a, FixRelatedTransitionCode>,
    pub intermediate_fix_3_identifier: RecordField<'a, FixIdentifier>,
    pub intermediate_fix_3_icao_code: RecordField<'a, IcaoCode>,
    pub intermediate_fix_3_section_code: RecordField<'a, Section>,
    pub intermediate_fix_3_subsection_code: RecordField<'a, GenericSubsection>,
    pub intermediate_fix_3_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub intermediate_fix_3_related_transition_code: RecordField<'a, FixRelatedTransitionCode>,
    pub intermediate_fix_4_identifier: RecordField<'a, FixIdentifier>,
    pub intermediate_fix_4_icao_code: RecordField<'a, IcaoCode>,
    pub intermediate_fix_4_section_code: RecordField<'a, Section>,
    pub intermediate_fix_4_subsection_code: RecordField<'a, GenericSubsection>,
    pub intermediate_fix_4_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub intermediate_fix_4_related_transition_code: RecordField<'a, FixRelatedTransitionCode>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for FlightPlanningSIDSTARContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "FlightPlanningSIDSTARContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                                          RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                                   RecordField::from_bytes(input, 2, 3)?,
            section:                                              RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                                   RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                                    RecordField::from_bytes(input, 11, 2)?,
            subsection:                                           RecordField::from_bytes(input, 13, 1)?,
            sid_star_identifier:                                  RecordField::from_bytes(input, 14, 6)?,
            procedure_type:                                       RecordField::from_bytes(input, 20, 1)?,
            runway_transition_identifier:                         RecordField::from_bytes(input, 21, 5)?,
            runway_transition_fix:                                RecordField::from_bytes(input, 26, 5)?,
            runway_transition_fix_icao_code:                      RecordField::from_bytes(input, 31, 2)?,
            runway_transition_fix_section_code:                   RecordField::from_bytes(input, 33, 1)?,
            runway_transition_fix_subsection_code:                RecordField::from_bytes(input, 34, 1)?,
            runway_transition_fix_along_track_distance:           RecordField::from_bytes(input, 35, 3)?,
            common_segment_transition_fix:                        RecordField::from_bytes(input, 38, 5)?,
            common_segment_transition_fix_icao_code:              RecordField::from_bytes(input, 43, 2)?,
            common_segment_transition_fix_section_code:           RecordField::from_bytes(input, 45, 1)?,
            common_segment_transition_fix_subsection_code:        RecordField::from_bytes(input, 46, 1)?,
            common_segment_transition_fix_along_track_distance:   RecordField::from_bytes(input, 47, 3)?,
            enroute_transition_identifier:                        RecordField::from_bytes(input, 50, 5)?,
            enroute_transition_fix:                               RecordField::from_bytes(input, 55, 5)?,
            enroute_transition_fix_icao_code:                     RecordField::from_bytes(input, 60, 2)?,
            enroute_transition_fix_section_code:                  RecordField::from_bytes(input, 62, 1)?,
            enroute_transition_fix_subsection_code:               RecordField::from_bytes(input, 63, 1)?,
            enroute_transition_fix_along_track_distance:          RecordField::from_bytes(input, 64, 3)?,
            sequence_number:                                      RecordField::from_bytes(input, 67, 3)?,
            continuation_record_number:                           RecordField::from_bytes(input, 70, 1)?,
            application_type:                                     RecordField::from_bytes(input, 71, 1)?,
            intermediate_fix_1_identifier:                        RecordField::from_bytes(input, 72, 5)?,
            intermediate_fix_1_icao_code:                         RecordField::from_bytes(input, 77, 2)?,
            intermediate_fix_1_section_code:                      RecordField::from_bytes(input, 79, 1)?,
            intermediate_fix_1_subsection_code:                   RecordField::from_bytes(input, 80, 1)?,
            intermediate_fix_1_along_track_distance:              RecordField::from_bytes(input, 81, 3)?,
            intermediate_fix_1_related_transition_code:           RecordField::from_bytes(input, 84, 1)?,
            intermediate_fix_2_identifier:                        RecordField::from_bytes(input, 85, 5)?,
            intermediate_fix_2_icao_code:                         RecordField::from_bytes(input, 90, 2)?,
            intermediate_fix_2_section_code:                      RecordField::from_bytes(input, 92, 1)?,
            intermediate_fix_2_subsection_code:                   RecordField::from_bytes(input, 93, 1)?,
            intermediate_fix_2_along_track_distance:              RecordField::from_bytes(input, 94, 3)?,
            intermediate_fix_2_related_transition_code:           RecordField::from_bytes(input, 97, 1)?,
            intermediate_fix_3_identifier:                        RecordField::from_bytes(input, 98, 5)?,
            intermediate_fix_3_icao_code:                         RecordField::from_bytes(input, 103, 2)?,
            intermediate_fix_3_section_code:                      RecordField::from_bytes(input, 105, 1)?,
            intermediate_fix_3_subsection_code:                   RecordField::from_bytes(input, 106, 1)?,
            intermediate_fix_3_along_track_distance:              RecordField::from_bytes(input, 107, 3)?,
            intermediate_fix_3_related_transition_code:           RecordField::from_bytes(input, 110, 1)?,
            intermediate_fix_4_identifier:                        RecordField::from_bytes(input, 111, 5)?,
            intermediate_fix_4_icao_code:                         RecordField::from_bytes(input, 116, 2)?,
            intermediate_fix_4_section_code:                      RecordField::from_bytes(input, 118, 1)?,
            intermediate_fix_4_subsection_code:                   RecordField::from_bytes(input, 119, 1)?,
            intermediate_fix_4_along_track_distance:              RecordField::from_bytes(input, 120, 3)?,
            intermediate_fix_4_related_transition_code:           RecordField::from_bytes(input, 123, 1)?,
            file_record_number:                                   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                                           RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.runway_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "runway transition fix reference",
                is_valid_reference(
                    &self.runway_transition_fix,
                    &self.runway_transition_fix_section_code,
                    &self.runway_transition_fix_subsection_code,
                ),
            );
        }
        if !self.common_segment_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "common segment transition fix reference",
                is_valid_reference(
                    &self.common_segment_transition_fix,
                    &self.common_segment_transition_fix_section_code,
                    &self.common_segment_transition_fix_subsection_code,
                ),
            );
        }
        if !self.enroute_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "enroute transition fix reference",
                is_valid_reference(
                    &self.enroute_transition_fix,
                    &self.enroute_transition_fix_section_code,
                    &self.enroute_transition_fix_subsection_code,
                ),
            );
        }

        if !self.intermediate_fix_1_identifier.value.is_none() {
            validation_result.extend_messages(
                "intermediate fix 1 reference",
                is_valid_reference(
                    &self.intermediate_fix_1_identifier,
                    &self.intermediate_fix_1_section_code,
                    &self.intermediate_fix_1_subsection_code,
                ),
            );
        }
        if !self.intermediate_fix_2_identifier.value.is_none() {
            validation_result.extend_messages(
                "intermediate fix 2 reference",
                is_valid_reference(
                    &self.intermediate_fix_2_identifier,
                    &self.intermediate_fix_2_section_code,
                    &self.intermediate_fix_2_subsection_code,
                ),
            );
        }
        if !self.intermediate_fix_3_identifier.value.is_none() {
            validation_result.extend_messages(
                "intermediate fix 3 reference",
                is_valid_reference(
                    &self.intermediate_fix_3_identifier,
                    &self.intermediate_fix_3_section_code,
                    &self.intermediate_fix_3_subsection_code,
                ),
            );
        }
        if !self.intermediate_fix_4_identifier.value.is_none() {
            validation_result.extend_messages(
                "intermediate fix 4 reference",
                is_valid_reference(
                    &self.intermediate_fix_4_identifier,
                    &self.intermediate_fix_4_section_code,
                    &self.intermediate_fix_4_subsection_code,
                ),
            );
        }

        validation_result.as_result()
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.airport_identifier.raw_bytes,
            self.sid_star_identifier.raw_bytes,
        ])
    }
}

/// 4.1.27.2(B) Flight Planning Approach Continuation Record
#[derive(Debug)]
pub struct FlightPlanningApproachContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub approach_identifier: RecordField<'a, ApproachRouteIdentifier>,
    pub procedure_type: RecordField<'a, ProcedureType>,
    pub runway_transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub runway_transition_fix: RecordField<'a, FixIdentifier>,
    pub runway_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub runway_transition_fix_section_code: RecordField<'a, Section>,
    pub runway_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub runway_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub common_segment_transition_fix: RecordField<'a, FixIdentifier>,
    pub common_segment_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub common_segment_transition_fix_section_code: RecordField<'a, Section>,
    pub common_segment_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub common_segment_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub enroute_transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub enroute_transition_fix: RecordField<'a, FixIdentifier>,
    pub enroute_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub enroute_transition_fix_section_code: RecordField<'a, Section>,
    pub enroute_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub enroute_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub intermediate_fix_1_identifier: RecordField<'a, FixIdentifier>,
    pub intermediate_fix_1_icao_code: RecordField<'a, IcaoCode>,
    pub intermediate_fix_1_section_code: RecordField<'a, Section>,
    pub intermediate_fix_1_subsection_code: RecordField<'a, GenericSubsection>,
    pub intermediate_fix_1_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub intermediate_fix_1_related_transition_code: RecordField<'a, FixRelatedTransitionCode>,
    pub intermediate_fix_2_identifier: RecordField<'a, FixIdentifier>,
    pub intermediate_fix_2_icao_code: RecordField<'a, IcaoCode>,
    pub intermediate_fix_2_section_code: RecordField<'a, Section>,
    pub intermediate_fix_2_subsection_code: RecordField<'a, GenericSubsection>,
    pub intermediate_fix_2_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub intermediate_fix_2_related_transition_code: RecordField<'a, FixRelatedTransitionCode>,
    pub intermediate_fix_3_identifier: RecordField<'a, FixIdentifier>,
    pub intermediate_fix_3_icao_code: RecordField<'a, IcaoCode>,
    pub intermediate_fix_3_section_code: RecordField<'a, Section>,
    pub intermediate_fix_3_subsection_code: RecordField<'a, GenericSubsection>,
    pub intermediate_fix_3_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub intermediate_fix_3_related_transition_code: RecordField<'a, FixRelatedTransitionCode>,
    pub intermediate_fix_4_identifier: RecordField<'a, FixIdentifier>,
    pub intermediate_fix_4_icao_code: RecordField<'a, IcaoCode>,
    pub intermediate_fix_4_section_code: RecordField<'a, Section>,
    pub intermediate_fix_4_subsection_code: RecordField<'a, GenericSubsection>,
    pub intermediate_fix_4_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub intermediate_fix_4_related_transition_code: RecordField<'a, FixRelatedTransitionCode>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for FlightPlanningApproachContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "FlightPlanningApproachContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                                          RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                                   RecordField::from_bytes(input, 2, 3)?,
            section:                                              RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                                   RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                                    RecordField::from_bytes(input, 11, 2)?,
            subsection:                                           RecordField::from_bytes(input, 13, 1)?,
            approach_identifier:                                  RecordField::from_bytes(input, 14, 6)?,
            procedure_type:                                       RecordField::from_bytes(input, 20, 1)?,
            runway_transition_identifier:                         RecordField::from_bytes(input, 21, 5)?,
            runway_transition_fix:                                RecordField::from_bytes(input, 26, 5)?,
            runway_transition_fix_icao_code:                      RecordField::from_bytes(input, 31, 2)?,
            runway_transition_fix_section_code:                   RecordField::from_bytes(input, 33, 1)?,
            runway_transition_fix_subsection_code:                RecordField::from_bytes(input, 34, 1)?,
            runway_transition_fix_along_track_distance:           RecordField::from_bytes(input, 35, 3)?,
            common_segment_transition_fix:                        RecordField::from_bytes(input, 38, 5)?,
            common_segment_transition_fix_icao_code:              RecordField::from_bytes(input, 43, 2)?,
            common_segment_transition_fix_section_code:           RecordField::from_bytes(input, 45, 1)?,
            common_segment_transition_fix_subsection_code:        RecordField::from_bytes(input, 46, 1)?,
            common_segment_transition_fix_along_track_distance:   RecordField::from_bytes(input, 47, 3)?,
            enroute_transition_identifier:                        RecordField::from_bytes(input, 50, 5)?,
            enroute_transition_fix:                               RecordField::from_bytes(input, 55, 5)?,
            enroute_transition_fix_icao_code:                     RecordField::from_bytes(input, 60, 2)?,
            enroute_transition_fix_section_code:                  RecordField::from_bytes(input, 62, 1)?,
            enroute_transition_fix_subsection_code:               RecordField::from_bytes(input, 63, 1)?,
            enroute_transition_fix_along_track_distance:          RecordField::from_bytes(input, 64, 3)?,
            sequence_number:                                      RecordField::from_bytes(input, 67, 3)?,
            continuation_record_number:                           RecordField::from_bytes(input, 70, 1)?,
            application_type:                                     RecordField::from_bytes(input, 71, 1)?,
            intermediate_fix_1_identifier:                        RecordField::from_bytes(input, 72, 5)?,
            intermediate_fix_1_icao_code:                         RecordField::from_bytes(input, 77, 2)?,
            intermediate_fix_1_section_code:                      RecordField::from_bytes(input, 79, 1)?,
            intermediate_fix_1_subsection_code:                   RecordField::from_bytes(input, 80, 1)?,
            intermediate_fix_1_along_track_distance:              RecordField::from_bytes(input, 81, 3)?,
            intermediate_fix_1_related_transition_code:           RecordField::from_bytes(input, 84, 1)?,
            intermediate_fix_2_identifier:                        RecordField::from_bytes(input, 85, 5)?,
            intermediate_fix_2_icao_code:                         RecordField::from_bytes(input, 90, 2)?,
            intermediate_fix_2_section_code:                      RecordField::from_bytes(input, 92, 1)?,
            intermediate_fix_2_subsection_code:                   RecordField::from_bytes(input, 93, 1)?,
            intermediate_fix_2_along_track_distance:              RecordField::from_bytes(input, 94, 3)?,
            intermediate_fix_2_related_transition_code:           RecordField::from_bytes(input, 97, 1)?,
            intermediate_fix_3_identifier:                        RecordField::from_bytes(input, 98, 5)?,
            intermediate_fix_3_icao_code:                         RecordField::from_bytes(input, 103, 2)?,
            intermediate_fix_3_section_code:                      RecordField::from_bytes(input, 105, 1)?,
            intermediate_fix_3_subsection_code:                   RecordField::from_bytes(input, 106, 1)?,
            intermediate_fix_3_along_track_distance:              RecordField::from_bytes(input, 107, 3)?,
            intermediate_fix_3_related_transition_code:           RecordField::from_bytes(input, 110, 1)?,
            intermediate_fix_4_identifier:                        RecordField::from_bytes(input, 111, 5)?,
            intermediate_fix_4_icao_code:                         RecordField::from_bytes(input, 116, 2)?,
            intermediate_fix_4_section_code:                      RecordField::from_bytes(input, 118, 1)?,
            intermediate_fix_4_subsection_code:                   RecordField::from_bytes(input, 119, 1)?,
            intermediate_fix_4_along_track_distance:              RecordField::from_bytes(input, 120, 3)?,
            intermediate_fix_4_related_transition_code:           RecordField::from_bytes(input, 123, 1)?,
            file_record_number:                                   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                                           RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.runway_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "runway transition fix reference",
                is_valid_reference(
                    &self.runway_transition_fix,
                    &self.runway_transition_fix_section_code,
                    &self.runway_transition_fix_subsection_code,
                ),
            );
        }
        if !self.common_segment_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "common segment transition fix reference",
                is_valid_reference(
                    &self.common_segment_transition_fix,
                    &self.common_segment_transition_fix_section_code,
                    &self.common_segment_transition_fix_subsection_code,
                ),
            );
        }
        if !self.enroute_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "enroute transition fix reference",
                is_valid_reference(
                    &self.enroute_transition_fix,
                    &self.enroute_transition_fix_section_code,
                    &self.enroute_transition_fix_subsection_code,
                ),
            );
        }

        if !self.intermediate_fix_1_identifier.value.is_none() {
            validation_result.extend_messages(
                "intermediate fix 1 reference",
                is_valid_reference(
                    &self.intermediate_fix_1_identifier,
                    &self.intermediate_fix_1_section_code,
                    &self.intermediate_fix_1_subsection_code,
                ),
            );
        }
        if !self.intermediate_fix_2_identifier.value.is_none() {
            validation_result.extend_messages(
                "intermediate fix 2 reference",
                is_valid_reference(
                    &self.intermediate_fix_2_identifier,
                    &self.intermediate_fix_2_section_code,
                    &self.intermediate_fix_2_subsection_code,
                ),
            );
        }
        if !self.intermediate_fix_3_identifier.value.is_none() {
            validation_result.extend_messages(
                "intermediate fix 3 reference",
                is_valid_reference(
                    &self.intermediate_fix_3_identifier,
                    &self.intermediate_fix_3_section_code,
                    &self.intermediate_fix_3_subsection_code,
                ),
            );
        }
        if !self.intermediate_fix_4_identifier.value.is_none() {
            validation_result.extend_messages(
                "intermediate fix 4 reference",
                is_valid_reference(
                    &self.intermediate_fix_4_identifier,
                    &self.intermediate_fix_4_section_code,
                    &self.intermediate_fix_4_subsection_code,
                ),
            );
        }

        validation_result.as_result()
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.airport_identifier.raw_bytes,
            self.approach_identifier.raw_bytes,
        ])
    }
}

/// 4.1.27.3(A) Flight Planning SID/STAR Time Continuation Record
#[derive(Debug)]
pub struct FlightPlanningSIDSTARTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub sid_star_identifier: RecordField<'a, SidStarRouteIdentifier>,
    pub procedure_type: RecordField<'a, ProcedureType>,
    pub runway_transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub runway_transition_fix: RecordField<'a, FixIdentifier>,
    pub runway_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub runway_transition_fix_section_code: RecordField<'a, Section>,
    pub runway_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub runway_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub common_segment_transition_fix: RecordField<'a, FixIdentifier>,
    pub common_segment_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub common_segment_transition_fix_section_code: RecordField<'a, Section>,
    pub common_segment_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub common_segment_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub enroute_transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub enroute_transition_fix: RecordField<'a, FixIdentifier>,
    pub enroute_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub enroute_transition_fix_section_code: RecordField<'a, Section>,
    pub enroute_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub enroute_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
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
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for FlightPlanningSIDSTARTimeContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "FlightPlanningSIDSTARTimeContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                                          RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                                   RecordField::from_bytes(input, 2, 3)?,
            section:                                              RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                                   RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                                    RecordField::from_bytes(input, 11, 2)?,
            subsection:                                           RecordField::from_bytes(input, 13, 1)?,
            sid_star_identifier:                                  RecordField::from_bytes(input, 14, 6)?,
            procedure_type:                                       RecordField::from_bytes(input, 20, 1)?,
            runway_transition_identifier:                         RecordField::from_bytes(input, 21, 5)?,
            runway_transition_fix:                                RecordField::from_bytes(input, 26, 5)?,
            runway_transition_fix_icao_code:                      RecordField::from_bytes(input, 31, 2)?,
            runway_transition_fix_section_code:                   RecordField::from_bytes(input, 33, 1)?,
            runway_transition_fix_subsection_code:                RecordField::from_bytes(input, 34, 1)?,
            runway_transition_fix_along_track_distance:           RecordField::from_bytes(input, 35, 3)?,
            common_segment_transition_fix:                        RecordField::from_bytes(input, 38, 5)?,
            common_segment_transition_fix_icao_code:              RecordField::from_bytes(input, 43, 2)?,
            common_segment_transition_fix_section_code:           RecordField::from_bytes(input, 45, 1)?,
            common_segment_transition_fix_subsection_code:        RecordField::from_bytes(input, 46, 1)?,
            common_segment_transition_fix_along_track_distance:   RecordField::from_bytes(input, 47, 3)?,
            enroute_transition_identifier:                        RecordField::from_bytes(input, 50, 5)?,
            enroute_transition_fix:                               RecordField::from_bytes(input, 55, 5)?,
            enroute_transition_fix_icao_code:                     RecordField::from_bytes(input, 60, 2)?,
            enroute_transition_fix_section_code:                  RecordField::from_bytes(input, 62, 1)?,
            enroute_transition_fix_subsection_code:               RecordField::from_bytes(input, 63, 1)?,
            enroute_transition_fix_along_track_distance:          RecordField::from_bytes(input, 64, 3)?,
            sequence_number:                                      RecordField::from_bytes(input, 67, 3)?,
            continuation_record_number:                           RecordField::from_bytes(input, 70, 1)?,
            application_type:                                     RecordField::from_bytes(input, 71, 1)?,
            time_code:                                            RecordField::from_bytes(input, 72, 1)?,
            time_indicator:                                       RecordField::from_bytes(input, 73, 1)?,
            time_of_operation_1:                                  RecordField::from_bytes(input, 74, 10)?,
            time_of_operation_2:                                  RecordField::from_bytes(input, 84, 10)?,
            time_of_operation_3:                                  RecordField::from_bytes(input, 94, 10)?,
            time_of_operation_4:                                  RecordField::from_bytes(input, 104, 10)?,
            time_of_operation_5:                                  RecordField::from_bytes(input, 114, 10)?,
            file_record_number:                                   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                                           RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.runway_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "runway transition fix reference",
                is_valid_reference(
                    &self.runway_transition_fix,
                    &self.runway_transition_fix_section_code,
                    &self.runway_transition_fix_subsection_code,
                ),
            );
        }
        if !self.common_segment_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "common segment transition fix reference",
                is_valid_reference(
                    &self.common_segment_transition_fix,
                    &self.common_segment_transition_fix_section_code,
                    &self.common_segment_transition_fix_subsection_code,
                ),
            );
        }
        if !self.enroute_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "enroute transition fix reference",
                is_valid_reference(
                    &self.enroute_transition_fix,
                    &self.enroute_transition_fix_section_code,
                    &self.enroute_transition_fix_subsection_code,
                ),
            );
        }
        validation_result.as_result()
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.airport_identifier.raw_bytes,
            self.sid_star_identifier.raw_bytes,
        ])
    }
}

/// 4.1.27.3(B) Flight Planning Approach Time Continuation Record
#[derive(Debug)]
pub struct FlightPlanningApproachTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub approach_identifier: RecordField<'a, ApproachRouteIdentifier>,
    pub procedure_type: RecordField<'a, ProcedureType>,
    pub runway_transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub runway_transition_fix: RecordField<'a, FixIdentifier>,
    pub runway_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub runway_transition_fix_section_code: RecordField<'a, Section>,
    pub runway_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub runway_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub common_segment_transition_fix: RecordField<'a, FixIdentifier>,
    pub common_segment_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub common_segment_transition_fix_section_code: RecordField<'a, Section>,
    pub common_segment_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub common_segment_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
    pub enroute_transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub enroute_transition_fix: RecordField<'a, FixIdentifier>,
    pub enroute_transition_fix_icao_code: RecordField<'a, IcaoCode>,
    pub enroute_transition_fix_section_code: RecordField<'a, Section>,
    pub enroute_transition_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub enroute_transition_fix_along_track_distance: RecordField<'a, AlongTrackDistance>,
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
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for FlightPlanningApproachTimeContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "FlightPlanningApproachTimeContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                                          RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                                   RecordField::from_bytes(input, 2, 3)?,
            section:                                              RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                                   RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                                    RecordField::from_bytes(input, 11, 2)?,
            subsection:                                           RecordField::from_bytes(input, 13, 1)?,
            approach_identifier:                                  RecordField::from_bytes(input, 14, 6)?,
            procedure_type:                                       RecordField::from_bytes(input, 20, 1)?,
            runway_transition_identifier:                         RecordField::from_bytes(input, 21, 5)?,
            runway_transition_fix:                                RecordField::from_bytes(input, 26, 5)?,
            runway_transition_fix_icao_code:                      RecordField::from_bytes(input, 31, 2)?,
            runway_transition_fix_section_code:                   RecordField::from_bytes(input, 33, 1)?,
            runway_transition_fix_subsection_code:                RecordField::from_bytes(input, 34, 1)?,
            runway_transition_fix_along_track_distance:           RecordField::from_bytes(input, 35, 3)?,
            common_segment_transition_fix:                        RecordField::from_bytes(input, 38, 5)?,
            common_segment_transition_fix_icao_code:              RecordField::from_bytes(input, 43, 2)?,
            common_segment_transition_fix_section_code:           RecordField::from_bytes(input, 45, 1)?,
            common_segment_transition_fix_subsection_code:        RecordField::from_bytes(input, 46, 1)?,
            common_segment_transition_fix_along_track_distance:   RecordField::from_bytes(input, 47, 3)?,
            enroute_transition_identifier:                        RecordField::from_bytes(input, 50, 5)?,
            enroute_transition_fix:                               RecordField::from_bytes(input, 55, 5)?,
            enroute_transition_fix_icao_code:                     RecordField::from_bytes(input, 60, 2)?,
            enroute_transition_fix_section_code:                  RecordField::from_bytes(input, 62, 1)?,
            enroute_transition_fix_subsection_code:               RecordField::from_bytes(input, 63, 1)?,
            enroute_transition_fix_along_track_distance:          RecordField::from_bytes(input, 64, 3)?,
            sequence_number:                                      RecordField::from_bytes(input, 67, 3)?,
            continuation_record_number:                           RecordField::from_bytes(input, 70, 1)?,
            application_type:                                     RecordField::from_bytes(input, 71, 1)?,
            time_code:                                            RecordField::from_bytes(input, 72, 1)?,
            time_indicator:                                       RecordField::from_bytes(input, 73, 1)?,
            time_of_operation_1:                                  RecordField::from_bytes(input, 74, 10)?,
            time_of_operation_2:                                  RecordField::from_bytes(input, 84, 10)?,
            time_of_operation_3:                                  RecordField::from_bytes(input, 94, 10)?,
            time_of_operation_4:                                  RecordField::from_bytes(input, 104, 10)?,
            time_of_operation_5:                                  RecordField::from_bytes(input, 114, 10)?,
            file_record_number:                                   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                                           RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.runway_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "runway transition fix reference",
                is_valid_reference(
                    &self.runway_transition_fix,
                    &self.runway_transition_fix_section_code,
                    &self.runway_transition_fix_subsection_code,
                ),
            );
        }
        if !self.common_segment_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "common segment transition fix reference",
                is_valid_reference(
                    &self.common_segment_transition_fix,
                    &self.common_segment_transition_fix_section_code,
                    &self.common_segment_transition_fix_subsection_code,
                ),
            );
        }
        if !self.enroute_transition_fix.value.is_none() {
            validation_result.extend_messages(
                "enroute transition fix reference",
                is_valid_reference(
                    &self.enroute_transition_fix,
                    &self.enroute_transition_fix_section_code,
                    &self.enroute_transition_fix_subsection_code,
                ),
            );
        }
        validation_result.as_result()
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.airport_identifier.raw_bytes,
            self.approach_identifier.raw_bytes,
        ])
    }
}
