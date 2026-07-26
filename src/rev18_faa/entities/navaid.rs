use crate::rev18_faa::definitions::*;
use crate::rev18_faa::records::*;
use crate::types::entities::Arinc424Entity;
use crate::types::records::GroupKey;

#[derive(Debug, PartialEq)]
pub enum NavaidKind {
    VOR,
    DME,
    VORDME,
    NDB,
    TerminalNDB,
}

impl std::fmt::Display for NavaidKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::VOR => "VOR Navaid",
                Self::DME => "DME Navaid",
                Self::VORDME => "VORDME Navaid",
                Self::NDB => "NDB Navaid",
                Self::TerminalNDB => "Terminal NDB Navaid",
            }
        )
    }
}

#[derive(Debug)]
pub enum NavaidClass {
    NDBClass(NDBNavaidClass),
    VHFClass(VHFNavaidClass),
}

#[derive(Debug)]
pub struct NavaidReference {
    pub identifier: Box<str>,
    pub icao_code: Box<str>,
    pub airport_identifier: Option<Box<str>>,
    pub airport_icao_code: Option<Box<str>>,
}

#[derive(Debug)]
pub struct VORData {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug)]
pub struct DMEData {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: i64,
    pub ils_dme_bias: Option<f64>,
}

#[derive(Debug)]
pub struct NDBData {
    pub latitude: f64,
    pub longitude: f64,
    pub magnetic_variation: f64,
}

#[derive(Debug)]
pub struct NavaidSimulationData {
    pub facility_characteristics: FacilityCharacteristics,
    pub facility_elevation: i64,
}

#[derive(Debug)]
pub struct NavaidFlightPlanningData {
    pub fir_identifier: Box<str>,
    pub uir_identifier: Box<str>,
    pub start_end_indicator: Box<str>,
    pub start_end_date: Box<str>,
}

#[derive(Debug)]
pub struct NavaidLimitationData {
    pub component_affected_indicator: ComponentAffectedIndicator,
    pub sector_from_sector_to: [char; 2],
    pub distance_description: DistanceDescription,
    pub distance_limit: [u64; 2],
    pub altitude_description: CrossingAltitudeDescription,
    pub altitude_limit: [u64; 2],
}

#[derive(Debug)]
pub struct Navaid {
    pub kind: NavaidKind,
    pub reference: NavaidReference,
    pub vor_data: Option<VORData>,
    pub dme_data: Option<DMEData>,
    pub ndb_data: Option<NDBData>,
    pub notes: Vec<Notes>,
    pub limitations: Vec<NavaidLimitationData>,
    pub cycle_date: u64,
    pub simulation_data: Option<NavaidSimulationData>,
    pub flight_planning_data: Option<NavaidFlightPlanningData>,
}

impl Navaid {
    pub fn new(record: ARINCRecord<'_>) -> Self {
        match record {
            ARINCRecord::NDBNavaidPrimary(r) => Self {
                kind: NavaidKind::NDB,
                reference: NavaidReference {
                    identifier: r.ndb_identifier.value.unwrap().into(),
                    icao_code: r.ndb_icao_code.value.unwrap().into(),
                    airport_identifier: r.airport_icao_identifier.value.map(|id| id.into()),
                    airport_icao_code: r.airport_icao_code.value.map(|id| id.into()),
                },
                vor_data: None,
                dme_data: None,
                ndb_data: Some(NDBData {
                    latitude: r.ndb_latitude.value.unwrap().into(),
                    longitude: r.ndb_longitude.value.unwrap().into(),
                    magnetic_variation: r.magnetic_variation.value.unwrap().into(),
                }),
                notes: vec![],
                limitations: vec![],
                cycle_date: r.cycle_date.value.unwrap().into(),
                simulation_data: None,
                flight_planning_data: None,
            },
            ARINCRecord::TerminalNDBNavaidPrimary(r) => Self {
                kind: NavaidKind::TerminalNDB,
                reference: NavaidReference {
                    identifier: r.ndb_identifier.value.unwrap().into(),
                    icao_code: r.ndb_icao_code.value.unwrap().into(),
                    airport_identifier: r.airport_icao_identifier.value.map(|id| id.into()),
                    airport_icao_code: r.airport_icao_code.value.map(|id| id.into()),
                },
                vor_data: None,
                dme_data: None,
                ndb_data: Some(NDBData {
                    latitude: r.ndb_latitude.value.unwrap().into(),
                    longitude: r.ndb_longitude.value.unwrap().into(),
                    magnetic_variation: r.magnetic_variation.value.unwrap().into(),
                }),
                notes: vec![],
                limitations: vec![],
                cycle_date: r.cycle_date.value.unwrap().into(),
                simulation_data: None,
                flight_planning_data: None,
            },
            ARINCRecord::VHFNavaidPrimary(r) => {
                match (r.vor_latitude.value, r.dme_latitude.value) {
                    (Some(vor_latitude), Some(dme_latitude)) => Self {
                        kind: NavaidKind::VORDME,
                        reference: NavaidReference {
                            identifier: r.vor_identifier.value.unwrap().into(),
                            icao_code: r.vor_icao_code.value.unwrap().into(),
                            airport_identifier: r.airport_icao_identifier.value.map(|id| id.into()),
                            airport_icao_code: r.airport_icao_code.value.map(|id| id.into()),
                        },
                        vor_data: Some(VORData {
                            latitude: vor_latitude.into(),
                            longitude: r.vor_longitude.value.unwrap().into(),
                        }),
                        dme_data: Some(DMEData {
                            latitude: dme_latitude.into(),
                            longitude: r.dme_longitude.value.unwrap().into(),
                            elevation: r.dme_elevation.value.unwrap().into(),
                            ils_dme_bias: r.ils_dme_bias.value.map_or(None, |v| Some(v.into())),
                        }),
                        ndb_data: None,
                        notes: vec![],
                        limitations: vec![],
                        cycle_date: r.cycle_date.value.unwrap().into(),
                        simulation_data: None,
                        flight_planning_data: None,
                    },
                    (Some(vor_latitude), None) => Self {
                        kind: NavaidKind::VOR,
                        reference: NavaidReference {
                            identifier: r.vor_identifier.value.unwrap().into(),
                            icao_code: r.vor_icao_code.value.unwrap().into(),
                            airport_identifier: r.airport_icao_identifier.value.map(|id| id.into()),
                            airport_icao_code: r.airport_icao_code.value.map(|id| id.into()),
                        },
                        vor_data: Some(VORData {
                            latitude: vor_latitude.into(),
                            longitude: r.vor_longitude.value.unwrap().into(),
                        }),
                        dme_data: None,
                        ndb_data: None,
                        notes: vec![],
                        limitations: vec![],
                        cycle_date: r.cycle_date.value.unwrap().into(),
                        simulation_data: None,
                        flight_planning_data: None,
                    },
                    (None, Some(dme_latitude)) => Self {
                        kind: NavaidKind::DME,
                        reference: NavaidReference {
                            identifier: r.dme_identifier.value.unwrap().into(),
                            icao_code: r.vor_icao_code.value.unwrap().into(),
                            airport_identifier: r.airport_icao_identifier.value.map(|id| id.into()),
                            airport_icao_code: r.airport_icao_code.value.map(|id| id.into()),
                        },
                        vor_data: None,
                        dme_data: Some(DMEData {
                            latitude: dme_latitude.into(),
                            longitude: r.dme_longitude.value.unwrap().into(),
                            elevation: r.dme_elevation.value.unwrap().into(),
                            ils_dme_bias: r.ils_dme_bias.value.map_or(None, |v| Some(v.into())),
                        }),
                        ndb_data: None,
                        notes: vec![],
                        limitations: vec![],
                        cycle_date: r.cycle_date.value.unwrap().into(),
                        simulation_data: None,
                        flight_planning_data: None,
                    },
                    _ => {
                        panic!("Invalid VORDME Navaid record");
                    }
                }
            }
            _ => {
                panic!("Invalid Navaid record");
            }
        }
    }
    fn merge_continuation_record(&mut self, record: ARINCRecord<'_>) -> () {
        match record {
            ARINCRecord::VHFNavaidContinuation(record) => {
                self.notes.push(record.notes.value.unwrap().into());
            }
            ARINCRecord::NDBNavaidContinuation(record) => {
                self.notes.push(record.notes.value.unwrap().into());
            }
            _ => {}
        };
    }

    fn merge_simulation_record(&mut self, record: ARINCRecord<'_>) -> () {
        match record {
            ARINCRecord::VHFNavaidSimulationContinuation(r) => {
                self.simulation_data = Some(NavaidSimulationData {
                    facility_characteristics: r.facility_characteristics.value.unwrap().into(),
                    facility_elevation: r.facility_elevation.value.unwrap().into(),
                });
            }
            ARINCRecord::NDBNavaidSimulationContinuation(r) => {
                self.simulation_data = Some(NavaidSimulationData {
                    facility_characteristics: r.facility_characteristics.value.unwrap().into(),
                    facility_elevation: r.facility_elevation.value.unwrap().into(),
                });
            }
            ARINCRecord::TerminalNDBNavaidSimulationContinuation(r) => {
                self.simulation_data = Some(NavaidSimulationData {
                    facility_characteristics: r.facility_characteristics.value.unwrap().into(),
                    facility_elevation: r.facility_elevation.value.unwrap().into(),
                });
            }
            _ => {}
        };
    }

    fn _convert_limitation(
        affected_indicator: ComponentAffectedIndicator,
        sector_from_sector_to: SectorFromTo,
        distance_description: DistanceDescription,
        distance_limit: NavaidDistanceLimitation,
        altitude_description: CrossingAltitudeDescription,
        altitude_limit: NavaidAltitudeLimitation,
    ) -> NavaidLimitationData {
        NavaidLimitationData {
            component_affected_indicator: affected_indicator,
            sector_from_sector_to: sector_from_sector_to.into(),
            distance_description: distance_description,
            distance_limit: distance_limit.into(),
            altitude_description: altitude_description,
            altitude_limit: altitude_limit.into(),
        }
    }

    fn merge_limitation_continuation_record(&mut self, record: ARINCRecord<'_>) -> () {
        match record {
            ARINCRecord::VHFNavaidLimitationContinuation(r) => {
                let component_affected_indicator =
                    if let Some(v) = r.component_affected_indicator.value {
                        v
                    } else {
                        panic!("Invalid component affected indicator");
                    };
                // Note: this assumes that if one field is Some, then all are Some.
                if let Some(ad) = r.limitation_1_altitude_description.value {
                    self.limitations.push(Self::_convert_limitation(
                        component_affected_indicator,
                        r.limitation_1_sector_from_sector_to.value.unwrap(),
                        r.limitation_1_distance_description.value.unwrap(),
                        r.limitation_1_distance_limit.value.unwrap(),
                        ad,
                        r.limitation_1_altitude_limit.value.unwrap(),
                    ));
                }
                if let Some(ad) = r.limitation_2_altitude_description.value {
                    self.limitations.push(Self::_convert_limitation(
                        component_affected_indicator,
                        r.limitation_2_sector_from_sector_to.value.unwrap(),
                        r.limitation_2_distance_description.value.unwrap(),
                        r.limitation_2_distance_limit.value.unwrap(),
                        ad,
                        r.limitation_2_altitude_limit.value.unwrap(),
                    ));
                }
                if let Some(ad) = r.limitation_3_altitude_description.value {
                    self.limitations.push(Self::_convert_limitation(
                        component_affected_indicator,
                        r.limitation_3_sector_from_sector_to.value.unwrap(),
                        r.limitation_3_distance_description.value.unwrap(),
                        r.limitation_3_distance_limit.value.unwrap(),
                        ad,
                        r.limitation_3_altitude_limit.value.unwrap(),
                    ));
                }
                if let Some(ad) = r.limitation_4_altitude_description.value {
                    self.limitations.push(Self::_convert_limitation(
                        component_affected_indicator,
                        r.limitation_4_sector_from_sector_to.value.unwrap(),
                        r.limitation_4_distance_description.value.unwrap(),
                        r.limitation_4_distance_limit.value.unwrap(),
                        ad,
                        r.limitation_4_altitude_limit.value.unwrap(),
                    ));
                }
                if let Some(ad) = r.limitation_5_altitude_description.value {
                    self.limitations.push(Self::_convert_limitation(
                        component_affected_indicator,
                        r.limitation_5_sector_from_sector_to.value.unwrap(),
                        r.limitation_5_distance_description.value.unwrap(),
                        r.limitation_5_distance_limit.value.unwrap(),
                        ad,
                        r.limitation_5_altitude_limit.value.unwrap(),
                    ));
                }
            }
            _ => {}
        };
    }
}

impl Arinc424Entity for Navaid {
    type Record<'a> = ARINCRecord<'a>;
    fn merge_record(&mut self, record: Self::Record<'_>) -> () {
        match record {
            ARINCRecord::VHFNavaidContinuation(_)
            | ARINCRecord::NDBNavaidContinuation(_)
            | ARINCRecord::TerminalNDBNavaidContinuation(_) => {
                self.merge_continuation_record(record)
            }

            ARINCRecord::VHFNavaidSimulationContinuation(_)
            | ARINCRecord::NDBNavaidSimulationContinuation(_)
            | ARINCRecord::TerminalNDBNavaidSimulationContinuation(_) => {
                self.merge_simulation_record(record)
            }

            ARINCRecord::VHFNavaidLimitationContinuation(_) => {
                self.merge_limitation_continuation_record(record)
            }
            _ => {}
        };
    }

    fn group_key(&self) -> GroupKey {
        GroupKey::from_byte_slices(&[
            self.reference.identifier.as_bytes(),
            self.reference.icao_code.as_bytes(),
            self.reference
                .airport_identifier
                .as_ref()
                .map(|id| id.as_bytes())
                .unwrap_or(&[]),
            self.reference
                .airport_icao_code
                .as_ref()
                .map(|id| id.as_bytes())
                .unwrap_or(&[]),
        ])
    }
}
