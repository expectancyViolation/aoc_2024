use std::ops::{Add, Mul, Rem};
use hashbrown::HashSet;
use itertools::Itertools;
use regex::Regex;
use crate::day10::DIRECTIONS;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct V(i32, i32);


impl Add for V {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        V(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<i32> for V {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        V(self.0 * rhs, self.1 * rhs)
    }
}

impl Rem<Self> for V {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        V(self.0.rem_euclid(rhs.0), self.1.rem_euclid(rhs.1))
    }
}

#[derive(Debug, Clone)]
struct Robot {
    pos: V,
    vel: V,
}

impl Robot {
    fn simulate(&self, time_in_sec: i32, limits: V) -> Self {
        let new_pos = (self.pos + self.vel * time_in_sec) % limits;
        Robot { pos: new_pos, vel: self.vel }
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
            _ => 0
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

pub(crate) fn solve(data: &str) -> (i64, i64) {
    let width = 101;
    let height = 103;
    let limits = V(width, height);

    let reg = Regex::new(r"(-?\d+)").unwrap();
    let robots = data.lines().map(|line| {
        let (x, y, vx, vy) = reg.find_iter(line).map(|x| x.as_str().parse::<i32>().unwrap()).collect_tuple().unwrap();
        Robot { pos: V(x, y), vel: V(vx, vy) }
    }).collect_vec();
    let quadrant_counts = get_quadrant_counts(&robots, 100, V(width, height));
    println!("Quadrant count: {:?}", quadrant_counts);

    let mut m = 0;
    let mut p2 = 0;
    println!("{:?}", get_quadrant_counts(&robots, 6771, limits));

    for i in 0..10000 {
        let qc = get_quadrant_counts(&robots, i, V(width, height));
        let positions: HashSet<_> = robots.iter().map(|robot| { robot.simulate(i, limits).pos }).collect();
        let mut nb_counts = 0;
        for pos in positions.iter() {
            for (dx, dy) in DIRECTIONS {
                let nb_pos = V(pos.0 + dx, pos.1 + dy);
                if positions.contains(&nb_pos) {
                    nb_counts += 1;
                }
            }
        }
        if nb_counts > m {
            m = nb_counts;
            p2 = i;
        }
    }

    // let mut pic = vec![vec!["."; width as usize]; height as usize];
    // for robot in robots {
    //     let stepped = robot.simulate(am, limits);
    //     pic[stepped.pos.1 as usize][stepped.pos.0 as usize] = "X";
    // }
    // println!("----------------");
    // println!("{}", 6771);
    // for row in pic.iter() {
    //     println!("{}", row.join(""))
    // }

    let p1 = quadrant_counts[1..].iter().product::<i32>() as i64;
    // 2581 too low
    // 7577 too high
    (p1, p2 as i64)
}