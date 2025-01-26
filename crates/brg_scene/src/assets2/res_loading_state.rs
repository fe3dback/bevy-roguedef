use bevy::asset::UntypedHandle;
use bevy::prelude::Resource;

use super::dto_status::DtoLoadingStatus;

#[derive(Resource, Default)]
pub struct ResLoadingState {
    pub status:          DtoLoadingStatus,
    pub loading_handles: Vec<UntypedHandle>,
}
