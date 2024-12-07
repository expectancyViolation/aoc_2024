use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;
use rayon::prelude::*;


// fn combine(
//     p: i64,
//     ops: &Vec<Op>,
//     it: Box<dyn Iterator<Item=i64>>,
// ) -> Box<dyn Iterator<Item=i64> + '_> {
//     Box::new(iproduct!(it, ops.iter()).map(move |(x, op_)| op_(p, x)))
// }


type Op = fn(i64, i64) -> i64;
type OptOp = fn(i64, i64) -> Option<i64>;


fn concat(x: i64, y: i64) -> i64 {
    let n_digs: u32 = y.checked_ilog10().unwrap_or(0) + 1;
    x * i64::pow(10, n_digs) + y
}
fn un_concatenate(x: i64, y: i64) -> Option<i64> {
    let n_digs: u32 = y.checked_ilog10().unwrap_or(0) + 1;
    let ten_pow = i64::pow(10, n_digs);
    if x % ten_pow != y {
        None
    } else {
        Some(x / ten_pow)
    }
}

fn divide(x: i64, y: i64) -> Option<i64> {
    if y == 0 {
        return None;
    }
    if x % y != 0 {
        None
    } else {
        Some(x / y)
    }
}

fn subtract(x: i64, y: i64) -> Option<i64> {
    if y > x {
        None
    } else {
        Some(x - y)
    }
}

lazy_static! {
    static ref part1_unops: Vec<OptOp> = vec![subtract, divide];
    static ref part1_ops: Vec<Op> = vec![|x,y|x+y,|x,y|x*y];
    static ref part2_unops: Vec<OptOp> = vec![subtract, divide, un_concatenate];
    static ref part2_ops: Vec<Op> = vec![|x,y|x+y,|x,y|x*y,concat];
}

fn parts(data: &str, part2: bool) -> i64 {
    let unops: &Vec<OptOp> = if part2 { &part2_unops } else { &part1_unops };
    let ops: &Vec<Op> = if part2 {&part2_ops} else {&part1_ops};
    let lines:Vec<_>=data.lines().collect();
    lines.par_iter()
        .filter_map(|line| {
            let (target, parts) = line.split(":").collect_tuple().unwrap();
            let target = target.parse::<i64>().unwrap();
            let x: Box<dyn Iterator<Item=i64>> = Box::new(vec![target].into_iter());
            let nums: Vec<_> = parts
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap()).collect();
            // meet in the middle does not seem to be worth it (apart from last step)
            let break_=1;
            // println!("{}",break_);

            let y: Box<dyn Iterator<Item=i64>> = Box::new(vec![0].into_iter());
            let mut fw_poss_results = nums[..break_]
                .iter()
                .fold(y, |poss_state, &part| {
                    Box::new(iproduct!(poss_state, ops)
                        .map(move |(x, op_)| op_(x, part))
                        .filter(|&x| x < target))
            });
            let mut bw_poss_results = nums[break_..]
                .iter()
                .rev()
                .fold(x, |poss_state, &part| {
                    Box::new(iproduct!(poss_state, unops)
                        .filter_map(move |(x, op_)| op_(x, part)))
            });
            let mut fw_poss_results: Vec<_> = fw_poss_results.collect();
            fw_poss_results.sort();
            let found = bw_poss_results
                .any(|x| fw_poss_results.binary_search(&x).is_ok());
            if found {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

pub fn solve(data: &str) -> (i64, i64) {
    (parts(data, false), parts(data, true))
}
