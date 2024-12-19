use std::collections::HashMap;
use std::time::Instant;

use bevy::prelude::{EventWriter, ResMut, Resource, With, World};
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::egui;
use strum::IntoEnumIterator;

use crate::plugins::gameplay::integrate_steps::dto_state::State;
use crate::plugins::gameplay::integrate_steps::enums::EventType;
use crate::plugins::gameplay::integrate_steps::evt::EvtOnIntegration;

#[derive(Resource)]
pub struct ResLocalIntegrationTimings {
    started: Instant,
    state:   HashMap<EventType, State>,
}

impl Default for ResLocalIntegrationTimings {
    fn default() -> Self {
        Self {
            started: Instant::now(),
            state:   HashMap::new(),
        }
    }
}

pub fn init(mut timers: ResMut<ResLocalIntegrationTimings>) {
    timers.started = Instant::now();

    for integration in EventType::iter() {
        timers.state.insert(
            integration,
            State {
                active:  true,
                target:  0,
                current: 0,
                last:    0,
            },
        );
    }
}

pub fn clear(mut timers: ResMut<ResLocalIntegrationTimings>) {
    timers.state.clear();
}

pub fn update(
    mut timers: ResMut<ResLocalIntegrationTimings>,
    mut writer: EventWriter<EvtOnIntegration>,
) {
    let started = timers.started.elapsed().as_millis() as f32;

    // update state
    for integration in EventType::iter() {
        let state = match timers.state.get_mut(&integration) {
            Some(s) => s,
            None => continue,
        };

        let normalized_interval = 1000.0 / integration.interval_ms(); // 1000ms/60f = 16.6ms
        let normalized_counter: u64 = (started / normalized_interval).floor() as u64; // 325.304 -> 325

        // check integration

        let integration_steps = normalized_counter - state.target;
        state.target = state.target + integration_steps;

        if !state.active {
            continue;
        }

        state.current += integration_steps;
    }

    // integrate state
    for integration in EventType::iter() {
        let state = match timers.state.get_mut(&integration) {
            Some(s) => s,
            None => continue,
        };

        let mut integrated = 0;
        for step_index in state.last..state.current {
            integrated += 1;
            writer.send(EvtOnIntegration {
                cause: integration,
                step:  step_index,
            });
        }

        state.last += integrated;
    }
}

pub fn editor_update(world: &mut World) {
    world.resource_scope::<ResLocalIntegrationTimings, _>(|world, mut timers| {
        let Ok(egui_context) = world
            .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
            .get_single(world)
        else {
            return;
        };

        let mut ctx = egui_context.clone();

        egui::Window::new("Integrations").show(ctx.get_mut(), |ui| {
            for integration in EventType::iter() {
                let state = match timers.state.get_mut(&integration) {
                    Some(s) => s,
                    None => continue,
                };

                ui.horizontal(|row| {
                    let toggle_btn_title = match state.active {
                        true => "||",
                        false => "|>",
                    };

                    row.label(format!("{}", integration));
                    if row.button(toggle_btn_title).clicked() {
                        state.active = !state.active;
                    }
                    if row.button("step").clicked() {
                        state.current += 1;
                    }
                });
            }
        });
    });
}
