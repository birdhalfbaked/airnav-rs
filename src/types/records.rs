//! # ARINC 424 Common types in the scope of records when parsing
//! This module contains the common record parsing types for the ARINC 424 data. These are used in all versions of the parser
//!
//! The is_primary_record function is used to determine if a record is a primary record but is also
//! fairly misplaced imo. Will leave it here for now as it is not the highest of prios to refactor this.
//! Core prio is stability across revisions.

use crate::types::fields::{BLANK, FieldParseError, ParseableField};
use std::any::type_name;
use std::fmt::{Debug, Display};

pub enum RecordError {
    Parse(RecordParseError),
    Validation(RecordValidationError),
}

impl RecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "Record parse error: {}", e),
            Self::Validation(e) => write!(f, "Record validation error: {}", e),
        }
    }
}

impl From<RecordParseError> for RecordError {
    fn from(e: RecordParseError) -> Self {
        Self::Parse(e)
    }
}

impl From<RecordValidationError> for RecordError {
    fn from(e: RecordValidationError) -> Self {
        Self::Validation(e)
    }
}

impl Display for RecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for RecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

#[derive(Debug)]
pub struct RecordParseError {
    pub message: String,
    pub raw_line: String,
}

impl RecordParseError {
    pub fn new(message: String, raw_line: Option<String>) -> Self {
        Self {
            message,
            raw_line: raw_line.unwrap_or_else(|| "".to_string()),
        }
    }
    pub fn into_result(self) -> Result<(), Self> {
        Err(self)
    }
}

impl From<FieldParseError> for RecordParseError {
    fn from(error: FieldParseError) -> Self {
        let message = if let (Some(column), Some(column_type)) = (error.column, error.column_type) {
            format!(
                "Record parse error: {} at column {} of type {}",
                error.message,
                column,
                column_type.split("::").last().unwrap_or(&column_type)
            )
        } else {
            format!("Record parse error: {}", error.message)
        };
        Self::new(message, None)
    }
}

impl Display for RecordParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Record parse error: {}", self.message)?;
        if !self.raw_line.is_empty() {
            write!(f, "\n\tRaw line: {}", self.raw_line)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RecordValidationMessage {
    pub message: &'static str,
    pub field_positions: Vec<(usize, usize)>,
}

impl RecordValidationMessage {
    pub fn new(message: &'static str, fields: Vec<(usize, usize)>) -> Self {
        Self {
            message,
            field_positions: fields,
        }
    }
}

#[derive(Debug)]
pub struct RecordValidationError {
    pub record: &'static str,
    pub messages: Vec<(&'static str, RecordValidationMessage)>,
    pub raw_line: String,
}

impl RecordValidationError {
    pub fn new(record: &'static str) -> Self {
        Self {
            record,
            messages: Vec::new(),
            raw_line: "".to_string(),
        }
    }
    pub fn extend_messages(
        &mut self,
        context: &'static str,
        messages: Vec<RecordValidationMessage>,
    ) {
        self.messages
            .extend(messages.into_iter().map(|message| (context, message)));
    }

    pub fn with_raw_line(&self, raw_line: String) -> Self {
        Self {
            record: self.record,
            messages: self.messages.clone(),
            raw_line,
        }
    }

    pub fn as_result(self) -> Result<(), Self> {
        if self.messages.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl Display for RecordValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for message in &self.messages {
            // we loop on all field positions then adjust the raw_line to wrap line characters between the column
            // positions in the fields we have in the message
            let mut adjusted_line = self.raw_line.clone();
            let mut message_field_positions = message.1.field_positions.clone();
            message_field_positions.sort_by_key(|(start_column, _)| *start_column);
            // make it so we descend so applying the wrapping to the line doesn't disturb the next field positions
            // we handle
            message_field_positions.reverse();
            for (start_column, end_column) in message_field_positions.iter() {
                adjusted_line = adjusted_line[..*start_column - 1].to_string()
                    + &"["
                    + &adjusted_line[*start_column - 1..*end_column - 1]
                    + &"]"
                    + &adjusted_line[*end_column - 1..];
            }
            writeln!(
                f,
                "{}:\n\tcontext: {}\n\traw: {}\n\terrors: {}",
                message.1.message, message.0, self.raw_line, adjusted_line
            )?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct RecordField<'a, T> {
    pub raw_bytes: &'a [u8],
    pub start_column: usize,
    pub end_column: usize,
    pub value: Option<T>,
}
impl<'a, T: ParseableField> RecordField<'a, T> {
    pub fn from_bytes(
        input: &'a [u8],
        column: usize,
        length: usize,
    ) -> Result<Self, FieldParseError> {
        // to make it 1:1 with the spec, let's use 1-indexed columns
        let value = T::from_bytes(&input[column - 1..column - 1 + length])
            .map_err(|e| e.with_context(type_name::<T>(), column))?;
        Ok(Self {
            raw_bytes: &input[column - 1..column - 1 + length],
            start_column: column,
            end_column: column + length,
            value,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupKey {
    pub key: Box<[u8]>,
}

impl GroupKey {
    pub fn from_byte_slices(bytes: &[&[u8]]) -> Self {
        let len: usize = bytes.iter().map(|p| p.len()).sum();
        let mut buf = Vec::with_capacity(len);
        for p in bytes {
            buf.extend_from_slice(p);
        }
        Self { key: buf.into() }
    }
}

pub trait Arinc424RecordSpec<'a>: Sized + 'a {
    fn record_name() -> &'static str;
    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError>;
    fn validate(&self) -> Result<(), RecordValidationError>;
    fn group_key(&self) -> GroupKey;
}

// Layout dispatch helpers
pub fn is_primary_record(input: &[u8], continuation_column: usize) -> bool {
    matches!(input[continuation_column - 1], b'0' | b'1' | BLANK)
}

pub trait Arinc424Record<'a>: Sized + 'a {
    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError>;
    fn validate(&self) -> Result<(), RecordValidationError>;
    fn group_key(&self) -> GroupKey;
}
