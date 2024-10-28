use crate::trit::Trit3;

const ORANGE_PIECES: [char; 3] = ['O', 'o', '.'];
const BLUE_PIECES: [char; 3] = ['X', 'x', ','];

pub struct BoardMap {
    pub bit: u64,
    pub board: [[char; 3]; 3],
    pub hand_orange: Vec<char>,
    pub hand_blue: Vec<char>,
}

impl BoardMap {
    pub fn new() -> Self {
        Self {
            bit: 0,
            board: [[' '; 3]; 3],
            hand_orange: Vec::with_capacity(9),
            hand_blue: Vec::with_capacity(9) }
    }

    pub fn from_board(bit: u64) -> BoardMap {
        let mut bm = BoardMap::new();
        for i in 0..9 {
            let b = bit >> (52 - i*5) & 31;
            let trit = Trit3::from_bit(b);
            println!("{}, {:?}", b, trit.data);
            for j in 0..3 {
                let (y, x) = ((i/3) as usize, (i%3) as usize);
                bm.board[y][x] = match trit.data[2-j] {
                    2 => ORANGE_PIECES[2-j],
                    1 => BLUE_PIECES[2-j],
                    _ => bm.board[y][x],
                };
            }
        }
        for i in 0..3 {
            for _ in 0..(3 - (bit >> 10 - i * 2 & 3)) {
                bm.hand_orange.push(ORANGE_PIECES[i]);
            }
            for _ in 0..(3 - (bit >> 5 - i * 2 & 3)) {
                bm.hand_blue.push(BLUE_PIECES[i]);
            }
        }
        bm
    }

    pub fn put(&self, piece: usize, y: usize, x: usize) -> u64 {
        let i = y * 3 + x;
        let bit = self.bit >> (52 - i*5) & 31;
        let trit = Trit3::from_bit(bit);
        if piece >= 3 {
            
        }
        0
    }

    pub fn result(&self) -> i8 {
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
            if scores[i] == 3 { return 1; }
            if scores[i] == -3 { return -1; }
        }
        0
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