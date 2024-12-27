use bevy::color::palettes::basic::GRAY;
use bevy::color::palettes::tailwind::{BLUE_700, GRAY_600, RED_700};
use bevy::prelude::{Alpha, Entity, EventReader, Query, ResMut};
use brg_core::prelude::{GizmosX, V2};

use crate::game::buildings::electro::cmp::{CmpBuildingElectricity, CmpBuildingOccupied};
use crate::game::buildings::electro::cmp_debug::CmpDebugElectricityOutline;
use crate::game::buildings::electro::enums::EChargeDirection;
use crate::game::buildings::electro::evt::EvtOnBuildingChargeTransfered;
use crate::game::buildings::electro::res_graph::ResBuildingWorldGraphs;
use crate::game::buildings::electro::types::CHANNEL_COLOR;
use crate::plugins::gameplay::integrate_steps::enums::EventType;
use crate::plugins::gameplay::integrate_steps::evt::EvtOnIntegration;

pub fn debug_outline_reset(
    mut events: EventReader<EvtOnIntegration>,
    mut query: Query<&mut CmpDebugElectricityOutline>,
) {
    for ev in events.read() {
        if ev.cause != EventType::UpdateBuildingsElectricity {
            continue;
        }

        for mut x in query.iter_mut() {
            x.on = false;
        }
    }
}

pub fn debug_outline_show(
    mut events: EventReader<EvtOnBuildingChargeTransfered>,
    mut query: Query<&mut CmpDebugElectricityOutline>,
) {
    for ev in events.read() {
        if let Ok(mut node) = query.get_mut(ev.id) {
            node.on = true;
            node.color = match ev.direction {
                EChargeDirection::In => CHANNEL_COLOR[&ev.channel].to_owned(),
                EChargeDirection::Out => GRAY.into(),
            }
        }
    }
}

pub fn draw_graph_tree(
    mut gz: GizmosX,
    query: Query<(
        Entity,
        &CmpBuildingOccupied,
        &CmpBuildingElectricity,
        &CmpDebugElectricityOutline,
    )>,
    graph: ResMut<ResBuildingWorldGraphs>,
) {
    #[derive(PartialEq, Eq, Ord, PartialOrd)]
    enum Severity {
        Primary,
        Verbose1,
        Verbose2,
        Verbose3,
    }

    let cur_severity = Severity::Verbose3;

    for (id, occupied, ecity, outline) in &query {
        let graph_node = graph.find(id);

        // if outline.on && cur_severity >= Severity::Primary {
        //     gz.color = outline.color;
        //     gz.hollow = true;
        //     gz.transform.translation = occupied.range().position().as_3d();
        //
        //     let size = occupied.range().size();
        //     gz.rect(Vec2::new(size.x, size.x));
        // }
        //
        // let text_top =
        //     props.grid_position.position() - V2::new(0.0, ecity.count_channels as f32 * 20.0);
        //
        // if cur_severity == Severity::Verbose1 {
        //     gz.text(
        //         String::from(format!(
        //             "{:.1}/{:.1}",
        //             ecity.channels.iter().fold(0.0, |acc, x| acc + x.charge),
        //             ecity.channels.iter().fold(0.0, |acc, x| acc + x.capacity),
        //         )),
        //         Point::from(text_top),
        //         18.0,
        //     );
        // } else if cur_severity == Severity::Verbose2 {
        //     for cid in 0..ecity.count_channels {
        //         gz.text(
        //             String::from(format!(
        //                 "{:.1}/{:.1}",
        //                 ecity.channels[cid as usize].charge, ecity.channels[cid as usize].capacity
        //             )),
        //             Point::with_color(
        //                 text_top + V2::new(0.0, cid as f32 * 20.0),
        //                 CHANNEL_COLOR[&cid].to_owned(),
        //             ),
        //             18.0,
        //         );
        //     }
        // } else if cur_severity >= Severity::Verbose3 {
        //     for tree in &graph_node.trees {
        //         if let Some(selected) = selection.selected {
        //             if selected == tree.root_id {
        //                 gz.text(
        //                     String::from(format!("#{:?} at L{}", id, tree.level,)),
        //                     Point::from(props.grid_position.position() - V2::new(0.0, 40.0)),
        //                     18.0,
        //                 );
        //             }
        //         }
        //     }
        // }
        //
        if cur_severity >= Severity::Verbose1 {
            let size = occupied.range().size();
            let (w, h) = (size.x, size.y);
            let tl = occupied.grid_position.position();
            let bar_tl = tl - V2::new(0.0, 0.5);
            let bar_size = V2::new(w, 0.3);
            let total_capacity = ecity.channels.iter().fold(0.0, |x, chan| x + chan.capacity);
            let total_charge = ecity.channels.iter().fold(0.0, |x, chan| x + chan.charge);
            let bar_percent = total_charge / total_capacity;
            let padding = V2::new(0.05, 0.05);

            gz.rect(bar_tl, bar_size, GRAY_600);
            gz.rect(
                bar_tl + padding,
                (bar_size - padding) * V2::new(bar_percent, 1.0),
                BLUE_700,
            );
        }

        if cur_severity >= Severity::Verbose2 {
            for neighbour_id in graph_node.neighbours {
                // todo: dirtyfix (need remove deleted entities from graph)
                let neighbour_occupied = match query.get(neighbour_id) {
                    Ok((_, x, _, _)) => x,
                    Err(_) => continue,
                };

                gz.line_gradient(
                    occupied.center() + V2::new(0.05, 0.05),
                    neighbour_occupied.center() + V2::new(0.05, 0.05),
                    GRAY_600.with_alpha(0.05),
                    GRAY_600.with_alpha(0.05),
                );
            }
        }

        if cur_severity == Severity::Verbose3 {
            for tree in &graph_node.trees {
                //if let Some(selected) = selection.selected {
                //if selected == tree.root_id {
                for child_id in &tree.child {
                    let child_occupied = match query.get(*child_id) {
                        Ok((_, x, _, _)) => x,
                        Err(_) => continue,
                    };

                    gz.line_gradient(
                        occupied.center(),
                        child_occupied.center(),
                        RED_700,
                        BLUE_700.with_alpha(0.0),
                    );
                }
                //}
                //}
            }
        }
    }
}
