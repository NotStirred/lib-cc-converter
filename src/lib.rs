use std::path::Path;

use convert::{
    anvil2cc::{conv::Anvil2CCConverter, info::Anvil2CCLevelInfoConverter},
    run_conversion,
    waiter::ConverterWaiter,
    ConverterCreateCtx,
};
use io::{anvil::reader::create_anvil_region_reader, cubic::writer::CubicRegionWriter};

mod convert;
mod dimension;
mod io;
mod util;

pub struct Anvil2CCConfig {
    pub fix_missing_tile_entities: bool,
    pub ctx: ConverterCreateCtx,
}

pub fn anvil2cc(src_path: &Path, dst_path: &Path, config: Anvil2CCConfig) -> Result<ConverterWaiter, std::io::Error> {
    let reader = create_anvil_region_reader(src_path);
    let converter = Anvil2CCConverter::new(config.fix_missing_tile_entities);
    let writer = CubicRegionWriter::new(dst_path, 64)?;

    let info_converter = Anvil2CCLevelInfoConverter::new(src_path, dst_path, |base, path| {
        if let Some(file_name) = path.file_name() {
            if file_name == "level.dat" || file_name == "cubicChunksData.dat" {
                return true;
            }
            for dimension in crate::dimension::DIMENSIONS {
                if base.join(dimension.directory).join("region") == path {
                    return true;
                }
            }
        }
        false
    });

    let waiter = run_conversion(config.ctx, reader, converter, info_converter, writer);
    Ok(waiter)
}

#[cfg(test)]
mod tests {
    use crate::anvil2cc;

    use crate::util::test_utils;

    #[test]
    fn anvil2cc_test() {
        let src_path = test_utils::test_resources_path().join("anvil2cc/in");
        let dst_path = test_utils::test_resources_path().join("anvil2cc/out");

        let waiter = anvil2cc(
            &src_path,
            &dst_path,
            crate::Anvil2CCConfig {
                fix_missing_tile_entities: true,
                ctx: Default::default(),
            },
        )
        .unwrap();

        waiter.join_all().unwrap();
    }
}
