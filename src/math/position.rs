#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn to_index(self, row_width: i32) -> usize {
        super::get_index(self, row_width)
    }
}

#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        Position::new($x, $y)
    };
}
