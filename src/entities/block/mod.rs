pub mod live;

#[derive(Debug)]
pub enum Block {
    Live(),
    Dirt(),
    Stone(),
    Sand(),
    Water(),
}
