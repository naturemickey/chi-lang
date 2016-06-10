
use std::collections::HashMap;

pub type Chunk = Vec<i64>;

pub struct Context {
    lambda_map:HashMap<String, Box<Chunk>>
}

pub struct Stack {
    data:Vec<i64>
}

impl Stack {
    pub fn new() -> Stack {
        Stack {data : Vec::new()}
    }
}