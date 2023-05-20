use std::{
    num::NonZeroUsize,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::TrySendError,
        Arc,
    },
};

use self::{
    converter::{Converter, Reader, Writer},
    info_converter::InfoConverter,
    waiter::ConverterWaiter,
};

pub mod anvil2cc;
pub mod converter;
pub mod data;
pub mod entry_location;
pub mod info_converter;
pub mod waiter;

#[derive(Copy, Clone, Debug)]
pub struct ConverterCreateCtx {
    pub convert_queue_size: usize,
    pub write_queue_size: usize,
}

impl Default for ConverterCreateCtx {
    fn default() -> Self {
        Self {
            convert_queue_size: 1024,
            write_queue_size: 4096,
        }
    }
}

pub fn run_conversion<KEY, IN, OUT, READ, CONVERT, INFO, WRITE>(
    ctx: ConverterCreateCtx,
    mut reader: READ,
    converter: CONVERT,
    info_converter: INFO,
    mut writer: WRITE,
) -> ConverterWaiter
where
    IN: Send + 'static,
    OUT: Send + 'static + Clone,
    READ: Reader<KEY, IN> + 'static,
    CONVERT: Converter<IN, OUT> + 'static,
    INFO: InfoConverter + 'static,
    WRITE: Writer<OUT> + 'static,
{
    let tasks_sent = Arc::new(AtomicUsize::new(0));
    let convert_queue_fill = Arc::new(AtomicUsize::new(0));
    let write_queue_fill = Arc::new(AtomicUsize::new(0));

    let (convert_sender, convert_receiver) = multiqueue::mpmc_queue(ctx.convert_queue_size.try_into().unwrap());
    let (write_sender, write_receiver) = multiqueue::mpmc_queue(ctx.write_queue_size.try_into().unwrap());

    let tasks_fill = tasks_sent.clone();
    let convert_fill = convert_queue_fill.clone();
    let read_thread = std::thread::spawn(move || {
        println!("Read thread start");
        reader
            .load_all_chunks(|mut data| {
                tasks_fill.fetch_add(1, Ordering::Relaxed);
                loop {
                    match convert_sender.try_send(data) {
                        Ok(_) => {
                            convert_fill.fetch_add(1, Ordering::Relaxed);
                            break;
                        }
                        Err(err) => match err {
                            TrySendError::Full(returned_data) => {
                                data = returned_data;
                                std::thread::yield_now();
                            }
                            TrySendError::Disconnected(_) => {
                                break;
                            }
                        },
                    };
                }
            })
            .unwrap();
        println!("Read thread end");
    });

    let converter = Arc::new(converter);
    let mut convert_threads = Vec::new();

    for _ in 0..std::thread::available_parallelism().unwrap_or(NonZeroUsize::new(1).unwrap()).into() {
        let converter = converter.clone();
        let convert_receiver = convert_receiver.clone();
        let write_sender = write_sender.clone();
        let convert_fill = convert_queue_fill.clone();
        let write_fill = write_queue_fill.clone();
        convert_threads.push(std::thread::spawn(move || {
            println!("Convert thread start");
            while let Ok(data) = convert_receiver.recv() {
                convert_fill.fetch_sub(1, Ordering::Relaxed);
                let converted = converter.convert(data).unwrap();

                for mut data in converted {
                    while let Err(err) = write_sender.try_send(data) {
                        match err {
                            TrySendError::Full(returned_data) => {
                                data = returned_data;
                                std::thread::yield_now();
                            }
                            TrySendError::Disconnected(_) => {
                                break;
                            }
                        }
                    }
                    write_fill.fetch_add(1, Ordering::Relaxed);
                }
            }

            println!("Convert thread end");
        }));
    }
    convert_receiver.unsubscribe();
    write_sender.unsubscribe();

    let write_fill = write_queue_fill.clone();
    let write_thread = std::thread::spawn(move || {
        println!("Write thread start");

        while let Ok(data) = write_receiver.recv() {
            write_fill.fetch_sub(1, Ordering::Relaxed);
            writer.write(data).unwrap();
        }
        writer.flush().unwrap();
        println!("Write thread end");
    });

    let info_converter_thread = std::thread::spawn(move || info_converter.convert().unwrap());

    ConverterWaiter {
        read_thread,
        convert_threads,
        info_converter_thread,
        write_thread,
        tasks_sent,
        convert_queue_fill,
        convert_queue_size: ctx.convert_queue_size,
        write_queue_fill,
        write_queue_size: ctx.write_queue_size,
    }
}
