pub mod mem;
pub mod cpu;

use self::mem::Chunk;

struct val_stack {
    data:Vec<Chunk>
}