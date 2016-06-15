
use std::collections::HashMap;

pub type Chunk = Vec<i64>;

pub struct Context {
    lambda_map:HashMap<String, Box<Chunk>>
}

pub struct Data_stack {
    data:Chunk
}

impl Data_stack {
    pub fn new() -> Data_stack {
        Data_stack {data : Chunk::new()}
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
    pub fn get(&self, idx:usize) -> i64 {
        self.data[idx]
    }
    pub fn set(&mut self, idx:usize, v:i64) {
        self.data[idx] = v;
    }
}

pub struct ThreadMem {
    pub stack: Data_stack,
    pub sp_stack : Vec<usize>
}

impl ThreadMem {
    pub fn new() -> ThreadMem {
        ThreadMem {stack: Data_stack::new(), sp_stack: vec![]}
    }
}