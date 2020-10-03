#![macro_use] extern crate text_io;
extern crate clap;
extern crate idioma;

mod shared;
mod preprocessor;
mod tape;
mod vm;

use clap::{Arg, App};

use vm::VM;
use crate::preprocessor::Preprocessor;
use idioma::*;


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

    let mut pre = exit_if_error(Preprocessor::read(source))?;
    exit_if_error(pre.process())?;
    exit_if_error(VM::init(pre.instructions).boot())?;
    Ok(())
}
