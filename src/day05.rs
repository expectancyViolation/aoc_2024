use std::cmp::Ordering;
use itertools::Itertools;

pub fn solve(data: &str) -> (i64, i64) {
    let mut rules = [[Ordering::Equal; 100]; 100];
    let mut part1 = 0;
    let mut part2 = 0;
    data.split("\n\n").enumerate().for_each(
        |(part, x)| {
            if part == 0 {
                x.lines().for_each(|line| {
                    let (rule_from, rule_to) = line.split("|").map(|n| n.parse::<usize>().unwrap()).collect_tuple().unwrap();
                    rules[rule_from][rule_to] = Ordering::Less;
                    rules[rule_to][rule_from] = Ordering::Greater;
                })
            } else {
                x.lines().for_each(|line| {
                    let mut nums = line
                        .split(",")
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<usize>>();
                    let mut sorted: Vec<usize> = nums.clone();
                    sorted.sort_by(|&a, &b| rules[a][b]);
                    if nums == sorted {
                        part1 += (nums[nums.len() / 2]) as i64;
                    } else {
                        part2 += (sorted[sorted.len() / 2]) as i64;
                    }
                });
            }
        }
    );
    (part1, part2)
}
