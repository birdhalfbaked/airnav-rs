use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
};

pub(super) struct GridMORARecords;
impl GridMORARecords {
    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        Ok(ARINCRecord::GridMORAPrimary(GridMORAPrimaryRecord::parse(
            input,
        )?))
    }
}

/// 4.1.19.1 Grid MORA Primary Record
#[derive(Debug)]
pub struct GridMORAPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub starting_latitude: RecordField<'a, MoraStartingLatitude>,
    pub starting_longitude: RecordField<'a, MoraStartingLongitude>,
    pub mora_1: RecordField<'a, GridMora>,
    pub mora_2: RecordField<'a, GridMora>,
    pub mora_3: RecordField<'a, GridMora>,
    pub mora_4: RecordField<'a, GridMora>,
    pub mora_5: RecordField<'a, GridMora>,
    pub mora_6: RecordField<'a, GridMora>,
    pub mora_7: RecordField<'a, GridMora>,
    pub mora_8: RecordField<'a, GridMora>,
    pub mora_9: RecordField<'a, GridMora>,
    pub mora_10: RecordField<'a, GridMora>,
    pub mora_11: RecordField<'a, GridMora>,
    pub mora_12: RecordField<'a, GridMora>,
    pub mora_13: RecordField<'a, GridMora>,
    pub mora_14: RecordField<'a, GridMora>,
    pub mora_15: RecordField<'a, GridMora>,
    pub mora_16: RecordField<'a, GridMora>,
    pub mora_17: RecordField<'a, GridMora>,
    pub mora_18: RecordField<'a, GridMora>,
    pub mora_19: RecordField<'a, GridMora>,
    pub mora_20: RecordField<'a, GridMora>,
    pub mora_21: RecordField<'a, GridMora>,
    pub mora_22: RecordField<'a, GridMora>,
    pub mora_23: RecordField<'a, GridMora>,
    pub mora_24: RecordField<'a, GridMora>,
    pub mora_25: RecordField<'a, GridMora>,
    pub mora_26: RecordField<'a, GridMora>,
    pub mora_27: RecordField<'a, GridMora>,
    pub mora_28: RecordField<'a, GridMora>,
    pub mora_29: RecordField<'a, GridMora>,
    pub mora_30: RecordField<'a, GridMora>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for GridMORAPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "GridMORAPrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:          RecordField::from_bytes(input, 1, 1)?,
            section:              RecordField::from_bytes(input, 5, 1)?,
            subsection:           RecordField::from_bytes(input, 6, 1)?,
            starting_latitude:    RecordField::from_bytes(input, 14, 3)?,
            starting_longitude:   RecordField::from_bytes(input, 17, 4)?,
            mora_1:               RecordField::from_bytes(input, 31, 3)?,
            mora_2:               RecordField::from_bytes(input, 34, 3)?,
            mora_3:               RecordField::from_bytes(input, 37, 3)?,
            mora_4:               RecordField::from_bytes(input, 40, 3)?,
            mora_5:               RecordField::from_bytes(input, 43, 3)?,
            mora_6:               RecordField::from_bytes(input, 46, 3)?,
            mora_7:               RecordField::from_bytes(input, 49, 3)?,
            mora_8:               RecordField::from_bytes(input, 52, 3)?,
            mora_9:               RecordField::from_bytes(input, 55, 3)?,
            mora_10:              RecordField::from_bytes(input, 58, 3)?,
            mora_11:              RecordField::from_bytes(input, 61, 3)?,
            mora_12:              RecordField::from_bytes(input, 64, 3)?,
            mora_13:              RecordField::from_bytes(input, 67, 3)?,
            mora_14:              RecordField::from_bytes(input, 70, 3)?,
            mora_15:              RecordField::from_bytes(input, 73, 3)?,
            mora_16:              RecordField::from_bytes(input, 76, 3)?,
            mora_17:              RecordField::from_bytes(input, 79, 3)?,
            mora_18:              RecordField::from_bytes(input, 82, 3)?,
            mora_19:              RecordField::from_bytes(input, 85, 3)?,
            mora_20:              RecordField::from_bytes(input, 88, 3)?,
            mora_21:              RecordField::from_bytes(input, 91, 3)?,
            mora_22:              RecordField::from_bytes(input, 94, 3)?,
            mora_23:              RecordField::from_bytes(input, 97, 3)?,
            mora_24:              RecordField::from_bytes(input, 100, 3)?,
            mora_25:              RecordField::from_bytes(input, 103, 3)?,
            mora_26:              RecordField::from_bytes(input, 106, 3)?,
            mora_27:              RecordField::from_bytes(input, 109, 3)?,
            mora_28:              RecordField::from_bytes(input, 112, 3)?,
            mora_29:              RecordField::from_bytes(input, 115, 3)?,
            mora_30:              RecordField::from_bytes(input, 118, 3)?,
            file_record_number:   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:           RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.starting_latitude.raw_bytes,
            self.starting_longitude.raw_bytes,
        ])
    }
}
