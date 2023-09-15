use crate::stack::LoopType::{Anchor, Piece, Reversed, Rotate90, Square};

#[derive(Default)]
pub struct SolveStack {
    items: Vec<StackItem>,
    reset_target_index: Option<usize>,
}

#[derive(Clone)]
pub struct StackItem {
    loop_type: LoopType,
    index: usize,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum LoopType {
    Anchor,
    Piece,
    Reversed,
    Square,
    Rotate90,
}

impl SolveStack {
    pub fn new() -> SolveStack {
        SolveStack {
            items: Vec::new(),
            reset_target_index: None,
        }
    }

    pub fn push(&mut self, loop_type: LoopType, index: usize) {
        self.items.push(StackItem { loop_type, index });
    }

    pub fn pop(&mut self) -> Result<(), String> {
        let items = self.items.pop();
        match items {
            Some(_) => Ok(()),
            None => Err("Stack is empty".to_string()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn print(&self) {
        for item in self.items.iter() {
            match item.loop_type {
                Anchor => println!("Anchor: {}", item.index),
                Piece => println!("Piece: {}", item.index),
                Reversed => println!("Reversed: {}", item.index),
                Square => println!("Square: {}", item.index),
                Rotate90 => println!("Rotate90: {}", item.index),
            }
        }
    }

    fn current(&self) -> Option<&StackItem> {
        self.items.last()
    }

    pub fn latest_type(&self) -> Option<LoopType> {
        self.current().map(|item| item.loop_type.clone())
    }

    pub fn current_index(&self) -> Option<usize> {
        self.current().map(|item| item.index)
    }

    pub fn latest_index_of(&self, loop_type: LoopType) -> Option<usize> {
        for item in self.items.iter().rev() {
            if item.loop_type == loop_type {
                return Some(item.index);
            }
        }
        None
    }

    pub fn latest_nth_index_of(&self, index: usize, loop_type: LoopType) -> Option<usize> {
        let mut count = 0;
        for item in self.items.iter().rev() {
            if item.loop_type == loop_type {
                count += 1;
                if count == index {
                    return Some(item.index);
                }
            }
        }
        None
    }

    pub fn set_latest_index_of(&mut self, loop_type: LoopType, index: usize) {
        for item in self.items.iter_mut().rev() {
            if item.loop_type == loop_type {
                item.index = index;
                return;
            }
        }
    }

    pub fn set_reset_target_stack_index(&mut self, index: usize) {
        self.reset_target_index = Some(index);
    }

    pub fn clear_reset_target_stack_index(&mut self) {
        self.reset_target_index = None;
    }

    pub fn target_stack_index(&self) -> Option<usize> {
        self.reset_target_index
    }

    pub fn latest_stack_index_of(&self, loop_type: LoopType) -> Option<usize> {
        for (i, item) in self.items.iter().rev().enumerate() {
            if item.loop_type == loop_type {
                return Some(self.items.len() - i - 1);
            }
        }
        None
    }
}
