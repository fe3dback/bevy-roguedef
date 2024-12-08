use crate::components::player::CmpMarkerPlayer;
use crate::plugins::assets::asset_creatures::AssetCreature;
use crate::plugins::InGame;
use crate::prefabs::sup::SupPrefabs;
use bevy::prelude::{Name, StateScoped, Transform};
use bevy::sprite::Sprite;

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn player(&mut self) -> (StateScoped<InGame>, CmpMarkerPlayer, (Name, Transform, Sprite)) {
        let player = self.creature("player");
        (
            StateScoped(InGame),
            CmpMarkerPlayer {},
            player
        )
    }

    fn creature(&mut self, name: &str) -> (Name, Transform, Sprite) {
        let creature = self.asset_creature(format!("data/creatures/{}.ron", name).as_str());
        (
            Name::from(creature.name.clone()),
            Transform::default(),
            Sprite::from_image(
                creature.agent.sprite.handle.clone(),
            )
        )
    }

    fn asset_creature(&mut self, name: &str) -> &AssetCreature {
        let handle = self.assets.creatures.get(name);

        if handle.is_some() {
            return self.assets_creature.get(handle.unwrap()).unwrap();
        }

        panic!("AssetCreature '{}' not found, but exists: [{}]", name, self.assets.creatures.keys().cloned().collect::<Vec<String>>().join(", "));
    }
}
