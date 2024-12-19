use bevy::ecs::reflect::ReflectResource;
use bevy::prelude::{Reflect, Resource};

use crate::components::lib::V2;
use crate::game::buildings::electro::enums::EArchetype;
use crate::game::buildings::electro::internal;
use crate::game::buildings::electro::internal::graph::{Found, FoundInDepth, FoundTree, Graph};
use crate::game::buildings::electro::types::{ChannelBitSize, ID};

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct ResBuildingWorldGraphs {
    graphs: Vec<Graph>,
}

impl Default for ResBuildingWorldGraphs {
    fn default() -> Self {
        Self {
            graphs: vec![Graph::new()],
        }
    }
}

impl ResBuildingWorldGraphs {
    pub fn insert(
        &mut self,
        unique_id: ID,
        center: V2,
        archetype: EArchetype,
        can_produce: bool,
        can_consume: bool,
    ) {
        // todo: find graphs and filter by AABB
        let intersected = vec![&mut self.graphs[0]]; // todo: remove hardcode

        // todo: if len(intersected) == 0, create new graph
        // todo: if len(intersected) == 1, best case
        // todo: if len(intersected) > 1 - need merge

        for graph in intersected {
            graph.insert(unique_id, center, archetype, can_produce, can_consume);
        }
    }

    pub fn clear(&mut self) {
        for graph in &mut self.graphs {
            graph.clear();
        }
    }

    /// lookup by id and return list of child and neighbours
    pub fn find(&self, unique_id: ID) -> Found {
        // must by only one graph with this element
        // (or we expect that two+ graphs should merge together early)
        let parent_graph = &self.graphs[0]; // todo: remove hardcode
        let node = parent_graph.nodes.get(&unique_id).unwrap();

        let mut found_in_trees: Vec<FoundTree> = Vec::with_capacity(4);
        for (root_id, root) in &parent_graph.tree_roots {
            if unique_id == *root_id {
                found_in_trees.push(FoundTree {
                    root_id: *root_id,
                    channel: root.channel,
                    is_root: true,
                    child:   root.child.clone(),
                    level:   internal::tree::ROOT_LEVEL,
                });
                continue;
            }

            let leaf = root.get(unique_id);
            if leaf.is_none() {
                // this tree not has this node
                continue;
            }

            let leaf = leaf.unwrap();
            found_in_trees.push(FoundTree {
                root_id: *root_id,
                channel: root.channel,
                is_root: false,
                child:   leaf.child.clone(),
                level:   leaf.level,
            });
        }

        return Found {
            id:         unique_id,
            trees:      found_in_trees,
            channels:   parent_graph.tree_roots.len() as ChannelBitSize, // todo: validate trees count (and reject adding new source in fulfilled graph)
            neighbours: node.neighbours.clone(),
        };
    }

    /// find all nodes in depth level of graph
    /// return exactly one node per source tree
    /// depth can be larger than tree max_level, it will loop by mod (61 % 60 = 1)
    pub fn find_on_depth(&self, depth: u32) -> Vec<FoundInDepth> {
        let mut result: Vec<FoundInDepth> = Vec::with_capacity(32);

        for graph in &self.graphs {
            for (root_id, root) in &graph.tree_roots {
                for id in root.find_on_depth(depth) {
                    let node = self.find(id);

                    for in_tree in node.trees {
                        if in_tree.root_id != *root_id {
                            continue;
                        }

                        result.push(FoundInDepth {
                            id,
                            count_channels: node.channels,
                            tree: in_tree,
                        });
                    }
                }
            }
        }

        return result;
    }
}
