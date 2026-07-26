use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError, is_primary_record,
};
pub(super) struct HeliportMSARecords;
impl HeliportMSARecords {
    const CONTINUATION_COLUMN: usize = 39;
    const CONTINUATION_APPLICATION_COLUMN: usize = 40;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::HeliportMSAPrimary(
                HeliportMSAPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    Ok(ARINCRecord::HeliportMSAContinuation(
                        HeliportMSAContinuationRecord::parse(input)?,
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

/// 4.1.20.1 Airport MSA Primary Record
#[derive(Debug)]
pub struct HeliportMSAPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub msa_center: RecordField<'a, CenterFix>,
    pub msa_center_icao_code: RecordField<'a, IcaoCode>,
    pub msa_center_section_code: RecordField<'a, Section>,
    pub msa_center_subsection_code: RecordField<'a, GenericSubsection>,
    pub multiple_code: RecordField<'a, MultipleCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub sector_1_bearing: RecordField<'a, SectorBearing>,
    pub sector_1_altitude: RecordField<'a, SectorAltitude>,
    pub sector_1_radius: RecordField<'a, MsaRadiusLimit>,
    pub sector_2_bearing: RecordField<'a, SectorBearing>,
    pub sector_2_altitude: RecordField<'a, SectorAltitude>,
    pub sector_2_radius: RecordField<'a, MsaRadiusLimit>,
    pub sector_3_bearing: RecordField<'a, SectorBearing>,
    pub sector_3_altitude: RecordField<'a, SectorAltitude>,
    pub sector_3_radius: RecordField<'a, MsaRadiusLimit>,
    pub sector_4_bearing: RecordField<'a, SectorBearing>,
    pub sector_4_altitude: RecordField<'a, SectorAltitude>,
    pub sector_4_radius: RecordField<'a, MsaRadiusLimit>,
    pub sector_5_bearing: RecordField<'a, SectorBearing>,
    pub sector_5_altitude: RecordField<'a, SectorAltitude>,
    pub sector_5_radius: RecordField<'a, MsaRadiusLimit>,
    pub sector_6_bearing: RecordField<'a, SectorBearing>,
    pub sector_6_altitude: RecordField<'a, SectorAltitude>,
    pub sector_6_radius: RecordField<'a, MsaRadiusLimit>,
    pub sector_7_bearing: RecordField<'a, SectorBearing>,
    pub sector_7_altitude: RecordField<'a, SectorAltitude>,
    pub sector_7_radius: RecordField<'a, MsaRadiusLimit>,
    pub magnetic_true_indicator: RecordField<'a, MagneticTrueIndicator>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

impl<'a> Arinc424RecordSpec<'a> for HeliportMSAPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportMSAPrimaryRecord"
    }
    
    #[rustfmt::skip]
    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:          RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:           RecordField::from_bytes(input, 11, 2)?,
            subsection:                   RecordField::from_bytes(input, 13, 1)?,
            msa_center:                   RecordField::from_bytes(input, 14, 5)?,
            msa_center_icao_code:         RecordField::from_bytes(input, 19, 2)?,
            msa_center_section_code:      RecordField::from_bytes(input, 21, 1)?,
            msa_center_subsection_code:   RecordField::from_bytes(input, 22, 1)?,
            multiple_code:                RecordField::from_bytes(input, 23, 1)?,
            continuation_record_number:   RecordField::from_bytes(input, 39, 1)?,
            sector_1_bearing:             RecordField::from_bytes(input, 43, 6)?,
            sector_1_altitude:            RecordField::from_bytes(input, 49, 3)?,
            sector_1_radius:              RecordField::from_bytes(input, 52, 2)?,
            sector_2_bearing:             RecordField::from_bytes(input, 54, 6)?,
            sector_2_altitude:            RecordField::from_bytes(input, 60, 3)?,
            sector_2_radius:              RecordField::from_bytes(input, 63, 2)?,
            sector_3_bearing:             RecordField::from_bytes(input, 65, 6)?,
            sector_3_altitude:            RecordField::from_bytes(input, 71, 3)?,
            sector_3_radius:              RecordField::from_bytes(input, 74, 2)?,
            sector_4_bearing:             RecordField::from_bytes(input, 76, 6)?,
            sector_4_altitude:            RecordField::from_bytes(input, 82, 3)?,
            sector_4_radius:              RecordField::from_bytes(input, 85, 2)?,
            sector_5_bearing:             RecordField::from_bytes(input, 87, 6)?,
            sector_5_altitude:            RecordField::from_bytes(input, 93, 3)?,
            sector_5_radius:              RecordField::from_bytes(input, 96, 2)?,
            sector_6_bearing:             RecordField::from_bytes(input, 98, 6)?,
            sector_6_altitude:            RecordField::from_bytes(input, 104, 3)?,
            sector_6_radius:              RecordField::from_bytes(input, 107, 2)?,
            sector_7_bearing:             RecordField::from_bytes(input, 109, 6)?,
            sector_7_altitude:            RecordField::from_bytes(input, 115, 3)?,
            sector_7_radius:              RecordField::from_bytes(input, 118, 2)?,
            magnetic_true_indicator:      RecordField::from_bytes(input, 120, 1)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.msa_center.value.is_none() {
            validation_result.extend_messages(
                "msa center reference",
                is_valid_reference(
                    &self.msa_center,
                    &self.msa_center_section_code,
                    &self.msa_center_subsection_code,
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
            self.msa_center.raw_bytes,
            self.multiple_code.raw_bytes,
        ])
    }
}

/// 4.1.20.2 Airport MSA Continuation Record
#[derive(Debug)]
pub struct HeliportMSAContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub msa_center: RecordField<'a, CenterFix>,
    pub msa_center_icao_code: RecordField<'a, IcaoCode>,
    pub msa_center_section_code: RecordField<'a, Section>,
    pub msa_center_subsection_code: RecordField<'a, GenericSubsection>,
    pub multiple_code: RecordField<'a, MultipleCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for HeliportMSAContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportMSAContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:          RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:           RecordField::from_bytes(input, 11, 2)?,
            subsection:                   RecordField::from_bytes(input, 13, 1)?,
            msa_center:                   RecordField::from_bytes(input, 14, 5)?,
            msa_center_icao_code:         RecordField::from_bytes(input, 19, 2)?,
            msa_center_section_code:      RecordField::from_bytes(input, 21, 1)?,
            msa_center_subsection_code:   RecordField::from_bytes(input, 22, 1)?,
            multiple_code:                RecordField::from_bytes(input, 23, 1)?,
            continuation_record_number:   RecordField::from_bytes(input, 39, 1)?,
            application_type:             RecordField::from_bytes(input, 40, 1)?,
            notes:                        RecordField::from_bytes(input, 41, 69)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.msa_center.value.is_none() {
            validation_result.extend_messages(
                "msa center reference",
                is_valid_reference(
                    &self.msa_center,
                    &self.msa_center_section_code,
                    &self.msa_center_subsection_code,
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
            self.msa_center.raw_bytes,
            self.multiple_code.raw_bytes,
        ])
    }
}
