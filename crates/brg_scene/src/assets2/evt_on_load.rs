use bevy::prelude::{Event, UntypedHandle};

#[derive(Event)]
pub struct EvtOnLoad {
    pub handle: UntypedHandle,
}
