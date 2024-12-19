use std::collections::HashMap;

use bevy::prelude::{Color, Entity};
use lazy_static::lazy_static;

use crate::consts::TEAM_COLORS;

pub type ID = Entity;

// max 255 (u8)
pub type Channel = u8;
pub type ChannelBitSize = u8;

pub const MAX_CHANNELS: usize = 16;
pub const GRAPH_CONNECTION_RADIUS: f32 = 10.0;

lazy_static! {
    pub static ref CHANNEL_COLOR: HashMap<Channel, &'static Color> = {
        let hm = HashMap::from([
            (0, &TEAM_COLORS[0]),
            (1, &TEAM_COLORS[1]),
            (2, &TEAM_COLORS[2]),
            (3, &TEAM_COLORS[3]),
            (4, &TEAM_COLORS[4]),
            (5, &TEAM_COLORS[5]),
            (6, &TEAM_COLORS[6]),
            (7, &TEAM_COLORS[7]),
            (8, &TEAM_COLORS[8]),
            (9, &TEAM_COLORS[9]),
            (10, &TEAM_COLORS[10]),
            (11, &TEAM_COLORS[11]),
            (12, &TEAM_COLORS[12]),
            (13, &TEAM_COLORS[13]),
            (14, &TEAM_COLORS[14]),
            (15, &TEAM_COLORS[15]),
        ]);
        assert_eq!(hm.len(), MAX_CHANNELS);

        hm
    };
}
