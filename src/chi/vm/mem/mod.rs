
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
    pub fn top(&self) -> i64 {
        let len = self.data.len();
        self.data[len - 1]
    }
    pub fn deep(&self) -> usize {
        self.data.len()
    }
}

struct val_stack {
    data:Vec<Chunk>
}

impl val_stack {
    pub fn new() -> val_stack {
        val_stack {data : Vec::new()}
    }
    pub fn prepare_call(&mut self, capacity:usize) {
        self.data.push(vec![0; capacity]);
    }
    pub fn get(&self, idx:usize) -> i64 {
        let len = self.data.len();
        let lastChunk = &self.data[len - 1];
        lastChunk[idx]
    }
    pub fn set(&mut self, idx:usize, v:i64) {
        let len = self.data.len();
        let mut lastChunk = &mut self.data[len - 1];
        lastChunk[idx] = v;
    }
    pub fn before_return(&mut self) {
        let len = self.data.len();
        self.data.remove(len);
    }
}