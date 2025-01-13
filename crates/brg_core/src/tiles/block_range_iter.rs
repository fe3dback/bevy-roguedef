use super::block::{Block, BlockXY};
use super::block_range::Range;

pub struct RangeIter<'a, T: Block + BlockXY + Copy> {
    data:   &'a Range<T>,
    cursor: T,
}

impl<'a, T: Block + BlockXY + Copy> IntoIterator for &'a Range<T> {
    type Item = T;
    type IntoIter = RangeIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        RangeIter {
            data:   &self,
            cursor: T::at(self.min_x - 1, self.min_y),
        }
    }
}

impl<T: Block + BlockXY + Copy> Iterator for RangeIter<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // left -> right
        self.cursor.set_x(self.cursor.x() + 1);

        // top -> bottom
        if self.cursor.x() > self.data.max_x {
            self.cursor.set_x(self.data.min_x);
            self.cursor.set_y(self.cursor.y() + 1);
        }

        if self.cursor.y() > self.data.max_y {
            return None;
        }

        Some(self.cursor)
    }
}
