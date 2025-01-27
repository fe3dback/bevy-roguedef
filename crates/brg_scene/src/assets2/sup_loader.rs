use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use bevy::asset::LoadState;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{info, warn, Asset, AssetServer, EventWriter, NextState, Res, ResMut};
use brg_core::prelude::{Id, IdCategory};

use super::assets_mgas::AssetMGA;
use super::assets_mgas_doodads::AssetMGADoodad;
use super::dto_status::{DtoLoadingStatus, ELoadingStage};
use super::evt_on_load::EvtOnLoad;
use super::res_loading_state::ResLoadingState;
use super::sup_assets::SupAssets;
use crate::prelude::GameState;

#[derive(SystemParam)]
pub struct SupAssetLoader<'w> {
    assets:         SupAssets<'w>,
    server:         Res<'w, AssetServer>,
    state:          ResMut<'w, ResLoadingState>,
    on_load_writer: EventWriter<'w, EvtOnLoad>,
    next_state:     ResMut<'w, NextState<GameState>>,
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
        match self.state.status.stage {
            ELoadingStage::CalculateAssetsToLoad => {}
            ELoadingStage::Loading => match self.load_until_done() {
                true => self.state.status.stage = ELoadingStage::Validation,
                false => {}
            },
            ELoadingStage::Validation => {
                info!("[ASSET] validating assets..");
                match self.validate() {
                    Ok(_) => self.state.status.stage = ELoadingStage::Ready,
                    Err(e) => {
                        self.state.status.last_info_error = Some(format!("{:#}", e));
                        self.state.status.stage = ELoadingStage::NotValid;
                    }
                }
            }
            ELoadingStage::NotValid => {}
            ELoadingStage::Ready => {
                info!("[ASSET] ALL ASSETS LOADED SUCCESSFULLY!");
                self.next_state
                    .set(GameState::Loaded { game_paused: false });
                self.state.status.stage = ELoadingStage::Completed;
            }
            ELoadingStage::Completed => {}
        }

        self.state.status.clone()
    }

    fn load_until_done(&mut self) -> bool {
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
                    self.on_load_writer.send(EvtOnLoad { handle: h.clone() });

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

        // check state
        let all_loaded = self.state.status.cnt_total == self.state.status.cnt_loaded;
        let all_ready = self.state.status.cnt_total == self.state.status.cnt_ready;

        if all_loaded && all_ready {
            return true;
        }

        false
    }

    fn validate(&self) -> Result<()> {
        for asset in self.assets.all::<AssetMGADoodad>() {
            self.validate_have_asset_by_id(*asset.id, *asset.category)?;
            self.validate_id_must_have_category(
                *asset.id,
                *asset.category,
                IdCategory::DoodadsCategory,
            )?;
        }

        Ok(())
    }

    fn validate_id_must_have_category(
        &self,
        asset_id: Id,
        dep_id: Id,
        expected: IdCategory,
    ) -> Result<()> {
        if dep_id.category() == expected {
            return Ok(());
        }

        bail!(
            "asset id '{}' have field '{}' that must have category '{}' ({}), but actual is - '{}' ({})",
            asset_id,
            dep_id,
            expected,
            expected.to_char().unwrap_or('_'),
            dep_id.category(),
            dep_id.category().to_char().unwrap_or('_'),
        )
    }

    fn validate_have_asset_by_id(&self, asset_id: Id, dep_id: Id) -> Result<()> {
        if !self.assets.has(dep_id) {
            bail!(
                "asset id '{}' have dependency on '{}', but this asset is not known",
                asset_id,
                dep_id
            )
        }

        Ok(())
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
