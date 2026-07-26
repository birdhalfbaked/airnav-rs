//! # ARINC 424 Parser
//! This module contains the parser for the ARINC 424 data. It is a wrapper around the various revisions
//! we support in the package for processing data. This allows for easy revision switching without
//! making us dynamically allocate a bunch more heap objects to handle dynamicity within a trait-based approach,
//! so while it might be a bit more verbose, it is more efficient.

#[cfg(feature = "rev18")]
use crate::rev18::records::record::ARINCRecord as Rev18ArincRecord;
#[cfg(feature = "rev18_faa")]
use crate::rev18_faa::entities::entity::Entity as Rev18FAAEntity;
#[cfg(feature = "rev18_faa")]
use crate::rev18_faa::records::record::ARINCRecord as Rev18FAAArincRecord;

#[cfg(feature = "rev23")]
use crate::rev23::records::record::ARINCRecord as Rev23ArincRecord;

use crate::types::records::{Arinc424Record, GroupKey, RecordError, RecordValidationError};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arinc424Version {
    #[cfg(feature = "rev18")]
    Rev18,
    #[cfg(feature = "rev18_faa")]
    Rev18FAA,
    #[cfg(feature = "rev23")]
    Rev23,
}

#[derive(Debug)]
pub enum Arinc424VersionedRecord<'a> {
    #[cfg(feature = "rev18")]
    Rev18(Rev18ArincRecord<'a>),
    #[cfg(feature = "rev18_faa")]
    Rev18FAA(Rev18FAAArincRecord<'a>),
    #[cfg(feature = "rev23")]
    Rev23(Rev23ArincRecord<'a>),
}

impl<'a> Arinc424VersionedRecord<'a> {
    pub fn validate(&self) -> Result<(), RecordValidationError> {
        match self {
            #[cfg(feature = "rev18")]
            Self::Rev18(_) => Ok(()),
            #[cfg(feature = "rev18_faa")]
            Self::Rev18FAA(record) => record.validate(),
            #[cfg(feature = "rev23")]
            Self::Rev23(_) => Ok(()),
        }
    }
    pub fn group_key(&self) -> GroupKey {
        match self {
            #[cfg(feature = "rev18")]
            Self::Rev18(_) => GroupKey::default(),
            #[cfg(feature = "rev18_faa")]
            Self::Rev18FAA(record) => record.group_key(),
            #[cfg(feature = "rev23")]
            Self::Rev23(_) => GroupKey::default(),
        }
    }
}

#[derive(Debug)]
pub enum Arinc424VersionedEntity {
    #[cfg(feature = "rev18_faa")]
    Rev18FAA(Rev18FAAEntity),
}

impl Arinc424VersionedEntity {
    pub fn validate(&self) -> Result<(), RecordValidationError> {
        match self {
            #[cfg(feature = "rev18_faa")]
            Self::Rev18FAA(_) => todo!(),
        }
    }
    pub fn merge_record(
        &mut self,
        record: Arinc424VersionedRecord<'_>,
    ) -> Result<(), RecordValidationError> {
        match (self, record) {
            #[cfg(feature = "rev18_faa")]
            (Self::Rev18FAA(entity), Arinc424VersionedRecord::Rev18FAA(record)) => {
                entity.merge_record(record);
                Ok(())
            } // _ => Err(RecordValidationError::new("invalid record type")),
        }
    }
    pub fn group_key(&self) -> GroupKey {
        match self {
            #[cfg(feature = "rev18_faa")]
            Self::Rev18FAA(entity) => entity.group_key(),
        }
    }
}

#[derive(Debug)]
pub enum Arinc424Parser {
    #[cfg(feature = "rev18")]
    Rev18,
    #[cfg(feature = "rev18_faa")]
    Rev18FAA,
    #[cfg(feature = "rev23")]
    Rev23,
}

impl Arinc424Parser {
    pub fn new(version: Arinc424Version) -> Self {
        match version {
            #[cfg(feature = "rev18")]
            Arinc424Version::Rev18 => Self::Rev18,
            #[cfg(feature = "rev18_faa")]
            Arinc424Version::Rev18FAA => Self::Rev18FAA,
            #[cfg(feature = "rev23")]
            Arinc424Version::Rev23 => Self::Rev23,
        }
    }

    pub fn parse<'a>(&self, input: &'a [u8]) -> Result<Arinc424VersionedRecord<'a>, RecordError> {
        let record = match self {
            #[cfg(feature = "rev18")]
            Self::Rev18 => Ok(Arinc424VersionedRecord::Rev18(Rev18ArincRecord::parse(
                input,
            )?)),
            #[cfg(feature = "rev18_faa")]
            Self::Rev18FAA => Ok(Arinc424VersionedRecord::Rev18FAA(
                Rev18FAAArincRecord::parse(input)?,
            )),
            #[cfg(feature = "rev23")]
            Self::Rev23 => Ok(Arinc424VersionedRecord::Rev23(Rev23ArincRecord::parse(
                input,
            )?)),
        };
        match record {
            Ok(record) => match record.validate() {
                Ok(()) => Ok(record),
                Err(e) => Err(RecordError::from(
                    e.with_raw_line(String::from_utf8_lossy(input).to_string()),
                )),
            },
            Err(e) => Err(e),
        }
    }
}

pub struct Arinc424Reader<R: BufRead> {
    parser: Arinc424Parser,
    lines: std::io::Lines<R>,
    lookahead: Option<String>, // Lines yields String
}
impl<R: BufRead> Arinc424Reader<R> {
    pub fn new(version: Arinc424Version, reader: R) -> Self {
        Self {
            parser: Arinc424Parser::new(version),
            lines: reader.lines(),
            lookahead: None,
        }
    }
    fn _next_line(&mut self) -> Option<Result<String, std::io::Error>> {
        if let Some(line) = self.lookahead.take() {
            return Some(Ok(line));
        }
        match self.lines.next()? {
            Ok(s) => Some(Ok(s)),
            Err(e) => Some(Err(e)),
        }
    }
}
impl Arinc424Reader<BufReader<File>> {
    pub fn from_path(version: Arinc424Version, path: impl AsRef<Path>) -> std::io::Result<Self> {
        Ok(Self::new(version, BufReader::new(File::open(path)?)))
    }
}

impl<R: BufRead> Iterator for Arinc424Reader<R> {
    type Item = Result<Arinc424VersionedEntity, std::io::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let first_line = match self._next_line()? {
            Ok(l) => l,
            Err(e) => return Some(Err(e)),
        };
        let first = match self.parser.parse(first_line.as_bytes()) {
            Ok(r) => r,
            Err(_) => {
                return Some(Err(std::io::Error::new(
                    std::io::ErrorKind::Interrupted,
                    "invalid record",
                )));
            }
        };
        let mut entity = match first {
            Arinc424VersionedRecord::Rev18FAA(record) => {
                match Rev18FAAEntity::from_primary(record) {
                    Ok(entity) => Arinc424VersionedEntity::Rev18FAA(entity),
                    Err(_) => {
                        return Some(Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "invalid record",
                        )));
                    }
                }
            }
            _ => {
                return Some(Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "invalid record",
                )));
            }
        };

        // pull continuations until next primary ---
        loop {
            let next_line = match self._next_line() {
                None => break, // EOF → yield entity we have
                Some(Ok(l)) => l,
                Some(Err(e)) => {
                    return Some(Err(e));
                }
            };
            let rec = match self.parser.parse(next_line.as_bytes()) {
                Ok(r) => r,
                Err(e) => {
                    return Some(Err(std::io::Error::new(
                        std::io::ErrorKind::Unsupported,
                        "invalid record",
                    )));
                }
            };
            if rec.group_key() == entity.group_key() {
                // optional: same entity_reference as entity
                if let Err(e) = entity.merge_record(rec) {
                    return Some(Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid record",
                    )));
                }
                // next_line dropped; values already moved into entity
            } else {
                // Next primary (or new entity family) — belongs to the *next* next()
                self.lookahead = Some(next_line);
                break;
            }
        }
        Some(Ok(entity))
    }
}
