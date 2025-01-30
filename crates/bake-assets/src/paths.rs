use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};

pub struct Paths {}

impl Paths {
    pub fn assets_src(&self) -> Result<PathBuf> {
        Ok(env::current_dir()?.join("assets_src"))
    }

    pub fn assets_dst(&self) -> Result<PathBuf> {
        Ok(env::current_dir()?.join("assets"))
    }

    pub fn dst_path_from(&self, path_src: PathBuf) -> Result<PathBuf> {
        Ok(PathBuf::from(
            path_src.to_str().unwrap().replace(
                self.assets_src()
                    .context("getting src path")?
                    .to_str()
                    .expect("path src contains invalid UTF-8"),
                self.assets_dst()
                    .context("getting dst path")?
                    .to_str()
                    .expect("path dst contains invalid UTF-8"),
            ),
        ))
    }
}
