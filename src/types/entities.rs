//! Entities are the main data structures that represent the data in the ARINC 424 files.
//! They are used to store the data in a structured way and to perform operations on the data
//! which can be merged across both primary and continuation records.
use crate::types::records::GroupKey;

pub trait Arinc424Entity {
    type Record<'a>;
    fn merge_record(&mut self, record: Self::Record<'_>) -> ();
    fn group_key(&self) -> GroupKey;
}
