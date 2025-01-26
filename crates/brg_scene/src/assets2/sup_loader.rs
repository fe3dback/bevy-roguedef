use std::path::{Path, PathBuf};

use anyhow::Result;
use bevy::asset::LoadState;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{info, warn, Asset, AssetServer, Res, ResMut};

use super::assets_mgas::AssetMGA;
use super::dto_status::DtoLoadingStatus;
use super::res_loading_state::ResLoadingState;
use super::res_storage::{ELoadingStage, ResAssetsStorage};

#[derive(SystemParam)]
pub struct SupAssetLoader<'w> {
    server:  Res<'w, AssetServer>,
    storage: ResMut<'w, ResAssetsStorage>,
    state:   ResMut<'w, ResLoadingState>,
}

impl<'w> SupAssetLoader<'w> {
    pub fn load_all(&mut self) -> Result<()> {
        // collect assets info
        info!("[ASSET] start assets loading");
        self.load_mgas()?;

        // start loading
        self.state.status.stage = ELoadingStage::Loading;
        Ok(())
    }

    pub fn update_loading_status(&mut self) -> DtoLoadingStatus {
        if self.state.status.stage != ELoadingStage::Loading {
            return self.state.status.clone();
        }

        let handles = self.state.loading_handles.clone();
        self.state.loading_handles.clear();

        for h in handles {
            let Some(status) = self.server.get_load_state(&h) else {
                self.state.status.cnt_failed += 1;
                warn!("[ASSET] cant load asset {:?}: not found", h);
                continue;
            };

            match status {
                LoadState::Loaded => {
                    self.state.status.cnt_loaded += 1;
                    if Some(h) == self.state.status.last_info_handle {
                        self.state.status.last_info_handle = None;
                    }

                    continue;
                }
                LoadState::Failed(err) => {
                    self.state.status.cnt_failed += 1;

                    if self.state.status.last_info_error.is_none() {
                        self.state.status.last_info_error = Some(err.to_string());
                    }

                    warn!("[ASSET] cant load asset {:?}: {:?}", h, err);
                    continue;
                }
                LoadState::Loading | LoadState::NotLoaded => {
                    if self.state.status.last_info_handle.is_none() {
                        self.state.status.last_info_title = match self.server.get_path(h.id()) {
                            Some(v) => v.to_string(),
                            None => String::from(format!("{:?}", h)),
                        };
                        self.state.status.last_info_handle = Some(h.clone_weak());
                    }
                }
            }

            // return back h for next tick check
            self.state.loading_handles.push(h);
        }

        if self.state.status.cnt_total == self.state.status.cnt_loaded {
            self.state.status.stage = ELoadingStage::Ready;
        }

        self.state.status.clone()
    }

    fn load_mgas(&mut self) -> Result<()> {
        let paths = Self::find_paths("assets", "mga.ron")?;
        for path in paths {
            self.load::<AssetMGA>(path);
        }

        Ok(())
    }

    fn load<T: Asset>(&mut self, path: PathBuf) {
        let mut path = path;
        if path.starts_with("assets/") {
            path = path.strip_prefix("assets/").unwrap().into();
        }

        info!("[ASSET] - loading '{}'..", path.display());
        let h = self.server.load::<T>(path).untyped();
        self.state.loading_handles.push(h);
        self.state.status.cnt_total += 1;
    }

    fn find_paths<P: AsRef<Path>>(dir: P, required_ext: &str) -> Result<Vec<PathBuf>> {
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
                let child_buff = Self::find_paths(&entry.path(), required_ext)?;
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
}
