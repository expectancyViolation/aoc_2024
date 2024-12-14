use hashbrown::HashMap;

pub(crate) fn n_digs(x: i64) -> i64 {
    (x.ilog10() + 1) as i64
}

fn count(x: i64, n: i64) -> i64 {
    let mut curr = HashMap::new();
    let mut buff = HashMap::new();
    curr.insert(x, 1);
    for i in 0..n {
        let (mut from_, mut to_) = if (i % 2 == 0) {
            (&mut curr, &mut buff)
        } else {
            (&mut buff, &mut curr)
        };
        to_.clear();
        for (y, cnt) in from_.iter() {
            if *y == 0 {
                *to_.entry(1).or_insert(0) += cnt;
            } else {
                let digs = n_digs(*y);
                match digs % 2 {
                    0 => {
                        let ten_pow = 10_i64.pow((digs / 2) as u32);
                        *to_.entry(y / ten_pow).or_insert(0) += cnt;
                        *to_.entry(y % ten_pow).or_insert(0) += cnt;
                    }
                    _ => {
                        *to_.entry(y * 2024).or_insert(0) += cnt;
                    }
                }
            }
        }
    }
    if n % 2 == 0 {
        curr.values().sum()
    } else {
        buff.values().sum()
    }
}

pub(crate) fn solve(data: &str) -> (i64, i64) {
    let nums = data
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();
    let p1 = nums.iter().map(|&x| count(x, 25)).sum();
    let p2 = nums.iter().map(|&x| count(x, 75)).sum();

    (p1, p2)
}
