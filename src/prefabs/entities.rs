use {
    crate::{
        components::{
            movement::{CmpMarkerMovementRestrictInPlayableArea, CmpMovement},
            unit_creature_player::CmpUnitCreaturePlayer,
        },
        game::collisions::CmpCollisionDesiredVolume,
        plugins::{assets::asset_creatures::AssetCreature, InGame},
        prefabs::sup::SupPrefabs,
    },
    bevy::{
        prelude::{default, Circle, Handle, Name, StateScoped},
        sprite::Sprite,
    },
};

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn player(
        &mut self,
    ) -> (
        StateScoped<InGame>,
        CmpUnitCreaturePlayer,
        CmpCollisionDesiredVolume,
        (
            Name,
            CmpMovement,
            CmpMarkerMovementRestrictInPlayableArea,
            Sprite,
        ),
    ) {
        let game = self.assets_game.get(&self.assets.game).unwrap();
        let creature_h = self.asset_creature_handle_by_name(game.player.as_str());
        let creature = self.assets_creatures.get(&creature_h).unwrap();
        let player = self.creature(creature);
        (
            StateScoped(InGame),
            CmpUnitCreaturePlayer {},
            CmpCollisionDesiredVolume::Circle(Circle::new(32.0)),
            player,
        )
    }

    fn creature(
        &self,
        creature: &AssetCreature,
    ) -> (
        Name,
        CmpMovement,
        CmpMarkerMovementRestrictInPlayableArea,
        Sprite,
    ) {
        (
            Name::from(creature.name.clone()),
            CmpMovement {
                speed: creature.movement.speed,
                ..default()
            },
            CmpMarkerMovementRestrictInPlayableArea {},
            Sprite::from_image(creature.agent.sprite.handle.clone()),
        )
    }

    fn asset_creature_handle_by_name(&self, name: &str) -> Handle<AssetCreature> {
        match self.assets.creatures.get(name) {
            Some(handle) => handle.clone(),
            None => {
                panic!(
                    "AssetCreature '{}' not found, but exists: [{}]",
                    name,
                    self.assets
                        .creatures
                        .keys()
                        .cloned()
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }
        }
    }
}
