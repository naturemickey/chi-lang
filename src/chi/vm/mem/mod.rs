
use std::collections::HashMap;

pub type Chunk = Vec<i64>;

pub struct Context {
    lambda_map:HashMap<String, Chunk>
}
