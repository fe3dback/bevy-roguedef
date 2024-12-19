use bevy::prelude::Event;

use crate::game::buildings::electro::enums::EChargeDirection;
use crate::game::buildings::electro::types::{Channel, ID};

#[derive(Event)]
pub struct EvtOnBuildingChargeChanged {
    // id здания у которого меняется charge
    pub id:            ID,
    // от кого получили, или кому мы передали заряд (зависит от direction)
    pub pair_id:       ID,
    // направление передачи (получили или отдали)
    pub direction:     EChargeDirection,
    // канал изменения
    pub channel:       Channel,
    // сколько у меня здания было ДО
    pub charge_before: f32,
    // сколько стало после
    pub charge_after:  f32,
}
