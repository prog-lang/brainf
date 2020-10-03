#![allow(dead_code)]

extern crate idioma;

use idioma::*;
use crate::tape::Tape;
use crate::shared::{*, Instruction::*};


pub struct VM {
    memory: Tape,
    program: Vec<Instruction>,
    ip: usize,
}

impl VM {
    pub fn new() -> Self {
        VM {
            memory: Tape::new(),
            program: Vec::new(),
            ip: 0,
        }
    }

    /// Returns new instance of `VM` with `program` field specified by the parameter.
    pub fn init(program: Vec<Instruction>) -> Self {
        VM {
            memory: Tape::new(),
            program,
            ip: 0,
        }
    }

    /// Enters the main execution loop and returns an error if encountered during execution.
    pub fn boot(&mut self) -> Result<(), Error> {
        while self.ip < self.program.len() {
            self.exec()?;
        }
        Ok(())
    }
}

impl VM {
    fn exec(&mut self) -> Result<(), Error> {
        match self.program[self.ip] {
            Right => self.memory.right(),
            Left => self.memory.left(),
            Increment => self.memory.increment(),
            Decrement => self.memory.decrement(),
            Output => self.memory.output(),
            Input => self.memory.input(),
            Jump(pointer) => self.jump(pointer, |b| b)?,
            Back(pointer) => self.jump(pointer, |b| !b)?,
        }
        self.ip += 1;
        Ok(())
    }

    fn jump(&mut self, pointer: Option<usize>, direction: fn(bool) -> bool) -> Result<(), Error> {
        if let Some(jump_location) = pointer {
            if direction(self.memory.current_cell_is_zero()) {
                self.ip = jump_location;
            }
            Ok(())
        } else {
            Err(error("empty jump location pointer"))
        }
    }
}

#[cfg(test)]
mod vm_tests {
    use super::*;

    #[test]
    fn output() -> Result<(), Error>{
        let source = vec![Output];
        let mut vm = VM::init(source);
        for _ in 0..100 {
            vm.memory.increment();
        }
        vm.boot()?;
        assert!(!vm.memory.current_cell_is_zero());
        Ok(())
    }

    #[test]
    fn input() -> Result<(), Error>{
        let source = vec![Input];
        let mut vm = VM::init(source);
        vm.boot()?;
        Ok(())
    }

    #[test]
    fn jump() -> Result<(), Error> {
        let source = vec![
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Right,
            Increment,
            Increment,
            Increment,
            Jump(Some(107)), Left, Output, Right, Decrement, Back(Some(105))];
        let mut vm = VM::init(source);
        vm.boot()?;
        Ok(())
    }
}
