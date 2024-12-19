use bevy::prelude::{EventReader, EventWriter, Query, Res};

use super::cmp::CmpBuildingElectricity;
use super::dto::ChannelState;
use super::enums::EChargeDirection;
use super::evt::EvtOnBuildingChargeChanged;
use super::internal::graph::{FoundInDepth, FoundTree};
use super::res_graph::ResBuildingWorldGraphs;
use super::types::{Channel, ChannelBitSize, ID};
use crate::plugins::gameplay::integrate_steps::enums::EventType;
use crate::plugins::gameplay::integrate_steps::evt::EvtOnIntegration;

pub fn electricity_tick(
    mut integration_events: EventReader<EvtOnIntegration>,
    mut charge_events: EventWriter<EvtOnBuildingChargeChanged>,
    mut query: Query<&mut CmpBuildingElectricity>,
    graph: Res<ResBuildingWorldGraphs>,
) {
    for ev in integration_events.read() {
        if ev.cause != EventType::UpdateBuildingsElectricity {
            continue;
        }

        // get nodes for current wave
        let nodes = graph.find_on_depth(ev.step as u32);

        // apply transactions
        for node in &nodes {
            // clear throughput values
            reset_throughput(&mut query, node);

            // resize channels
            resize_channels(&mut query, node);

            // can we do some transfer here?
            if node.tree.child.len() <= 0 {
                continue;
            }

            let channel = node.tree.channel;

            let transferring = calculate_transferring(&query, channel, node.id, node.tree.clone());
            if transferring.len() <= 0 {
                continue;
            }

            for (ind, dst_id) in node.tree.child.iter().enumerate() {
                let amount = transferring[ind];
                if amount <= 0.0 {
                    continue;
                }

                let [mut src, mut dst] = query.many_mut([node.id, *dst_id]);

                let before_src = src.channels[channel as usize].charge;
                let before_dst = dst.channels[channel as usize].charge;

                apply_electricity(&mut src, &mut dst, channel, amount);

                let after_src = src.channels[channel as usize].charge;
                let after_dst = dst.channels[channel as usize].charge;

                // notify
                charge_events.send_batch([
                    EvtOnBuildingChargeChanged {
                        id: node.id,
                        pair_id: *dst_id,
                        direction: EChargeDirection::Out,
                        channel,
                        charge_before: before_src,
                        charge_after: after_src,
                    },
                    EvtOnBuildingChargeChanged {
                        id: *dst_id,
                        pair_id: node.id,
                        direction: EChargeDirection::In,
                        channel,
                        charge_before: before_dst,
                        charge_after: after_dst,
                    },
                ]);
            }
        }
    }
}

fn reset_throughput(query: &mut Query<&mut CmpBuildingElectricity>, node: &FoundInDepth) {
    if let Ok(mut src) = query.get_mut(node.id) {
        for cid in 0..src.count_channels {
            src.channels[cid as usize].throughput_out = 0.0;
        }
    }

    for child_id in &node.tree.child {
        if let Ok(mut dst) = query.get_mut(*child_id) {
            for cid in 0..dst.count_channels {
                dst.channels[cid as usize].throughput_in = 0.0;
            }
        }
    }
}

fn resize_channels(query: &mut Query<&mut CmpBuildingElectricity>, node: &FoundInDepth) {
    if let Ok(mut src) = query.get_mut(node.id) {
        resize_node(&mut src, node.count_channels);
    }

    for child_id in &node.tree.child {
        if let Ok(mut dst) = query.get_mut(*child_id) {
            resize_node(&mut dst, node.count_channels);
        }
    }
}

fn resize_node(building: &mut CmpBuildingElectricity, new_count: ChannelBitSize) {
    if building.count_channels == new_count {
        return;
    }

    let total_charge = building.channels.iter().fold(0.0, |chr, x| chr + x.charge);
    let total_thr_in = building
        .channels
        .iter()
        .fold(0.0, |chr, x| chr + x.throughput_in);
    let total_thr_out = building
        .channels
        .iter()
        .fold(0.0, |chr, x| chr + x.throughput_out);

    // reset old data
    for chan in 0..building.count_channels {
        building.channels[chan as usize].charge = 0.0;
        building.channels[chan as usize].throughput_in = 0.0;
        building.channels[chan as usize].throughput_out = 0.0;
        building.channels[chan as usize].capacity = 0.0;
        building.channels[chan as usize].throughput_max_in = 0.0;
        building.channels[chan as usize].throughput_max_out = 0.0;
    }

    // set new data
    for chan in 0..new_count {
        let x = new_count as f32;
        building.channels[chan as usize].charge = total_charge / x;
        building.channels[chan as usize].throughput_in = total_thr_in / x;
        building.channels[chan as usize].throughput_out = total_thr_out / x;
        building.channels[chan as usize].capacity = building.capacity / x;
        building.channels[chan as usize].throughput_max_in = building.throughput_max_in / x;
        building.channels[chan as usize].throughput_max_out = building.throughput_max_out / x;
    }

    // finalize
    building.count_channels = new_count;
}

fn apply_electricity(
    src: &mut CmpBuildingElectricity,
    dst: &mut CmpBuildingElectricity,
    channel: Channel,
    amount: f32,
) {
    src.channels[channel as usize].throughput_out += amount;
    src.channels[channel as usize].charge -= amount;

    dst.channels[channel as usize].throughput_in += amount;
    dst.channels[channel as usize].charge += amount;
}

/// first ID - src
/// other IDs - dst (childs)
fn calculate_transferring(
    query: &Query<&mut CmpBuildingElectricity>,
    channel: Channel,
    src_id: ID,
    src_tree: FoundTree,
) -> Vec<f32> {
    let mut ids = vec![src_id];
    ids.extend(src_tree.child.iter().cloned());

    let mut src: Option<&CmpBuildingElectricity> = None;
    let mut dst_list: Vec<&CmpBuildingElectricity> = Vec::with_capacity(ids.len());

    for (ind, building) in query.iter_many(&ids).enumerate() {
        match ind {
            0 => src = Some(building),
            _ => dst_list.push(building),
        }
    }

    if src.is_none() {
        return vec![];
    }

    return calculate_transfer_amount_per_child(channel, src.unwrap(), dst_list);
}

fn calculate_transfer_amount_per_child(
    channel: Channel,
    src: &CmpBuildingElectricity,
    dst_list: Vec<&CmpBuildingElectricity>,
) -> Vec<f32> {
    let mut reserve: f32 = 0.0;

    // copy for data mutation
    let mut emul_src = src.clone();
    let mut emul_dst_list: Vec<CmpBuildingElectricity> =
        dst_list.iter().map(|x| (*x).clone()).collect();

    // calculate
    let max_out = match emul_src.can_produce {
        true => calculate_max_out_amount(&emul_src.channels[channel as usize]),
        false => 0.0,
    };
    let mut throughput_per_child = max_out / dst_list.len() as f32;

    // emulating
    let mut results: Vec<f32> = vec![0.0; dst_list.len()];

    for mut ind in 0..=dst_list.len() {
        // check first child one more time (using reserve)
        if ind == dst_list.len() {
            ind = 0;
            throughput_per_child = 0.0;
        }

        let emul_dst = emul_dst_list.get_mut(ind).unwrap();
        let amount = f32::min(
            calculate_max_transfer_amount(channel, &emul_src, emul_dst),
            throughput_per_child + reserve,
        );

        let remainder = throughput_per_child - amount;
        reserve += remainder;

        // commit
        emul_src.channels[channel as usize].charge -= amount;
        emul_src.channels[channel as usize].throughput_out += amount;

        emul_dst.channels[channel as usize].charge += amount;
        emul_dst.channels[channel as usize].throughput_in += amount;

        // calculate results
        results[ind] = results[ind] + amount;
    }

    return results;
}

fn calculate_max_transfer_amount(
    channel: Channel,
    src: &CmpBuildingElectricity,
    dst: &CmpBuildingElectricity,
) -> f32 {
    let max_out = match src.can_produce {
        true => calculate_max_out_amount(&src.channels[channel as usize]),
        false => return 0.0,
    };

    let max_in = match dst.can_consume {
        true => calculate_max_in_amount(&dst.channels[channel as usize]),
        false => return 0.0,
    };

    let amount = f32::min(max_out, max_in);
    return f32::clamp(amount, 0.0, amount);
}

fn calculate_max_out_amount(src: &ChannelState) -> f32 {
    let free_throughput = src.throughput_max_out - src.throughput_out;

    let amount = f32::min(src.charge, free_throughput);
    return f32::clamp(amount, 0.0, amount);
}

fn calculate_max_in_amount(dst: &ChannelState) -> f32 {
    let deficient_charge = dst.capacity - dst.charge;
    let free_throughput = dst.throughput_max_in - dst.throughput_in;

    let amount = f32::min(deficient_charge, free_throughput);
    return f32::clamp(amount, 0.0, amount);
}
