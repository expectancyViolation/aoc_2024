use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;

type Op = fn(i64, i64) -> i64;

// fn combine(
//     p: i64,
//     ops: &Vec<Op>,
//     it: Box<dyn Iterator<Item=i64>>,
// ) -> Box<dyn Iterator<Item=i64> + '_> {
//     Box::new(iproduct!(it, ops.iter()).map(move |(x, op_)| op_(p, x)))
// }

fn concat(x: i64, y: i64) -> i64 {
    let n_digs: u32 = y.checked_ilog10().unwrap_or(0) + 1;
    x * i64::pow(10, n_digs) + y
}


lazy_static! {
    static ref part1_ops: Vec<Op> = vec![|x, y| x + y, |x, y| x * y];
    static ref part2_ops: Vec<Op> = vec![|x, y| x + y, |x, y| x * y,concat];
}
fn parts(data: &str, part2: bool) -> i64 {
    let ops: &Vec<Op> = if part2 { &part2_ops } else { &part1_ops };
    data.lines()
        .filter_map(|line| {
            let (target, parts) = line.split(":").collect_tuple().unwrap();
            let target = target.parse::<i64>().unwrap();
            let x: Box<dyn Iterator<Item=i64>> = Box::new(vec![0i64].into_iter());
            let mut poss_results = parts.split_whitespace().fold(x, |poss_states, part| {
                let p = part.parse::<i64>().unwrap();
                Box::new(iproduct!(poss_states,ops).map(move |(x, op_)| op_(p, x)))
            });
            if poss_results.contains(&target) {
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
