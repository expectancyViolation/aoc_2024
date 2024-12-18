use std::cmp::max;
use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;
use super::day07::{ten_pow, OptOp, part2_unops, part1_unops};
use rayon::prelude::*;

fn concat(x: i64, y: i64) -> i64 {
    x * ten_pow(y) + y
}

type Op = fn(i64, i64) -> i64;
lazy_static! {
    static ref part1_ops: Vec<Op> = vec![|x, y| x + y, |x, y| x * y];
    static ref part2_ops: Vec<Op> = vec![|x, y| x + y, |x, y| x * y, concat];
}

fn parts(data: &str, part2: bool) -> i64 {
    let unops: &Vec<OptOp> = if part2 { &part2_unops } else { &part1_unops };
    let ops: &Vec<Op> = if part2 { &part2_ops } else { &part1_ops };
    let lines: Vec<_> = data.lines().collect();
    lines
        .par_iter()
        .filter_map(|line| {
            let (target, parts) = line.split(":").collect_tuple().unwrap();
            let target = target.parse::<i64>().unwrap();
            let nums: Vec<_> = parts
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            // meet in the middle does not seem to be worth it (apart from last step)
            let break_ = max(1, nums.len() / 3 + 1);
            // println!("{}",break_);

            let y: Box<dyn Iterator<Item=i64>> = Box::new(vec![nums[0]].into_iter());
            let fw_poss_results = nums[1..break_].iter().fold(y, |poss_state, &part| {
                Box::new(
                    iproduct!(poss_state, ops)
                        .map(move |(x, op_)| op_(x, part))
                        .filter(|&x| x <= target),
                )
            });

            let x: Box<dyn Iterator<Item=i64>> = Box::new(vec![target].into_iter());
            let mut bw_poss_results = nums[break_..].iter().rev().fold(x, |poss_state, &part| {
                Box::new(iproduct!(poss_state, unops).filter_map(move |(x, op_)| op_(x, part)))
            });
            let mut fw_poss_results: Vec<_> = fw_poss_results.collect();
            fw_poss_results.sort();
            let found = bw_poss_results.any(|x| {
                let found = fw_poss_results.binary_search(&x).is_ok();
                found
            });
            Some(target).filter(|&_| found)
        })
        .sum()
}
pub fn solve(data: &str) -> (String, String) {
    (parts(data, false).to_string(), parts(data, true).to_string())
}
