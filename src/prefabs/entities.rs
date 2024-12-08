use crate::components::player::CmpMarkerPlayer;
use crate::plugins::assets::asset_creatures::AssetCreature;
use crate::plugins::InGame;
use crate::prefabs::sup::SupPrefabs;
use bevy::prelude::{Handle, Name, StateScoped, Transform};
use bevy::sprite::Sprite;

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn player(&mut self) -> (StateScoped<InGame>, CmpMarkerPlayer, (Name, Transform, Sprite)) {
        let game = self.assets_game.get(&self.assets.game).unwrap();
        let creature_h = self.asset_creature_handle_by_name(game.player.as_str());
        let creature = self.assets_creatures.get(&creature_h).unwrap();
        let player = self.creature(creature);
        (
            StateScoped(InGame),
            CmpMarkerPlayer {},
            player
        )
    }

    fn creature(&self, creature: &AssetCreature) -> (Name, Transform, Sprite) {
        (
            Name::from(creature.name.clone()),
            Transform::default(),
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
