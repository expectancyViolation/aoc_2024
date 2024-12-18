use super::assembunny::{AssembunnyState, Instruction};

pub(crate) fn solve(data: &str) -> (String, String) {
    //         let data = "cpy 41 a
    // inc a
    // inc a
    // dec a
    // jnz a 2
    // dec a";
    let instructions: Vec<_> = data.lines().map(|l| Instruction::parse(l)).collect();
    let mut state = AssembunnyState::new(instructions.as_slice());
    while state.step_instruction().is_ok() {};
    let p1 = state.registers[0];
    let mut state = AssembunnyState::new(instructions.as_slice());
    state.registers[2] = 1;
    while state.step_instruction().is_ok() {};
    let p2 = state.registers[0];
    (p1.to_string(), p2.to_string())
}