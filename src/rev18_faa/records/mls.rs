use crate::rev18_faa::records::record::ARINCRecord;

use crate::rev18_faa::definitions::*;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError, is_primary_record,
};
pub(super) struct MLSRecords;
impl MLSRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::MLSPrimary(MLSPrimaryRecord::parse(input)?))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => Ok(
                    ARINCRecord::MLSContinuation(MLSContinuationRecord::parse(input)?),
                ),
                _ => Err(RecordParseError::new(
                    "Invalid continuation record application type".to_string(),
                    Some(String::from_utf8_lossy(input).into_owned()),
                )),
            }
        }
    }
}

/// 4.1.22.1 MLS Primary Record
#[derive(Debug)]
pub struct MLSPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub mls_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub mls_category: RecordField<'a, IlsMlsGlsCategory>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub channel: RecordField<'a, MLSChannel>,
    pub runway_identifier: RecordField<'a, RunwayIdentifier>,
    pub azimuth_latitude: RecordField<'a, Latitude>,
    pub azimuth_longitude: RecordField<'a, Longitude>,
    pub azimuth_bearing: RecordField<'a, MLSAzimuthBearing>,
    pub elevation_latitude: RecordField<'a, Latitude>,
    pub elevation_longitude: RecordField<'a, Longitude>,
    pub azimuth_position: RecordField<'a, LocalizerPosition>,
    pub azimuth_position_reference: RecordField<'a, LocalizerAzimuthPositionReference>,
    pub elevation_position: RecordField<'a, GlideslopePosition>,
    pub azimuth_proportional_angle_right: RecordField<'a, MLSAzimuthProportionalAngle>,
    pub azimuth_proportional_angle_left: RecordField<'a, MLSAzimuthProportionalAngle>,
    pub azimuth_coverage_right: RecordField<'a, MLSAzimuthCoverageSector>,
    pub azimuth_coverage_left: RecordField<'a, MLSAzimuthCoverageSector>,
    pub elevation_angle_span: RecordField<'a, MLSElevationAngleSpan>,
    pub magnetic_variation: RecordField<'a, MagneticVariation>,
    pub el_elevation: RecordField<'a, ComponentElevation>,
    pub nominal_elevation_angle: RecordField<'a, MLSNominalElevationAngle>,
    pub minimum_glidepath_angle: RecordField<'a, GlideslopeAngle>,
    pub supporting_facility_id: RecordField<'a, VORNDBIdentifier>,
    pub supporting_facility_icao_code: RecordField<'a, IcaoCode>,
    pub supporting_facility_section: RecordField<'a, Section>,
    pub supporting_facility_subsection: RecordField<'a, NavaidSubsection>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_data: RecordField<'a, CycleDate>,
}

impl<'a> Arinc424RecordSpec<'a> for MLSPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "MLSPrimaryRecord"
    }
    
    #[rustfmt::skip]
    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            airport_heliport_identifier:        RecordField::from_bytes(input, 7, 4)?,
            airport_heliport_icao_code:         RecordField::from_bytes(input, 11, 2)?,
            subsection:                         RecordField::from_bytes(input, 13, 1)?,
            mls_identifier:                     RecordField::from_bytes(input, 14, 4)?,
            mls_category:                       RecordField::from_bytes(input, 18, 1)?,
            continuation_record_number:         RecordField::from_bytes(input, 22, 1)?,
            channel:                            RecordField::from_bytes(input, 23, 3)?,
            runway_identifier:                  RecordField::from_bytes(input, 28, 5)?,
            azimuth_latitude:                   RecordField::from_bytes(input, 33, 9)?,
            azimuth_longitude:                  RecordField::from_bytes(input, 42, 10)?,
            azimuth_bearing:                    RecordField::from_bytes(input, 52, 4)?,
            elevation_latitude:                 RecordField::from_bytes(input, 56, 9)?,
            elevation_longitude:                RecordField::from_bytes(input, 65, 10)?,
            azimuth_position:                   RecordField::from_bytes(input, 75, 4)?,
            azimuth_position_reference:         RecordField::from_bytes(input, 79, 1)?,
            elevation_position:                 RecordField::from_bytes(input, 80, 4)?,
            azimuth_proportional_angle_right:   RecordField::from_bytes(input, 84, 3)?,
            azimuth_proportional_angle_left:    RecordField::from_bytes(input, 87, 3)?,
            azimuth_coverage_right:             RecordField::from_bytes(input, 90, 3)?,
            azimuth_coverage_left:              RecordField::from_bytes(input, 93, 3)?,
            elevation_angle_span:               RecordField::from_bytes(input, 96, 3)?,
            magnetic_variation:                 RecordField::from_bytes(input, 99, 5)?,
            el_elevation:                       RecordField::from_bytes(input, 104, 5)?,
            nominal_elevation_angle:            RecordField::from_bytes(input, 109, 4)?,
            minimum_glidepath_angle:            RecordField::from_bytes(input, 113, 3)?,
            supporting_facility_id:             RecordField::from_bytes(input, 116, 4)?,
            supporting_facility_icao_code:      RecordField::from_bytes(input, 120, 2)?,
            supporting_facility_section:        RecordField::from_bytes(input, 122, 1)?,
            supporting_facility_subsection:     RecordField::from_bytes(input, 123, 1)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_data:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.supporting_facility_id.value.is_none() {
            validation_result.extend_messages(
                "supporting facility reference",
                is_valid_reference(
                    &self.supporting_facility_id,
                    &self.supporting_facility_section,
                    &self.supporting_facility_subsection,
                ),
            );
        }
        validation_result.as_result()
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.airport_heliport_identifier.raw_bytes,
            self.mls_identifier.raw_bytes,
        ])
    }
}

/// 4.1.22.2 MLS Continuation Record
#[derive(Debug)]
pub struct MLSContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub mls_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub mls_category: RecordField<'a, IlsMlsGlsCategory>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub facility_characteristics: RecordField<'a, FacilityCharacteristics>,
    pub back_azimuth_latitude: RecordField<'a, Latitude>,
    pub back_azimuth_longitude: RecordField<'a, Longitude>,
    pub back_azimuth_bearing: RecordField<'a, MLSAzimuthBearing>,
    pub mls_datum_point_latitude: RecordField<'a, Latitude>,
    pub mls_datum_point_longitude: RecordField<'a, Longitude>,
    pub back_azimuth_position: RecordField<'a, LocalizerPosition>,
    pub back_azimuth_position_reference: RecordField<'a, LocalizerAzimuthPositionReference>,
    pub back_azimuth_proportional_angle_right: RecordField<'a, MLSAzimuthProportionalAngle>,
    pub back_azimuth_proportional_angle_left: RecordField<'a, MLSAzimuthProportionalAngle>,
    pub back_azimuth_coverage_right: RecordField<'a, MLSAzimuthCoverageSector>,
    pub back_azimuth_coverage_left: RecordField<'a, MLSAzimuthCoverageSector>,
    pub back_azimuth_true_bearing: RecordField<'a, TrueBearing>,
    pub back_azimuth_bearing_source: RecordField<'a, GovernmentSource>,
    pub azimuth_true_bearing: RecordField<'a, TrueBearing>,
    pub azimuth_bearing_source: RecordField<'a, GovernmentSource>,
    pub glide_path_height_at_landing_threshold: RecordField<'a, LandingThresholdElevation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_data: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for MLSContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "MLSContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                              RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                       RecordField::from_bytes(input, 2, 3)?,
            section:                                  RecordField::from_bytes(input, 5, 1)?,
            airport_heliport_identifier:              RecordField::from_bytes(input, 7, 4)?,
            airport_heliport_icao_code:               RecordField::from_bytes(input, 11, 2)?,
            subsection:                               RecordField::from_bytes(input, 13, 1)?,
            mls_identifier:                           RecordField::from_bytes(input, 14, 4)?,
            mls_category:                             RecordField::from_bytes(input, 18, 1)?,
            continuation_record_number:               RecordField::from_bytes(input, 22, 1)?,
            application_type:                         RecordField::from_bytes(input, 23, 1)?,
            facility_characteristics:                 RecordField::from_bytes(input, 28, 5)?,
            back_azimuth_latitude:                    RecordField::from_bytes(input, 33, 9)?,
            back_azimuth_longitude:                   RecordField::from_bytes(input, 42, 10)?,
            back_azimuth_bearing:                     RecordField::from_bytes(input, 52, 4)?,
            mls_datum_point_latitude:                 RecordField::from_bytes(input, 56, 9)?,
            mls_datum_point_longitude:                RecordField::from_bytes(input, 65, 10)?,
            back_azimuth_position:                    RecordField::from_bytes(input, 75, 4)?,
            back_azimuth_position_reference:          RecordField::from_bytes(input, 79, 1)?,
            back_azimuth_proportional_angle_right:    RecordField::from_bytes(input, 84, 3)?,
            back_azimuth_proportional_angle_left:     RecordField::from_bytes(input, 87, 3)?,
            back_azimuth_coverage_right:              RecordField::from_bytes(input, 90, 3)?,
            back_azimuth_coverage_left:               RecordField::from_bytes(input, 93, 3)?,
            back_azimuth_true_bearing:                RecordField::from_bytes(input, 96, 5)?,
            back_azimuth_bearing_source:              RecordField::from_bytes(input, 101, 1)?,
            azimuth_true_bearing:                     RecordField::from_bytes(input, 102, 5)?,
            azimuth_bearing_source:                   RecordField::from_bytes(input, 107, 1)?,
            glide_path_height_at_landing_threshold:   RecordField::from_bytes(input, 108, 3)?,
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
            self.airport_heliport_identifier.raw_bytes,
            self.mls_identifier.raw_bytes,
        ])
    }
}
