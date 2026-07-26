use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct AirportSIDRecords;
impl AirportSIDRecords {
    const CONTINUATION_COLUMN: usize = 39;
    const CONTINUATION_APPLICATION_COLUMN: usize = 40;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::AirportSIDPrimary(
                AirportSIDPrimaryRecord::parse(input)?,
            ))
        } else {
            if let Ok(Some(application_type)) = ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            ) {
                match application_type {
                    ContinuationRecordApplicationType::FlightPlanningContinuation => {
                        Ok(ARINCRecord::AirportSIDFlightPlanningContinuation(
                            AirportSIDFlightPlanningContinuationRecord::parse(input)?,
                        ))
                    }
                    _ => Err(RecordParseError::new(
                        "Invalid continuation record application type".to_string(),
                        Some(String::from_utf8_lossy(input).into_owned()),
                    )),
                }
            } else {
                Ok(ARINCRecord::AirportSIDChangedDataContinuation(
                    AirportSIDChangedDataContinuationRecord::parse(input)?,
                ))
            }
        }
    }
}

/// 4.1.9.1(A) Airport SID Primary Record
#[derive(Debug)]
pub struct AirportSIDPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub sid_identifier: RecordField<'a, SidStarRouteIdentifier>,
    pub route_type: RecordField<'a, SIDRouteType>,
    pub transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub waypoint_description: RecordField<'a, WaypointDescriptionCode>,
    pub turn_direction: RecordField<'a, TurnDirection>,
    pub rnp: RecordField<'a, RequiredNavigationPerformance>,
    pub path_and_termination: RecordField<'a, PathAndTermination>,
    pub turn_direction_valid: RecordField<'a, TurnDirectionValid>,
    pub recommended_navaid: RecordField<'a, RecommendedNavaid>,
    pub recommended_navaid_icao_code: RecordField<'a, IcaoCode>,
    pub arc_radius: RecordField<'a, ArcRadius>,
    pub theta: RecordField<'a, Theta>,
    pub rho: RecordField<'a, Rho>,
    pub magnetic_course: RecordField<'a, OutboundCourse>,
    pub route_or_holding_distance: RecordField<'a, RouteDistanceFrom>,
    pub recommended_navaid_section_code: RecordField<'a, Section>,
    pub recommended_navaid_subsection_code: RecordField<'a, NavaidSubsection>,
    pub altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub altitude_atc_indicator: RecordField<'a, AtcIndicator>,
    pub altitude_1: RecordField<'a, MinimumAltitude>,
    pub altitude_2: RecordField<'a, MinimumAltitude>,
    pub transition_altitude: RecordField<'a, TransitionAltitudeLevel>,
    pub speed_limit: RecordField<'a, SpeedLimit>,
    pub vertical_angle: RecordField<'a, VerticalAngle>,
    pub center_fix: RecordField<'a, CenterFix>,
    pub multiple_code: RecordField<'a, MultipleCode>,
    pub center_fix_icao_code: RecordField<'a, IcaoCode>,
    pub center_fix_section_code: RecordField<'a, Section>,
    pub center_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub gnss_fms_indication: RecordField<'a, GNSSFMSIndicator>,
    pub speed_limit_description: RecordField<'a, SpeedLimitDescription>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

impl<'a> Arinc424RecordSpec<'a> for AirportSIDPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "AirportSIDPrimaryRecord"
    }
    
    #[rustfmt::skip]
    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                          RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                   RecordField::from_bytes(input, 2, 3)?,
            section:                              RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                   RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                    RecordField::from_bytes(input, 11, 2)?,
            subsection:                           RecordField::from_bytes(input, 13, 1)?,
            sid_identifier:                       RecordField::from_bytes(input, 14, 6)?,
            route_type:                           RecordField::from_bytes(input, 20, 1)?,
            transition_identifier:                RecordField::from_bytes(input, 21, 5)?,
            sequence_number:                      RecordField::from_bytes(input, 27, 3)?,
            fix_identifier:                       RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                        RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:                     RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:                  RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:           RecordField::from_bytes(input, 39, 1)?,
            waypoint_description:                 RecordField::from_bytes(input, 40, 4)?,
            turn_direction:                       RecordField::from_bytes(input, 44, 1)?,
            rnp:                                  RecordField::from_bytes(input, 45, 3)?,
            path_and_termination:                 RecordField::from_bytes(input, 48, 2)?,
            turn_direction_valid:                 RecordField::from_bytes(input, 50, 1)?,
            recommended_navaid:                   RecordField::from_bytes(input, 51, 4)?,
            recommended_navaid_icao_code:         RecordField::from_bytes(input, 55, 2)?,
            arc_radius:                           RecordField::from_bytes(input, 57, 6)?,
            theta:                                RecordField::from_bytes(input, 63, 4)?,
            rho:                                  RecordField::from_bytes(input, 67, 4)?,
            magnetic_course:                      RecordField::from_bytes(input, 71, 4)?,
            route_or_holding_distance:            RecordField::from_bytes(input, 75, 4)?,
            recommended_navaid_section_code:      RecordField::from_bytes(input, 79, 1)?,
            recommended_navaid_subsection_code:   RecordField::from_bytes(input, 80, 1)?,
            altitude_description:                 RecordField::from_bytes(input, 83, 1)?,
            altitude_atc_indicator:               RecordField::from_bytes(input, 84, 1)?,
            altitude_1:                           RecordField::from_bytes(input, 85, 5)?,
            altitude_2:                           RecordField::from_bytes(input, 90, 5)?,
            transition_altitude:                  RecordField::from_bytes(input, 95, 5)?,
            speed_limit:                          RecordField::from_bytes(input, 100, 3)?,
            vertical_angle:                       RecordField::from_bytes(input, 103, 4)?,
            center_fix:                           RecordField::from_bytes(input, 107, 5)?,
            multiple_code:                        RecordField::from_bytes(input, 112, 1)?,
            center_fix_icao_code:                 RecordField::from_bytes(input, 113, 2)?,
            center_fix_section_code:              RecordField::from_bytes(input, 115, 1)?,
            center_fix_subsection_code:           RecordField::from_bytes(input, 116, 1)?,
            gnss_fms_indication:                  RecordField::from_bytes(input, 117, 1)?,
            speed_limit_description:              RecordField::from_bytes(input, 118, 1)?,
            file_record_number:                   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                           RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.fix_identifier.value.is_none() {
            validation_result.extend_messages(
                "fix reference",
                is_valid_reference(
                    &self.fix_identifier,
                    &self.fix_section_code,
                    &self.fix_subsection_code,
                ),
            );
        }
        if !self.recommended_navaid.value.is_none() {
            validation_result.extend_messages(
                "recommended navaid reference",
                is_valid_reference(
                    &self.recommended_navaid,
                    &self.recommended_navaid_section_code,
                    &self.recommended_navaid_subsection_code,
                ),
            );
        }
        if !self.center_fix.value.is_none() {
            validation_result.extend_messages(
                "center fix reference",
                is_valid_reference(
                    &self.center_fix,
                    &self.center_fix_section_code,
                    &self.center_fix_subsection_code,
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
            self.sid_identifier.raw_bytes,
        ])
    }
}

/// 4.1.9.3(A) Airport SID Flight Planning Continuation Record
#[derive(Debug)]
pub struct AirportSIDFlightPlanningContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub sid_identifier: RecordField<'a, SidStarRouteIdentifier>,
    pub route_type: RecordField<'a, SIDRouteType>,
    pub transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub start_end_indicator: RecordField<'a, StartEndIndicator>,
    pub start_end_date: RecordField<'a, StartEndDate>,
    pub leg_distance: RecordField<'a, TerminalProcedureFlightPlanningLegDistance>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for AirportSIDFlightPlanningContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "AirportSIDFlightPlanningContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                 RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                  RecordField::from_bytes(input, 11, 2)?,
            subsection:                         RecordField::from_bytes(input, 13, 1)?,
            sid_identifier:                     RecordField::from_bytes(input, 14, 6)?,
            route_type:                         RecordField::from_bytes(input, 20, 1)?,
            transition_identifier:              RecordField::from_bytes(input, 21, 5)?,
            sequence_number:                    RecordField::from_bytes(input, 27, 3)?,
            fix_identifier:                     RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                      RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:                   RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:                RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:         RecordField::from_bytes(input, 39, 1)?,
            application_type:                   RecordField::from_bytes(input, 40, 1)?,
            start_end_indicator:                RecordField::from_bytes(input, 41, 1)?,
            start_end_date:                     RecordField::from_bytes(input, 42, 11)?,
            leg_distance:                       RecordField::from_bytes(input, 75, 4)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.fix_identifier.value.is_none() {
            validation_result.extend_messages(
                "fix reference",
                is_valid_reference(
                    &self.fix_identifier,
                    &self.fix_section_code,
                    &self.fix_subsection_code,
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
            self.sid_identifier.raw_bytes,
        ])
    }
}

/// 4.1.9.4(A) Airport Changed Data Continuation Record
pub type AirportSIDChangedDataContinuationRecord<'a> = AirportSIDPrimaryRecord<'a>;
