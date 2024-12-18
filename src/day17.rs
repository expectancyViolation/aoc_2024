use regex::Regex;

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Hash, Default)]
struct D17ProgramState {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    inst: usize,
}

impl D17ProgramState {
    fn yield_(&mut self, instructions: &Vec<i64>) -> Option<i64> {
        while self.inst < instructions.len() {
            let op = instructions[self.inst + 1];
            let co = match op {
                x @ 0..=3 => x,
                4 => self.reg_a,
                5 => self.reg_b,
                6 => self.reg_c,
                _ => 0,
            };
            let curr_i = instructions[self.inst];
            self.inst += 2;
            match curr_i {
                0 => {
                    self.reg_a >>= co;
                    // co is always 3 here, else the solution would not work
                    // (i.e. we always have to feed 3 bytes to stay ahead
                }
                1 => {
                    self.reg_b ^= op;
                }
                2 => {
                    self.reg_b = co % 8;
                }
                3 => match self.reg_a {
                    0 => {}
                    _ => self.inst = op as usize,
                },
                4 => {
                    self.reg_b ^= self.reg_c;
                }
                5 => {
                    return Some(co % 8);
                }
                6 => {
                    self.reg_b = self.reg_a >> co;
                }
                7 => {
                    self.reg_c = self.reg_a >> co;
                }
                _ => unreachable!(),
            }
        }
        None
    }
}

pub(crate) fn solve(data: &str) -> (i64, i64) {
//     let data="Register A: 12345678
// Register B: 0
// Register C: 0
//
// Program: 2,4,1,0,7,5,1,5,0,3,4,5,5,5,3,0 ";
    let reg = Regex::new(r"(-?\d+)").unwrap();
    let nums = reg
        .find_iter(data)
        .map(|x| x.as_str().parse().unwrap())
        .collect::<Vec<i64>>();

    let instructions = nums[3..].to_vec();
    let mut p1_prog = D17ProgramState {
        reg_a: nums[0],
        reg_b: nums[1],
        reg_c: nums[2],
        inst: 0,
    };
    // encode p1 as digits of decimal number
    let mut p1 = 0;
    while let Some(o) = p1_prog.yield_(&instructions) {
        p1 = 10 * p1 + o;
    }

    // 7 bits "lookahead" seems to be enough
    let lookahead_bits = 7;
    let mut candidates = (0..(1 << lookahead_bits))
        .map(|x| {
            (
                D17ProgramState {
                    reg_a: x,
                    ..Default::default()
                },
                x,
            )
        })
        .collect::<Vec<(D17ProgramState, i64)>>();
    for i in 0..instructions.len() {
        let mut nc = Vec::new();
        for (p_state, a_reg) in candidates.into_iter() {
            for feed in 0..(1 << 3) {
                let mut n_cand = p_state.clone();
                // each "out" instruction seems to match a single "adv 3" i.e. "discard 3 bits"
                n_cand.reg_a += feed << lookahead_bits;
                n_cand.yield_(&instructions).map(
                    |x| {
                        if x == instructions[i] {
                            nc.push((n_cand, a_reg + (feed << (lookahead_bits + 3 * i))));
                        }
                    });
            }
        }
        candidates = nc;
    }
    let p2 = candidates.iter_mut().filter_map(|(ref mut state, a_reg)| {
        if state.yield_(&instructions).is_none() {
            Some(*a_reg)
        } else {
            None
        }
    }).min().unwrap();
    (p1, p2)
}
