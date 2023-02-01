use std::error::Error;

use crate::util::errors::error_from;

#[derive(Debug)]
pub enum ReadError {
    StdIo(std::io::Error),
    Custom(Box<dyn Error>),
}
error_from!(ReadError, std::io::Error, ReadError::StdIo);

pub trait Reader<K, IN> {
    fn load_all_chunks<F>(&mut self, f: F) -> Result<(), ReadError>
    where
        F: Fn(IN);
}

#[derive(Debug)]
pub enum WriteError {
    StdIo(std::io::Error),
    Custom(Box<dyn Error>),
}
error_from!(WriteError, std::io::Error, WriteError::StdIo);

pub trait Writer<OUT> {
    fn write(&mut self, out_data: OUT) -> Result<(), WriteError>;
    fn flush(&mut self) -> Result<(), WriteError>;
}

#[derive(Debug)]
pub enum ConversionError {
    Custom(Box<dyn Error>),
}
pub trait Converter<IN, OUT> {
    fn convert(&self, in_data: IN) -> Result<Vec<OUT>, ConversionError>;
}
