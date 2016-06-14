
extern crate chilang;

use chilang::chi::vm::cpu::alu::*;
use chilang::chi::vm::cpu::*;
use chilang::chi::vm::mem::*;

#[test]
fn test_add() {
    let mut mem = ThreadMem::new();
    let mut l = Box::new(Chunk::new());
    l.push(STORE);
    l.push(1);
    l.push(STORE);
    l.push(2);
    l.push(I_ADD);
    execute(&mut mem, &l);
    let mut s = &mem.stack;
    assert_eq!(s.deep(), 1);
    assert_eq!(s.top(), 3);
}

#[test]
fn test_sub() {
    let mut mem = ThreadMem::new();
    let mut l = Box::new(Chunk::new());
    l.push(STORE);
    l.push(1);
    l.push(STORE);
    l.push(2);
    l.push(I_SUB);
    execute(&mut mem, &l);
    let mut s = &mem.stack;
    assert_eq!(s.deep(), 1);
    assert_eq!(s.top(), -1);
}
