use std::path::{Path, PathBuf};

pub fn find_files_with_ext_recursive<P: AsRef<Path>>(
    dir: P,
    required_ext: &str,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut buff: Vec<PathBuf> = Vec::with_capacity(32);

    let content = std::fs::read_dir(dir)?;
    for entry in content {
        let Ok(entry) = entry else {
            continue;
        };

        let Ok(entry_type) = entry.file_type() else {
            continue;
        };

        if entry_type.is_dir() {
            let child_buff = find_files_with_ext_recursive(&entry.path(), required_ext)?;
            buff.extend(child_buff);

            continue;
        }

        let path = entry.path();
        if !path.to_string_lossy().ends_with(required_ext) {
            continue;
        }

        buff.push(path);
    }

    Ok(buff)
}
