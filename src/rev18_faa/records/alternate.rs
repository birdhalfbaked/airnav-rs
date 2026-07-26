use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
};
pub(super) struct AlternateRecords;
impl AlternateRecords {
    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        Ok(ARINCRecord::AlternatePrimary(
            AlternatePrimaryRecord::parse(input)?,
        ))
    }
}

/// 4.1.30.1 Alternate Record Primary
#[derive(Debug)]
pub struct AlternatePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub relation_airport_or_fix: RecordField<'a, FromToAirportHeliportFix>,
    pub relation_icao_code: RecordField<'a, IcaoCode>,
    pub relation_section: RecordField<'a, Section>,
    pub relation_subsection: RecordField<'a, GenericSubsection>,
    pub relation_record_type: RecordField<'a, AlternateRecordType>,
    pub primary_alternate_distance: RecordField<'a, DistanceToAlternate>,
    pub primary_alternate_type: RecordField<'a, AlternateType>,
    pub primary_alternate_identifier: RecordField<'a, PrimaryAndAdditionalAlternateIdentifier>,
    pub additional_alternate_1_distance: RecordField<'a, DistanceToAlternate>,
    pub additional_alternate_1_type: RecordField<'a, AlternateType>,
    pub additional_alternate_1_identifier: RecordField<'a, PrimaryAndAdditionalAlternateIdentifier>,
    pub additional_alternate_2_distance: RecordField<'a, DistanceToAlternate>,
    pub additional_alternate_2_type: RecordField<'a, AlternateType>,
    pub additional_alternate_2_identifier: RecordField<'a, PrimaryAndAdditionalAlternateIdentifier>,
    pub additional_alternate_3_distance: RecordField<'a, DistanceToAlternate>,
    pub additional_alternate_3_type: RecordField<'a, AlternateType>,
    pub additional_alternate_3_identifier: RecordField<'a, PrimaryAndAdditionalAlternateIdentifier>,
    pub additional_alternate_4_distance: RecordField<'a, DistanceToAlternate>,
    pub additional_alternate_4_type: RecordField<'a, AlternateType>,
    pub additional_alternate_4_identifier: RecordField<'a, PrimaryAndAdditionalAlternateIdentifier>,
    pub additional_alternate_5_distance: RecordField<'a, DistanceToAlternate>,
    pub additional_alternate_5_type: RecordField<'a, AlternateType>,
    pub additional_alternate_5_identifier: RecordField<'a, PrimaryAndAdditionalAlternateIdentifier>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for AlternatePrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "AlternatePrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                         RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                  RecordField::from_bytes(input, 2, 3)?,
            section:                             RecordField::from_bytes(input, 5, 1)?,
            subsection:                          RecordField::from_bytes(input, 6, 1)?,
            relation_airport_or_fix:             RecordField::from_bytes(input, 7, 5)?,
            relation_icao_code:                  RecordField::from_bytes(input, 12, 2)?,
            relation_section:                    RecordField::from_bytes(input, 14, 1)?,
            relation_subsection:                 RecordField::from_bytes(input, 15, 1)?,
            relation_record_type:                RecordField::from_bytes(input, 16, 2)?,
            primary_alternate_distance:          RecordField::from_bytes(input, 20, 3)?,
            primary_alternate_type:              RecordField::from_bytes(input, 23, 1)?,
            primary_alternate_identifier:        RecordField::from_bytes(input, 24, 10)?,
            additional_alternate_1_distance:     RecordField::from_bytes(input, 36, 3)?,
            additional_alternate_1_type:         RecordField::from_bytes(input, 39, 1)?,
            additional_alternate_1_identifier:   RecordField::from_bytes(input, 40, 10)?,
            additional_alternate_2_distance:     RecordField::from_bytes(input, 52, 3)?,
            additional_alternate_2_type:         RecordField::from_bytes(input, 55, 1)?,
            additional_alternate_2_identifier:   RecordField::from_bytes(input, 56, 10)?,
            additional_alternate_3_distance:     RecordField::from_bytes(input, 68, 3)?,
            additional_alternate_3_type:         RecordField::from_bytes(input, 71, 1)?,
            additional_alternate_3_identifier:   RecordField::from_bytes(input, 72, 10)?,
            additional_alternate_4_distance:     RecordField::from_bytes(input, 84, 3)?,
            additional_alternate_4_type:         RecordField::from_bytes(input, 87, 1)?,
            additional_alternate_4_identifier:   RecordField::from_bytes(input, 88, 10)?,
            additional_alternate_5_distance:     RecordField::from_bytes(input, 100, 3)?,
            additional_alternate_5_type:         RecordField::from_bytes(input, 103, 1)?,
            additional_alternate_5_identifier:   RecordField::from_bytes(input, 104, 10)?,
            file_record_number:                  RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                          RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.relation_airport_or_fix.value.is_none() {
            validation_result.extend_messages(
                "relation airport or fix reference",
                is_valid_reference(
                    &self.relation_airport_or_fix,
                    &self.relation_section,
                    &self.relation_subsection,
                ),
            );
        }
        validation_result.as_result()
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.relation_airport_or_fix.raw_bytes,
        ])
    }
}
