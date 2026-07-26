use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError, is_primary_record,
};
pub(super) struct HeliportTAARecords;
impl HeliportTAARecords {
    const CONTINUATION_COLUMN: usize = 39;
    const CONTINUATION_APPLICATION_COLUMN: usize = 40;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::HeliportTAAPrimary(
                HeliportTAAPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN
                    ..Self::CONTINUATION_APPLICATION_COLUMN + 1],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    Ok(ARINCRecord::HeliportTAAContinuation(
                        HeliportTAAContinuationRecord::parse(input)?,
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

/// 4.1.31.1 Airport TAA Primary Record
#[derive(Debug)]
pub struct HeliportTAAPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub approach_identifier: RecordField<'a, ApproachRouteIdentifier>,
    pub taa_sector_identifier: RecordField<'a, TaaSectorIdentifier>,
    pub taa_procedure_turn_indicator: RecordField<'a, ProcedureTurn>,
    pub taa_waypoint: RecordField<'a, TaaWaypoint>,
    pub taa_waypoint_icao_code: RecordField<'a, IcaoCode>,
    pub taa_waypoint_section: RecordField<'a, Section>,
    pub taa_waypoint_subsection: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub magnetic_true_indicator: RecordField<'a, MagneticTrueIndicator>,
    pub sector_1_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_1_bearing: RecordField<'a, SectorBearing>,
    pub sector_1_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub sector_2_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_2_bearing: RecordField<'a, SectorBearing>,
    pub sector_2_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub sector_3_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_3_bearing: RecordField<'a, SectorBearing>,
    pub sector_3_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub sector_4_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_4_bearing: RecordField<'a, SectorBearing>,
    pub sector_4_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub sector_5_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_5_bearing: RecordField<'a, SectorBearing>,
    pub sector_5_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub sector_6_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_6_bearing: RecordField<'a, SectorBearing>,
    pub sector_6_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

impl<'a> Arinc424RecordSpec<'a> for HeliportTAAPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportTAAPrimaryRecord"
    }
    
    #[rustfmt::skip]
    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                             RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                      RecordField::from_bytes(input, 2, 3)?,
            section:                                 RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                     RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                      RecordField::from_bytes(input, 11, 2)?,
            subsection:                              RecordField::from_bytes(input, 13, 1)?,
            approach_identifier:                     RecordField::from_bytes(input, 14, 6)?,
            taa_sector_identifier:                   RecordField::from_bytes(input, 20, 1)?,
            taa_procedure_turn_indicator:            RecordField::from_bytes(input, 21, 4)?,
            taa_waypoint:                            RecordField::from_bytes(input, 30, 5)?,
            taa_waypoint_icao_code:                  RecordField::from_bytes(input, 35, 2)?,
            taa_waypoint_section:                    RecordField::from_bytes(input, 37, 1)?,
            taa_waypoint_subsection:                 RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:              RecordField::from_bytes(input, 39, 1)?,
            magnetic_true_indicator:                 RecordField::from_bytes(input, 41, 1)?,
            sector_1_radius:                         RecordField::from_bytes(input, 42, 4)?,
            sector_1_bearing:                        RecordField::from_bytes(input, 46, 6)?,
            sector_1_minimum_altitude:               RecordField::from_bytes(input, 52, 3)?,
            sector_2_radius:                         RecordField::from_bytes(input, 55, 4)?,
            sector_2_bearing:                        RecordField::from_bytes(input, 59, 6)?,
            sector_2_minimum_altitude:               RecordField::from_bytes(input, 65, 3)?,
            sector_3_radius:                         RecordField::from_bytes(input, 68, 4)?,
            sector_3_bearing:                        RecordField::from_bytes(input, 72, 6)?,
            sector_3_minimum_altitude:               RecordField::from_bytes(input, 78, 3)?,
            sector_4_radius:                         RecordField::from_bytes(input, 81, 4)?,
            sector_4_bearing:                        RecordField::from_bytes(input, 85, 6)?,
            sector_4_minimum_altitude:               RecordField::from_bytes(input, 91, 3)?,
            sector_5_radius:                         RecordField::from_bytes(input, 94, 4)?,
            sector_5_bearing:                        RecordField::from_bytes(input, 98, 6)?,
            sector_5_minimum_altitude:               RecordField::from_bytes(input, 104, 3)?,
            sector_6_radius:                         RecordField::from_bytes(input, 107, 4)?,
            sector_6_bearing:                        RecordField::from_bytes(input, 111, 6)?,
            sector_6_minimum_altitude:               RecordField::from_bytes(input, 117, 3)?,
            file_record_number:                      RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                              RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.taa_waypoint.value.is_none() {
            validation_result.extend_messages(
                "taa waypoint reference",
                is_valid_reference(
                    &self.taa_waypoint,
                    &self.taa_waypoint_section,
                    &self.taa_waypoint_subsection,
                ),
            );
        }
        validation_result.as_result()
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.heliport_identifier.raw_bytes,
            self.approach_identifier.raw_bytes,
        ])
    }
}

/// 4.1.31.2 Airport TAA Continuation Record
#[derive(Debug)]
pub struct HeliportTAAContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub approach_identifier: RecordField<'a, ApproachRouteIdentifier>,
    pub taa_sector_identifier: RecordField<'a, TaaSectorIdentifier>,
    pub taa_procedure_turn_indicator: RecordField<'a, ProcedureTurn>,
    pub taa_waypoint: RecordField<'a, TaaWaypoint>,
    pub taa_waypoint_icao_code: RecordField<'a, IcaoCode>,
    pub taa_waypoint_section: RecordField<'a, Section>,
    pub taa_waypoint_subsection: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

impl<'a> Arinc424RecordSpec<'a> for HeliportTAAContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportTAAContinuationRecord"
    }
    
    #[rustfmt::skip]
    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                             RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                      RecordField::from_bytes(input, 2, 3)?,
            section:                                 RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                     RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                      RecordField::from_bytes(input, 11, 2)?,
            subsection:                              RecordField::from_bytes(input, 13, 1)?,
            approach_identifier:                     RecordField::from_bytes(input, 14, 6)?,
            taa_sector_identifier:                   RecordField::from_bytes(input, 20, 1)?,
            taa_procedure_turn_indicator:            RecordField::from_bytes(input, 21, 4)?,
            taa_waypoint:                            RecordField::from_bytes(input, 30, 5)?,
            taa_waypoint_icao_code:                  RecordField::from_bytes(input, 35, 2)?,
            taa_waypoint_section:                    RecordField::from_bytes(input, 37, 1)?,
            taa_waypoint_subsection:                 RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:              RecordField::from_bytes(input, 39, 1)?,
            application_type:                        RecordField::from_bytes(input, 40, 1)?,
            notes:                                   RecordField::from_bytes(input, 41, 69)?,
            file_record_number:                      RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                              RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.taa_waypoint.value.is_none() {
            validation_result.extend_messages(
                "taa waypoint reference",
                is_valid_reference(
                    &self.taa_waypoint,
                    &self.taa_waypoint_section,
                    &self.taa_waypoint_subsection,
                ),
            );
        }
        validation_result.as_result()
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.heliport_identifier.raw_bytes,
            self.approach_identifier.raw_bytes,
        ])
    }
}
