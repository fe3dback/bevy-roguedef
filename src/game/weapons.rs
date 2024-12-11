use {
    crate::{
        components::{transform::CmpTransform2D, unit_creature_player::CmpUnitCreaturePlayer},
        game::{
            common::{CmpTimeToLife, ResMouse},
            damage::Damage,
            projectiles::CmpProjectile,
            sound::SupSounds,
            teams::Team,
        },
    },
    bevy::prelude::{
        default,
        ButtonInput,
        Commands,
        Component,
        Entity,
        MouseButton,
        Query,
        Reflect,
        ReflectResource,
        Res,
        ResMut,
        Resource,
        Time,
        With,
    },
    std::time::Duration,
};

#[derive(Reflect)]
pub struct Weapon {
    pub name:                 String,
    pub damage:               Damage,
    pub shooting_reload_time: Duration,
    pub ammo_in_magazine:     u8,
    pub magazine_reload_time: Duration,
}

#[derive(Reflect, PartialEq, Eq, Clone, Copy)]
pub enum ShootingPhase {
    StandBy,
    Shooting,
    Reloading,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ResPlayerWeapon {
    pub current:             Weapon,
    pub phase:               ShootingPhase,
    pub ammo_left:           u8,
    pub reloading_time_left: Duration,
    pub fire_time:           Duration,
}

impl Default for ResPlayerWeapon {
    fn default() -> Self {
        Self {
            current:             Weapon {
                name:                 String::from("ak-magic-47"),
                damage:               Damage {
                    amount: 11.0,
                    dice_faces: 2,
                    dices_amount: 3,
                    ..default()
                },
                ammo_in_magazine:     30,
                shooting_reload_time: Duration::from_millis(80),
                magazine_reload_time: Duration::from_millis(1800),
            },
            phase:               ShootingPhase::Reloading,
            ammo_left:           0,
            reloading_time_left: Duration::ZERO,
            fire_time:           Duration::ZERO,
        }
    }
}

pub fn shot(
    mut cmd: Commands,
    mut sounds: SupSounds,
    mouse: Res<ButtonInput<MouseButton>>,
    player_query: Query<(Entity, &CmpTransform2D), With<CmpUnitCreaturePlayer>>,
    mouse_data: Res<ResMouse>,
    mut weaponary: ResMut<ResPlayerWeapon>,
    time: Res<Time>,
) {
    if weaponary.phase == ShootingPhase::Reloading {
        let reload_time_left = match weaponary.reloading_time_left <= time.delta() {
            true => Duration::ZERO,
            false => weaponary.reloading_time_left - time.delta(),
        };

        if reload_time_left <= Duration::ZERO {
            weaponary.reloading_time_left = Duration::ZERO;
            weaponary.phase = ShootingPhase::StandBy;
            weaponary.fire_time = Duration::ZERO;
            weaponary.ammo_left = weaponary.current.ammo_in_magazine;
            return;
        }

        weaponary.reloading_time_left = reload_time_left;
    }

    if mouse.just_released(MouseButton::Left) {
        if weaponary.phase == ShootingPhase::Shooting {
            weaponary.phase = ShootingPhase::StandBy;
            return;
        }
    }

    if !(mouse.pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Left)) {
        return;
    }

    // reload
    let mut shot = false;

    match weaponary.phase {
        ShootingPhase::Reloading => {}
        ShootingPhase::StandBy => {
            weaponary.phase = ShootingPhase::Shooting;
            shot = true;
        }
        ShootingPhase::Shooting => {
            weaponary.fire_time = weaponary.fire_time + time.delta();
            shot = true;
        }
    };

    if !shot {
        return;
    }

    let (player_ent, trm2d) = match player_query.get_single() {
        Ok(x) => x,
        Err(_) => return,
    };

    // if we not have ammo -> go to reload
    if weaponary.ammo_left <= 0 {
        weaponary.phase = ShootingPhase::Reloading;
        weaponary.reloading_time_left = weaponary.current.magazine_reload_time;
        sounds.play_reload(trm2d.position);
        return;
    }

    // calculate how much ammo we can fire
    let ammo_already_fired = weaponary.current.ammo_in_magazine - weaponary.ammo_left;
    let ammo_can_be_fired = f32::floor(
        weaponary.fire_time.as_secs_f32() / weaponary.current.shooting_reload_time.as_secs_f32(),
    ) as i32;
    let mut ammo_need_fire = ammo_can_be_fired - ammo_already_fired as i32;
    if ammo_need_fire > 100 {
        ammo_need_fire = 100;
    }

    // wait for ammo is ready
    if ammo_need_fire <= 0 {
        return;
    }

    weaponary.ammo_left -= ammo_need_fire as u8;
    for _ in 0..ammo_need_fire {
        sounds.play_shot(trm2d.position);
        cmd.spawn((
            CmpTimeToLife { seconds_left: 2.0 },
            CmpTransform2D {
                position: trm2d.position,
                angle:    trm2d.position.angle_to(mouse_data.world_pos),
            },
            CmpProjectile {
                team: Team::Player,
                caster: Some(player_ent),
                allow_friendly_fire: true,
                damage: weaponary.current.damage,
                ..default()
            },
        ));
    }
}
