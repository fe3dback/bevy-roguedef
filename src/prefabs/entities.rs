use bevy::prelude::{default, AlphaMode, Circle, Handle, Name, SpatialListener, StateScoped};
use bevy_sprite3d::{Sprite3dBuilder, Sprite3dBundle};

use crate::components::movement::{CmpMarkerMovementRestrictInPlayableArea, CmpMovement};
use crate::components::unit::EUnitType;
use crate::components::unit_creature_player::CmpUnitCreaturePlayer;
use crate::game::collisions::CmpCollisionDesiredVolume;
use crate::game::teams::{CmpTeam, Team};
use crate::game::weapons::CmpWeapon;
use crate::plugins::assets::asset_creatures::AssetCreature;
use crate::plugins::InGame;
use crate::prefabs::sup::SupPrefabs;

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn player(
        &mut self,
    ) -> (
        StateScoped<InGame>,
        CmpUnitCreaturePlayer,
        CmpCollisionDesiredVolume,
        SpatialListener,
        CmpWeapon,
        CmpTeam,
        EUnitType,
        Sprite3dBundle,
        (Name, CmpMovement, CmpMarkerMovementRestrictInPlayableArea),
    ) {
        let game = self.assets_game.get(&self.assets.game).unwrap();
        let creature_h = self.asset_creature_handle_by_name(game.player.as_str());
        let creature = self.assets_creatures.get(&creature_h).unwrap();
        let player = self.creature(creature);
        (
            StateScoped(InGame),
            CmpUnitCreaturePlayer {},
            CmpCollisionDesiredVolume::Circle(Circle::new(1.0)),
            SpatialListener::new(100.0),
            CmpWeapon::default(),
            CmpTeam { team: Team::Player },
            EUnitType::Creature,
            Sprite3dBuilder {
                image: creature.agent.sprite.handle.clone(),
                pixels_per_metre: 24.0,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                double_sided: true,
                ..default()
            }
            .bundle(&mut self.sprite_params),
            player,
        )
    }

    fn creature(
        &self,
        creature: &AssetCreature,
    ) -> (Name, CmpMovement, CmpMarkerMovementRestrictInPlayableArea) {
        (
            Name::from(creature.name.clone()),
            CmpMovement {
                speed: creature.movement.speed,
                ..default()
            },
            CmpMarkerMovementRestrictInPlayableArea {},
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
