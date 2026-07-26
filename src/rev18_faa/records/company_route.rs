use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
};

pub(super) struct CompanyRouteRecords;
impl CompanyRouteRecords {
    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        Ok(ARINCRecord::CompanyRoutePrimary(
            CompanyRoutePrimaryRecord::parse(input)?,
        ))
    }
}

/// 4.1.12.1 Company Route Primary Record
#[derive(Debug)]
pub struct CompanyRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub from_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub from_icao_code: RecordField<'a, IcaoCode>,
    pub from_section_code: RecordField<'a, Section>,
    pub from_subsection_code: RecordField<'a, GenericSubsection>,
    pub to_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_icao_code: RecordField<'a, IcaoCode>,
    pub to_section_code: RecordField<'a, Section>,
    pub to_subsection_code: RecordField<'a, GenericSubsection>,
    pub company_route_id: RecordField<'a, CompanyRouteIdent>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub route_identifier: RecordField<'a, SidStarApproachAirwayIdentifier>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub to_fix: RecordField<'a, CompanyRouteToFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section: RecordField<'a, Section>,
    pub to_fix_subsection: RecordField<'a, GenericSubsection>,
    pub runway_transition: RecordField<'a, RunwayTransition>,
    pub enroute_transition: RecordField<'a, EnrouteTransition>,
    pub cruise_altitude: RecordField<'a, CruiseAltitude>,
    pub terminal_alternate_airport: RecordField<'a, TerminalAlternateAirport>,
    pub terminal_alternate_airport_icao_code: RecordField<'a, IcaoCode>,
    pub alternate_distance: RecordField<'a, AlternateDistance>,
    pub cost_index: RecordField<'a, CostIndex>,
    pub enroute_alternate_airport: RecordField<'a, EnrouteAlternateAirportHeliport>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for CompanyRoutePrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "CompanyRoutePrimaryRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(CompanyRoutePrimaryRecord {
            record_type:                            RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                     RecordField::from_bytes(input, 2,3)?,
            section:                                RecordField::from_bytes(input, 5, 1)?,
            subsection:                             RecordField::from_bytes(input, 6, 1)?,
            from_identifier:                        RecordField::from_bytes(input, 7, 5)?,
            from_icao_code:                         RecordField::from_bytes(input, 13, 2)?,
            from_section_code:                      RecordField::from_bytes(input, 15, 1)?,
            from_subsection_code:                   RecordField::from_bytes(input, 16, 1)?,
            to_identifier:                          RecordField::from_bytes(input, 17, 5)?,
            to_icao_code:                           RecordField::from_bytes(input, 23, 2)?,
            to_section_code:                        RecordField::from_bytes(input, 25, 1)?,
            to_subsection_code:                     RecordField::from_bytes(input, 26, 1)?,
            company_route_id:                       RecordField::from_bytes(input, 27, 10)?,
            sequence_number:                        RecordField::from_bytes(input, 37, 3)?,
            via_code:                               RecordField::from_bytes(input, 40, 3)?,
            route_identifier:                       RecordField::from_bytes(input, 43, 6)?,
            area:                                   RecordField::from_bytes(input, 49, 3)?,
            to_fix:                                 RecordField::from_bytes(input, 52, 6)?,
            to_fix_icao_code:                       RecordField::from_bytes(input, 58, 2)?,
            to_fix_section:                         RecordField::from_bytes(input, 60, 1)?,
            to_fix_subsection:                      RecordField::from_bytes(input, 61, 1)?,
            runway_transition:                      RecordField::from_bytes(input, 62, 5)?,
            enroute_transition:                     RecordField::from_bytes(input, 67, 5)?,
            cruise_altitude:                        RecordField::from_bytes(input, 73, 5)?,
            terminal_alternate_airport:             RecordField::from_bytes(input, 78, 4)?,
            terminal_alternate_airport_icao_code:   RecordField::from_bytes(input, 82, 2)?,
            alternate_distance:                     RecordField::from_bytes(input, 84, 4)?,
            cost_index:                             RecordField::from_bytes(input, 88, 3)?,
            enroute_alternate_airport:              RecordField::from_bytes(input, 91, 4)?,
            file_record_number:                     RecordField::from_bytes(input, 124,5)?,
            cycle_date:                             RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.from_identifier.value.is_none() {
            validation_result.extend_messages(
                "from identifier reference",
                is_valid_reference(
                    &self.from_identifier,
                    &self.from_section_code,
                    &self.from_subsection_code,
                ),
            );
        }
        if !self.to_identifier.value.is_none() {
            validation_result.extend_messages(
                "to identifier reference",
                is_valid_reference(
                    &self.to_identifier,
                    &self.to_section_code,
                    &self.to_subsection_code,
                ),
            );
        }
        if !self.to_fix.value.is_none() {
            validation_result.extend_messages(
                "to fix reference",
                is_valid_reference(
                    &self.to_fix,
                    &self.to_fix_section,
                    &self.to_fix_subsection,
                ),  
            );
        }
        validation_result.as_result()
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.company_route_id.raw_bytes,
        ])
    }
}
