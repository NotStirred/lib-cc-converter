use std::path::Path;

pub fn copy_everything_except(path: &Path, src: &Path, dst: &Path, exclude: &dyn Fn(&Path, &Path) -> bool) -> Result<(), std::io::Error>
where
{
    for file in std::fs::read_dir(path)? {
        let src_file = file?.path();
        if let Some(relative) = pathdiff::diff_paths(src, &src_file) {
            if !exclude(src, &src_file) {
                println!("copying {}", src_file.to_string_lossy());
                std::fs::copy(&src_file, dst.join(relative))?;

                if src_file.is_dir() {
                    copy_everything_except(&src_file, src, dst, exclude)?;
                }
            } else {
                println!("excluded {}", src_file.to_string_lossy());
            }
        }
    }

    Ok(())
}
