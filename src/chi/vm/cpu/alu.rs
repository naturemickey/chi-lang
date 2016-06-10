use chi::vm::cpu::*;
use chi::vm::mem::Chunk;
use chi::vm::mem::Stack;

pub fn execute(stack:&Stack, lambda:&Box<Chunk>, sp:&usize) {
    let a = lambda[*sp];
    match (a & FFFFFFFF) {
        ADD => {
            
        }
        SUB => {
            
        }
        MUT => {
            
        }
        DIV => {
            
        }
        _ => {
            
        }
    }
}
