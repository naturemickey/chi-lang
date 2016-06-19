
use std::collections::HashMap;

pub type Chunk = Vec<i64>;

pub struct Context {
    lambdas    : Vec<Chunk>,
    lambda_map : HashMap<String, usize>
}

impl Context {
    pub fn new() -> Context {
        Context {lambdas: vec![], lambda_map: HashMap::default()}
    }
    pub fn get_lambda(&self, idx: usize) -> &Chunk {
        unsafe { self.lambdas.get_unchecked(idx) }
    }
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

pub struct ThreadMem<'a> {
    pub stack    : Data_stack,
    pub sp_stack : Vec<usize>,
    pub sp_idx   : usize,
    pub lambda_stack : Vec<usize>,
    pub context      : &'a Context
}

impl <'a> ThreadMem<'a> {
    pub fn new(lbda_idx:usize, ctx:&Context) -> ThreadMem {
        ThreadMem {stack: Data_stack::new(), sp_stack: vec![1; 65536], sp_idx:0, lambda_stack:vec![lbda_idx], context : ctx }
    }
    pub fn get_lambda(&mut self) -> &Chunk {
        let lidx = self.lambda_stack.len() - 1;
        self.context.get_lambda(self.lambda_stack[lidx])
    }
}