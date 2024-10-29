#[derive(Clone, Copy)]
pub struct Trit3 {
    pub data: [u8; 3]
}

impl Trit3 {
    pub fn new() -> Self {
        Self { data: [0; 3] }
    }
    pub fn from_bit(bit: u64) -> Self {
        let mut trit = Trit3::new();
        let mut c = 1;
        for i in 0..3 {
            trit.data[i] = (bit / c % 3) as u8;
            c *= 3;
        }
        trit
    }
    pub fn to_bit(&self) -> u64 {
        (self.data[2] * 9 + self.data[1] * 3 + self.data[0]) as u64
    }
    pub fn max_empty(&self) -> usize {
        for i in 0..3 {
            if self.data[i] == 0 { return i; }
        }
        3
    }
}