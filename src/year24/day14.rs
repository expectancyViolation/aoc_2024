use crate::util::chinese_remainder_theorem;
use crate::v::V;
use itertools::Itertools;
use regex::Regex;
use std::cmp::max;

#[derive(Debug, Clone)]
struct Robot {
    pos: V,
    vel: V,
}

impl Robot {
    fn simulate(&self, time_in_sec: i32, limits: V) -> Self {
        let new_pos = (self.pos + self.vel * time_in_sec) % limits;
        Robot {
            pos: new_pos,
            vel: self.vel,
        }
    }

    fn get_quadrant(&self, limits: V) -> usize {
        let cx = limits.0 / 2;
        let cy = limits.1 / 2;
        let x_sign = (self.pos.0 - cx).signum();
        let y_sign = (self.pos.1 - cy).signum();
        match (x_sign, y_sign) {
            (1, -1) => 1,
            (-1, -1) => 2,
            (-1, 1) => 3,
            (1, 1) => 4,
            _ => 0,
        }
    }
}

fn get_quadrant_counts(robots: &Vec<Robot>, time: i32, limits: V) -> [i32; 5] {
    let mut quadrant_count = [0; 5];
    for robot in robots {
        let q = robot.simulate(time, limits).get_quadrant(limits);
        quadrant_count[q] += 1;
    }
    quadrant_count
}

fn get_variances(robots: &Vec<Robot>, time: i32, limits: V) -> (i32, i32) {
    let positions = robots
        .iter()
        .map(|robot| robot.simulate(time, limits).pos)
        .collect_vec();
    let n = positions.len() as i32;
    let exx: i32 = positions.iter().map(|&pos| pos.0 * pos.0).sum();
    let ex: i32 = positions.iter().map(|&pos| pos.0).sum();
    let eyy: i32 = positions.iter().map(|&pos| pos.1 * pos.1).sum();
    let ey: i32 = positions.iter().map(|&pos| pos.1).sum();

    (n * exx - ex * ex, n * eyy - ey * ey)
}

pub(crate) fn solve(data: &str) -> (String, String) {
    let width = 101;
    let height = 103;

    let reg = Regex::new(r"(-?\d+)").unwrap();
    let robots = data
        .lines()
        .map(|line| {
            let (x, y, vx, vy) = reg
                .find_iter(line)
                .map(|x| x.as_str().parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            Robot {
                pos: V(x, y),
                vel: V(vx, vy),
            }
        })
        .collect_vec();
    let quadrant_counts = get_quadrant_counts(&robots, 100, V(width, height));
    let p1 = quadrant_counts[1..].iter().product::<i32>() as i64;

    let max_period = max(width, height);

    let variances = (0..max_period)
        .map(|i| get_variances(&robots, i, V(width, height)))
        .collect_vec();
    let x_variances = variances.iter().map(|&(x, _y)| x).collect_vec();
    let y_variances = variances.iter().map(|&(_x, y)| y).collect_vec();
    let x_rem = x_variances.iter().position_min().unwrap();
    let y_rem = y_variances.iter().position_min().unwrap();

    let residues = [x_rem as i32, y_rem as i32];
    let moduli = [width, height];

    let p2 = chinese_remainder_theorem(&residues, &moduli).unwrap();

    (p1.to_string(), p2.to_string())
}
