use std::collections::HashMap;
use crate::binary_parser_types::*;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum IonParserError {
    Unimplemented,
    BadFormatLengthFound,
} 

impl From<ParsingError> for IonParserError {
    fn from(_: ParsingError) -> Self { 
        IonParserError::Unimplemented
    }
}

#[derive(PartialEq, Debug)]
pub enum IonValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Decimal((u64, i64)),
    Timestamp(DateTime<Utc>),
    String(String),
    Clob(Vec<u8>),
    Blob(Vec<u8>),
    List(Vec<IonValue>), 
    SExpr(Vec<IonValue>),
    Struct(HashMap<String, IonValue>),
}
