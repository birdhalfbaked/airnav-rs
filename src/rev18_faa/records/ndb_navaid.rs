use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct NDBNavaidRecords;
impl NDBNavaidRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::NDBNavaidPrimary(
                NDBNavaidPrimaryRecord::parse(input)?,
            ))
        } else {
            if let Ok(Some(application_type)) = ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            ) {
                match application_type {
                    ContinuationRecordApplicationType::StandardContinuation => {
                        Ok(ARINCRecord::NDBNavaidContinuation(
                            NDBNavaidContinuationRecord::parse(input)?,
                        ))
                    }
                    ContinuationRecordApplicationType::SimulationContinuation => {
                        Ok(ARINCRecord::NDBNavaidSimulationContinuation(
                            NDBNavaidSimulationContinuationRecord::parse(input)?,
                        ))
                    }
                    ContinuationRecordApplicationType::FlightPlanningContinuation => {
                        Ok(ARINCRecord::NDBNavaidFlightPlanningContinuation(
                            NDBNavaidFlightPlanningContinuationRecord::parse(input)?,
                        ))
                    }
                    _ => Err(RecordParseError::new(
                        "Invalid continuation record application type".to_string(),
                        Some(String::from_utf8_lossy(input).into_owned()),
                    )),
                }
            } else {
                Ok(ARINCRecord::NDBNavaidChangedDataContinuation(
                    NDBNavaidChangedDataContinuationRecord::parse(input)?,
                ))
            }
        }
    }
}

/// 4.1.3.1 NDB Navaid Primary Record
#[derive(Debug)]
pub struct NDBNavaidPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, NavaidSubsection>,
    pub airport_icao_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub ndb_identifier: RecordField<'a, VORNDBIdentifier>,
    pub ndb_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub ndb_frequency: RecordField<'a, NDBFrequency>,
    pub ndb_class: RecordField<'a, NDBNavaidClass>,
    pub ndb_latitude: RecordField<'a, Latitude>,
    pub ndb_longitude: RecordField<'a, Longitude>,
    pub magnetic_variation: RecordField<'a, MagneticVariation>,
    pub datum_code: RecordField<'a, DatumCode>,
    pub ndb_name: RecordField<'a, NameOfFacility>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for NDBNavaidPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "NDBNavaidPrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                    RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:             RecordField::from_bytes(input, 2, 3)?,
            section:                        RecordField::from_bytes(input, 5, 1)?,
            subsection:                     RecordField::from_bytes(input, 6, 1)?,
            airport_icao_identifier:        RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:              RecordField::from_bytes(input, 11, 2)?,
            ndb_identifier:                 RecordField::from_bytes(input, 14, 4)?,
            ndb_icao_code:                  RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:     RecordField::from_bytes(input, 22, 1)?,
            ndb_frequency:                  RecordField::from_bytes(input, 23, 5)?,
            ndb_class:                      RecordField::from_bytes(input, 28, 5)?,
            ndb_latitude:                   RecordField::from_bytes(input, 33, 9)?,
            ndb_longitude:                  RecordField::from_bytes(input, 42, 10)?,
            magnetic_variation:             RecordField::from_bytes(input, 75, 5)?,
            datum_code:                     RecordField::from_bytes(input, 91, 3)?,
            ndb_name:                       RecordField::from_bytes(input, 94, 30)?,
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
            self.airport_icao_identifier.raw_bytes,
            self.ndb_identifier.raw_bytes,
        ])
    }
}

/// 4.1.3.2 NDB Navaid Continuation Record
#[derive(Debug)]
pub struct NDBNavaidContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, NavaidSubsection>,
    pub airport_icao_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub ndb_identifier: RecordField<'a, VORNDBIdentifier>,
    pub ndb_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for NDBNavaidContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "NDBNavaidContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:         RecordField::from_bytes(input, 2, 3)?,
            section:                    RecordField::from_bytes(input, 5, 1)?,
            subsection:                 RecordField::from_bytes(input, 6, 1)?,
            airport_icao_identifier:    RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:          RecordField::from_bytes(input, 11, 2)?,
            ndb_identifier:             RecordField::from_bytes(input, 14, 4)?,
            ndb_icao_code:              RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number: RecordField::from_bytes(input, 22, 1)?,
            application_type:           RecordField::from_bytes(input, 23, 1)?,
            notes:                      RecordField::from_bytes(input, 24, 69)?,
            file_record_number:         RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                 RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.airport_icao_identifier.raw_bytes,
            self.ndb_identifier.raw_bytes,
        ])
    }
}

/// 4.1.3.3 NDB Navaid Simulation Continuation Record
#[derive(Debug)]
pub struct NDBNavaidSimulationContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, NavaidSubsection>,
    pub airport_icao_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub ndb_identifier: RecordField<'a, VORNDBIdentifier>,
    pub ndb_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub facility_characteristics: RecordField<'a, FacilityCharacteristics>,
    pub facility_elevation: RecordField<'a, FacilityElevation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for NDBNavaidSimulationContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "NDBNavaidSimulationContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:         RecordField::from_bytes(input, 2, 3)?,
            section:                    RecordField::from_bytes(input, 5, 1)?,
            subsection:                 RecordField::from_bytes(input, 6, 1)?,
            airport_icao_identifier:    RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:          RecordField::from_bytes(input, 11, 2)?,
            ndb_identifier:             RecordField::from_bytes(input, 14, 4)?,
            ndb_icao_code:              RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number: RecordField::from_bytes(input, 22, 1)?,
            application_type:           RecordField::from_bytes(input, 23, 1)?,
            facility_characteristics:   RecordField::from_bytes(input, 28, 5)?,
            facility_elevation:         RecordField::from_bytes(input, 80, 5)?,
            file_record_number:         RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                 RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.airport_icao_identifier.raw_bytes,
            self.ndb_identifier.raw_bytes,
        ])
    }
}

/// 4.1.3.4 NDB Navaid Flight Planning Continuation Record
#[derive(Debug)]
pub struct NDBNavaidFlightPlanningContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, NavaidSubsection>,
    pub airport_icao_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub ndb_identifier: RecordField<'a, VORNDBIdentifier>,
    pub ndb_icao_code: RecordField<'a, IcaoCode>,
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
impl<'a> Arinc424RecordSpec<'a> for NDBNavaidFlightPlanningContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "NDBNavaidFlightPlanningContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            subsection:                         RecordField::from_bytes(input, 6, 1)?,
            airport_icao_identifier:            RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                  RecordField::from_bytes(input, 11, 2)?,
            ndb_identifier:                     RecordField::from_bytes(input, 14, 4)?,
            ndb_icao_code:                      RecordField::from_bytes(input, 20, 2)?,
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
            self.airport_icao_identifier.raw_bytes,
            self.ndb_identifier.raw_bytes,
        ])
    }
}

/// 4.1.3.5 NDB Changed Data Continuation Record
pub type NDBNavaidChangedDataContinuationRecord<'a> = NDBNavaidPrimaryRecord<'a>;
