use bevy::prelude::Event;

use crate::old_plugins::gameplay::integrate_steps::enums::EventType;

#[derive(Event)]
pub struct EvtOnIntegration {
    // причина события (наступление конкретного интервала)
    pub cause: EventType,
    // уникальный № в рамках этого интервала (каждое событие увеличивает на 1)
    pub step:  u64,
}
