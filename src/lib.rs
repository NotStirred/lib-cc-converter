pub mod convert;
pub mod io;
pub mod util;

#[cfg(test)]
mod tests {
    use std::sync::mpsc::TrySendError;
    use std::sync::Arc;

    use crate::convert::anvil2cc::Anvil2CCConverter;

    use crate::convert::converter::{Converter, Reader, Writer};
    use crate::io::anvil_region_reader::AnvilRegionReader;
    use crate::io::cubic_chunks_writer::CubicChunksWriter;

    use crate::util::test_utils;

    #[test]
    fn simple_region_test() {
        let src_path = test_utils::test_resources_path().join("simple_region_test");
        let dst_path = test_utils::test_resources_path().join("simple_region_test/out");

        let mut reader = AnvilRegionReader::new(&src_path);
        let converter = Arc::new(Anvil2CCConverter::new(true));
        let mut writer = CubicChunksWriter::new(&dst_path, 64).unwrap();
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
                                std::thread::yield_now()
                            }
                            _ => break,
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
                loop {
                    if let Ok(data) = convert_receiver.recv() {
                        let converted = converter.convert(data).unwrap();

                        for mut data in converted.into_iter() {
                            loop {
                                match write_sender.try_send(data) {
                                    Ok(_) => break,
                                    Err(err) => match err {
                                        TrySendError::Full(returned_data) => {
                                            data = returned_data;
                                            std::thread::yield_now()
                                        }
                                        _ => break,
                                    },
                                };
                            }
                        }
                    } else {
                        break;
                    }
                }
                println!("Convert thread end");
            }));
        }
        convert_receiver.unsubscribe();
        write_sender.unsubscribe();

        let write_thread = std::thread::spawn(move || {
            println!("Write thread start");
            loop {
                if let Ok(data) = write_receiver.recv() {
                    writer.write(data).unwrap();
                } else {
                    break;
                }
            }
            writer.flush().unwrap();
            println!("Write thread end");
        });

        read_thread.join().unwrap();
        for thread in convert_threads {
            thread.join().unwrap()
        }
        write_thread.join().unwrap();
    }
}
