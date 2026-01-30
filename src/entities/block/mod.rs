pub mod live;
pub mod dirt;
pub mod sand;
pub mod stone;
pub mod water;

#[derive(Debug)]
pub enum Block {
    Live(live::Live),
    Dirt(dirt::Dirt),
    Stone(stone::Stone),
    Sand(sand::Sand),
    Water(water::Water),
}
