use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct EnrouteAirwayRecords;
impl EnrouteAirwayRecords {
    const CONTINUATION_COLUMN: usize = 39;
    const CONTINUATION_APPLICATION_COLUMN: usize = 40;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::EnrouteAirwayPrimary(
                EnrouteAirwayPrimaryRecord::parse(input)?,
            ))
        } else {
            if let Ok(Some(application_type)) = ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            ) {
                match application_type {
                    ContinuationRecordApplicationType::StandardContinuation => {
                        Ok(ARINCRecord::EnrouteAirwayContinuation(
                            EnrouteAirwayContinuationRecord::parse(input)?,
                        ))
                    }
                    ContinuationRecordApplicationType::FlightPlanningContinuation => {
                        Ok(ARINCRecord::EnrouteAirwayFlightPlanningContinuation(
                            EnrouteAirwayFlightPlanningContinuationRecord::parse(input)?,
                        ))
                    }
                    _ => Err(RecordParseError::new(
                        "Invalid continuation record application type".to_string(),
                        Some(String::from_utf8_lossy(input).into_owned()),
                    )),
                }
            } else {
                Ok(ARINCRecord::EnrouteAirwayChangedDataContinuation(
                    EnrouteAirwayChangedDataContinuationRecord::parse(input)?,
                ))
            }
        }
    }
}

/// 4.1.6.1 Enroute Airway Primary Record
#[derive(Debug)]
pub struct EnrouteAirwayPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub waypoint_description_code: RecordField<'a, WaypointDescriptionCode>,
    pub boundary_code: RecordField<'a, BoundaryCode>,
    pub route_type: RecordField<'a, EnrouteAirwayRouteType>,
    pub level: RecordField<'a, Level>,
    pub direction_restriction: RecordField<'a, EnrouteDirectionalRestriction>,
    pub cruise_table_indicator: RecordField<'a, CruiseTableIdentifier>,
    pub has_restrictions: RecordField<'a, EnrouteAirwayRestrictionFlag>,
    pub recommended_navaid: RecordField<'a, RecommendedNavaid>,
    pub recommended_navaid_icao_code: RecordField<'a, IcaoCode>,
    pub rnp: RecordField<'a, RequiredNavigationPerformance>,
    pub theta: RecordField<'a, Theta>,
    pub rho: RecordField<'a, Rho>,
    pub outbound_magnetic_course: RecordField<'a, OutboundCourse>,
    pub route_distance_from: RecordField<'a, RouteDistanceFrom>,
    pub inbound_magnetic_course: RecordField<'a, InboundCourse>,
    pub minimum_altitude_1: RecordField<'a, MinimumAltitude>,
    pub minimum_altitude_2: RecordField<'a, MinimumAltitude>,
    pub maximum_altitude_1: RecordField<'a, MaximumAltitude>,
    pub fix_radius_transition: RecordField<'a, FixedRadiusTransitionIndicator>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayPrimaryRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                          RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                   RecordField::from_bytes(input, 2, 3)?,
            section:                              RecordField::from_bytes(input, 5, 1)?,
            subsection:                           RecordField::from_bytes(input, 6, 1)?,
            route_identifier:                     RecordField::from_bytes(input, 14, 5)?,
            sequence_number:                      RecordField::from_bytes(input, 26, 4)?,
            fix_identifier:                       RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                        RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:                     RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:                  RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:           RecordField::from_bytes(input, 39, 1)?,
            waypoint_description_code:            RecordField::from_bytes(input, 40, 4)?,
            boundary_code:                        RecordField::from_bytes(input, 44, 1)?,
            route_type:                           RecordField::from_bytes(input, 45, 1)?,
            level:                                RecordField::from_bytes(input, 46, 1)?,
            direction_restriction:                RecordField::from_bytes(input, 47, 1)?,
            cruise_table_indicator:               RecordField::from_bytes(input, 48, 2)?,
            has_restrictions:                     RecordField::from_bytes(input, 50, 1)?,
            recommended_navaid:                   RecordField::from_bytes(input, 51, 4)?,
            recommended_navaid_icao_code:         RecordField::from_bytes(input, 55, 2)?,
            rnp:                                  RecordField::from_bytes(input, 57, 3)?,
            theta:                                RecordField::from_bytes(input, 63, 4)?,
            rho:                                  RecordField::from_bytes(input, 67, 4)?,
            outbound_magnetic_course:             RecordField::from_bytes(input, 71, 4)?,
            route_distance_from:                  RecordField::from_bytes(input, 75, 4)?,
            inbound_magnetic_course:              RecordField::from_bytes(input, 79, 4)?,
            minimum_altitude_1:                   RecordField::from_bytes(input, 84, 5)?,
            minimum_altitude_2:                   RecordField::from_bytes(input, 89, 5)?,
            maximum_altitude_1:                   RecordField::from_bytes(input, 94, 5)?,
            fix_radius_transition:                RecordField::from_bytes(input, 99, 3)?,
            file_record_number:                   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                           RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.fix_identifier.value.is_none() {
            validation_result.extend_messages(
                "fix identifier reference",
                is_valid_reference(
                    &self.fix_identifier,
                    &self.fix_section_code,
                    &self.fix_subsection_code,
                ),
            );
        }
        validation_result.extend_messages(
            "boundary code",
            is_valid_boundary_code(
                &self.customer_area_code,
                &self.boundary_code,
            ),
        );
        validation_result.as_result()
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.route_identifier.raw_bytes,
        ])
    }
}

/// 4.1.6.2 Enroute Airway Continuation Record
#[derive(Debug)]
pub struct EnrouteAirwayContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                          RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                   RecordField::from_bytes(input, 2, 3)?,
            section:                              RecordField::from_bytes(input, 5, 1)?,
            subsection:                           RecordField::from_bytes(input, 6, 1)?,
            route_identifier:                     RecordField::from_bytes(input, 14, 5)?,
            sequence_number:                      RecordField::from_bytes(input, 26, 4)?,
            fix_identifier:                       RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                        RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:                     RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:                  RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:           RecordField::from_bytes(input, 39, 1)?,
            application_type:                     RecordField::from_bytes(input, 40, 1)?,
            notes:                                RecordField::from_bytes(input, 41, 69)?,
            file_record_number:                   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                           RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.fix_identifier.value.is_none() {
            validation_result.extend_messages(
                "fix identifier reference",
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
            self.route_identifier.raw_bytes,
        ])
    }
}

/// 4.1.6.3 Enroute Airway Flight Planning Continuation Record
#[derive(Debug)]
pub struct EnrouteAirwayFlightPlanningContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub start_end_indicator: RecordField<'a, StartEndIndicator>,
    pub start_end_date: RecordField<'a, StartEndDate>,
    // restricted airspace block 1
    pub restricted_airspace_1_icao_code: RecordField<'a, IcaoCode>,
    pub restricted_airspace_1_type: RecordField<'a, RestrictiveAirspaceType>,
    pub restricted_airspace_1_designation: RecordField<'a, RestrictiveAirspaceDesignation>,
    pub restricted_airspace_1_multiple_code: RecordField<'a, MultipleCode>,
    // restricted airspace block 2
    pub restricted_airspace_2_icao_code: RecordField<'a, IcaoCode>,
    pub restricted_airspace_2_type: RecordField<'a, RestrictiveAirspaceType>,
    pub restricted_airspace_2_designation: RecordField<'a, RestrictiveAirspaceDesignation>,
    pub restricted_airspace_2_multiple_code: RecordField<'a, MultipleCode>,
    // restricted airspace block 3
    pub restricted_airspace_3_icao_code: RecordField<'a, IcaoCode>,
    pub restricted_airspace_3_type: RecordField<'a, RestrictiveAirspaceType>,
    pub restricted_airspace_3_designation: RecordField<'a, RestrictiveAirspaceDesignation>,
    pub restricted_airspace_3_multiple_code: RecordField<'a, MultipleCode>,
    // restricted airspace block 4
    pub restricted_airspace_4_icao_code: RecordField<'a, IcaoCode>,
    pub restricted_airspace_4_type: RecordField<'a, RestrictiveAirspaceType>,
    pub restricted_airspace_4_designation: RecordField<'a, RestrictiveAirspaceDesignation>,
    pub restricted_airspace_4_multiple_code: RecordField<'a, MultipleCode>,
    pub restricted_airspace_link_continuation: RecordField<'a, RestrictiveAirspaceLinkContinuation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayFlightPlanningContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayFlightPlanningContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                             RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                      RecordField::from_bytes(input, 2, 3)?,
            section:                                 RecordField::from_bytes(input, 5, 1)?,
            subsection:                              RecordField::from_bytes(input, 6, 1)?,
            route_identifier:                        RecordField::from_bytes(input, 14, 5)?,
            sequence_number:                         RecordField::from_bytes(input, 26, 4)?,
            fix_identifier:                          RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                           RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:                        RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:                     RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:              RecordField::from_bytes(input, 39, 1)?,
            application_type:                        RecordField::from_bytes(input, 40, 1)?,
            start_end_indicator:                     RecordField::from_bytes(input, 41, 1)?,
            start_end_date:                          RecordField::from_bytes(input, 42, 11)?,
            restricted_airspace_1_icao_code:         RecordField::from_bytes(input, 67, 2)?,
            restricted_airspace_1_type:              RecordField::from_bytes(input, 69, 1)?,
            restricted_airspace_1_designation:       RecordField::from_bytes(input, 70, 10)?,
            restricted_airspace_1_multiple_code:     RecordField::from_bytes(input, 80, 1)?,
            restricted_airspace_2_icao_code:         RecordField::from_bytes(input, 81, 2)?,
            restricted_airspace_2_type:              RecordField::from_bytes(input, 83, 1)?,
            restricted_airspace_2_designation:       RecordField::from_bytes(input, 84, 10)?,
            restricted_airspace_2_multiple_code:     RecordField::from_bytes(input, 94, 1)?,
            restricted_airspace_3_icao_code:         RecordField::from_bytes(input, 95, 2)?,
            restricted_airspace_3_type:              RecordField::from_bytes(input, 97, 1)?,
            restricted_airspace_3_designation:       RecordField::from_bytes(input, 98, 10)?,
            restricted_airspace_3_multiple_code:     RecordField::from_bytes(input, 108, 1)?,
            restricted_airspace_4_icao_code:         RecordField::from_bytes(input, 109, 2)?,
            restricted_airspace_4_type:              RecordField::from_bytes(input, 111, 1)?,
            restricted_airspace_4_designation:       RecordField::from_bytes(input, 112, 10)?,
            restricted_airspace_4_multiple_code:     RecordField::from_bytes(input, 122, 1)?,
            restricted_airspace_link_continuation:   RecordField::from_bytes(input, 123, 1)?,
            file_record_number:                      RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                              RecordField::from_bytes(input, 129, 4)?,
       })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.restricted_airspace_1_icao_code.value.is_none() {
            validation_result.extend_messages(
                "fix identifier reference",
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
            self.route_identifier.raw_bytes,
        ])
    }
}

/// 4.1.6.4 Enroute Airway Changed Data Continuation Record
pub type EnrouteAirwayChangedDataContinuationRecord<'a> = EnrouteAirwayPrimaryRecord<'a>;
