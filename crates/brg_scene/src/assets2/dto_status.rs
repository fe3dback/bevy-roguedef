use bevy::prelude::UntypedHandle;

use super::res_storage::ELoadingStage;

#[derive(Default, Clone, Debug)]
pub struct DtoLoadingStatus {
    pub stage:            ELoadingStage,
    pub cnt_total:        u16,
    pub cnt_loaded:       u16,
    pub cnt_failed:       u16,
    pub last_info_title:  String,
    pub last_info_error:  Option<String>,
    pub last_info_handle: Option<UntypedHandle>,
}
