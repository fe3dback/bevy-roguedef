use bevy::color::palettes::tailwind::{GRAY_900, RED_700};
use bevy::log::warn;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{default, Capsule3d, Mesh3d, Name};
use bevy_health_bar3d::configuration::BarSettings;
use bevy_health_bar3d::prelude::{BarBorder, BarHeight};
use brg_core::prelude::types::Speed;
use brg_core::prelude::{V2, V3};
use brg_fundamental::prelude::{CmpCollisionVolume, CmpTransform2D};
use brg_scene::prelude::{AssetCreature, AssetCreatureMovement, AssetCreatureStats};

use super::sup_prefabs::SupPrefabs;
use crate::units::cmp_team::{CmpTeam, ETeam};
use crate::units::cmp_unit_creature::CmpUnitMovementInput;
use crate::units::mobs::enum_mob_type::MobKind;
use crate::units::stats::health::cmp_health::CmpHealth;
use crate::units::weapon::cmp_weapon::{CmpWeaponHolder, Weapon};

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn mob(
        &mut self,
        kind: MobKind,
    ) -> (
        CmpTransform2D,
        Name,
        CmpTeam,
        CmpHealth,
        BarSettings<CmpHealth>,
        CmpUnitMovementInput,
        CmpCollisionVolume,
        CmpWeaponHolder,
        Mesh3d,
        MeshMaterial3d<StandardMaterial>,
    ) {
        let creature = self.creature_by_kind(kind);

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
        (
            CmpTransform2D {
                position: V2::new(0.0, 0.0),
                origin_visual_offset: V3::new(0.0, 0.0, 1.1),
                height: 0.0,
                ..default()
            },
            Name::from(format!("mob #{}", creature.name)),
            CmpTeam::new(ETeam::Enemies),
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
            CmpUnitMovementInput {
                speed: Speed::KMH(creature.movement.speed),
                ..default()
            },
            CmpCollisionVolume::Circle(creature.movement.collision_radius_m),
            weapon_holder,
            Mesh3d(self.basic_meshes.add(Capsule3d::new(0.35, 1.4))),
            MeshMaterial3d(self.materials.add(StandardMaterial {
                base_color: RED_700.into(),

                ..default()
            })),
        )
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
