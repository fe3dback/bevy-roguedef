#[bevy_trait_query::queryable]
pub trait CmpEnergyContainer {
    fn try_spend(&mut self, amount: f32) -> bool {
        false
    }
}
