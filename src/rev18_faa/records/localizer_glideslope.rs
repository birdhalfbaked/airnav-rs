use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::*;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct LocalizerGlideslopeRecords;
impl LocalizerGlideslopeRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;
    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::LocalizerGlideslopePrimary(
                LocalizerGlideslopePrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    Ok(ARINCRecord::LocalizerGlideslopeContinuation(
                        LocalizerGlideslopeContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::SimulationContinuation) => {
                    Ok(ARINCRecord::LocalizerGlideslopeSimulationContinuation(
                        LocalizerGlideslopeSimulationContinuationRecord::parse(input)?,
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

/// 4.1.11.1(A) Airport Localizer Glideslope Primary Record
#[derive(Debug)]
pub struct LocalizerGlideslopePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub localizer_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub ils_category: RecordField<'a, IlsMlsGlsCategory>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub localizer_frequency: RecordField<'a, LocalizerFrequency>,
    pub runway_identifier: RecordField<'a, RunwayIdentifier>,
    pub localizer_latitude: RecordField<'a, Latitude>,
    pub localizer_longitude: RecordField<'a, Longitude>,
    pub localizer_bearing: RecordField<'a, LocalizerBearing>,
    pub glideslope_latitude: RecordField<'a, Latitude>,
    pub glideslope_longitude: RecordField<'a, Longitude>,
    pub localizer_position: RecordField<'a, LocalizerPosition>,
    pub localizer_position_reference: RecordField<'a, LocalizerAzimuthPositionReference>,
    pub glideslope_position: RecordField<'a, GlideslopePosition>,
    pub localizer_width: RecordField<'a, LocalizerWidth>,
    pub glideslope_angle: RecordField<'a, GlideslopeAngle>,
    pub station_declination: RecordField<'a, StationDeclination>,
    pub glideslope_height_at_landing_threshold: RecordField<'a, ThresholdCrossingHeight>,
    pub glideslope_elevation: RecordField<'a, ComponentElevation>,
    pub supporting_facility_id: RecordField<'a, NameOfFacility>,
    pub supporting_facility_icao_code: RecordField<'a, IcaoCode>,
    pub supporting_facility_section: RecordField<'a, Section>,
    pub supporting_facility_subsection: RecordField<'a, GenericSubsection>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_data: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for LocalizerGlideslopePrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "LocalizerGlideslopePrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                              RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                       RecordField::from_bytes(input, 2, 3)?,
            section:                                  RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                       RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                        RecordField::from_bytes(input, 11, 2)?,
            subsection:                               RecordField::from_bytes(input, 13, 1)?,
            localizer_identifier:                     RecordField::from_bytes(input, 14, 4)?,
            ils_category:                             RecordField::from_bytes(input, 18, 1)?,
            continuation_record_number:               RecordField::from_bytes(input, 22, 1)?,
            localizer_frequency:                      RecordField::from_bytes(input, 23, 5)?,
            runway_identifier:                        RecordField::from_bytes(input, 28, 5)?,
            localizer_latitude:                       RecordField::from_bytes(input, 33, 9)?,
            localizer_longitude:                      RecordField::from_bytes(input, 42, 10)?,
            localizer_bearing:                        RecordField::from_bytes(input, 52, 4)?,
            glideslope_latitude:                      RecordField::from_bytes(input, 56, 9)?,
            glideslope_longitude:                     RecordField::from_bytes(input, 65, 10)?,
            localizer_position:                       RecordField::from_bytes(input, 75, 4)?,
            localizer_position_reference:             RecordField::from_bytes(input, 79, 1)?,
            glideslope_position:                      RecordField::from_bytes(input, 80, 4)?,
            localizer_width:                          RecordField::from_bytes(input, 84, 4)?,
            glideslope_angle:                         RecordField::from_bytes(input, 88, 3)?,
            station_declination:                      RecordField::from_bytes(input, 91, 5)?,
            glideslope_height_at_landing_threshold:   RecordField::from_bytes(input, 96, 2)?,
            glideslope_elevation:                     RecordField::from_bytes(input, 98, 5)?,
            supporting_facility_id:                   RecordField::from_bytes(input, 103, 4)?,
            supporting_facility_icao_code:            RecordField::from_bytes(input, 107, 2)?,
            supporting_facility_section:              RecordField::from_bytes(input, 109, 1)?,
            supporting_facility_subsection:           RecordField::from_bytes(input, 110, 1)?,
            file_record_number:                       RecordField::from_bytes(input, 124, 5)?,
            cycle_data:                               RecordField::from_bytes(input, 129, 4)?,
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
            self.localizer_identifier.raw_bytes,
        ])
    }
}

/// 4.1.11.2 Localizer Glideslope Continuation Record
#[derive(Debug)]
pub struct LocalizerGlideslopeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub localizer_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub ils_category: RecordField<'a, IlsMlsGlsCategory>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_data: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for LocalizerGlideslopeContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "LocalizerGlideslopeContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                              RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                       RecordField::from_bytes(input, 2, 3)?,
            section:                                  RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                       RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                        RecordField::from_bytes(input, 11, 2)?,
            subsection:                               RecordField::from_bytes(input, 13, 1)?,
            localizer_identifier:                     RecordField::from_bytes(input, 14, 4)?,
            ils_category:                             RecordField::from_bytes(input, 18, 1)?,
            continuation_record_number:               RecordField::from_bytes(input, 22, 1)?,
            application_type:                         RecordField::from_bytes(input, 23, 1)?,
            notes:                                    RecordField::from_bytes(input, 24, 69)?,
            file_record_number:                       RecordField::from_bytes(input, 124, 5)?,
            cycle_data:                               RecordField::from_bytes(input, 129, 4)?,
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
            self.localizer_identifier.raw_bytes,
        ])
    }
}

/// 4.1.11.3 Localizer Glideslope Simulation Continuation Record
#[derive(Debug)]
pub struct LocalizerGlideslopeSimulationContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub localizer_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub ils_category: RecordField<'a, IlsMlsGlsCategory>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub facility_characteristics: RecordField<'a, FacilityCharacteristics>,
    pub localizer_true_bearing: RecordField<'a, TrueBearing>,
    pub localizer_true_bearing_source: RecordField<'a, GovernmentSource>,
    pub glideslope_beam_width: RecordField<'a, GlideslopeBeamWidth>,
    pub approach_route1_ident: RecordField<'a, ApproachRouteIdentifier>,
    pub approach_route2_ident: RecordField<'a, ApproachRouteIdentifier>,
    pub approach_route3_ident: RecordField<'a, ApproachRouteIdentifier>,
    pub approach_route4_ident: RecordField<'a, ApproachRouteIdentifier>,
    pub approach_route5_ident: RecordField<'a, ApproachRouteIdentifier>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_data: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for LocalizerGlideslopeSimulationContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "LocalizerGlideslopeSimulationContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                              RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                       RecordField::from_bytes(input, 2, 3)?,
            section:                                  RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                       RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                        RecordField::from_bytes(input, 11, 2)?,
            subsection:                               RecordField::from_bytes(input, 13, 1)?,
            localizer_identifier:                     RecordField::from_bytes(input, 14, 4)?,
            ils_category:                             RecordField::from_bytes(input, 18, 1)?,
            continuation_record_number:               RecordField::from_bytes(input, 22, 1)?,
            application_type:                         RecordField::from_bytes(input, 23, 1)?,
            facility_characteristics:                 RecordField::from_bytes(input, 28, 5)?,
            localizer_true_bearing:                   RecordField::from_bytes(input, 52, 5)?,
            localizer_true_bearing_source:            RecordField::from_bytes(input, 57, 1)?,
            glideslope_beam_width:                    RecordField::from_bytes(input, 88, 3)?,
            approach_route1_ident:                    RecordField::from_bytes(input, 91, 6)?,
            approach_route2_ident:                    RecordField::from_bytes(input, 97, 6)?,
            approach_route3_ident:                    RecordField::from_bytes(input, 103, 6)?,
            approach_route4_ident:                    RecordField::from_bytes(input, 109, 6)?,
            approach_route5_ident:                    RecordField::from_bytes(input, 115, 6)?,
            file_record_number:                       RecordField::from_bytes(input, 124, 5)?,
            cycle_data:                               RecordField::from_bytes(input, 129, 4)?,
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
            self.localizer_identifier.raw_bytes,
        ])
    }
}
