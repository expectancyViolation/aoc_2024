use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::prelude::*;

pub(crate) type OptOp = fn(i64, i64) -> Option<i64>;

pub(crate) fn ten_pow(x: i64) -> i64 {
    let n_digs: u32 = x.checked_ilog10().unwrap_or(0) + 1;
    i64::pow(10, n_digs)
}
fn un_concat(x: i64, y: i64) -> Option<i64> {
    let tp = ten_pow(y);
    if x % tp != y {
        None
    } else {
        Some(x / tp)
    }
}

fn divide(x: i64, y: i64) -> Option<i64> {
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
    pub(crate) static ref part1_unops: Vec<OptOp> = vec![subtract, divide];
    pub(crate) static ref part2_unops: Vec<OptOp> = vec![subtract, divide, un_concat];
}


fn possible(val: i64, nums: &[i64], part2: bool) -> bool {
    if nums.len() == 1 {
        val == nums[0]
    } else {
        let un_ops: &Vec<OptOp> = if part2 { &part2_unops } else { &part1_unops };
        un_ops
            .iter()
            .any(|op| op(val, nums[0]).is_some_and(|x| possible(x, &nums[1..], part2)))
    }
}

fn parts_rec(data: &str, part2: bool) -> i64 {
    let lines: Vec<_> = data.lines().collect();
    lines
        .par_iter()
        .filter_map(|line| {
            let (target, parts) = line.split(":").collect_tuple().unwrap();
            let target = target.parse::<i64>().unwrap();
            let nums: Vec<_> = parts
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .rev()
                .collect();
            Some(target).filter(|&x| possible(x, &nums, part2))
        })
        .sum()
}


pub fn solve(data: &str) -> (String, String) {
    (parts_rec(data, false).to_string(), parts_rec(data, true).to_string())
}
