#[derive(Clone, Copy)]
pub enum Player {
    Orange = 2,
    Blue = 1,
}

#[derive(Clone, Copy)]
pub enum Piece {
    Large = 2,
    Middle = 1,
    Small = 0,
}

impl Piece {
    pub fn from_num(num: usize) -> Self {
        if num == 0 { Self::Small }
        else if num == 1 { Self::Middle }
        else { Self::Large }
    }
}