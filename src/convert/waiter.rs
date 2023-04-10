use std::{
    any::Any,
    sync::{atomic::AtomicUsize, Arc},
    thread::JoinHandle,
};

#[derive(Debug)]
pub enum ErrorCause {
    Reader,
    Converter(usize),
    InfoConverter,
    Writer,
}

#[derive(Debug)]
pub struct ConverterError {
    pub cause: ErrorCause,
    pub error: Box<dyn Any + Send>,
}

pub struct ConverterWaiter {
    pub read_thread: JoinHandle<()>,
    pub convert_threads: Vec<JoinHandle<()>>,
    pub info_converter_thread: JoinHandle<()>,
    pub write_thread: JoinHandle<()>,

    pub tasks_sent: Arc<AtomicUsize>,
    pub convert_queue_fill: Arc<AtomicUsize>,
    pub write_queue_fill: Arc<AtomicUsize>,
}

impl ConverterWaiter {
    pub fn is_finished(&self) -> bool {
        if !self.read_thread.is_finished() {
            return false;
        }
        for thread in &self.convert_threads {
            if !thread.is_finished() {
                return false;
            }
        }
        if !self.info_converter_thread.is_finished() {
            return false;
        }
        if !self.write_thread.is_finished() {
            return false;
        }
        true
    }

    pub fn join_all(self) -> Result<(), Vec<ConverterError>> {
        let mut errors = Vec::new();
        if let Err(err) = self.read_thread.join() {
            errors.push(ConverterError {
                cause: ErrorCause::Reader,
                error: err,
            });
        }
        for (i, thread) in self.convert_threads.into_iter().enumerate() {
            if let Err(err) = thread.join() {
                errors.push(ConverterError {
                    cause: ErrorCause::Converter(i),
                    error: err,
                });
            }
        }
        if let Err(err) = self.info_converter_thread.join() {
            errors.push(ConverterError {
                cause: ErrorCause::InfoConverter,
                error: err,
            });
        }
        if let Err(err) = self.write_thread.join() {
            errors.push(ConverterError {
                cause: ErrorCause::Writer,
                error: err,
            });
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
}
