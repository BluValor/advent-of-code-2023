use std::fmt;

pub struct Draw {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl Draw {
    pub fn contains(&self, draw: &Self) -> bool {
        self.r >= draw.r && self.g >= draw.g && self.b >= draw.b
    }
}

impl fmt::Display for Draw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

pub struct Game {
    pub draws: Vec<Draw>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        for draw in &self.draws {
            write!(f, "{} ", draw)?;
        }
        write!(f, "]")
    }
}
