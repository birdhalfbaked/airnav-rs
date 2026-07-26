use crate::rev18_faa::definitions::*;

use crate::rev18_faa::records::record::ARINCRecord;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, GroupKey, RecordField, RecordParseError, RecordValidationError,
    is_primary_record,
};
pub(super) struct HeliportCommsRecords;
impl HeliportCommsRecords {
    const CONTINUATION_COLUMN: usize = 26;
    const CONTINUATION_APPLICATION_COLUMN: usize = 27;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::HeliportCommsPrimary(
                HeliportCommsPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN
                    ..Self::CONTINUATION_APPLICATION_COLUMN + 1],
            )? {
                Some(ContinuationRecordApplicationType::SectorNarrativeContinuation) => {
                    Ok(ARINCRecord::HeliportCommsSectorNarrativeContinuation(
                        HeliportCommsSectorNarrativeContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::FormattedTimeOfOperationsContinuation) => {
                    Ok(ARINCRecord::HeliportCommsTimeContinuation(
                        HeliportCommsTimeContinuationRecord::parse(input)?,
                    ))
                }
                _ => {
                    return Err(RecordParseError::new(
                        "Invalid continuation record application type".to_string(),
                        Some(String::from_utf8_lossy(input).into_owned()),
                    ));
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
    let unit_bytes: &[u8] = &input[24..25];
    let communications_frequency_bytes: &[u8] = &input[16..23];
    let communications_frequency =
        CommunicationsFrequency::parse(unit_bytes, communications_frequency_bytes)?;
    Ok(RecordField {
        raw_bytes: communications_frequency_bytes,
        start_column: 16,
        end_column: 23,
        value: communications_frequency,
    })
}

// 4.1.14.1 Airport Communications Primary Record
#[derive(Debug)]
pub struct HeliportCommsPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub communications_type: RecordField<'a, CommunicationsType>,
    pub communication_frequency: RecordField<'a, CommunicationsFrequency>,
    pub guard_transmit_indicator: RecordField<'a, GuardTransmitIndicator>,
    pub frequency_units: RecordField<'a, FrequencyUnits>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub service_indicator: RecordField<'a, AirportHeliportCommunicationsServiceIndicator>,
    pub radar_service: RecordField<'a, Radar>,
    pub modulation: RecordField<'a, Modulation>,
    pub signal_emission: RecordField<'a, SignalEmission>,
    pub latitude: RecordField<'a, Latitude>,
    pub longitude: RecordField<'a, Longitude>,
    pub magnetic_variation: RecordField<'a, MagneticVariation>,
    pub facility_elevation: RecordField<'a, FacilityElevation>,
    pub is_h24: RecordField<'a, H24Indicator>,
    pub sectorization: RecordField<'a, CommunicationsSectorization>,
    pub altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub communication_altitude_1: RecordField<'a, CommunicationsAltitude>,
    pub communication_altitude_2: RecordField<'a, CommunicationsAltitude>,
    pub sector_facility: RecordField<'a, SectorFacility>,
    pub sector_facility_icao_code: RecordField<'a, IcaoCode>,
    pub sector_facility_section: RecordField<'a, Section>,
    pub sector_facility_subsection: RecordField<'a, GenericSubsection>,
    pub distance_description: RecordField<'a, DistanceDescription>,
    pub communication_distance: RecordField<'a, CommunicationsDistance>,
    pub remote_facility: RecordField<'a, RemoteFacility>,
    pub remote_facility_icao_code: RecordField<'a, IcaoCode>,
    pub remote_facility_section: RecordField<'a, Section>,
    pub remote_facility_subsection: RecordField<'a, GenericSubsection>,
    pub callsign: RecordField<'a, Callsign>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for HeliportCommsPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportCommsPrimaryRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {

        let communications_frequency = parse_communications_frequency(input)?;

        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:          RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:           RecordField::from_bytes(input, 11, 2)?,
            subsection:                   RecordField::from_bytes(input, 13, 1)?,
            communications_type:          RecordField::from_bytes(input, 14, 3)?,
            communication_frequency:      communications_frequency,
            guard_transmit_indicator:     RecordField::from_bytes(input, 24, 1)?,
            frequency_units:              RecordField::from_bytes(input, 25, 1)?,
            continuation_record_number:   RecordField::from_bytes(input, 26, 1)?,
            service_indicator:            RecordField::from_bytes(input, 27, 3)?,
            radar_service:                RecordField::from_bytes(input, 30, 1)?,
            modulation:                   RecordField::from_bytes(input, 31, 1)?,
            signal_emission:              RecordField::from_bytes(input, 32, 1)?,
            latitude:                     RecordField::from_bytes(input, 33, 9)?,
            longitude:                    RecordField::from_bytes(input, 42, 10)?,
            magnetic_variation:           RecordField::from_bytes(input, 52, 5)?,
            facility_elevation:           RecordField::from_bytes(input, 57, 5)?,
            is_h24:                       RecordField::from_bytes(input, 62, 1)?,
            sectorization:                RecordField::from_bytes(input, 63, 6)?,
            altitude_description:         RecordField::from_bytes(input, 69, 1)?,
            communication_altitude_1:     RecordField::from_bytes(input, 70, 5)?,
            communication_altitude_2:     RecordField::from_bytes(input, 75, 5)?,
            sector_facility:              RecordField::from_bytes(input, 80, 4)?,
            sector_facility_icao_code:    RecordField::from_bytes(input, 84, 2)?,
            sector_facility_section:      RecordField::from_bytes(input, 86, 1)?,
            sector_facility_subsection:   RecordField::from_bytes(input, 87, 1)?,
            distance_description:         RecordField::from_bytes(input, 88, 1)?,
            communication_distance:       RecordField::from_bytes(input, 89, 2)?,
            remote_facility:              RecordField::from_bytes(input, 91, 4)?,
            remote_facility_icao_code:    RecordField::from_bytes(input, 95, 2)?,
            remote_facility_section:      RecordField::from_bytes(input, 97, 1)?,
            remote_facility_subsection:   RecordField::from_bytes(input, 98, 1)?,
            callsign:                     RecordField::from_bytes(input, 99, 25)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        let mut validation_result = RecordValidationError::new(Self::record_name());
        if !self.sector_facility.value.is_none() {
            validation_result.extend_messages(
                "sector facility reference",
                is_valid_reference(
                    &self.sector_facility,
                    &self.sector_facility_section,
                    &self.sector_facility_subsection,
                ),
            );
        }
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
            self.heliport_identifier.raw_bytes,
            self.communications_type.raw_bytes,
            self.communication_frequency.raw_bytes,
        ])
    }
}

// 4.1.14.2 Airport Communications Sector Narrative Continuation Record
#[derive(Debug)]
pub struct HeliportCommsSectorNarrativeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub communications_type: RecordField<'a, CommunicationsType>,
    pub communication_frequency: RecordField<'a, CommunicationsFrequency>,
    pub guard_transmit_indicator: RecordField<'a, GuardTransmitIndicator>,
    pub frequency_units: RecordField<'a, FrequencyUnits>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub sectorization_narrative: RecordField<'a, SectorizationNarrative>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for HeliportCommsSectorNarrativeContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportCommsSectorNarrativeContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        let communications_frequency = parse_communications_frequency(input)?;
        Ok(Self {
            record_type:                      RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:               RecordField::from_bytes(input, 2, 3)?,
            section:                          RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:              RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:               RecordField::from_bytes(input, 11, 2)?,
            subsection:                       RecordField::from_bytes(input, 13, 1)?,
            communications_type:              RecordField::from_bytes(input, 14, 3)?,
            communication_frequency:          communications_frequency,
            guard_transmit_indicator:         RecordField::from_bytes(input, 24, 1)?,
            frequency_units:                  RecordField::from_bytes(input, 25, 1)?,
            continuation_record_number:       RecordField::from_bytes(input, 26, 1)?,
            application_type:                 RecordField::from_bytes(input, 23, 1)?,
            sectorization_narrative:          RecordField::from_bytes(input, 24, 60)?,
            file_record_number:               RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                       RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.heliport_identifier.raw_bytes,
            self.communications_type.raw_bytes,
            self.communication_frequency.raw_bytes,
        ])
    }
}

// 4.1.14.3 Airport Communications Time Continuation Record
#[derive(Debug)]
pub struct HeliportCommsTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub communications_type: RecordField<'a, CommunicationsType>,
    pub communication_frequency: RecordField<'a, CommunicationsFrequency>,
    pub guard_transmit_indicator: RecordField<'a, GuardTransmitIndicator>,
    pub frequency_units: RecordField<'a, FrequencyUnits>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub time_code: RecordField<'a, StandardContinuationRecordTimeCode>,
    pub notam: RecordField<'a, NotamFlag>,
    pub time_indicator: RecordField<'a, TimeIndicator>,
    pub time_of_operation_1: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_2: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_3: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_4: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_5: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_6: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_7: RecordField<'a, TimeOfOperation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for HeliportCommsTimeContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportCommsTimeContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        let communications_frequency = parse_communications_frequency(input)?;
        Ok(Self {
            record_type:                            RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                     RecordField::from_bytes(input, 2, 3)?,
            section:                                RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                    RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                     RecordField::from_bytes(input, 11, 2)?,
            subsection:                             RecordField::from_bytes(input, 13, 1)?,
            communications_type:                    RecordField::from_bytes(input, 14, 3)?,
            communication_frequency:                communications_frequency,
            guard_transmit_indicator:               RecordField::from_bytes(input, 24, 1)?,
            frequency_units:                        RecordField::from_bytes(input, 25, 1)?,
            continuation_record_number:             RecordField::from_bytes(input, 26, 1)?,
            application_type:                       RecordField::from_bytes(input, 27, 1)?,
            time_code:                              RecordField::from_bytes(input, 28, 1)?,
            notam:                                  RecordField::from_bytes(input, 29, 1)?,
            time_indicator:                         RecordField::from_bytes(input, 30, 1)?,
            time_of_operation_1:                    RecordField::from_bytes(input, 31, 10)?,
            time_of_operation_2:                    RecordField::from_bytes(input, 41, 10)?,
            time_of_operation_3:                    RecordField::from_bytes(input, 51, 10)?,
            time_of_operation_4:                    RecordField::from_bytes(input, 61, 10)?,
            time_of_operation_5:                    RecordField::from_bytes(input, 71, 10)?,
            time_of_operation_6:                    RecordField::from_bytes(input, 81, 10)?,
            time_of_operation_7:                    RecordField::from_bytes(input, 91, 10)?,
            file_record_number:                     RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                             RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.section.raw_bytes,
            self.subsection.raw_bytes,
            self.heliport_identifier.raw_bytes,
            self.communications_type.raw_bytes,
            self.communication_frequency.raw_bytes,
        ])
    }
}
