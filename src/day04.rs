use counter::Counter;
use itertools::iproduct;

fn part1(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let total = input.len() as i64;
    // +1 for newline
    let n = (input.lines().next().unwrap().len() + 1) as i64;
    let mut res = 0;
    for (dx, dy) in iproduct!(-1i64..2, -1..2) {
        for p in 0..total {
            let does_match = "XMAS"
                .bytes()
                .enumerate()
                .map(|(i, c)| {
                    let ind = p + (i as i64) * (dx * n + dy);
                    bytes.get(ind as usize).or(Some(&b'\0')).unwrap() == &c
                })
                .all(|x| x);
            if does_match {
                res += 1
            }
        }
    }
    res
}

// TODO: kinda repetitive, but "find_direction" has weird interface
fn part2(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let total = input.len() as i64;
    let n = (input.lines().next().unwrap().len() + 1) as i64;
    let mut mases = vec![0, 0];
    for (dx, dy) in iproduct!([-1, 1], [-1, 1]) {
        for p in 0..total {
            let does_match = "MAS"
                .bytes()
                .enumerate()
                .map(|(i, c)| {
                    let ind = p + (i as i64) * (dx * n + dy);
                    bytes.get(ind as usize).or(Some(&b'\0')).unwrap() == &c
                })
                .all(|x| x);
            if does_match {
                let match_center = (p + dx * n + dy) as usize;
                mases.push(match_center);
            }
        }
    }
    let counts = mases.iter().collect::<Counter<_>>();
    counts.values().filter(|&&x| x == 2).count() as i64
}

pub fn solve(input: &str) -> (i64, i64) {
    (part1(input), part2(input))
}
