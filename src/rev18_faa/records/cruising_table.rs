use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
};
pub(super) struct CruisingTableRecords;
impl CruisingTableRecords {
    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        Ok(ARINCRecord::CruisingTablePrimary(
            CruisingTablePrimaryRecord::parse(input)?,
        ))
    }
}

/// 4.1.16.1 Cruising Table Primary Record
#[derive(Debug)]
pub struct CruisingTablePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub cruise_table_identifier: RecordField<'a, CruiseTableIdentifier>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub course_from: RecordField<'a, CruiseTableCourseFromTo>,
    pub course_to: RecordField<'a, CruiseTableCourseFromTo>,
    pub magnetic_true_indicator: RecordField<'a, MagneticTrueIndicator>,
    pub entry_1_level_from: RecordField<'a, CruiseLevelFromTo>,
    pub entry_1_vertical_separation: RecordField<'a, VerticalSeparation>,
    pub entry_1_level_to: RecordField<'a, CruiseLevelFromTo>,
    pub entry_2_level_from: RecordField<'a, CruiseLevelFromTo>,
    pub entry_2_vertical_separation: RecordField<'a, VerticalSeparation>,
    pub entry_2_level_to: RecordField<'a, CruiseLevelFromTo>,
    pub entry_3_level_from: RecordField<'a, CruiseLevelFromTo>,
    pub entry_3_vertical_separation: RecordField<'a, VerticalSeparation>,
    pub entry_3_level_to: RecordField<'a, CruiseLevelFromTo>,
    pub entry_4_level_from: RecordField<'a, CruiseLevelFromTo>,
    pub entry_4_vertical_separation: RecordField<'a, VerticalSeparation>,
    pub entry_4_level_to: RecordField<'a, CruiseLevelFromTo>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for CruisingTablePrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "CruisingTablePrimaryRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                   RecordField::from_bytes(input, 1, 1)?,
            section:                       RecordField::from_bytes(input, 5, 1)?,
            subsection:                    RecordField::from_bytes(input, 6, 1)?,
            cruise_table_identifier:       RecordField::from_bytes(input, 7, 2)?,
            sequence_number:               RecordField::from_bytes(input, 9, 1)?,
            course_from:                   RecordField::from_bytes(input, 29, 4)?,
            course_to:                     RecordField::from_bytes(input, 33, 4)?,
            magnetic_true_indicator:       RecordField::from_bytes(input, 37, 1)?,
            entry_1_level_from:            RecordField::from_bytes(input, 40, 5)?,
            entry_1_vertical_separation:   RecordField::from_bytes(input, 45, 5)?,
            entry_1_level_to:              RecordField::from_bytes(input, 50, 5)?,
            entry_2_level_from:            RecordField::from_bytes(input, 55, 5)?,
            entry_2_vertical_separation:   RecordField::from_bytes(input, 60, 5)?,
            entry_2_level_to:              RecordField::from_bytes(input, 65, 5)?,
            entry_3_level_from:            RecordField::from_bytes(input, 70, 5)?,
            entry_3_vertical_separation:   RecordField::from_bytes(input, 75, 5)?,
            entry_3_level_to:              RecordField::from_bytes(input, 80, 5)?,
            entry_4_level_from:            RecordField::from_bytes(input, 85, 5)?,
            entry_4_vertical_separation:   RecordField::from_bytes(input, 90, 5)?,
            entry_4_level_to:              RecordField::from_bytes(input, 95, 5)?,
            file_record_number:            RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                    RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.cruise_table_identifier.raw_bytes,
        ])
    }
}
