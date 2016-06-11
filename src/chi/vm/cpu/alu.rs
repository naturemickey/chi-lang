use chi::vm::cpu::*;
use chi::vm::mem::*;

pub fn execute(stack:&mut Stack, lambda:&Box<Chunk>, sp:&mut usize) {
    let command = lambda[*sp];
    *sp += 1;
    match command & FFFFFFFF {
        I_ADD => {
            let a = stack.pop();
            let b = stack.pop();
            stack.push(a + b);
        }
        I_SUB => {
            let a = stack.pop();
            let b = stack.pop();
            stack.push(b - a);
        }
        I_MUT => {
            let a = stack.pop();
            let b = stack.pop();
            stack.push(a * b);
        }
        I_DIV => {
            let a = stack.pop();
            let b = stack.pop();
            stack.push(b / a);
        }
        F_ADD => {
            let a = stack.pop() as f64;
            let b = stack.pop() as f64;
            stack.push((a + b) as i64);
        }
        F_SUB => {
            let a = stack.pop() as f64;
            let b = stack.pop() as f64;
            stack.push((b - a) as i64);
        }
        F_MUT => {
            let a = stack.pop() as f64;
            let b = stack.pop() as f64;
            stack.push((a * b) as i64);
        }
        F_DIV => {
            let a = stack.pop() as f64;
            let b = stack.pop() as f64;
            stack.push((b / a) as i64);
        }
        STORE => {
            stack.push(lambda[*sp]);
            *sp += 1;
        }
        JUMP_IF_IS_ZERO => {
            let a = stack.pop();
            if a == 0 {
                *sp = (command >> 32) as usize;
            }
        }
        JUMP_IF_LESS_THEN_ZERO => {
            let a = stack.pop();
            if a < 0 {
                *sp = (command >> 32) as usize;
            }
        }
        JUMP_IF_LESS_OR_EQ_ZERO => {
            let a = stack.pop();
            if a <= 0 {
                *sp = (command >> 32) as usize;
            }
        }
        _ => {
            panic!("指令错误:{}", command);
        }
    }
}
