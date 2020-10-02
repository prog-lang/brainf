#[derive(PartialEq, Debug)]
pub enum Instruction {
    Right, Left,
    Increment, Decrement,
    Output, Input,
    Jump(Option<usize>),
    Back(Option<usize>),
}
