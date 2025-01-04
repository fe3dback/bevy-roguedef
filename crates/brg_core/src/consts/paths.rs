use std::path::PathBuf;

use super::global::APP_NAME;

pub fn path_config_dir() -> PathBuf {
    dirs::config_dir().unwrap().join(APP_NAME)
}
