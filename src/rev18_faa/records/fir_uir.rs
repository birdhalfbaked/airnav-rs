use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct FIRUIRRecords;
impl FIRUIRRecords {
    const CONTINUATION_COLUMN: usize = 20;
    const CONTINUATION_APPLICATION_COLUMN: usize = 21;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::FIRUIRPrimary(FIRUIRPrimaryRecord::parse(
                input,
            )?))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => Ok(
                    ARINCRecord::FIRUIRContinuation(FIRUIRContinuationRecord::parse(input)?),
                ),
                _ => Err(RecordParseError::new(
                    "Invalid continuation record application type".to_string(),
                    Some(String::from_utf8_lossy(input).into_owned()),
                )),
            }
        }
    }
}

/// 4.1.17.1 FIR/UIR Primary Record
#[derive(Debug)]
pub struct FIRUIRPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub fir_uir_identifier: RecordField<'a, FirUirIdentifier>,
    pub fir_uir_address: RecordField<'a, FirUirAddress>,
    pub fir_uir_indicator: RecordField<'a, FirUirIndicator>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub adjacent_fir_identifier: RecordField<'a, FirUirIdentifier>,
    pub adjacent_uir_identifier: RecordField<'a, FirUirIdentifier>,
    pub reporting_units_speed: RecordField<'a, FirUirReportingUnitsSpeed>,
    pub reporting_units_altitude: RecordField<'a, FirUirReportingUnitsAltitude>,
    pub entry_report: RecordField<'a, FirUirEntryReport>,
    pub boundary_via: RecordField<'a, BoundaryVia>,
    pub fir_uir_latitude: RecordField<'a, Latitude>,
    pub fir_uir_longitude: RecordField<'a, Longitude>,
    pub arc_origin_latitude: RecordField<'a, Latitude>,
    pub arc_origin_longitude: RecordField<'a, Longitude>,
    pub arc_distance: RecordField<'a, ArcDistance>,
    pub arc_bearing: RecordField<'a, ArcBearing>,
    pub fir_upper_limit: RecordField<'a, LowerUpperLimit>,
    pub uir_lower_limit: RecordField<'a, LowerUpperLimit>,
    pub uir_upper_limit: RecordField<'a, LowerUpperLimit>,
    pub cruise_table_identifier: RecordField<'a, CruiseTableIdentifier>,
    pub fir_uir_name: RecordField<'a, FirUirName>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for FIRUIRPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "FIRUIRPrimaryRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            fir_uir_identifier:           RecordField::from_bytes(input, 7, 4)?,
            fir_uir_address:              RecordField::from_bytes(input, 11, 4)?,
            fir_uir_indicator:            RecordField::from_bytes(input, 15, 1)?,
            sequence_number:              RecordField::from_bytes(input, 16, 4)?,
            continuation_record_number:   RecordField::from_bytes(input, 20, 1)?,
            adjacent_fir_identifier:      RecordField::from_bytes(input, 21, 4)?,
            adjacent_uir_identifier:      RecordField::from_bytes(input, 25, 4)?,
            reporting_units_speed:        RecordField::from_bytes(input, 29, 1)?,
            reporting_units_altitude:     RecordField::from_bytes(input, 30, 1)?,
            entry_report:                 RecordField::from_bytes(input, 31, 1)?,
            boundary_via:                 RecordField::from_bytes(input, 33, 2)?,
            fir_uir_latitude:             RecordField::from_bytes(input, 35, 9)?,
            fir_uir_longitude:            RecordField::from_bytes(input, 44, 10)?,
            arc_origin_latitude:          RecordField::from_bytes(input, 54, 9)?,
            arc_origin_longitude:         RecordField::from_bytes(input, 63, 10)?,
            arc_distance:                 RecordField::from_bytes(input, 73, 4)?,
            arc_bearing:                  RecordField::from_bytes(input, 77, 4)?,
            fir_upper_limit:              RecordField::from_bytes(input, 81, 5)?,
            uir_lower_limit:              RecordField::from_bytes(input, 86, 5)?,
            uir_upper_limit:              RecordField::from_bytes(input, 91, 5)?,
            cruise_table_identifier:      RecordField::from_bytes(input, 96, 2)?,
            fir_uir_name:                 RecordField::from_bytes(input, 99, 25)?,
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
            self.fir_uir_identifier.raw_bytes,
        ])
    }
}

/// 4.1.17.2 FIR/UIR Continuation Record
#[derive(Debug)]
pub struct FIRUIRContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub fir_uir_identifier: RecordField<'a, FirUirIdentifier>,
    pub fir_uir_address: RecordField<'a, FirUirAddress>,
    pub fir_uir_indicator: RecordField<'a, FirUirIndicator>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for FIRUIRContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "FIRUIRContinuationRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            fir_uir_identifier:           RecordField::from_bytes(input, 7, 4)?,
            fir_uir_address:              RecordField::from_bytes(input, 11, 4)?,
            fir_uir_indicator:            RecordField::from_bytes(input, 15, 1)?,
            sequence_number:              RecordField::from_bytes(input, 16, 4)?,
            continuation_record_number:   RecordField::from_bytes(input, 20, 1)?,
            application_type:             RecordField::from_bytes(input, 21, 1)?,
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
            self.fir_uir_identifier.raw_bytes,
        ])
    }
}
