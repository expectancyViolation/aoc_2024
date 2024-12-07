use itertools::{iproduct, Itertools};

type Op = fn(i64, i64) -> i64;

fn parts(data: &str, part2: bool) -> i64 {
    let concat: Op = |x, y| {
        let n_digs: u32 = y.checked_ilog10().unwrap_or(0) + 1;
        x * i64::pow(10, n_digs) + y
    };

    let operations: Vec<Op> = if (part2) {
        vec![|x, y| x + y, |x, y| x * y]
    } else {
        vec![|x, y| x + y, |x, y| x * y, concat]
    };


    data.lines().filter_map(|line| {
        let (target, parts) = line.split(":").collect_tuple().unwrap();
        let target = target.parse::<i64>().unwrap();
        let poss_results = parts.split_whitespace().fold(vec![0i64], |poss_states, part| {
            let p = part.parse::<i64>().unwrap();
            iproduct!(poss_states, operations.iter()).map(|(x, op_)| { op_(x, p) }).filter(|&res| res <= target).collect_vec()
        });
        if poss_results.contains(&target) {
            Some(target)
        } else {
            None
        }
    }).sum()
}

pub fn solve(data: &str) -> (i64, i64) {
    (parts(data, false), parts(data, true))
}
