use std::path::{Path, PathBuf};

use quartz_nbt::{NbtCompound, NbtTag};

use crate::{
    convert::{converter::ConversionError, info_converter::InfoConverter},
    util::{compress::read_compressed, file::copy_everything_except},
};

pub struct Anvil2CCLevelInfoConverter<F>
where
    F: Send,
{
    src_dir: PathBuf,
    dst_dir: PathBuf,
    exclude: F,
}

impl<F: Fn(&Path, &Path) -> bool + Send> Anvil2CCLevelInfoConverter<F> {
    pub fn new(src_path: &Path, dst_path: &Path, exclude: F) -> Self {
        Self {
            src_dir: src_path.to_path_buf(),
            dst_dir: dst_path.to_path_buf(),
            exclude,
        }
    }
}

impl<F: Fn(&Path, &Path) -> bool + Send> InfoConverter for Anvil2CCLevelInfoConverter<F> {
    fn convert(&self) -> Result<(), ConversionError> {
        std::fs::create_dir_all(&self.dst_dir)?;

        let _ignored = copy_any_modify_level_dat(&self.src_dir.join("level.dat"), &self.dst_dir.join("level.dat"));

        copy_everything_except(&self.src_dir, &self.src_dir, &self.dst_dir, &self.exclude)?;

        Ok(())
    }
}

fn copy_any_modify_level_dat(src: &Path, dst: &Path) -> Result<(), ConversionError> {
    let data = std::fs::read(src)?;
    let mut tag = read_compressed(&data)?;

    let data_tag: &mut NbtCompound = tag.get_mut("Data")?;

    let gen_name: &mut NbtTag = data_tag.get_mut("generatorName")?;
    if let NbtTag::String(name) = gen_name {
        if name.eq_ignore_ascii_case("default") {
            *name = "VanillaCubic".to_string();
        }
    }
    data_tag.insert("isCubicWorld", NbtTag::Byte(1i8));

    let mut out_data = Vec::new();
    quartz_nbt::io::write_nbt(&mut out_data, None, &tag, quartz_nbt::io::Flavor::Uncompressed)?;
    std::fs::write(dst, out_data)?;

    Ok(())
}
