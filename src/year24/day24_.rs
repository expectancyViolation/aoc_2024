use crate::year24::day24_::Value::Undetermined;
use hashbrown::HashMap;
use itertools::Itertools;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use std::ops::{BitAnd, BitOr, BitXor};

#[derive(Debug, Clone, PartialEq, Copy, Ord, PartialOrd, Eq)]
enum Day24Operand {
    OR,
    AND,
    XOR,
}

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq)]
struct Day24Gate {
    arg1: usize,
    arg2: usize,
    operand: Day24Operand,
}

impl Day24Gate {
    fn new() -> Day24Gate {
        Day24Gate {
            arg1: usize::MAX,
            arg2: usize::MAX,
            operand: Day24Operand::OR,
        }
    }
}

impl Day24Gate {
    pub fn apply(&self, bit1: Value, bit2: Value) -> Value {
        match self.operand {
            Day24Operand::OR => bit1 | bit2,
            Day24Operand::AND => bit1 & bit2,
            Day24Operand::XOR => bit1 ^ bit2,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq)]
enum Value {
    On,
    Off,
    Undetermined,
}

impl BitAnd for Value {
    type Output = Value;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::On, Value::On) => Value::On,
            (Value::Undetermined, _) => Value::Undetermined,
            (_, Value::Undetermined) => Value::Undetermined,
            (_, _) => Value::Off,
        }
    }
}

impl BitOr for Value {
    type Output = Value;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Undetermined, _) => Value::Undetermined,
            (_, Value::Undetermined) => Value::Undetermined,
            (Value::Off, Value::Off) => Value::Off,
            (_, _) => Value::On,
        }
    }
}

impl BitXor for Value {
    type Output = Value;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Undetermined, _) => Value::Undetermined,
            (_, Value::Undetermined) => Value::Undetermined,
            (Value::Off, Value::Off) => Value::Off,
            (Value::Off, Value::On) => Value::On,
            (Value::On, Value::Off) => Value::On,
            (Value::On, Value::On) => Value::Off,
        }
    }
}

const OUTPUT_BITS: usize = 46;

const INPUT_BITS: usize = 45;
fn get_output(gates: &Vec<Day24Gate>, values: &mut Vec<Value>) -> u64 {
    for i in 0..gates.len() {
        if values[i] != Undetermined {
            continue;
        }
        let mut stack = vec![i];
        while !stack.is_empty() {
            if stack.len() > values.len() {
                return u64::MAX;
            }
            let last = stack.last().unwrap();
            if values[*last] != Undetermined {
                stack.pop();
                continue;
            }
            let arg1 = gates[*last].arg1;
            if values[arg1] == Undetermined {
                stack.push(arg1);
                continue;
            }
            let arg2 = gates[*last].arg2;
            if values[arg2] == Undetermined {
                stack.push(arg2);
                continue;
            }
            values[*last] = gates[*last].apply(values[arg1], values[arg2]);
        }
    }
    let mut res = 0;
    values[values.len() - OUTPUT_BITS..]
        .iter()
        .rev()
        .for_each(|b| {
            let curr_bit = match b {
                Value::On => 1,
                Value::Off => 0,
                Undetermined => 0,
            };
            res = 2 * res + curr_bit;
        });
    res
}

const LIMIT_NUM: u64 = (1u64 << INPUT_BITS);
fn validate(gates: &Vec<Day24Gate>, values: &mut Vec<Value>, n_digs: usize) -> bool {
    let mut rng = StdRng::from_os_rng();
    let mask = (1 << n_digs) - 1;
    for _ in 0..100 {
        let x = rng.random_range(0..LIMIT_NUM);
        let y = rng.random_range(0..LIMIT_NUM);
        let z_real = x + y;
        values.fill(Value::Undetermined);
        for i in 0..INPUT_BITS {
            let mask = 1 << i;
            values[i] = match x & mask {
                0 => Value::Off,
                _ => Value::On,
            };
            values[i + INPUT_BITS] = match y & mask {
                0 => Value::Off,
                _ => Value::On,
            };
        }
        let z = get_output(gates, values);
        if z & mask != z_real & mask {
            return false;
        }
    }
    true
}

fn solve_iterative(gates: &mut Vec<Day24Gate>) -> Vec<usize> {
    let mut buffer = vec![Value::Undetermined; gates.len()];
    let mut swaps = Vec::new();
    for i in 0..INPUT_BITS + 1 {
        let valid = validate(gates, &mut buffer, i);
        if valid {
            continue;
        }
        for swap in (2 * INPUT_BITS..gates.len()).combinations(2) {
            let s1 = swap[0];
            let s2 = swap[1];
            gates.swap(s1, s2);
            let valid = validate(gates, &mut buffer, i);
            if valid {
                swaps.push(s1);
                swaps.push(s2);
                break;
            }
            gates.swap(s1, s2);
        }
    }
    swaps
}

pub(crate) fn solve(data: &str) -> (String, String) {
    let split = data.find("\n\n").unwrap();
    let mut vert_nums = HashMap::new();
    let mut verts = Vec::new();

    let mut get_or_create_vert = |val: &str| -> usize {
        *vert_nums.entry(val.to_string()).or_insert_with(|| {
            let l = verts.len();
            verts.push(val.to_string());
            l
        })
    };

    let mut gates = vec![Day24Gate::new(); 1000];
    let mut input_val = vec![Value::Undetermined; 1000];
    data[..split].lines().for_each(|l| {
        let node = l[..3].to_string();
        let val = l[5..].parse::<i32>().unwrap();
        let i = get_or_create_vert(&node);
        input_val[i] = if val == 1 { Value::On } else { Value::Off };
    });
    data[split + 2..]
        .lines()
        .map(|l| {
            let elements = l.trim().split(" ").collect::<Vec<&str>>();
            elements
        })
        .sorted_by_key(|els| els[4])
        .for_each(|el| {
            let i = get_or_create_vert(&el[0]);
            let j = get_or_create_vert(&el[2]);
            let k = get_or_create_vert(&el[4]);
            let op = match el[1] {
                "OR" => Day24Operand::OR,
                "AND" => Day24Operand::AND,
                "XOR" => Day24Operand::XOR,
                &_ => unreachable!(),
            };
            gates[k].arg1 = i;
            gates[k].arg2 = j;
            gates[k].operand = op;
        });
    gates.truncate(verts.len());
    input_val.truncate(verts.len());

    let mut p1_vals = input_val.clone();
    let p1 = get_output(&gates, &mut p1_vals);
    let swaps = solve_iterative(&mut gates);
    let p2 = swaps.iter().map(|i| verts[*i].clone()).sorted().join(",");

    (p1.to_string(), p2)
}
