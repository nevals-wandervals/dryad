use crate::entities::{air::Air, block::Block};

pub mod air;
pub mod block;
pub mod traits;

#[derive(Debug)]
pub enum Entity {
    Air(Air),
    Block(Block),
}
