use crate::entities::{air::Air, block::Block};

pub mod air;
pub mod block;


#[derive(Debug)]
pub enum Entity {
    Air(Air),
    Block(Block)
}
