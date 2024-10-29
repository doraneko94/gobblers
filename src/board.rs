use crate::bit::Bit;
use crate::enums::{Piece, Player};

const ORANGE_PIECES: [char; 3] = ['.', 'o', 'O'];
const BLUE_PIECES: [char; 3] = [',', 'x', 'X'];

pub struct BoardMap {
    pub board: [[char; 3]; 3],
    pub hand_orange: Vec<char>,
    pub hand_blue: Vec<char>,
}

impl BoardMap {
    pub fn new() -> Self {
        Self {
            board: [[' '; 3]; 3],
            hand_orange: Vec::with_capacity(9),
            hand_blue: Vec::with_capacity(9) }
    }

    pub fn from_board(bit: Bit) -> BoardMap {
        let mut bm = BoardMap::new();
        for i in 0..9 {
            let (y, x) = ((i/3) as usize, (i%3) as usize);
            let trit = bit.get_place(y, x);
            for j in 0..3 {
                bm.board[y][x] = match trit.data[j] {
                    2 => ORANGE_PIECES[j],
                    1 => BLUE_PIECES[j],
                    _ => bm.board[y][x],
                };
            }
        }
        for pi in 0..3 {
            for _ in 0..bit.get_hand(Player::Orange, Piece::from_num(pi)) {
                bm.hand_orange.push(ORANGE_PIECES[pi])
            }
            for _ in 0..bit.get_hand(Player::Blue, Piece::from_num(pi)) {
                bm.hand_blue.push(BLUE_PIECES[pi])
            }
        }
        bm
    }

    pub fn result(&self) -> Option<Player> {
        let mut scores = [0; 8];
        for i in 0..3 {
            for j in 0..3 {
                scores[i*2] += _score(self.board[i][j]);
                scores[i*2+1] += _score(self.board[j][i]);
            }
            scores[6] += _score(self.board[i][i]);
            scores[7] += _score(self.board[i][2-i]);
        }
        for i in 0..8 {
            if scores[i] == 3 { return Some(Player::Orange); }
            if scores[i] == -3 { return Some(Player::Blue); }
        }
        None
    }

    pub fn show(&self) {
        let _ = self.hand_blue.iter().map(|c| print!("{}", c)).collect::<()>();
        println!("");
        println!("-------------");
        for i in 0..3 {
            for j in 0..3 {
                print!("| {} ", self.board[i][j]);
            }
            println!("|");
        }
        println!("-------------");
        let _ = self.hand_orange.iter().map(|c| print!("{}", c)).collect::<()>();
        println!("");
    }
}

fn _score(c: char) -> i8 {
    if ORANGE_PIECES.contains(&c) { 1 }
    else if BLUE_PIECES.contains(&c) { -1 }
    else { 0 }
}