use crate::rev18_faa::definitions::*;
use crate::types::fields::TimeDistanceNumeric;

#[derive(Debug)]
pub struct Reference {
    pub identifier: Box<str>,
    pub icao_code: Box<str>,
    pub section: Option<CombinedSectionSubsection>,
}

#[derive(Debug)]
pub struct RecordMetadata {
    pub record_type: RecordType,
    pub record_number: u64,
    pub customer_area_code: Option<CustomerAreaCode>,
    pub section: CombinedSectionSubsection,
}

#[derive(Debug)]
pub enum Distance {
    Time(f64),
    Distance(f64),
}

impl From<TimeDistanceNumeric> for Distance {
    fn from(value: TimeDistanceNumeric) -> Self {
        match value {
            TimeDistanceNumeric::Time(time) => Distance::Time(time.into()),
            TimeDistanceNumeric::Distance(distance) => Distance::Distance(distance.into()),
        }
    }
}
