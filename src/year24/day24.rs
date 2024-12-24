use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};


#[derive(Debug, Clone, PartialEq, Copy, Ord, PartialOrd, Eq)]
enum Day24Operand {
    OR,
    AND,
    XOR,
}

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq)]
struct Day24Gate {
    arg1: String,
    arg2: String,
    operand: Day24Operand,
    output: String,
}

impl Day24Gate {
    pub fn apply(&self, bit1: bool, bit2: bool) -> bool {
        match self.operand {
            Day24Operand::OR => { bit1 | bit2 }
            Day24Operand::AND => { bit1 & bit2 }
            Day24Operand::XOR => { bit1 ^ bit2 }
        }
    }
}


fn get_output(gates: &HashMap<String, Day24Gate>, inputs: &HashMap<String, bool>, swaps: &HashMap<String, String>) -> u64 {
    let mut determined = inputs.clone();
    let mut to_determine = gates.iter().map(|(output, gate)| {
        let mut outname = output;
        if swaps.contains_key(outname) {
            outname = &swaps[outname];
        }
        (outname.clone(), gate.clone())
    }).collect::<HashMap<_, _>>();
    for (x, _) in determined.iter() {
        let mut ps = HashSet::new();
        ps.insert(x.clone());
    }
    while !to_determine.is_empty() {
        let could_determine_gates = to_determine.iter().filter_map(|(output, gate)|
            {
                if determined.contains_key(&gate.arg1) && determined.contains_key(&gate.arg2) {
                    Some(gate)
                } else {
                    None
                }
            }).cloned().collect_vec();
        if could_determine_gates.is_empty() {
            return u64::MAX;
        }
        for gate in could_determine_gates {
            let mut out_name = &gate.output;
            if swaps.contains_key(out_name) {
                out_name = &swaps[out_name];
            }
            to_determine.remove(out_name);
            let opres = gate.apply(determined[&gate.arg1], determined[&gate.arg2]);
            determined.insert(out_name.clone(), opres);
        }
    }
    let zvals = determined.iter().filter(|&(k, v)| {
        k.starts_with("z")
    }).sorted().collect_vec();
    let mut res: u64 = 0;
    for &(val_name, &val) in zvals.iter().rev() {
        res = 2 * res + if val { 1 } else { 0 };
    };

    res
}


const LIMIT: usize = 45;

const LIMIT_NUM: u64 = (1u64 << LIMIT);

fn get_ips(x: u64, y: u64) -> (HashMap<String, bool>) {
    let mut determined = HashMap::<String, bool>::new();
    let mut xx = x;
    for i in 0..LIMIT {
        determined.insert(format!("x{:02}", i), xx % 2 == 1);
        xx /= 2;
    }

    let mut yy = y;

    for i in 0..LIMIT {
        determined.insert(format!("y{:02}", i), yy % 2 == 1);
        yy /= 2;
    }
    determined
}


fn validate(gates: &HashMap<String, Day24Gate>, swaps: &HashMap<String, String>, n_digs: usize) -> bool {
    let mut rng = StdRng::from_os_rng();
    let mask = (1 << n_digs) - 1;
    for _ in 0..100 {
        let x = rng.random_range(0..LIMIT_NUM);
        let y = rng.random_range(0..LIMIT_NUM);
        let z_real = x + y;
        let ips = get_ips(x, y);
        let z = get_output(gates, &ips, swaps);
        if z & mask == z_real & mask {
            //println!("valid");
        } else {
            return false;
        }
    };
    true
}


fn solve_iterative(gates: &HashMap<String, Day24Gate>) -> HashMap<String, String> {
    let mut swaps = HashMap::<String, String>::new();
    for i in 0..LIMIT + 1 {
        let valid = validate(gates, &swaps, i);
        if valid {
            continue;
        }
        let mut j = 0;
        let target = gates.len() * gates.len();
        println!("{} -> {}", i, target);
        for swap in gates.keys().combinations(2) {
            j += 1;
            if i % 100 == 0 {
                println!("{}", j);
            }
            let mut new_swaps = swaps.clone();
            new_swaps.insert(swap[0].clone(), swap[1].clone());
            new_swaps.insert(swap[1].clone(), swap[0].clone());
            let valid = validate(gates, &new_swaps, i);
            if valid {
                println!("found swap {:?}", swap);
                swaps = new_swaps;
                break;
            }
        }
        //break;
    };
    swaps
}


pub(crate) fn solve(data: &str) -> (String, String) {
    let split = data.find("\n\n").unwrap();
    let mut inputs = HashMap::new();
    data[..split].lines().for_each(|l| {
        let node = l[..3].to_string();
        let val = l[5..].parse::<i32>().unwrap();
        inputs.insert(node, val == 1);
    });
    let gates = data[split + 2..].lines().map(|l| {
        let elements = l.trim().split(" ").collect::<Vec<&str>>();
        let operand = match elements[1] {
            "OR" => Day24Operand::OR,
            "AND" => Day24Operand::AND,
            "XOR" => Day24Operand::XOR,
            _ => unreachable!(),
        };
        (elements[4].to_string(), Day24Gate {
            arg1: elements[0].to_string(),
            arg2: elements[2].to_string(),
            operand,
            output: elements[4].to_string(),
        })
    }).collect::<HashMap<_, _>>();


    let noswaps = HashMap::<String, String>::new();
    let p1 = get_output(&gates, &inputs, &noswaps);


    let swaps = solve_iterative(&gates);

    // gcd,gmn,jtm,jtv,ntn,pcj,qdj,sgv is wrong
    // cvp,fqv,jvj,mvs,ndm,tsp,z10,z25
    //11460720 not right
    let mut swappees = swaps.values().collect_vec();
    swappees.sort();
    (p1.to_string(), swappees.into_iter().join(","))
}