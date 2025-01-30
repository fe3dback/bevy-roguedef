mod paths;

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use brg_core::prelude::find_files_with_ext_recursive;
use serde::Deserialize;
use strum::Display;

use self::paths::Paths;

/// see: https://github.khronos.org/KTX-Software/ktxtools/ktx_create.html
#[derive(Display, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TextureFormat {
    R8_UNORM,
    R8_SRGB,
    R8G8_UNORM,
    R8G8_SRGB,
    R8G8B8_UNORM,
    R8G8B8_SRGB,
    R8G8B8A8_UNORM,
    R8G8B8A8_SRGB,
}

#[derive(PartialEq, Eq, Deserialize)]
pub enum TextureMipMaps {
    WithoutMipMaps,
    MipMaps,
}

#[derive(Deserialize)]
pub struct TextureSettings {
    format:   TextureFormat,
    mip_maps: TextureMipMaps,
}

fn main() -> Result<()> {
    let paths = Paths {};

    let textures = find_files_with_ext_recursive(paths.assets_src()?, "png")?;
    for path_src in textures {
        let mut path_dst = paths
            .dst_path_from(path_src.clone())
            .context("getting dst path from src")?;
        path_dst.set_extension("ktx2");

        bake_texture(path_src.clone(), path_dst.clone()).context(format!(
            "copy texture from={:?} to={:?}",
            path_src, path_dst
        ))?;
    }

    Ok(())
}

fn bake_texture<P: AsRef<Path>>(src: P, dst: P) -> Result<()> {
    let mut path_settings = PathBuf::from(src.as_ref());
    path_settings.set_extension("ron");
    let settings = ron::de::from_str::<TextureSettings>(
        &fs::read_to_string(path_settings).context("read settings file")?,
    )
    .context("decode settings file")?;

    // create dirs
    fs::create_dir_all(dst.as_ref().parent().expect("dst parent dir is empty"))
        .context("create dirs")?;

    // write file
    let mut cmd = &mut Command::new("ktx");

    cmd = cmd.arg("create");

    cmd = cmd.arg("--assign-oetf");
    cmd = cmd.arg("srgb");

    cmd = cmd.arg("--format");
    cmd = cmd.arg(settings.format.to_string());

    if settings.mip_maps == TextureMipMaps::MipMaps {
        cmd = cmd.arg("--generate-mipmap");
    }

    cmd = cmd.arg::<PathBuf>(src.as_ref().into());
    cmd = cmd.arg::<PathBuf>(dst.as_ref().into());

    match cmd.output() {
        Ok(out) => match out.status.code() {
            Some(code) => match code {
                0 => {
                    println!("- texture converted: {}", dst.as_ref().display());
                    Ok(())
                }
                _ => Err(anyhow!(
                    "'{:?}' exit with code {}: {:?}",
                    cmd,
                    code,
                    String::from_utf8(out.stderr)
                )),
            },
            None => Err(anyhow!("ktx create terminated?")),
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!(
                    "ktx tools not installed? (https://github.khronos.org/KTX-Software/ktxtools/ktx_create.html)"
                );
                Err(e.into())
            }
            _ => Err(e.into()),
        },
    }
}
