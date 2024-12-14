use itertools::Itertools;
use regex::Regex;

fn solve_2x2_sle_integer(
    m11: i64,
    m21: i64,
    m12: i64,
    m22: i64,
    b1: i64,
    b2: i64,
) -> Option<(i64, i64)> {
    let det = m11 * m22 - m12 * m21;
    let mut sol_a = m22 * b1 - m12 * b2;
    let mut sol_b = -m21 * b1 + m11 * b2;
    if (sol_a % det != 0) || (sol_b % det != 0) {
        return None;
    }
    sol_a /= det;
    sol_b /= det;
    if (sol_a < 0) || (sol_b < 0) {
        return None;
    }
    Some((sol_a, sol_b))
}

const PART2_HIGHER: i64 = 10000000000000;
pub(crate) fn solve(data: &str) -> (i64, i64) {
    let reg = Regex::new(r"([0-9]+)").unwrap();
    let blocks = data.split("\n\n");
    let mut p1 = 0;
    let mut p2 = 0;
    for block in blocks {
        let (m11, m21, m12, m22, b1, b2) = reg
            .find_iter(block)
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        solve_2x2_sle_integer(m11, m21, m12, m22, b1, b2)
            .iter()
            .for_each(|(a, b)| p1 += 3 * a + b);
        solve_2x2_sle_integer(m11, m21, m12, m22, b1 + PART2_HIGHER, b2 + PART2_HIGHER)
            .iter()
            .for_each(|(a, b)| p2 += 3 * a + b);
    }
    (p1, p2)
}
