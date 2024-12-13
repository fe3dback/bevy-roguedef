use {
    crate::components::tiles::{Range, Tile},
    core::ops::{Add, Sub},
    std::{
        collections::{hash_set::Iter, HashSet},
        ops::{AddAssign, SubAssign},
    },
};

#[derive(Clone, Default)]
pub struct Selection {
    set: HashSet<Tile>,
}

impl Selection {
    pub fn from_vec(arr: Vec<Tile>) -> Self {
        let mut set = HashSet::with_capacity(arr.len());
        for tile in arr {
            set.insert(tile);
        }

        Self { set }
    }

    pub fn from_range(range: Range) -> Self {
        let mut set = HashSet::with_capacity(range.len());

        for tile in &range {
            set.insert(tile);
        }

        Self { set }
    }

    pub fn len(&self) -> usize {
        return self.set.len();
    }

    pub fn is_contains(&self, tile: Tile) -> bool {
        return self.set.contains(&tile);
    }
}

impl<'a> IntoIterator for &'a Selection {
    type Item = &'a Tile;
    type IntoIter = Iter<'a, Tile>;
    fn into_iter(self) -> Iter<'a, Tile> {
        return self.set.iter();
    }
}

#[auto_impl_ops::auto_ops]
impl AddAssign<&Selection> for Selection {
    fn add_assign(&mut self, other: &Self) {
        for other_tile in &other.set {
            self.set.insert(other_tile.to_owned());
        }
    }
}

#[auto_impl_ops::auto_ops]
impl SubAssign<&Selection> for Selection {
    fn sub_assign(&mut self, other: &Self) {
        for other_tile in &other.set {
            self.set.remove(other_tile);
        }
    }
}

#[auto_impl_ops::auto_ops]
impl AddAssign<&Range> for Selection {
    fn add_assign(&mut self, other: &Range) {
        for other_tile in other {
            self.set.insert(other_tile.to_owned());
        }
    }
}

#[auto_impl_ops::auto_ops]
impl SubAssign<&Range> for Selection {
    fn sub_assign(&mut self, other: &Range) {
        for other_tile in other {
            self.set.remove(&other_tile);
        }
    }
}
