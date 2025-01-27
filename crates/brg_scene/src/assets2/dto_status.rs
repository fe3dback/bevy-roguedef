use bevy::prelude::UntypedHandle;

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub enum ELoadingStage {
    #[default]
    CalculateAssetsToLoad,
    Loading,
    Validation,
    NotValid,
    Ready,
    Completed,
}

#[derive(Default, Clone, Debug)]
pub struct DtoLoadingStatus {
    pub stage:            ELoadingStage,
    pub cnt_total:        u16,
    pub cnt_loaded:       u16,
    pub cnt_failed:       u16,
    pub cnt_ready:        u16,
    pub last_info_title:  String,
    pub last_info_error:  Option<String>,
    pub last_info_handle: Option<UntypedHandle>,
}
