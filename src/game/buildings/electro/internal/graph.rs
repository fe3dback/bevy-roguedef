use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;

use bevy::prelude::Reflect;

use crate::components::lib::V2;
use crate::game::buildings::electro::enums::EArchetype;
use crate::game::buildings::electro::internal::tree::TreeRoot;
use crate::game::buildings::electro::types::{
    Channel,
    ChannelBitSize,
    GRAPH_CONNECTION_RADIUS,
    ID,
};

#[derive(Reflect, Debug)]
pub struct Graph {
    pub nodes:      HashMap<ID, Node>,
    pub tree_roots: HashMap<ID, TreeRoot>,
    // aabb: 2DRect // todo: AABB for optimization, useful for find possible graphs for new buildings
}

#[derive(Reflect, Debug)]
pub struct Node {
    pub id:          ID,
    pub archetype:   EArchetype,
    pub can_produce: bool,
    pub can_consume: bool,
    pub neighbours:  Vec<ID>,
    pub center:      V2,
}

#[derive(Clone)]
pub struct Found {
    pub id:         ID,
    pub trees:      Vec<FoundTree>,
    pub channels:   ChannelBitSize,
    pub neighbours: Vec<ID>,
}

#[derive(Clone)]
pub struct FoundInDepth {
    pub id:             ID,
    pub count_channels: ChannelBitSize,
    pub tree:           FoundTree,
}

#[derive(Clone)]
pub struct FoundTree {
    pub root_id: ID,
    pub channel: Channel,
    pub is_root: bool,
    pub level:   u32,
    pub child:   Vec<ID>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes:      HashMap::with_capacity(256),
            tree_roots: HashMap::with_capacity(4),
        }
    }

    pub fn insert(
        &mut self,
        unique_id: ID,
        center: V2,
        archetype: EArchetype,
        can_produce: bool,
        can_consume: bool,
    ) {
        let neighbours = self.find_neighbours(center);

        // update neighbours (add link to new node)
        for id in &neighbours {
            let neightbour = self
                .nodes
                .get_mut(id)
                .expect(&format!("unexpected empty node {:?}", unique_id));
            neightbour.neighbours.push(unique_id);
        }

        // insert new node
        self.nodes.insert(
            unique_id,
            Node {
                id: unique_id,
                center,
                archetype,
                can_produce,
                can_consume,
                neighbours,
            },
        );

        // update tree indexes
        self.update_indexes();
    }

    pub fn clear(&mut self) {
        self.tree_roots.clear();
        self.nodes.clear();
    }

    pub fn remove(&mut self, unique_id: ID) {
        let neighbours = self
            .nodes
            .get(&unique_id)
            .expect(format!("unexpected empty node {:?}", unique_id).as_str())
            .neighbours
            .clone();

        // remove links to removing node
        for id in neighbours {
            let neightbour = self.nodes.get_mut(&id).expect(&format!(
                "unexpected empty neighbour {:?} at node {:?}",
                id, unique_id
            ));
            neightbour.neighbours = new_list_exclude_elem(&neightbour.neighbours, unique_id);
        }

        // remove node
        self.nodes.remove(&unique_id);

        // update tree indexes
        self.update_indexes();
    }

    fn find_neighbours(&self, center: V2) -> Vec<ID> {
        let mut nodes: Vec<ID> = self
            .nodes
            .iter()
            .filter(|(_, node)| node.center.distance(center) <= GRAPH_CONNECTION_RADIUS)
            .map(|(_, node)| node.id)
            .collect();

        nodes.sort_by(|a, b| {
            let dist_a = self.nodes.get(a).unwrap().center.distance(center);
            let dist_b = self.nodes.get(b).unwrap().center.distance(center);

            return dist_a.partial_cmp(&dist_b).unwrap_or(Ordering::Equal);
        });

        return nodes;
    }

    fn update_indexes(&mut self) {
        self.tree_roots.clear();

        for (channel, source_id) in self.find_sources_sort_id_asc().iter().enumerate() {
            let channel = channel as Channel;
            let source_id = *source_id;

            // create root
            self.tree_roots
                .insert(source_id, TreeRoot::new(source_id, channel));
            let root = self.tree_roots.get_mut(&source_id).expect(&format!(
                "unexpected empty just inserted root of source {:?}",
                source_id
            ));

            // create tmp search queue (parent, child)
            let mut deq: VecDeque<(ID, ID)> = VecDeque::with_capacity(64);

            // add first level childs
            for neighbour_id in &self.nodes.get(&source_id).unwrap().neighbours {
                deq.push_back((source_id, *neighbour_id));
            }

            loop {
                let next = deq.pop_front();
                if next.is_none() {
                    // all nodes from this source is found
                    break;
                }

                let (parent_id, child_id) = next.unwrap();
                root.insert_into(parent_id, child_id);

                for neighbour_id in &self.nodes.get(&child_id).unwrap().neighbours {
                    if root.has(*neighbour_id) {
                        // this node has <= level then current (already processed)
                        continue;
                    }

                    deq.push_back((child_id, *neighbour_id));
                }
            }
        }
    }

    fn find_sources_sort_id_asc(&self) -> Vec<ID> {
        let priority_filters: Vec<Box<dyn Fn(&Node) -> bool>> = vec![
            Box::new(|x| x.archetype == EArchetype::Source),
            Box::new(|x| x.archetype == EArchetype::Castle),
            Box::new(|x| x.can_produce && !x.can_consume),
            Box::new(|x| x.can_produce),
        ];

        for boxed_filter in priority_filters {
            let mut sources: Vec<ID> = self
                .nodes
                .iter()
                .filter(|(_, x)| boxed_filter.deref()(x))
                .map(|(id, _)| *id)
                .collect();

            sources.sort();
            if sources.len() >= 1 {
                return sources;
            }
        }

        vec![]
    }
}

#[inline]
fn new_list_exclude_elem<T: Ord + Copy>(list: &Vec<T>, elem: T) -> Vec<T> {
    list.clone()
        .iter()
        .filter(|x| **x != elem)
        .map(|x| *x)
        .collect()
}
