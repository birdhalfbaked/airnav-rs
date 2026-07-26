use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct RunwayRecords;
impl RunwayRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::RunwayPrimary(RunwayPrimaryRecord::parse(
                input,
            )?))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => Ok(
                    ARINCRecord::RunwayContinuation(RunwayContinuationRecord::parse(input)?),
                ),
                Some(ContinuationRecordApplicationType::SimulationContinuation) => {
                    Ok(ARINCRecord::RunwaySimulationContinuation(
                        RunwaySimulationContinuationRecord::parse(input)?,
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

/// 4.1.10.1 Runway Primary Record
#[derive(Debug)]
pub struct RunwayPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, AirportSubsection>,
    pub runway_identifier: RecordField<'a, RunwayIdentifier>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub runway_length: RecordField<'a, RunwayLength>,
    pub runway_bearing: RecordField<'a, RunwayBearing>,
    pub runway_latitude: RecordField<'a, Latitude>,
    pub runway_longitude: RecordField<'a, Longitude>,
    pub runway_gradient: RecordField<'a, RunwayGradient>,
    pub runway_ellipsoid_height: RecordField<'a, WGS84EllipsoidHeight>,
    pub landing_threshold_elevation: RecordField<'a, LandingThresholdElevation>,
    pub displaced_threshold_distance: RecordField<'a, ThresholdDisplacementDistance>,
    pub runway_width: RecordField<'a, RunwayWidth>,
    pub tch_value_indicator: RecordField<'a, TCHValueIndicator>,
    pub guidance_1_reference_path_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub guidance_1_category_class: RecordField<'a, IlsMlsGlsCategory>,
    pub stopway: RecordField<'a, Stopway>,
    pub guidance_2_reference_path_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub guidance_2_category_class: RecordField<'a, IlsMlsGlsCategory>,
    pub runway_description: RecordField<'a, RunwayDescription>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_data: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for RunwayPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "RunwayPrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                                            RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                                     RecordField::from_bytes(input, 2, 3)?,
            section:                                                RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                                     RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                                      RecordField::from_bytes(input, 11, 2)?,
            subsection:                                             RecordField::from_bytes(input, 13, 1)?,
            runway_identifier:                                      RecordField::from_bytes(input, 14, 5)?,
            continuation_record_number:                             RecordField::from_bytes(input, 22, 1)?,
            runway_length:                                          RecordField::from_bytes(input, 23, 5)?,
            runway_bearing:                                         RecordField::from_bytes(input, 28, 4)?,
            runway_latitude:                                        RecordField::from_bytes(input, 33, 9)?,
            runway_longitude:                                       RecordField::from_bytes(input, 42, 10)?,
            runway_gradient:                                        RecordField::from_bytes(input, 52, 5)?,
            runway_ellipsoid_height:                                RecordField::from_bytes(input, 61, 6)?,
            landing_threshold_elevation:                            RecordField::from_bytes(input, 67, 5)?,
            displaced_threshold_distance:                           RecordField::from_bytes(input, 72, 4)?,
            runway_width:                                           RecordField::from_bytes(input, 78, 3)?,
            tch_value_indicator:                                    RecordField::from_bytes(input, 81, 1)?,
            guidance_1_reference_path_identifier:                   RecordField::from_bytes(input, 82, 4)?,
            guidance_1_category_class:                              RecordField::from_bytes(input, 86, 1)?,
            stopway:                                                RecordField::from_bytes(input, 87, 4)?,
            guidance_2_reference_path_identifier:                   RecordField::from_bytes(input, 91, 4)?,
            guidance_2_category_class:                              RecordField::from_bytes(input, 95, 1)?,
            runway_description:                                     RecordField::from_bytes(input, 102, 22)?,
            file_record_number:                                     RecordField::from_bytes(input, 124, 5)?,
            cycle_data:                                             RecordField::from_bytes(input, 129, 4)?,
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
            self.runway_identifier.raw_bytes,
        ])
    }
}

/// 4.1.10.2 Runway Continuation Record
#[derive(Debug)]
pub struct RunwayContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, AirportSubsection>,
    pub runway_identifier: RecordField<'a, RunwayIdentifier>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_data: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for RunwayContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "RunwayContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                                            RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                                     RecordField::from_bytes(input, 2, 3)?,
            section:                                                RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                                     RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                                      RecordField::from_bytes(input, 11, 2)?,
            subsection:                                             RecordField::from_bytes(input, 13, 1)?,
            runway_identifier:                                      RecordField::from_bytes(input, 14, 5)?,
            continuation_record_number:                             RecordField::from_bytes(input, 22, 1)?,
            application_type:                                       RecordField::from_bytes(input, 23, 1)?,
            notes:                                                  RecordField::from_bytes(input, 24, 69)?,
            file_record_number:                                     RecordField::from_bytes(input, 124, 5)?,
            cycle_data:                                             RecordField::from_bytes(input, 129, 4)?,
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
            self.runway_identifier.raw_bytes,
        ])
    }
}

/// 4.1.10.3 Runway Simulation Continuation Record
#[derive(Debug)]
pub struct RunwaySimulationContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, AirportSubsection>,
    pub runway_identifier: RecordField<'a, RunwayIdentifier>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub runway_true_bearing: RecordField<'a, TrueBearing>,
    pub true_bearing_source: RecordField<'a, GovernmentSource>,
    pub tdz_elevation_type: RecordField<'a, ElevationType>,
    pub touchdown_zone_elevation: RecordField<'a, TouchdownZoneElevation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_data: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for RunwaySimulationContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "RunwaySimulationContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                                            RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                                     RecordField::from_bytes(input, 2, 3)?,
            section:                                                RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                                     RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                                      RecordField::from_bytes(input, 11, 2)?,
            subsection:                                             RecordField::from_bytes(input, 13, 1)?,
            runway_identifier:                                      RecordField::from_bytes(input, 14, 5)?,
            continuation_record_number:                             RecordField::from_bytes(input, 22, 1)?,
            application_type:                                       RecordField::from_bytes(input, 23, 1)?,
            runway_true_bearing:                                    RecordField::from_bytes(input, 52, 5)?,
            true_bearing_source:                                    RecordField::from_bytes(input, 57, 1)?,
            tdz_elevation_type:                                     RecordField::from_bytes(input, 66, 1)?,
            touchdown_zone_elevation:                               RecordField::from_bytes(input, 67, 5)?,
            file_record_number:                                     RecordField::from_bytes(input, 124, 5)?,
            cycle_data:                                             RecordField::from_bytes(input, 129, 4)?,
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
            self.runway_identifier.raw_bytes,
        ])
    }
}
