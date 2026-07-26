use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct GLSRecords;
impl GLSRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;
    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::GLSPrimary(GLSPrimaryRecord::parse(input)?))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => Ok(
                    ARINCRecord::GLSContinuation(GLSContinuationRecord::parse(input)?),
                ),
                _ => Err(RecordParseError::new(
                    "Invalid continuation record application type".to_string(),
                    Some(String::from_utf8_lossy(input).into_owned()),
                )),
            }
        }
    }
}

/// 4.1.29.1 GLS Primary Record
#[derive(Debug)]
pub struct GLSPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub gls_ref_path_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub gls_category: RecordField<'a, IlsMlsGlsCategory>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub gbas_sbas_channel: RecordField<'a, GLSPathPointChannel>,
    pub runway_identifier: RecordField<'a, RunwayIdentifier>,
    pub gls_approach_bearing: RecordField<'a, LocalizerBearing>,
    pub station_latitude: RecordField<'a, Latitude>,
    pub station_longitude: RecordField<'a, Longitude>,
    pub station_identifier: RecordField<'a, ComponentElevation>,
    pub service_volume_radius: RecordField<'a, GLSServiceVolumeRadius>,
    pub tdma_slots: RecordField<'a, GlsTdmaSlots>,
    pub gls_approach_slope: RecordField<'a, GlideslopeAngle>,
    pub magnetic_variation: RecordField<'a, MagneticVariation>,
    pub station_elevation: RecordField<'a, ComponentElevation>,
    pub datum_code: RecordField<'a, DatumCode>,
    pub station_type: RecordField<'a, GLSStationType>,
    pub station_ellipsoid_height: RecordField<'a, GLSWgs84StationElevation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for GLSPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "GLSPrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:           RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:            RecordField::from_bytes(input, 11, 2)?,
            subsection:                   RecordField::from_bytes(input, 13, 1)?,
            gls_ref_path_identifier:      RecordField::from_bytes(input, 14, 4)?,
            gls_category:                 RecordField::from_bytes(input, 18, 1)?,
            continuation_record_number:   RecordField::from_bytes(input, 22, 1)?,
            gbas_sbas_channel:            RecordField::from_bytes(input, 23, 5)?,
            runway_identifier:            RecordField::from_bytes(input, 28, 5)?,
            gls_approach_bearing:         RecordField::from_bytes(input, 52, 4)?,
            station_latitude:             RecordField::from_bytes(input, 56, 9)?,
            station_longitude:            RecordField::from_bytes(input, 65, 10)?,
            station_identifier:           RecordField::from_bytes(input, 75, 4)?,
            service_volume_radius:        RecordField::from_bytes(input, 84, 2)?,
            tdma_slots:                   RecordField::from_bytes(input, 86, 2)?,
            gls_approach_slope:           RecordField::from_bytes(input, 88, 2)?,
            magnetic_variation:           RecordField::from_bytes(input, 91, 5)?,
            station_elevation:            RecordField::from_bytes(input, 98, 5)?,
            datum_code:                   RecordField::from_bytes(input, 103, 3)?,
            station_type:                 RecordField::from_bytes(input, 106, 3)?,
            station_ellipsoid_height:     RecordField::from_bytes(input, 111, 5)?,
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
            self.airport_identifier.raw_bytes,
            self.gls_ref_path_identifier.raw_bytes,
        ])
    }
}

/// 4.1.29.2 GLS Continuation Record
#[derive(Debug)]
pub struct GLSContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub gls_ref_path_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub gls_category: RecordField<'a, IlsMlsGlsCategory>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for GLSContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "GLSContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:           RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:            RecordField::from_bytes(input, 11, 2)?,
            subsection:                   RecordField::from_bytes(input, 13, 1)?,
            gls_ref_path_identifier:      RecordField::from_bytes(input, 14, 4)?,
            gls_category:                 RecordField::from_bytes(input, 18, 1)?,
            continuation_record_number:   RecordField::from_bytes(input, 22, 1)?,
            application_type:             RecordField::from_bytes(input, 23, 1)?,
            notes:                        RecordField::from_bytes(input, 24, 69)?,
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
            self.airport_identifier.raw_bytes,
            self.gls_ref_path_identifier.raw_bytes,
        ])
    }
}
