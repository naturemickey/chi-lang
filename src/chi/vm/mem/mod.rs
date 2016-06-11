
use std::collections::HashMap;

pub type Chunk = Vec<i64>;

pub struct Context {
    lambda_map:HashMap<String, Box<Chunk>>
}

pub struct Stack {
    data:Chunk
}

impl Stack {
    pub fn new() -> Stack {
        Stack {data : Chunk::new()}
    }
    pub fn push(&mut self, v:i64) {
        self.data.push(v)
    }
    pub fn pop(&mut self) -> i64 {
        let len = self.data.len();
        self.data.remove(len - 1)
    }
    pub fn top(&mut self) -> i64 {
        let len = self.data.len();
        self.data[len - 1]
    }
}