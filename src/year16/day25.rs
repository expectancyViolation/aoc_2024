use super::assembunny::{AssembunnyState, Instruction};


pub(crate) fn solve(data: &str) -> (String, String) {
    let instructions: Vec<_> = data.lines().map(|l| Instruction::parse(l)).collect();
    let res = (0..).filter_map(|n| {
        let mut state = AssembunnyState::new(instructions.as_slice());
        state.registers[0] = n;
        let mut clock_signal = vec![0; 0];
        while clock_signal.len() < 10 {
            match state.step_instruction() {
                Ok(Some(val)) => clock_signal.push(val),
                Ok(None) => (),
                _ => unreachable!()
            }
        };
        let is_ok = clock_signal.iter().enumerate().all(|(i, &val)| {
            if i % 2 == 1 { val == 1 } else { val == 0 }
        });
        Some(n).filter(|_| is_ok)
    }).next().unwrap();
    (res.to_string(), String::from(""))
}