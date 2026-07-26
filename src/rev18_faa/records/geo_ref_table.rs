use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct GeographicalReferenceTableRecords;
impl GeographicalReferenceTableRecords {
    const CONTINUATION_COLUMN: usize = 39;
    const CONTINUATION_APPLICATION_COLUMN: usize = 40;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::GeographicalReferenceTablePrimary(
                GeographicalReferenceTablePrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    Ok(ARINCRecord::GeographicalReferenceTableContinuation(
                        GeographicalReferenceTableContinuationRecord::parse(input)?,
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

/// 4.1.26.1 Geographical Reference Table Primary Record
#[derive(Debug)]
pub struct GeographicalReferenceTablePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub geographical_reference_table_identifier:
        RecordField<'a, GeographicalReferenceTableIdentifier>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub geographical_entity: RecordField<'a, GeographicalEntity>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub preferred_route_1_identifier: RecordField<'a, PreferredRouteIdentifier>,
    pub preferred_route_1_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub preferred_route_2_identifier: RecordField<'a, PreferredRouteIdentifier>,
    pub preferred_route_2_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub preferred_route_3_identifier: RecordField<'a, PreferredRouteIdentifier>,
    pub preferred_route_3_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub preferred_route_4_identifier: RecordField<'a, PreferredRouteIdentifier>,
    pub preferred_route_4_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub preferred_route_5_identifier: RecordField<'a, PreferredRouteIdentifier>,
    pub preferred_route_5_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub preferred_route_6_identifier: RecordField<'a, PreferredRouteIdentifier>,
    pub preferred_route_6_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for GeographicalReferenceTablePrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "GeographicalReferenceTablePrimaryRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(GeographicalReferenceTablePrimaryRecord {
            record_type:                               RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                        RecordField::from_bytes(input, 2, 3)?,
            section:                                   RecordField::from_bytes(input, 5, 1)?,
            subsection:                                RecordField::from_bytes(input, 6, 1)?,
            geographical_reference_table_identifier:   RecordField::from_bytes(input, 7, 2)?,
            sequence_number:                           RecordField::from_bytes(input, 9, 1)?,
            geographical_entity:                       RecordField::from_bytes(input, 10, 29)?,
            continuation_record_number:                RecordField::from_bytes(input, 39, 1)?,
            preferred_route_1_identifier:              RecordField::from_bytes(input, 41, 10)?,
            preferred_route_1_use_indicator:           RecordField::from_bytes(input, 51, 2)?,
            preferred_route_2_identifier:              RecordField::from_bytes(input, 53, 10)?,
            preferred_route_2_use_indicator:           RecordField::from_bytes(input, 63, 2)?,
            preferred_route_3_identifier:              RecordField::from_bytes(input, 65, 10)?,
            preferred_route_3_use_indicator:           RecordField::from_bytes(input, 75, 2)?,
            preferred_route_4_identifier:              RecordField::from_bytes(input, 77, 10)?,
            preferred_route_4_use_indicator:           RecordField::from_bytes(input, 87, 2)?,
            preferred_route_5_identifier:              RecordField::from_bytes(input, 89, 10)?,
            preferred_route_5_use_indicator:           RecordField::from_bytes(input, 99, 2)?,
            preferred_route_6_identifier:              RecordField::from_bytes(input,101, 10 )?,
            preferred_route_6_use_indicator:           RecordField::from_bytes(input, 111, 2)?,
            file_record_number:                        RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                                RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.geographical_reference_table_identifier.raw_bytes,
            self.geographical_entity.raw_bytes,
        ])
    }
}

/// 4.1.26.2 Geographical Reference Table Continuation Record
#[derive(Debug)]
pub struct GeographicalReferenceTableContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub geographical_reference_table_identifier:
        RecordField<'a, GeographicalReferenceTableIdentifier>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub geographical_entity: RecordField<'a, GeographicalEntity>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for GeographicalReferenceTableContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "GeographicalReferenceTableContinuationRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(GeographicalReferenceTableContinuationRecord {
            record_type:                               RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                        RecordField::from_bytes(input, 2, 3)?,
            section:                                   RecordField::from_bytes(input, 5, 1)?,
            subsection:                                RecordField::from_bytes(input, 6, 1)?,
            geographical_reference_table_identifier:   RecordField::from_bytes(input, 7, 2)?,
            sequence_number:                           RecordField::from_bytes(input, 9, 1)?,
            geographical_entity:                       RecordField::from_bytes(input, 10, 29)?,
            continuation_record_number:                RecordField::from_bytes(input, 39, 1)?,
            application_type:                          RecordField::from_bytes(input, 40, 1)?,
            file_record_number:                        RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                                RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.geographical_reference_table_identifier.raw_bytes,
            self.geographical_entity.raw_bytes,
        ])
    }
}
