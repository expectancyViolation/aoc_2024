use super::assembunny::AssembunnyError::{InvalidOperand, OutOfBounds};

#[derive(Clone, Debug, Copy)]
pub(crate) enum Operand {
    Number(i64),
    Register(usize),
}

impl Operand {
    fn parse(s: &str) -> Operand {
        match s.parse::<i64>() {
            Ok(x) => Operand::Number(x),
            _ => {
                let reg_index = (s.chars().next().unwrap() as usize) - ('a' as usize);
                Operand::Register(reg_index)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Instruction {
    CPY(Operand, Operand),
    INC(Operand),
    DEC(Operand),
    JNZ(Operand, Operand),
    TGL(Operand),
    OUT(Operand),
    MUL(Operand, Operand, Operand),
}

impl Instruction {
    pub fn parse(input: &str) -> Instruction {
        let parts: Vec<&str> = input.split(' ').collect();
        let ops = parts[1..].iter().map(|&s| Operand::parse(s)).collect::<Vec<_>>();
        match parts[0] {
            "cpy" => Instruction::CPY(ops[0], ops[1]),
            "inc" => Instruction::INC(ops[0]),
            "dec" => Instruction::DEC(ops[0]),
            "jnz" => Instruction::JNZ(ops[0], ops[1]),
            "tgl" => Instruction::TGL(ops[0]),
            "out" => Instruction::OUT(ops[0]),
            "mul" => Instruction::MUL(ops[0], ops[1], ops[2]),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssembunnyState {
    pub registers: [i64; 4],
    pub instruction_index: i64,
    pub instructions: Vec<Instruction>,
}


#[derive(Debug, Clone, PartialEq)]
pub(crate) enum AssembunnyError {
    OutOfBounds,
    InvalidOperand,
}

type ClockSignal = i64;
impl AssembunnyState {
    pub(crate) fn new(instructions: &[Instruction]) -> AssembunnyState {
        AssembunnyState { registers: [0; 4], instruction_index: 0, instructions: instructions.to_vec() }
    }
    fn get_operand(&self, operand: &Operand) -> i64 {
        match operand {
            Operand::Number(x) => *x,
            Operand::Register(r) => self.registers[*r],
        }
    }

    fn set_operand(&mut self, operand: &Operand, value: i64) -> Result<(), AssembunnyError> {
        match operand {
            Operand::Number(_) => Err(InvalidOperand),
            Operand::Register(r) => {
                self.registers[*r] = value;
                Ok(())
            }
        }
    }


    pub(crate) fn step_instruction(&mut self) -> Result<Option<ClockSignal>, AssembunnyError> {
        let instruction = &self.instructions.get(self.instruction_index as usize).cloned();
        if let Some(instruction) = instruction {
            match instruction {
                Instruction::CPY(operand1, operand2) => {
                    self.instruction_index += 1;
                    let copy_val = self.get_operand(operand1);
                    self.set_operand(operand2, copy_val).map(|()| None)
                }
                Instruction::INC(operand) => {
                    self.instruction_index += 1;
                    self.set_operand(operand, self.get_operand(operand) + 1).map(|()| None)
                }
                Instruction::DEC(operand) => {
                    self.instruction_index += 1;
                    self.set_operand(operand, self.get_operand(operand) - 1).map(|()| None)
                }
                Instruction::JNZ(operand1, operand2) => {
                    let condition = self.get_operand(operand1);
                    if condition != 0 {
                        let jump_dist = self.get_operand(operand2);
                        self.instruction_index += jump_dist;
                    } else { self.instruction_index += 1 }
                    Ok(None)
                }
                Instruction::OUT(operand) => {
                    self.instruction_index += 1;
                    let out_val = self.get_operand(operand);
                    Ok(Some(out_val))
                }
                Instruction::MUL(operand1, operand2, operand3) => {
                    self.instruction_index += 1;
                    let mul_val = self.get_operand(operand1);
                    let op_val = self.get_operand(operand2);
                    self.set_operand(operand3, op_val * mul_val).map(|()| None)
                }
                Instruction::TGL(operand) => {
                    let target_instruction_index = self.instruction_index + self.get_operand(operand);
                    let target_instruction = self.instructions.get(target_instruction_index as usize).cloned();
                    self.instruction_index += 1;
                    target_instruction.map(|inst| {
                        let toggled = match inst {
                            Instruction::CPY(op1, op2) => Instruction::JNZ(op1, op2),
                            Instruction::JNZ(op1, op2) => Instruction::CPY(op1, op2),
                            Instruction::TGL(op) => Instruction::INC(op),
                            Instruction::INC(op) => Instruction::DEC(op),
                            Instruction::DEC(op) => Instruction::INC(op),
                            Instruction::OUT(op) => Instruction::INC(op),
                            _ => unreachable!()
                        };
                        self.instructions[target_instruction_index as usize] = toggled;
                    }).map(|()| None).ok_or(InvalidOperand)
                }
            }
        } else { Err(OutOfBounds) }
    }
}
