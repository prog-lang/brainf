extern crate colored;

use std::process;

use colored::*;


pub fn exit_with_error(error: &str) {
    println!("{} {}", "error:".red().bold(), error);
    process::exit(1);
}
