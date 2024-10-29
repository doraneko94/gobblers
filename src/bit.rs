use std::cmp::min;
use std::collections::HashSet;

use crate::enums::{Piece, Player};
use crate::trit::Trit3;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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

    pub fn get_used(&self, player: Player, piece: Piece) -> u8 {
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
    pub fn set_used(&self, player: Player, piece: Piece, value: u8) -> Self {
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
    pub fn rotate(&self) -> Self {
        self.set_place(0, 0, &self.get_place(0, 2))
            .set_place(0, 2, &self.get_place(2, 2))
            .set_place(2, 2, &self.get_place(2, 0))
            .set_place(2, 0, &self.get_place(0, 0))
            .set_place(0, 1, &self.get_place(1, 2))
            .set_place(1, 2, &self.get_place(2, 1))
            .set_place(2, 1, &self.get_place(1, 0))
            .set_place(1, 0, &self.get_place(0, 1))
    }

    pub fn have(&self, player: Player, piece: Piece) -> bool {
        self.get_used(player, piece) < 3
    }
    pub fn can_put(&self, y: usize, x: usize, piece: Piece) -> bool {
        piece as usize >= self.get_place(y, x).max_empty()
    }
    pub fn put(&self, y: usize, x: usize, player: Player, piece: Piece) -> Self {
        let mut trit = self.get_place(y, x);
        trit.data[piece as usize] = player as u8;
        let used = self.get_used(player, piece);
        self.set_used(player, piece, used + 1).set_place(y, x, &trit)
    }
    pub fn piece(&self, y: usize, x: usize) -> Option<(Player, Piece)> {
        let trit = self.get_place(y, x);
        let pi = trit.max_empty();
        if pi == 0 { return None; }
        let player = if trit.data[pi - 1] == 2 { Player::Orange } else { Player::Blue };
        match pi {
            1 => Some((player, Piece::Small)),
            2 => Some((player, Piece::Middle)),
            _ => Some((player, Piece::Large))
        }
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

    pub fn minimize(&self) -> Self {
        let mut v = vec![self.clone()];
        for i in 0..3 { v.push(v[i].rotate()); }
        for i in 0..4 { v.push(v[i].mirror()); }
        Bit::from(v.iter().fold(u64::MAX, |m, b| min(m, b.data)))
    }
    pub fn next(&self, player: Player) -> HashSet<Self> {
        let mut set = HashSet::new();
        for piece in [Piece::Small, Piece::Middle, Piece::Large] {
            for y in 0..3 {
                for x in 0..3 {
                    if self.have(player, piece) && self.can_put(y, x, piece) {
                        set.insert(self.put(y, x, player, piece).minimize());
                    }
                }
            }
        }
        for y0 in 0..3 {
            for x0 in 0..3 {
                match self.piece(y0, x0) {
                    Some((pl, piece)) => {
                        if pl != player { continue; }
                        for y1 in 0..3 {
                            for x1 in 0..3 {
                                if y0 != y1 && x0 != x1 && self.can_put(y1, x1, piece) {
                                    set.insert(self.replace(y0, x0, y1, x1).minimize());
                                }
                            }
                        }
                    }
                    None => {}
                }
            }
        }
        set
    } 
}