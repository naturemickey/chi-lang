pub mod alu;

pub const FFFFFFFF : i64 = 0xFFFFFFFF;

pub const I_ADD                     : i64 =   1;
pub const I_SUB                     : i64 =   2;
pub const I_MUT                     : i64 =   3;
pub const I_DIV                     : i64 =   4;

pub const F_ADD                     : i64 =   9;
pub const F_SUB                     : i64 =  10;
pub const F_MUT                     : i64 =  11;
pub const F_DIV                     : i64 =  12;

pub const STORE                     : i64 =   5;

pub const JUMP_IF_IS_ZERO           : i64 =   6;
pub const JUMP_IF_LESS_THEN_ZERO    : i64 =   7;
pub const JUMP_IF_LESS_OR_EQ_ZERO   : i64 =   8;
