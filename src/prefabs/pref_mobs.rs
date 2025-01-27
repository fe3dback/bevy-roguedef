use bevy::color::palettes::tailwind::{GRAY_700, GRAY_900, GREEN_700, RED_700};
use bevy::log::warn;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{
    default,
    Bundle,
    Capsule3d,
    EntityCommands,
    Mesh3d,
    Name,
    StateScoped,
};
use bevy_health_bar3d::configuration::BarSettings;
use bevy_health_bar3d::prelude::{BarBorder, BarHeight};
use brg_core::prelude::types::{Angle, Speed};
use brg_core::prelude::{V2, V3};
use brg_fundamental::prelude::{CmpCollisionVolume, CmpTransform2D};
use brg_scene::prelude::{AssetCreature, AssetCreatureMovement, AssetCreatureStats, Loaded};

use super::sup_prefabs::SupPrefabs;
use crate::units::cmp_team::{CmpTeam, ETeam};
use crate::units::cmp_unit_creature::CmpUnitMovement;
use crate::units::mobs::enum_mob_type::MobKind;
use crate::units::stats::health::cmp_health::CmpHealth;
use crate::units::weapon::cmp_weapon::{CmpWeaponHolder, Weapon};

pub struct MobSettings {
    pub kind:  MobKind,
    pub team:  ETeam,
    pub pos:   V2,
    pub angle: Angle,
}

impl Default for MobSettings {
    fn default() -> Self {
        Self {
            kind:  MobKind::SlimeSmall,
            team:  ETeam::Neutral,
            pos:   V2::ZERO,
            angle: 0.0,
        }
    }
}

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn mob<T: Bundle>(&mut self, settings: &MobSettings, with: T) -> EntityCommands {
        let creature = self.creature_by_kind(settings.kind);

        // weapon
        let mut weapon_holder = CmpWeaponHolder::default();
        if let Some(weapon) = creature.weapon {
            weapon_holder.weapons.insert(
                weapon.path,
                Weapon {
                    handle: weapon.handle,
                    ..default()
                },
            );
        }

        // assemble
        self.cmd.spawn((
            (
                StateScoped(Loaded),
                Name::from(format!("mob #{}", creature.name)),
                CmpTransform2D {
                    position: settings.pos,
                    origin_visual_offset: V3::new(0.0, 0.0, 1.1),
                    height: 0.0,
                    angle: settings.angle,
                    ..default()
                },
                CmpTeam::new(settings.team),
                CmpHealth::new_splat(creature.stats.health),
                BarSettings::<CmpHealth> {
                    // todo: make own health bar and spawn this cmp automatically
                    width: 1.4,
                    offset: 1.1,
                    height: BarHeight::Static(0.08),
                    border: BarBorder {
                        width: 0.015,
                        color: GRAY_900.into(),
                    },
                    ..default()
                },
                CmpUnitMovement {
                    speed: Speed::KMH(creature.movement.speed),
                },
                CmpCollisionVolume::Circle(creature.movement.collision_radius_m),
                weapon_holder,
                Mesh3d(self.basic_meshes.add(Capsule3d::new(0.35, 1.4))),
                MeshMaterial3d(
                    self.materials.add(StandardMaterial {
                        base_color: match settings.team {
                            ETeam::Enemies => RED_700,
                            ETeam::Player => GREEN_700,
                            _ => GRAY_700,
                        }
                        .into(),

                        ..default()
                    }),
                ),
            ),
            with,
        ))
    }

    fn creature_by_kind(&mut self, kind: MobKind) -> AssetCreature {
        let def = AssetCreature {
            name:     "Unknown creature".to_string(),
            movement: AssetCreatureMovement {
                speed:              1.0,
                collision_radius_m: 1.0,
            },
            stats:    AssetCreatureStats { health: 10.0 },
            weapon:   None,
        };

        let creature_path = format!("data/creatures/{}.creature.ron", kind.to_string());

        let Some(h) = self.assets.creatures.get(creature_path.as_str()) else {
            warn!("unknown creature kind: {}", creature_path);
            return def;
        };

        let Some(creature) = self.assets_creatures.get(h) else {
            warn!("not found creature by handle {:?} ({})", h, creature_path);
            return def;
        };

        creature.clone()
    }
}
