
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

struct Val_stack {
    data:Vec<Chunk>
}

impl Val_stack {
    pub fn new() -> Val_stack {
        Val_stack {data : Vec::new()}
    }
    pub fn prepare_call(&mut self, capacity:usize) {
        self.data.push(vec![0; capacity]);
    }
    pub fn get(&self, idx:usize) -> i64 {
        let len = self.data.len();
        let last_chunk = &self.data[len - 1];
        last_chunk[idx]
    }
    pub fn set(&mut self, idx:usize, v:i64) {
        let len = self.data.len();
        let mut last_chunk = &mut self.data[len - 1];
        last_chunk[idx] = v;
    }
    pub fn before_return(&mut self) {
        let len = self.data.len();
        self.data.remove(len);
    }
}

pub struct ThreadMem {
    stack: Stack,
    val_s: Val_stack
}

impl ThreadMem {
    pub fn new() -> ThreadMem {
        ThreadMem {stack: Stack::new(), val_s: Val_stack::new()}
    }
}