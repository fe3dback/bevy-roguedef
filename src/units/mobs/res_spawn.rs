use bevy::prelude::{Reflect, ReflectResource, Resource};

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct ResMobsSpawnRules {
    pub spawn_clicked: bool,
    pub dice_sides:    u16,
    pub dice_count:    u16,
}

impl Default for ResMobsSpawnRules {
    fn default() -> Self {
        Self {
            spawn_clicked: false,
            dice_sides:    2,
            dice_count:    2,
        }
    }
}
