use strum::{Display, EnumIter};

#[derive(EnumIter, Copy, Clone, Hash, PartialEq, Eq, Display)]
pub enum EventType {
    UpdateBuildingsElectricity,
}

impl EventType {
    pub const fn interval_ms(&self) -> f32 {
        match self {
            EventType::UpdateBuildingsElectricity => 10.0,
        }
    }
}
