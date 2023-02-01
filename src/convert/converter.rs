use std::error::Error;

pub enum ReadError {}
pub trait Reader<K, IN> {
    fn load_all_chunks<F>(&mut self, f: F)
    where
        F: Fn(IN) -> ();
}

#[derive(Debug)]
pub enum WriteError {
    Custom(Box<dyn Error>),
}

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
