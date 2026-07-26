use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct EnrouteCommsRecords;
impl EnrouteCommsRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::EnrouteCommsPrimary(
                EnrouteCommsPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::CombinedControllingAgencyFormattedTimeOfOperationsContinuation) => {
                    Ok(ARINCRecord::EnrouteCommsCallsignAndTimeContinuation(
                        EnrouteCommsCallsignAndTimeContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::FormattedTimeOfOperationsContinuation) => {
                    Ok(ARINCRecord::EnrouteCommsTimeContinuation(
                        EnrouteCommsTimeContinuationRecord::parse(input)?,
                    ))
                }
                _ => {
                    return Err(RecordParseError::new("Invalid continuation record application type".to_string(), Some(String::from_utf8_lossy(input).into_owned())));
                }
            }
        }
    }
}

/// Parse the communications frequency from the input bytes
/// since we need to manually construct the frequency based on the frequency unit
fn parse_communications_frequency<'a>(
    input: &'a [u8],
) -> Result<RecordField<'a, CommunicationsFrequency>, RecordParseError> {
    let unit_bytes: &[u8] = &input[54..55];
    let communications_frequency_bytes: &[u8] = &input[46..53];
    let communications_frequency =
        CommunicationsFrequency::parse(unit_bytes, communications_frequency_bytes)?;
    Ok(RecordField {
        raw_bytes: communications_frequency_bytes,
        start_column: 46,
        end_column: 53,
        value: communications_frequency,
    })
}

// 4.1.23.1 Enroute Communications Primary Record
#[derive(Debug)]
pub struct EnrouteCommsPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub fir_rdo_identifier: RecordField<'a, FirRdoIdentifier>,
    pub fir_uir_address: RecordField<'a, FirUirAddress>,
    pub fir_uir_indicator: RecordField<'a, FirUirIndicator>,
    pub remote_name: RecordField<'a, RemoteSiteName>,
    pub communications_type: RecordField<'a, CommunicationsType>,
    pub communications_frequency: RecordField<'a, CommunicationsFrequency>,
    pub guard_transmit_indicator: RecordField<'a, GuardTransmitIndicator>,
    pub frequency_units: RecordField<'a, FrequencyUnits>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub service_indicator: RecordField<'a, EnrouteCommunicationsServiceIndicator>,
    pub radar_service: RecordField<'a, Radar>,
    pub modulation: RecordField<'a, Modulation>,
    pub signal_emission: RecordField<'a, SignalEmission>,
    pub latitude: RecordField<'a, Latitude>,
    pub longitude: RecordField<'a, Longitude>,
    pub magnetic_variation: RecordField<'a, MagneticVariation>,
    pub facility_elevation: RecordField<'a, FacilityElevation>,
    pub is_h24: RecordField<'a, H24Indicator>,
    pub altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub communication_altitude_1: RecordField<'a, CommunicationsAltitude>,
    pub communication_altitude_2: RecordField<'a, CommunicationsAltitude>,
    pub remote_facility: RecordField<'a, RemoteFacility>,
    pub remote_facility_icao_code: RecordField<'a, IcaoCode>,
    pub remote_facility_section: RecordField<'a, Section>,
    pub remote_facility_subsection: RecordField<'a, GenericSubsection>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteCommsPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteCommsPrimaryRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        let comms_frequency = parse_communications_frequency(input)?;
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            fir_rdo_identifier:           RecordField::from_bytes(input, 7, 4)?,
            fir_uir_address:              RecordField::from_bytes(input, 11, 4)?,
            fir_uir_indicator:            RecordField::from_bytes(input, 15, 1)?,
            remote_name:                  RecordField::from_bytes(input, 19, 4)?,
            communications_type :         RecordField::from_bytes(input, 44, 3)?,
            communications_frequency:     comms_frequency,
            guard_transmit_indicator:     RecordField::from_bytes(input, 54, 1)?,
            frequency_units:              RecordField::from_bytes(input, 55, 1)?,
            continuation_record_number:   RecordField::from_bytes(input, 56, 1)?,
            service_indicator:            RecordField::from_bytes(input, 57, 3)?,
            radar_service:                RecordField::from_bytes(input, 60, 1)?,
            modulation:                   RecordField::from_bytes(input, 61, 1)?,
            signal_emission:              RecordField::from_bytes(input, 62, 1)?,
            latitude:                     RecordField::from_bytes(input, 63, 9)?,
            longitude:                    RecordField::from_bytes(input, 72, 10)?,
            magnetic_variation:           RecordField::from_bytes(input, 82, 3)?,
            facility_elevation:           RecordField::from_bytes(input, 87, 5)?,
            is_h24:                       RecordField::from_bytes(input, 92, 1)?,
            altitude_description:         RecordField::from_bytes(input, 93, 1)?,
            communication_altitude_1:     RecordField::from_bytes(input, 94, 3)?,
            communication_altitude_2:     RecordField::from_bytes(input, 99, 3)?,
            remote_facility:              RecordField::from_bytes(input, 104, 4)?,
            remote_facility_icao_code:    RecordField::from_bytes(input, 108, 2)?,
            remote_facility_section:      RecordField::from_bytes(input, 110, 1)?,
            remote_facility_subsection:   RecordField::from_bytes(input, 111, 1)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.remote_facility.value.is_none() {
            validation_result.extend_messages(
                "remote facility reference",
                is_valid_reference(
                    &self.remote_facility,
                    &self.remote_facility_section,
                    &self.remote_facility_subsection,
                ),
            );
        }
        validation_result.extend_messages(
            "altitude description",
            is_valid_altitude_description(
                &self.section,
                &self.subsection,
                &self.altitude_description,
            ),
        );
        validation_result.as_result()
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.fir_rdo_identifier.raw_bytes,
            self.communications_type.raw_bytes,
        ])
    }
}

// 4.1.23.2 Enroute Communications Callsign And Time Continuation Record
#[derive(Debug)]
pub struct EnrouteCommsCallsignAndTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub fir_rdo_identifier: RecordField<'a, FirRdoIdentifier>,
    pub fir_uir_address: RecordField<'a, FirUirAddress>,
    pub fir_uir_indicator: RecordField<'a, FirUirIndicator>,
    pub remote_name: RecordField<'a, RemoteSiteName>,
    pub communications_type: RecordField<'a, CommunicationsType>,
    pub communications_frequency: RecordField<'a, CommunicationsFrequency>,
    pub guard_transmit_indicator: RecordField<'a, GuardTransmitIndicator>,
    pub frequency_units: RecordField<'a, FrequencyUnits>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub time_code: RecordField<'a, StandardContinuationRecordTimeCode>,
    pub notam: RecordField<'a, NotamFlag>,
    pub time_indicator: RecordField<'a, TimeIndicator>,
    pub time_of_operation: RecordField<'a, TimeOfOperation>,
    pub callsign: RecordField<'a, Callsign>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteCommsCallsignAndTimeContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteCommsCallsignAndTimeContinuationRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        let comms_frequency = parse_communications_frequency(input)?;
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            fir_rdo_identifier:           RecordField::from_bytes(input, 7, 4)?,
            fir_uir_address:              RecordField::from_bytes(input, 11, 4)?,
            fir_uir_indicator:            RecordField::from_bytes(input, 15, 1)?,
            remote_name:                  RecordField::from_bytes(input, 19, 4)?,
            communications_type:          RecordField::from_bytes(input, 44, 3)?,
            communications_frequency:     comms_frequency,
            guard_transmit_indicator:     RecordField::from_bytes(input, 54, 1)?,
            frequency_units:              RecordField::from_bytes(input, 55, 1)?,
            continuation_record_number:   RecordField::from_bytes(input, 56, 1)?,
            application_type:             RecordField::from_bytes(input, 57, 1)?,
            time_code:                    RecordField::from_bytes(input, 58, 1)?,
            notam:                        RecordField::from_bytes(input, 59, 1)?,
            time_indicator:               RecordField::from_bytes(input, 60, 1)?,
            time_of_operation:            RecordField::from_bytes(input, 61, 10)?,
            callsign:                     RecordField::from_bytes(input, 71, 30)?,
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
            self.fir_rdo_identifier.raw_bytes,
            self.communications_type.raw_bytes,
        ])
    }
}

// 4.1.23.3 Enroute Communications Time Continuation Record
#[derive(Debug)]
pub struct EnrouteCommsTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub fir_rdo_identifier: RecordField<'a, FirRdoIdentifier>,
    pub fir_uir_address: RecordField<'a, FirUirAddress>,
    pub fir_uir_indicator: RecordField<'a, FirUirIndicator>,
    pub remote_name: RecordField<'a, RemoteSiteName>,
    pub communications_type: RecordField<'a, CommunicationsType>,
    pub communications_frequency: RecordField<'a, CommunicationsFrequency>,
    pub guard_transmit_indicator: RecordField<'a, GuardTransmitIndicator>,
    pub frequency_units: RecordField<'a, FrequencyUnits>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub time_of_operation_1: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_2: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_3: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_4: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_5: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_6: RecordField<'a, TimeOfOperation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteCommsTimeContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteCommsTimeContinuationRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        let comms_frequency = parse_communications_frequency(input)?;
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            fir_rdo_identifier:           RecordField::from_bytes(input, 7, 4)?,
            fir_uir_address:              RecordField::from_bytes(input, 11, 4)?,
            fir_uir_indicator:            RecordField::from_bytes(input, 15, 1)?,
            remote_name:                  RecordField::from_bytes(input, 19, 4)?,
            communications_type:          RecordField::from_bytes(input, 44, 3)?,
            communications_frequency:     comms_frequency,
            guard_transmit_indicator:     RecordField::from_bytes(input, 54, 1)?,
            frequency_units:              RecordField::from_bytes(input, 55, 1)?,
            continuation_record_number:   RecordField::from_bytes(input, 56, 1)?,
            application_type:             RecordField::from_bytes(input, 57, 1)?,
            time_of_operation_1:          RecordField::from_bytes(input, 61, 10)?,
            time_of_operation_2:          RecordField::from_bytes(input, 71, 10)?,
            time_of_operation_3:          RecordField::from_bytes(input, 81, 10)?,
            time_of_operation_4:          RecordField::from_bytes(input, 91, 10)?,
            time_of_operation_5:          RecordField::from_bytes(input, 101, 10)?,
            time_of_operation_6:          RecordField::from_bytes(input, 111, 10)?,
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
            self.fir_rdo_identifier.raw_bytes,
            self.communications_type.raw_bytes,
        ])
    }
}
