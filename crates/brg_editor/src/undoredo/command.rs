use anyhow::Result;
use bevy::prelude::World;

pub trait EditorCommand {
    fn title(&self) -> String;
    fn apply(&mut self, w: &mut World) -> Result<()>;
    fn compensate(&mut self, w: &mut World) -> Result<()>;
}

pub type EditorCommandSync = dyn EditorCommand + Send + Sync + 'static;
