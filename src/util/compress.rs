use byteorder::{BigEndian, WriteBytesExt};
use flate2::bufread::GzDecoder;
use quartz_nbt::io::{Flavor, NbtIoError};
use quartz_nbt::NbtCompound;
use std::io::BufReader;

pub fn read_compressed(data: &[u8]) -> Result<NbtCompound, NbtIoError> {
    let format = data[0];
    let data = &data[1..];
    if data.is_empty() {
        return Err(NbtIoError::Custom(
            format!("Format: {}", format).into_boxed_str(),
        ));
    }

    Ok(match format {
        1 => quartz_nbt::io::read_nbt(&mut GzDecoder::new(data), Flavor::Uncompressed),
        2 => quartz_nbt::io::read_nbt(&mut BufReader::new(data), Flavor::ZlibCompressed),
        _ => {
            panic!("Unrecognised format {}", format)
        }
    }?
    .0)
}

pub fn read_compressed_cc(reader: BufReader<&[u8]>) -> Result<NbtCompound, NbtIoError> {
    Ok(quartz_nbt::io::read_nbt(&mut GzDecoder::new(reader), Flavor::Uncompressed)?.0)
}

pub fn write_compressed(tag: &NbtCompound, prefix_format: bool) -> Result<Vec<u8>, NbtIoError> {
    let mut data = Vec::new();

    if prefix_format {
        data.write_i32::<BigEndian>(1)?;
    }
    quartz_nbt::io::write_nbt(&mut data, Some(""), tag, Flavor::GzCompressed)?;
    Ok(data)
}
