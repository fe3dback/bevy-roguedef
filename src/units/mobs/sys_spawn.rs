use bevy::prelude::{Commands, Mut, ResMut, StateScoped, With, World};
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::egui;
use brg_core::prelude::{ResRandomSource, V2};
use brg_scene::prelude::InGame;
use strum::IntoEnumIterator;

use crate::prefabs::sup_prefabs::SupPrefabs;
use crate::units::mobs::enum_mob_type::MobKind;
use crate::units::mobs::res_spawn::ResMobsSpawnRules;

pub fn editor_enemies_window_update(world: &mut World) {
    world.resource_scope(|world, mut rules: Mut<ResMobsSpawnRules>| {
        let Ok(egui_context) = world
            .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
            .get_single(world)
        else {
            return;
        };

        let mut ctx = egui_context.clone();

        egui::Window::new("Enemies").show(ctx.get_mut(), |ui| {
            ui.horizontal(|row| {
                row.label("Dice sides/count");
                row.add(egui::widgets::DragValue::new(&mut rules.dice_sides));
                row.add(egui::widgets::DragValue::new(&mut rules.dice_count));
            });
            ui.horizontal(|row| {
                if row.button("spawn").clicked() {
                    rules.spawn_clicked = true;
                }
            });
        });
    });
}

pub fn spawn_mobs(
    mut cmd: Commands,
    mut rand: ResMut<ResRandomSource>,
    mut pref: SupPrefabs,
    mut rules: ResMut<ResMobsSpawnRules>,
) {
    if !rules.spawn_clicked {
        return;
    }

    rules.spawn_clicked = false;

    let chances = vec![(MobKind::SlimeBig, 25), (MobKind::SlimeSmall, 100)];

    for _ in 0..rand.rand_roll_dices(rules.dice_count, rules.dice_sides) {
        let rnd_angle = rand.rand_angle();
        let rnd_dist = rand.rand_int32_in_range(10, 15) as f32;
        let pos_spawn = V2::ZERO.polar_offset(rnd_dist, rnd_angle);

        let kind = {
            let mut result: Option<MobKind> = None;

            for (kind, chance) in chances.iter() {
                if rand.rand_int32_in_range(0, 100) >= *chance {
                    continue;
                }

                result = Some(*kind);
                break;
            }

            result
        };

        let Some(kind) = kind else {
            continue;
        };

        let mut mob = pref.mob(kind);
        mob.0.position = pos_spawn;
        mob.0.angle = pos_spawn.angle_to(V2::ZERO);

        cmd.spawn((mob, StateScoped(InGame)));
    }
}
