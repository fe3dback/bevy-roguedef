use crate::components::movement::CmpMovement;
use crate::components::unit_creature_player::CmpUnitCreaturePlayer;
use crate::plugins::assets::asset_creatures::AssetCreature;
use crate::plugins::InGame;
use crate::prefabs::sup::SupPrefabs;
use bevy::prelude::{default, Handle, Name, StateScoped};
use bevy::sprite::Sprite;

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn player(&mut self) -> (StateScoped<InGame>, CmpUnitCreaturePlayer, (Name, CmpMovement, Sprite)) {
        let game = self.assets_game.get(&self.assets.game).unwrap();
        let creature_h = self.asset_creature_handle_by_name(game.player.as_str());
        let creature = self.assets_creatures.get(&creature_h).unwrap();
        let player = self.creature(creature);
        (
            StateScoped(InGame),
            CmpUnitCreaturePlayer {},
            player
        )
    }

    fn creature(&self, creature: &AssetCreature) -> (Name, CmpMovement, Sprite) {
        (
            Name::from(creature.name.clone()),
            CmpMovement {
                speed: creature.movement.speed,
                ..default()
            },
            Sprite::from_image(
                creature.agent.sprite.handle.clone(),
            )
        )
    }

    fn asset_creature_handle_by_name(&self, name: &str) -> Handle<AssetCreature> {
        match self.assets.creatures.get(name) {
            Some(handle) => handle.clone(),
            None => {
                panic!("AssetCreature '{}' not found, but exists: [{}]", name, self.assets.creatures.keys()
                    .cloned().
                    collect::<Vec<String>>().
                    join(", ")
                );
            }
        }
    }
}
