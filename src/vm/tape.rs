#![allow(dead_code)]

use std::{
    fmt,
    io, io::Read,
    process,
    collections::VecDeque,
};


/// Tape represents machine's memory. It has an "infinite" memory tape that consists of Cells
/// and a head. Head points to the currently active memory Cell.
pub struct Tape {
    cells: VecDeque<Cell>,
    head: usize,
}

impl Tape {
    pub fn new() -> Self {
        Tape {
            cells: VecDeque::from(vec![Cell::new()]),
            head: 0,
        }
    }

    /// Move head one cell to the right.
    pub fn right(&mut self) {
        self.head += 1;
        if self.head >= self.cells.len() {
            self.cells.push_back(Cell::new());
        }
    }

    /// Move head one cell to the left.
    pub fn left(&mut self) {
        if self.head == 0 {
            self.cells.push_front(Cell::new());
        } else {
            self.head -= 1;
        }
    }

    /// Increment current cell's value.
    pub fn increment(&mut self) {
        self.current_cell().increment();
    }

    /// Decrement current cell's value.
    pub fn decrement(&mut self) {
        self.current_cell().decrement();
    }

    /// Output current cell's value as an ASCII character.
    pub fn output(&mut self) {
        print!("{}", self.current_cell());
    }

    /// Read one byte from stdin and replace current cell's value with it.
    pub fn input(&mut self) {
        let mut buf: [u8; 1] = [127];
        if let Err(e) = io::stdin().lock().read_exact(&mut buf) {
            eprintln!("{}", e);
            process::exit(1);
        }
        self.current_cell().value = buf[0];
    }

    pub fn current_cell_is_zero(&self) -> bool {
        self.cells[self.head].value == 0
    }

    fn current_cell(&mut self) -> &mut Cell {
        &mut self.cells[self.head]
    }
}

#[cfg(test)]
mod tape_tests {
    use super::*;

    #[test]
    fn right() {
        let mut tape = Tape::new();
        tape.right();
        assert_eq!(1, tape.head);
        assert_eq!(2, tape.cells.len());
    }

    #[test]
    fn left() {
        let mut tape = Tape::new();
        tape.left();
        assert_eq!(0, tape.head);
        assert_eq!(2, tape.cells.len());
    }

    #[test]
    fn increment() {
        let mut tape = Tape::new();
        tape.increment();
        assert_eq!(0, tape.head);
        assert_eq!(1, tape.cells[tape.head].value);
    }

    #[test]
    fn decrement() {
        let mut tape = Tape::new();
        tape.decrement();
        assert_eq!(0, tape.head);
        assert_eq!(255, tape.cells[tape.head].value);
    }

    #[test]
    fn public_cell_is_zero() {
        let mut tape = Tape::new();
        assert!(tape.current_cell_is_zero());
        tape.increment();
        assert!(!tape.current_cell_is_zero());
    }
}


/// Cell represents a single memory cell in memory. Its value is a byte-long integer which is
/// capable of under- and overflowing on decrement and increment operations.
#[derive(Clone)]
struct Cell {
    value: u8,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value as char)
    }
}

impl Cell {
    pub fn new() -> Self {
        Cell { value: 0 }
    }

    pub fn increment(&mut self) {
        self.value = if self.value == 255 { 0 } else { self.value + 1 }
    }

    pub fn decrement(&mut self) {
        self.value = if self.value == 0 { 255 } else { self.value - 1 }
    }
}

#[cfg(test)]
mod cell_tests {
    use super::*;

    #[test]
    fn increment() {
        let mut cell = Cell::new();
        cell.increment();
        assert_eq!(1, cell.value);

        cell.value = 255;
        cell.increment();
        assert_eq!(0, cell.value);
    }

    #[test]
    fn decrement() {
        let mut cell = Cell::new();
        cell.decrement();
        assert_eq!(255, cell.value);

        cell.decrement();
        assert_eq!(254, cell.value);
    }
}
