#![macro_use] extern crate text_io;
extern crate clap;

mod shared;
mod preprocessor;
mod tape;
mod vm;
mod util;

use clap::{Arg, App};

use shared::*;
use vm::VM;
use crate::preprocessor::Preprocessor;
use crate::util::exit_with_error;


fn main() -> Result<(), Error> {
    let matches = App::new("Brainfuck Interpreter")
        .version("0.1.0")
        .author("Viktor A. Rozenko Voitenko <sharp.vik@gmail.com>")
        .about("Executes Brainf code")
        .arg(Arg::with_name("SOURCE")
            .help("Path to Brainf source code")
            .required(true)
            .index(1))
        .get_matches();

    // Unwrap is justified here. If it fails, Clap will show a helpful error message.
    let source = matches.value_of("SOURCE").unwrap();

    let pre = Preprocessor::read(source);
    if let Ok(mut preprocessor) = pre {
        preprocessor.process()?;
        VM::init(preprocessor.instructions).boot()?;
    } else {
        exit_with_error("Failed to read source");
    }
    Ok(())
}
