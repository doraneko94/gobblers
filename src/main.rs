use gobblers::bit::Bit;
use gobblers::board::{self, BoardMap};
use gobblers::enums::{Piece, Player};
use gobblers::tree::Node;
use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::rc::Rc;

fn main() {
    //let mut root = Node::new(Bit::new());
    //root.expand();
    //println!("{:?}", root.children.borrow().len());
    /*let bit = Bit::new()
        .put(0, 0, Player::Blue, Piece::Middle).put(0, 0, Player::Orange, Piece::Large)
        .put(0, 1, Player::Blue, Piece::Middle).put(0, 1, Player::Orange, Piece::Large)
        .put(0, 2, Player::Orange, Piece::Middle).put(0, 2, Player::Blue, Piece::Large)
        .put(1, 0, Player::Orange, Piece::Small).put(1, 0, Player::Blue, Piece::Large)
        .put(1, 1, Player::Blue, Piece::Small).put(1, 1, Player::Orange, Piece::Large)
        .put(1, 2, Player::Blue, Piece::Small).put(1, 2, Player::Orange, Piece::Middle)
        .put(2, 0, Player::Blue, Piece::Small).put(2, 0, Player::Orange, Piece::Middle)
        .put(2, 1, Player::Orange, Piece::Small).put(2, 1, Player::Blue, Piece::Large)
        .put(2, 2, Player::Orange, Piece::Small).put(2, 2, Player::Blue, Piece::Middle);
    let board = BoardMap::from_bit(bit);
    board.show();
    */let bit = Bit::new();
    let root = Rc::new(Node::new(bit, Player::Orange));
    let mut q = VecDeque::new();
    q.push_back(root);
    while q.len() > 0 {
        println!("{}", q.len());
        let node = q.pop_front().unwrap();
        node.expand();
        for child in node.children.borrow().iter() {
            q.push_back(child.clone());
        }
    }
}
