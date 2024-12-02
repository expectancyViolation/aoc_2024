use counter::Counter;
use itertools::Itertools;
use crate::day01;

fn parse(raw_input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();
    raw_input.lines().for_each(|line| {
        let (x, y) = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple().unwrap();
        left_list.push(x);
        right_list.push(y);
    });
    (left_list, right_list)
}

fn part1(left_list: &mut Vec<i64>, right_list: &mut Vec<i64>) -> i64 {
    left_list.sort();
    right_list.sort();
    left_list.iter().zip(right_list.iter()).map(|(l, &r)| (l - r).abs()).sum::<i64>()
}

fn part2(left_list: &Vec<i64>, right_list: &Vec<i64>) -> i64 {
    let counts = right_list.iter().collect::<Counter<_>>();
    left_list.iter().map(|x| x * (counts.get(x).cloned().unwrap_or(0)) as i64).sum::<i64>()
}

pub fn solve(input: &str) -> (i64, i64) {
    let (mut left_list, mut right_list) = parse(&input);
    // mutating list is ok, since part2 does not require order
    let part1_result = day01::part1(&mut left_list, &mut right_list);
    let part2_result = day01::part2(&left_list, &right_list);
    (part1_result, part2_result)
}
