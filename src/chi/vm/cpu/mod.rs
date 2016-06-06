pub mod alu;

const FFFFFFFF : i64 = 0xFFFFFFFF;

const ADD : i64 = 1;
const SUB : i64 = 2;
const MUT : i64 = 3;
const DIV : i64 = 4;

const STORE : i64 = 5;

const JUMP_IF_IS_ZERO : i64 = 6;
const JUMP_IF_LESS_THEN_ZERO : i64 = 7;
const JUMP_IF_LESS_OR_EQ_ZERO : i64 = 8;
