pub mod alu;

pub const FFFFFFFF : i64 = 0xFFFFFFFF;

pub const ADD : i64 = 1;
pub const SUB : i64 = 2;
pub const MUT : i64 = 3;
pub const DIV : i64 = 4;

pub const STORE : i64 = 5;

pub const JUMP_IF_IS_ZERO : i64 = 6;
pub const JUMP_IF_LESS_THEN_ZERO : i64 = 7;
pub const JUMP_IF_LESS_OR_EQ_ZERO : i64 = 8;
