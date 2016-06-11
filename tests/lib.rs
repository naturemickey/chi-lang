
extern crate chilang;

use chilang::chi::vm::cpu::alu::*;
use chilang::chi::vm::cpu::*;
use chilang::chi::vm::mem::*;

#[test]
fn add() {
    let mut s = Stack::new();
    let mut l = Box::new(Chunk::new());
    l.push(STORE);
    l.push(1);
    l.push(STORE);
    l.push(2);
    l.push(I_ADD);
    execute(&mut s, &l);
    assert!(l.len() == 1);
}
