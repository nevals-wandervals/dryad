#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn get_mut_x(&mut self) -> &mut i32 {
        &mut self.x
    }

    pub fn set_x(&mut self, value: i32) {
        self.x = value;
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn get_mut_y(&mut self) -> &mut i32 {
        &mut self.y
    }

    pub fn set_y(&mut self, value: i32) {
        self.y = value;
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
