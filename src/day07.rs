use std::cmp::max;
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
fn un_concat(x: i64, y: i64) -> Option<i64> {
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
    static ref part2_unops: Vec<OptOp> = vec![subtract, divide, un_concat];
    static ref part2_ops: Vec<Op> = vec![|x,y|x+y,|x,y|x*y,concat];
}

fn parts(data: &str, part2: bool) -> i64 {
    let unops: &Vec<OptOp> = if part2 { &part2_unops } else { &part1_unops };
    let ops: &Vec<Op> = if part2 { &part2_ops } else { &part1_ops };
    let lines: Vec<_> = data.lines().collect();
    lines.par_iter()
        .filter_map(|line| {
            let (target, parts) = line.split(":").collect_tuple().unwrap();
            let target = target.parse::<i64>().unwrap();
            let nums: Vec<_> = parts
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap()).collect();
            // meet in the middle does not seem to be worth it (apart from last step)
            let break_ = max(1, nums.len() / 3 + 1);
            // println!("{}",break_);

            let y: Box<dyn Iterator<Item=i64>> = Box::new(vec![nums[0]].into_iter());
            let mut fw_poss_results = nums[1..break_]
                .iter()
                .fold(y, |poss_state, &part| {
                    Box::new(iproduct!(poss_state, ops)
                        .map(move |(x, op_)| op_(x, part))
                        .filter(|&x| x < target))
                });

            let x: Box<dyn Iterator<Item=i64>> = Box::new(vec![target].into_iter());
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
                .any(|x| {
                    let found = fw_poss_results.binary_search(&x).is_ok();
                    found
                });
            if found {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

fn possible(i: usize, val: i64, nums: &Vec<i64>, un_ops: &Vec<OptOp>) -> bool {
    if i == 0 {
        return val == nums[0];
    }
    un_ops.iter().any(|op| {
        op(val, nums[i]).is_some_and(|x| possible(i - 1, x, nums, un_ops))
    })
}

fn parts_rec(data: &str, part2: bool) -> i64 {
    let un_ops = if part2 { vec![subtract, divide, un_concat] } else { vec![subtract, divide] };
    let lines: Vec<_> = data.lines().collect();
    lines.par_iter()
        .filter_map(|line| {
            let (target, parts) = line.split(":").collect_tuple().unwrap();
            let target = target.parse::<i64>().unwrap();
            let nums: Vec<_> = parts
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap()).collect();

            if possible(nums.len() - 1, target, &nums, &un_ops) {
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

pub fn solve_recursive(data: &str) -> (i64, i64) {
    (parts(data, false), parts(data, true))
}
