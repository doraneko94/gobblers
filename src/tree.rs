use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::bit::Bit;
use crate::board::BoardMap;
use crate::enums::Player;

#[derive(Clone)]
pub struct Node {
    pub bit: Bit,
    pub end: RefCell<bool>,
    pub winner: RefCell<Option<Player>>,
    next: Player,
    pub children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

impl Node {
    pub fn new(bit: Bit, next: Player) -> Self {
        Self {
            bit: bit,
            end: RefCell::new(false),
            winner: RefCell::new(None),
            next: next,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new())
        }
    }
    pub fn set_parent(&self, parent: &Rc<Node>) {
        *self.parent.borrow_mut() = Rc::downgrade(parent);
    }
    fn is_duplicated(&self, bit: Bit) -> bool {
        match self.parent.borrow().upgrade() {
            Some(parent) => {
                (bit == parent.bit) || (parent.is_duplicated(bit))
            }
            None => { false }
        }
    }
    pub fn expand(&self) {
        if self.is_duplicated(self.bit) {
            println!("D");
            *self.end.borrow_mut() = true;
            return;
        }
        match BoardMap::from_bit(self.bit).result() {
            Some(player) => {
                *self.end.borrow_mut() = true;
                *self.winner.borrow_mut() = Some(player);
            }
            None => {
                let set = self.bit.next(self.next);
                let rc = Rc::new(self.clone());
                for &bit in set.iter() {
                    let child = Rc::new(Node::new(bit, !self.next));
                    child.set_parent(&rc);
                    self.children.borrow_mut().push(child);
                }
            }
        }
    }
}