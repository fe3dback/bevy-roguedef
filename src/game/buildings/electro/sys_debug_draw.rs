use bevy::color::palettes::basic::GRAY;
use bevy::prelude::{Entity, EventReader, Query, ResMut, Vec2};
use bevy_vector_shapes::prelude::{RectPainter, ShapePainter};

use crate::consts::PIXELS_PER_METER;
use crate::game::buildings::electro::cmp::{CmpBuildingElectricity, CmpBuildingOccupied};
use crate::game::buildings::electro::cmp_debug::CmpDebugElectricityOutline;
use crate::game::buildings::electro::enums::EChargeDirection;
use crate::game::buildings::electro::evt::EvtOnBuildingChargeChanged;
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
    mut events: EventReader<EvtOnBuildingChargeChanged>,
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
    mut gz: ShapePainter,
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

    let cur_severity = Severity::Primary;

    for (id, occupied, ecity, outline) in &query {
        let graph_node = graph.find(id);

        if outline.on && cur_severity >= Severity::Primary {
            gz.color = outline.color;
            gz.hollow = true;
            gz.transform.translation = occupied.range().position().as_3d();

            let size = occupied.range().size();
            gz.rect(Vec2::new(
                size.x * PIXELS_PER_METER,
                size.x * PIXELS_PER_METER,
            ));
        }
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
        // if cur_severity >= Severity::Verbose3 {
        //     for neighbour_id in graph_node.neighbours {
        //         if id >= neighbour_id {
        //             continue;
        //         }
        //
        //         let (_, neighbour_props, _, _) = query
        //             .get(neighbour_id)
        //             .expect(&format!("not exist entity {:?}", neighbour_id));
        //
        //         gz.line(
        //             Point::with_color(props.center(), Color::GRAY),
        //             Point::with_color(neighbour_props.center(), Color::GRAY),
        //         );
        //     }
        // }
        //
        // if cur_severity >= Severity::Primary {
        //     for tree in &graph_node.trees {
        //         if let Some(selected) = selection.selected {
        //             if selected == tree.root_id {
        //                 for child_id in &tree.child {
        //                     let (_, child_props, _, _) = query
        //                         .get(*child_id)
        //                         .expect(&format!("not exist entity {:?}", child_id));
        //
        //                     gz.line(
        //                         Point::with_color(props.center(), Color::RED),
        //                         Point::with_color(child_props.center(), Color::GREEN),
        //                     );
        //                 }
        //             }
        //         }
        //     }
        // }
    }
}
