use super::converter::ConversionError;

pub trait InfoConverter: Send {
    fn convert(&self) -> Result<(), ConversionError>;
}
