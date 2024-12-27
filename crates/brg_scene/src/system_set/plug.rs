use bevy::app::{App, Startup};
use bevy::prelude::{OnEnter, OnExit, Plugin, Update};

use super::fun_init_sets::init_system_sets_for;
use crate::prelude::{GameState, InGame};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        init_system_sets_for(app, Startup);
        init_system_sets_for(app, Update);
        init_system_sets_for(app, OnEnter(GameState::Loading));
        init_system_sets_for(app, OnExit(GameState::Loading));
        init_system_sets_for(app, OnEnter(InGame));
        init_system_sets_for(app, OnExit(InGame));

        // check that expr ^
        // match all game states
        // this code will throw on compile time, if new enum values is added
        let x = GameState::Loading;
        match x {
            GameState::Loading => {}
            GameState::InGame { .. } => {}
        };

        app
        //

        //-
        ;
    }
}
