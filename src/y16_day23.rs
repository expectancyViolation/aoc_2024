use crate::y16_assembunny::{AssembunnyError, AssembunnyState, Instruction};


pub(crate) fn solve(data: &str) -> (i64, i64) {
    let instructions: Vec<_> = data.lines().map(|l| Instruction::parse(l)).collect();
    let mut state = AssembunnyState::new(instructions.as_slice());
    state.registers[0] = 7;
    while !state.step_instruction().is_err_and(|e| e == AssembunnyError::OutOfBounds) {
        // println!("-------------------");
        // println!("Stepped at {:?}", stepped);
        // println!("{:?}", state.registers);
        // println!("{:?}", state.instruction_index);
        // for inst in &state.instructions {
        //     println!("{:?}", inst);
    };
    let p1 = state.registers[0];

    let mut state = AssembunnyState::new(instructions.as_slice());
    state.registers[0] = 12;
    while !state.step_instruction().is_err_and(|e| e == AssembunnyError::OutOfBounds) {
    };
    let p2 = state.registers[0];
    (p1, p2)
}