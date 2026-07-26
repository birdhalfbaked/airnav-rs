use crate::rev18_faa::entities::navaid::Navaid;
use crate::rev18_faa::records::ARINCRecord;
use crate::types::entities::Arinc424Entity;
use crate::types::records::{GroupKey, RecordParseError};

#[derive(Debug)]
pub enum Entity {
    Navaid(Navaid),
}

impl Entity {
    pub fn from_primary(record: ARINCRecord<'_>) -> Result<Self, RecordParseError> {
        match record {
            ARINCRecord::VHFNavaidPrimary(_)
            | ARINCRecord::TerminalNDBNavaidPrimary(_)
            | ARINCRecord::NDBNavaidPrimary(_) => Ok(Self::Navaid(Navaid::new(record))),
            _ => Err(RecordParseError::new(
                "invalid record type".to_string(),
                None,
            )),
        }
    }
    pub fn merge_record(&mut self, record: ARINCRecord<'_>) -> Result<(), RecordParseError> {
        match self {
            Self::Navaid(entity) => Ok(entity.merge_record(record)),
        }
    }

    pub fn group_key(&self) -> GroupKey {
        match self {
            Self::Navaid(entity) => entity.group_key(),
        }
    }
}
