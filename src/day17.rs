use hashbrown::HashSet;
use regex::Regex;


fn run(program: &Vec<i128>, reg_a: i128, reg_b: i128, reg_c: i128, inst: usize, n_outputs: usize) -> (Vec<i128>, usize, usize) {
    let mut reg_a = reg_a;
    let mut reg_b = reg_b;
    let mut reg_c = reg_c;
    let mut inst = inst;
    let mut bits_discarded = 0;
    let mut max_bit_read = 0;

    let mut output = vec![];
    while inst < program.len() {
        let op = program[inst + 1];
        let co = match op {
            x @ 0..=3 => x,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => 0,
        };
        match program[inst] {
            0 => {
                reg_a /= (1 << co);
                assert_eq!(co, 3);
                max_bit_read = max_bit_read.max(bits_discarded + (co as usize));
                bits_discarded += 3;
                //println!("discarded3 {}", reg_a);
                //println!("AA{:?} {}", reg_a, (1 << co));
                inst += 2;
            }
            1 => {
                reg_b ^= op;
                inst += 2;
            }
            2 => {
                reg_b = co % 8;
                inst += 2;
            }
            3 => {
                match reg_a {
                    0 => { inst += 2; }
                    _ => { inst = op as usize }
                }
            }
            4 => {
                reg_b ^= reg_c;
                inst += 2;
            }
            5 => {
                output.push(co % 8);
                if output.len() == n_outputs {
                    return (output, bits_discarded, max_bit_read);
                }
                // println!("{:?}", output);
                inst += 2;
            }
            6 => {
                reg_b = reg_a / (1 << co);
                max_bit_read = max_bit_read.max(bits_discarded + 3 + (co as usize));
                inst += 2;
            }
            7 => {
                reg_c = reg_a / (1 << co);
                max_bit_read = max_bit_read.max(bits_discarded + 3 + (co as usize));
                //println!("read {}", co);
                inst += 2;
            }
            _ => unreachable!(),
        }
    }

    (output, bits_discarded, max_bit_read)
}

pub(crate) fn solve(data: &str) -> (i64, i64) {
    let data_ = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    let reg = Regex::new(r"(-?\d+)").unwrap();
    let nums = reg.find_iter(data).map(|x| x.as_str().parse().unwrap()).collect::<Vec<i128>>();

    let mut reg_a = nums[0];
    let mut reg_b = nums[1];
    let mut reg_c = nums[2];

    let mut inst = 0;

    let program = nums[3..].to_vec();
    println!("{:?}", program.len());

    // let co = |op: i128| -> i128{
    //     match op {
    //         x @ 0..=3 => x,
    //         4 => reg_a,
    //         5 => reg_b,
    //         6 => reg_c,
    //         _ => unreachable!(),
    //     }
    // };

    let mut candidates = vec![(0, 0)];
    let mut gotchas = HashSet::new();
    for i in 1..program.len() + 1 {
        let mut nc = HashSet::new();
        for &(low, discarded) in candidates.iter() {
            for reg_a in 1..(1 << (13)) {
                let reg_a = (reg_a << discarded) + low;
                let (output, bits_discarded, max_bit_read) = run(&program, reg_a, reg_b, reg_c, inst, i);
                if output == program[..output.len()] {
                    if output == program {
                        gotchas.insert(reg_a);
                    }
                    nc.insert((reg_a % (1 << bits_discarded), bits_discarded));
                }
            }
        }
        candidates = nc.into_iter().collect();
    }

    println!("GOTCHA:{}", gotchas.iter().min().unwrap());

    // let mut i = 0;
    // println!("{:?}", output);

    (0, 0)
}