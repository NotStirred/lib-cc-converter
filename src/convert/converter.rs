use std::error::Error;

use quartz_nbt::{io::NbtIoError, NbtReprError};

use crate::{io::write_region::RegionWriteError, util::errors::error_from};

#[derive(Debug)]
pub enum ReadError {
    StdIo(std::io::Error),
}
error_from!(ReadError, std::io::Error, ReadError::StdIo);

pub trait Reader<K, IN>: Send {
    fn load_all_chunks<F>(&mut self, f: F) -> Result<(), ReadError>
    where
        F: Fn(IN);
}

#[derive(Debug)]
pub enum WriteError {
    StdIo(std::io::Error),
    RegionWrite(RegionWriteError),
    Custom(Box<dyn Error>),
}
error_from!(WriteError, std::io::Error, WriteError::StdIo);

pub trait Writer<OUT>: Send {
    fn write(&mut self, out_data: OUT) -> Result<(), WriteError>;
    fn flush(&mut self) -> Result<(), WriteError>;
}

#[derive(Debug)]
pub enum ConversionError {
    StdIo(std::io::Error),
    NbtIo(NbtIoError),
    NbtRepr(NbtReprError),
    Custom(Box<dyn Error>),
}

error_from!(ConversionError, std::io::Error, Self::StdIo);
error_from!(ConversionError, NbtIoError, Self::NbtIo);
error_from!(ConversionError, NbtReprError, Self::NbtRepr);

pub trait Converter<IN, OUT>: Send + Sync {
    fn convert(&self, in_data: IN) -> Result<Vec<OUT>, ConversionError>;
}
