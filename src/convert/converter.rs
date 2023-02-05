use std::{
    error::Error,
    sync::{mpsc::TrySendError, Arc},
};

use crate::util::errors::error_from;

#[derive(Debug)]
pub enum ReadError {
    StdIo(std::io::Error),
    Custom(Box<dyn Error>),
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
    Custom(Box<dyn Error>),
}
error_from!(WriteError, std::io::Error, WriteError::StdIo);

pub trait Writer<OUT>: Send {
    fn write(&mut self, out_data: OUT) -> Result<(), WriteError>;
    fn flush(&mut self) -> Result<(), WriteError>;
}

#[derive(Debug)]
pub enum ConversionError {
    Custom(Box<dyn Error>),
}
pub trait Converter<IN, OUT>: Send + Sync {
    fn convert(&self, in_data: IN) -> Result<Vec<OUT>, ConversionError>;
}

pub fn run_conversion<KEY, IN, OUT, READ, CONVERT, WRITE>(mut reader: READ, converter: CONVERT, mut writer: WRITE)
where
    IN: Send + 'static,
    OUT: Send + 'static + Clone,
    READ: Reader<KEY, IN> + 'static,
    CONVERT: Converter<IN, OUT> + 'static,
    WRITE: Writer<OUT> + 'static,
{
    let converter = Arc::new(converter);

    let (convert_sender, convert_receiver) = multiqueue::mpmc_queue(64);
    let (write_sender, write_receiver) = multiqueue::mpmc_queue(64);

    let read_thread = std::thread::spawn(move || {
        println!("Read thread start");
        reader
            .load_all_chunks(|mut data| loop {
                match convert_sender.try_send(data) {
                    Ok(_) => break,
                    Err(err) => match err {
                        TrySendError::Full(returned_data) => {
                            data = returned_data;
                            std::thread::yield_now();
                        }
                        TrySendError::Disconnected(_) => break,
                    },
                };
            })
            .unwrap();
        println!("Read thread end");
    });

    let mut convert_threads = Vec::new();
    for _ in 0..16 {
        let converter = converter.clone();
        let convert_receiver = convert_receiver.clone();
        let write_sender = write_sender.clone();
        convert_threads.push(std::thread::spawn(move || {
            println!("Convert thread start");
            while let Ok(data) = convert_receiver.recv() {
                let converted = converter.convert(data).unwrap();

                for mut data in converted {
                    while let Err(err) = write_sender.try_send(data) {
                        match err {
                            TrySendError::Full(returned_data) => {
                                data = returned_data;
                                std::thread::yield_now();
                            }
                            TrySendError::Disconnected(_) => break,
                        }
                    }
                }
            }

            println!("Convert thread end");
        }));
    }
    convert_receiver.unsubscribe();
    write_sender.unsubscribe();

    let write_thread = std::thread::spawn(move || {
        println!("Write thread start");

        while let Ok(data) = write_receiver.recv() {
            writer.write(data).unwrap();
        }
        writer.flush().unwrap();
        println!("Write thread end");
    });

    read_thread.join().unwrap();
    for thread in convert_threads {
        thread.join().unwrap();
    }
    write_thread.join().unwrap();
}
