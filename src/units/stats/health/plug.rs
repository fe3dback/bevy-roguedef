use bevy::app::{App, Plugin};
use bevy::color::palettes::tailwind::{GRAY_950, LIME_50};
use bevy::prelude::{Alpha, IntoSystemConfigs, Update};
use bevy_health_bar3d::prelude::{ColorScheme, ForegroundColor, HealthBarPlugin};
use brg_scene::prelude::GameSystemSet;

use super::cmp_health::CmpHealth;
use super::sys_despawn_on_death::despawn_on_death;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<CmpHealth>()
            .add_plugins(HealthBarPlugin::<CmpHealth>::default())
            .insert_resource(ColorScheme::<CmpHealth>::new().background_color(GRAY_950.with_alpha(0.5).into()).foreground_color(ForegroundColor::Static(LIME_50.into())))
            .add_systems(Update, despawn_on_death.in_set(GameSystemSet::NOT_ON_PAUSE__DespawnObjects))
        //-
        ;
    }
}
