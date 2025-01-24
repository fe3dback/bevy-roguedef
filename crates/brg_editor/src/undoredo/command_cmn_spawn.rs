use anyhow::bail;
use bevy::prelude::{Bundle, DespawnRecursiveExt, Entity, World};

use super::command::EditorCommand;

pub struct EditorCommandSpawn<T: Bundle> {
    name:               String,
    spawned:            Option<Entity>,
    factory_components: Box<dyn Fn() -> T + Send + Sync + 'static>,
}

impl<T: Bundle> EditorCommandSpawn<T> {
    pub fn new(name: String, factory_components: impl Fn() -> T + Send + Sync + 'static) -> Self {
        Self {
            name,
            spawned: None,
            factory_components: Box::new(factory_components),
        }
    }
}

impl<T: Bundle> EditorCommand for EditorCommandSpawn<T> {
    fn title(&self) -> String {
        format!("spawn entity {}", self.name).to_string()
    }

    fn apply(&mut self, w: &mut World) -> anyhow::Result<()> {
        if self.spawned.is_some() {
            bail!("Spawned command is already applied");
        }

        let components = (*self.factory_components)();
        let id = w.commands().spawn(components).id();
        self.spawned = Some(id);

        Ok(())
    }

    fn compensate(&mut self, w: &mut World) -> anyhow::Result<()> {
        let Some(id) = self.spawned else {
            bail!("Unexpected empty entity ID in compensate state");
        };

        w.commands().entity(id).despawn_recursive();
        self.spawned = None;

        Ok(())
    }
}
