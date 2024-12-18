use itertools::Itertools;
use crate::str_map::StrMap;
// struct StrMap<'a> {
//     data: &'a mut [u8],
//     h: i32,
//     w: i32,
// }
//
// impl<'a> StrMap<'a> {
//     fn get(&self, x: i32, y: i32) -> u8 {
//         if x < 0 || y < 0 || (self.h < x) || (self.w < y) {
//             0
//         } else { self.data[(x * (self.w + 1) + y) as usize] }
//     }
//     fn set(&mut self, x: i32, y: i32, val: u8) {
//         self.data[(x * (self.w + 1) + y) as usize] = val;
//     }
// }

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

const MARKED: u8 = 0;

fn distances(map_: &mut StrMap, start_pos: (i32, i32)) -> Vec<(i32, i64)> {
    let mut ds: Vec<(i32, i64)> = Vec::new();
    let mut frontier = vec![start_pos];
    let mut dist = 0;
    let (x, y) = start_pos;
    map_.set(x, y, MARKED);
    while frontier.len() > 0 {
        dist += 1;
        let mut new_frontier: Vec<(i32, i32)> = Vec::new();
        for (x, y) in &frontier {
            //println!("visiting {:?}", (x, y));
            for (dx, dy) in DIRECTIONS {
                let (nx, ny) = (x + dx, y + dy);
                let c = map_.get(nx, ny) as char;
                c.to_digit(10).map(|d| {
                    ds.push((d as i32, dist));
                }
                );
                match c {
                    '0'..='9' | '.' => {
                        new_frontier.push((nx, ny));
                        map_.set(nx, ny, MARKED);
                    }
                    _ => {}
                }
            }
        }
        frontier = new_frontier;
    };
    ds
}

pub(crate) fn solve(data: &str) -> (String,String) {
    let mut positions: Vec<(i32, (i32, i32))> = Vec::new();
    data.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            match c.to_digit(10) {
                Some(d) => { positions.push((d as i32, (x as i32, y as i32))) }
                _ => {}
            }
        })
    });
    let num = positions.len();
    let h = data.lines().count() as i32;
    let w = data.lines().next().unwrap().len() as i32;
    let mut dists = vec![vec![-1; num]; num];
    positions.iter().for_each(|&(d, p)| {
        let mut data = String::from(data).into_bytes();
        let mut m = StrMap { data: data.as_mut_slice(), h, w };
        distances(&mut m, p).iter().for_each(|&(num, d_)| {
            dists[d as usize][num as usize] = d_;
        });
    });
    let dist = |p, p2| {
        let mut res = 0;
        let mut pos = 0;
        for np in p {
            res += dists[pos][np] as i64;
            pos = np;
        }
        if p2 {
            res += dists[pos][0];
        }
        res
    };
    let p1 = (1..num).permutations(num - 1).map(|p| dist(p, false)).min().unwrap();
    let p2 = (1..num).permutations(num - 1).map(|p| dist(p, true)).min().unwrap();
    (p1.to_string(), p2.to_string())
}
