use std::collections::HashMap;

use bevy::prelude::Reflect;

use crate::game::buildings::electro::types::{Channel, ID};

pub const ROOT_LEVEL: u32 = 1;

#[derive(Reflect, Debug)]
pub struct TreeRoot {
    root_id:     ID,
    leafs:       HashMap<ID, TreeLeaf>,
    depth_index: HashMap<u32, Vec<ID>>,
    pub channel: Channel,
    pub child:   Vec<ID>,
}

#[derive(Reflect, Debug)]
pub struct TreeLeaf {
    id:        ID,
    pub level: u32,
    pub child: Vec<ID>,
}

impl TreeRoot {
    pub fn new(id: ID, channel: Channel) -> Self {
        let mut depth_index = HashMap::with_capacity(64);
        depth_index.insert(0, vec![id]);

        Self {
            root_id: id,
            channel,
            leafs: HashMap::with_capacity(64),
            depth_index,
            child: Vec::with_capacity(4),
        }
    }

    pub fn insert_into(&mut self, parent_id: ID, child_id: ID) {
        // skip if already exist
        if self.has(child_id) {
            return;
        }

        // if parent is root
        if self.root_id == parent_id {
            self.child.push(child_id);
            self.leafs
                .insert(child_id, TreeLeaf::new(child_id, ROOT_LEVEL));
            self.index_depth_apply(ROOT_LEVEL, child_id);

            return;
        }

        // if parent is some level leaf
        let parent_leaf = self.leafs.get_mut(&parent_id).expect(&format!(
            "not found parent left {:?} in tree {:?}",
            parent_id, self.root_id
        ));
        let child_level = parent_leaf.level + 1;

        parent_leaf.child.push(child_id);
        self.leafs
            .insert(child_id, TreeLeaf::new(child_id, child_level));
        self.index_depth_apply(child_level, child_id);
    }

    pub fn has(&self, id: ID) -> bool {
        if self.root_id == id {
            // root itself
            return true;
        }

        // some child
        return self.leafs.contains_key(&id);
    }

    pub fn get(&self, id: ID) -> Option<&TreeLeaf> {
        return self.leafs.get(&id);
    }

    pub fn find_on_depth(&self, depth: u32) -> Vec<ID> {
        let clamped_depth = depth % (self.depth_index.len() as u32);

        return match self.depth_index.get(&clamped_depth) {
            Some(v) => v.to_owned(),
            None => vec![],
        };
    }

    fn index_depth_apply(&mut self, depth: u32, id: ID) {
        match self.depth_index.get_mut(&depth) {
            Some(v) => {
                v.push(id);
                return;
            }
            None => {
                let ids: Vec<ID> = vec![id];
                self.depth_index.insert(depth, ids);
                return;
            }
        };
    }
}

impl TreeLeaf {
    pub fn new(id: ID, level: u32) -> Self {
        Self {
            id,
            level,
            child: vec![],
        }
    }
}
