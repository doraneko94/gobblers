use crate::enums::{Piece, Player};
use crate::trit::Trit3;

#[derive(Clone, Copy)]
pub struct Bit {
    pub data: u64,
}

impl Bit {
    pub fn new() -> Self {
        Self { data: 0 }
    }
    pub fn from(data: u64) -> Self {
        Self { data }
    }

    pub fn get_hand(&self, player: Player, piece: Piece) -> u8 {
        let ans = match player {
            Player::Orange => self.data >> (piece as usize * 2 + 6) & 3,
            Player::Blue => self.data >> (piece as usize * 2) & 3
        };
        ans as u8
    }
    pub fn get_place(&self, y: usize, x: usize) -> Trit3 {
        let bit = self.data >> (52 - (y * 3 + x) * 5) & 31;
        Trit3::from_bit(bit)
    }
    pub fn set_hand(&self, player: Player, piece: Piece, value: u8) -> Self {
        let mut data = self.data;
        match player {
            Player::Orange => {
                data &= !(3 << (piece as usize * 2 + 6));
                data |= (value as u64) << (piece as usize * 2 + 6);
            }
            Player::Blue => {
                data &= !(3 << (piece as usize * 2));
                data |= (value as u64) << (piece as usize * 2);
            }
        }
        Self::from(data)
    }
    pub fn set_place(&self, y: usize, x: usize, trit: &Trit3) -> Self {
        let bit = trit.to_bit();
        let mut data = self.data;
        data &= !(31 << (52 - (y * 3 + x) * 5));
        data |= bit << (52 - (y * 3 + x) * 5);
        Self::from(data)
    }

    pub fn mirror(&self) -> Self {
        let mut ans = self.clone();
        for i in 0..3 {
            ans = ans.set_place(i, 0, &self.get_place(i, 2))
                .set_place(i, 2, &self.get_place(i, 0));
        }
        ans
    }

    pub fn have(&self, player: Player, piece: Piece) -> bool {
        self.get_hand(player, piece) < 3
    }
    pub fn can_put(&self, y: usize, x: usize, piece: Piece) -> bool {
        piece as usize >= self.get_place(y, x).max_empty()
    }
    pub fn put(&self, y: usize, x: usize, player: Player, piece: Piece) -> Self {
        let mut trit = self.get_place(y, x);
        trit.data[piece as usize] = player as u8;
        let hand = self.get_hand(player, piece);
        self.set_hand(player, piece, hand - 1).set_place(y, x, &trit)
    }
    pub fn replace(&self, y0: usize, x0: usize, y1: usize, x1: usize) -> Self {
        let mut trit0 = self.get_place(y0, x0);
        let mut trit1 = self.get_place(y1, x1);
        let piece = trit0.max_empty() - 1;
        let player = trit0.data[piece];
        trit0.data[piece] = 0;
        trit1.data[piece] = player;
        self.set_place(y0, x0, &trit0).set_place(y1, x1, &trit1)
    }
}