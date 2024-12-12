use {
    crate::{
        components::{lib::V2, transform::CmpTransform2D},
        game::{
            buildings::CmpBuilding,
            collisions::CmpCollisionDesiredVolume,
            damage::CmpHealth,
            teams::{CmpTeam, Team},
        },
    },
    bevy::{
        asset::AssetServer,
        ecs::system::SystemParam,
        prelude::{Commands, Name, Rectangle, Res, Sprite},
    },
};

#[derive(SystemParam)]
pub struct SupBuildingSpawner<'w, 's> {
    cmd:          Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
}

impl<'w, 's> SupBuildingSpawner<'w, 's> {
    fn span_to(val: f32) -> f32 {
        f32::round(val / 24.0) * 24.0
    }

    pub fn spawn_pole(&mut self, at: V2) {
        let snapped_pos = V2::new(Self::span_to(at.x), Self::span_to(at.y)) + V2::splat(12.0);

        self.cmd.spawn((
            Name::from("pole"),
            CmpTeam { team: Team::Player },
            CmpHealth {
                health:     20.0,
                max_health: 20.0,
            },
            CmpCollisionDesiredVolume::Aabb(Rectangle::new(24.0, 24.0)),
            Sprite::from_image(self.asset_server.load("sprites/buildings/pole.png")),
            CmpTransform2D {
                position: snapped_pos,
                angle:    0.0,
            },
            CmpBuilding {},
        ));
    }
}
