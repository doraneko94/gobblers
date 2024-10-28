use gobblers::board::BoardMap;

fn main() {
    let bm = BoardMap::from_board((1 << 12) + (1 << 17) + (1 << 24));
    bm.show();
    println!("{}", bm.result());
}
