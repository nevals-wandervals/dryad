pub mod position;

use crate::{math::position::Position, pos};

pub fn get_index(pos: Position, row_width: i32) -> usize {
    let idx = pos.y * row_width + pos.x;
    if idx < 0 {
        return 0;
    }

    idx as usize
}

pub fn get_position(index: usize, row_width: i32) -> Position {
    pos!(index as i32 % row_width, index as i32 / row_width)
}