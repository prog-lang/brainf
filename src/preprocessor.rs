#![allow(dead_code)]

use std::{
    io, io::prelude::*,
    fs::File,
};

use crate::shared::Instruction;


pub type Error = String;

pub struct Preprocessor {
    bytes: Vec<u8>,
    stack: Vec<usize>,
    instructions: Vec<Instruction>,
}

// Public interface.
impl Preprocessor {
    pub fn new() -> Self {
        Preprocessor {
            bytes: Vec::new(),
            stack: Vec::new(),
            instructions: Vec::new(),
        }
    }

    /// Returns `Preprocessor` instance with `bytes` passed through the function call.
    pub fn with_bytes(bytes: Vec<u8>) -> Self {
        Preprocessor {
            bytes: Vec::from(bytes),
            stack: Vec::new(),
            instructions: Vec::new(),
        }
    }

    /// Returns `Preprocessor` instance with `bytes` set by reading the file specified by `path`.
    /// You will most likely use this constructor to generate instances of `Preprocessor`.
    ///
    /// # Arguments
    ///
    /// * `path: &String` - Path to the source code file.
    ///
    /// # Examples
    ///
    /// This is the most common way to preprocess your Brainf code.
    ///
    /// ```
    /// fn run() -> Result<(), Error> {
    ///     let path = String::from("brainf/examples/hello-world.bf");
    ///     let mut pre = Preprocessor::read(&path);
    ///     let result = pre.process()?;
    ///     // do something with the result
    ///     Ok(())
    /// }
    /// ```
    pub fn read(path: &String) -> io::Result<Self> {
        let mut pre = Preprocessor::new();
        let mut file = File::open(path)?;
        file.read_to_end(&mut pre.bytes)?;
        Ok(pre)
    }

    /// # Examples
    ///
    /// This is the most common way to preprocess your Brainf code.
    ///
    /// ```
    /// fn run() -> Result<(), Error> {
    ///     let path = String::from("brainf/examples/hello-world.bf");
    ///     let mut pre = Preprocessor::read(&path);
    ///     let result = pre.process()?;
    ///     // do something with the result
    ///     Ok(())
    /// }
    /// ```
    pub fn process(&mut self) -> Result<(), Error> {
        // I had to do this through indices because Rust didn't allow me to pass self as mutable to
        // match_byte within a loop that borrows self as immutable due to .iter().
        for index in 0..self.bytes.len() {
            self.match_byte(self.bytes[index])?;
        }

        if self.stack.is_empty() { return Ok(()); }
        // The .unwrap() here is justified as we've checked that stack is not empty.
        Err(format!("bracket at position {} does not have a pair", self.stack.pop().unwrap()))
    }
}

impl Preprocessor {
    fn match_byte(&mut self, byte: u8) -> Result<(), Error> {
        match byte {
            b'>' => self.instructions.push(Instruction::Right),
            b'<' => self.instructions.push(Instruction::Left),
            b'+' => self.instructions.push(Instruction::Increment),
            b'-' => self.instructions.push(Instruction::Decrement),
            b'.' => self.instructions.push(Instruction::Output),
            b',' => self.instructions.push(Instruction::Input),
            b'[' => self.jump(),
            b']' => self.back()?,
            _ => (),
        }
        Ok(())
    }

    fn jump(&mut self) {
        self.stack.push(self.instructions.len());
        self.instructions.push(Instruction::Jump(None));
    }

    fn back(&mut self) -> Result<(), Error> {
        let jump_index;
        let top = self.stack.pop();
        match top {
            None => return Err("brackets mismatched".to_string()),
            Some(index) => jump_index = index,
        }

        let jump_instruction = &self.instructions[jump_index];
        if let &Instruction::Jump(_) = jump_instruction {
            self.instructions[jump_index] = Instruction::Jump(Some(self.instructions.len()));
            self.instructions.push(Instruction::Back(Some(jump_index)));
        } else {
            return Err("back instruction has an invalid matching jump".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod preprocessor_tests {
    use super::*;
    use Instruction::*;

    #[test]
    fn preprocess_good_code() -> Result<(), Error> {
        //                     0 1 2 3 4 5 6 7
        let source: Vec<u8> = "+ [ > + [ . ] ]".as_bytes().to_vec();

        let expect = vec![
            Increment,
            Jump(Some(7)),
            Right,
            Increment,
            Jump(Some(6)),
            Output,
            Back(Some(4)),
            Back(Some(1)),
        ];

        let mut pre = Preprocessor::with_bytes(source);
        pre.process()?;
        assert_eq!(expect, pre.instructions);

        Ok(())
    }

    #[test]
    fn preprocess_code_with_mismatched_brackets() -> Result<(), Error> {
        //                     0 1 2 3 4 5 6 7 8
        let source: Vec<u8> = "+ [ > + [ . [ ] ]".as_bytes().to_vec();
        //                       ^ This bracket does not have a pair :(

        let mut pre = Preprocessor::with_bytes(source);
        assert!(pre.process().is_err(), "expected Err");

        //                     0 1 2 3 4 5 6 7 8
        let source: Vec<u8> = "+ > [ . ] [ + ] ]".as_bytes().to_vec();
        //                                     ^ This bracket is sad and lonely :(

        let mut pre = Preprocessor::with_bytes(source);
        assert!(pre.process().is_err(), "expected Err");
        Ok(())
    }

    #[test]
    fn preprocess_code_with_comments() -> Result<(), Error> {
        let source = "\
            [(0)                                                                           (1)   (2)
                An area like this can be used at the top to write some text that uses commas, dots,
                and other Brainf command symbols.(3) They'll be ignored here since the first command
                is a JUMP down to the end of this text.(4) However,(5) (6)[brackets have to be
                balanced here](7),(8) since otherwise,(9) preprocessor is going to invalidate the
                program.(10)
            ](11)

            12 13 14 15 16 17 18 19
            +  [  >  +  [  .  ]  ]  \

            This example simply demonstrates that any text is ignored except for the Brainf command
            symbols which we aren't going to use down here :)\
            ".as_bytes().to_vec();

        let expect = vec![
            Jump(Some(11)),
                Input, Input,
                Output,
                Output, Input, Jump(Some(7)),
                Back(Some(6)), Input, Input, Output,
            Back(Some(0)),

            Increment, Jump(Some(19)),
                Right, Increment, Jump(Some(18)),
                    Output,
                Back(Some(16)),
            Back(Some(13)),
        ];

        let mut pre = Preprocessor::with_bytes(source);
        pre.process()?;
        assert_eq!(expect, pre.instructions);

        Ok(())
    }
}
