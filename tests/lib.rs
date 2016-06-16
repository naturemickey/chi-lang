
extern crate chilang;

use chilang::chi::vm::cpu::alu::*;
use chilang::chi::vm::cpu::*;
use chilang::chi::vm::mem::*;

#[test]
fn test_add() {
    let mut l = Chunk::new();
    l.push(0);
    l.push(STORE);
    l.push(1);
    l.push(STORE);
    l.push(2);
    l.push(I_ADD);
    let mut mem = ThreadMem::new(&mut l);
    execute(&mut mem);
    let mut s = &mem.stack;
    assert_eq!(s.deep(), 1);
    assert_eq!(s.top(), 3);
}

#[test]
fn test_sub() {
    let mut l = Chunk::new();
    l.push(0);
    l.push(STORE);
    l.push(1);
    l.push(STORE);
    l.push(2);
    l.push(I_SUB);
    let mut mem = ThreadMem::new(&mut l);
    execute(&mut mem);
    let mut s = &mem.stack;
    assert_eq!(s.deep(), 1);
    assert_eq!(s.top(), -1);
}
