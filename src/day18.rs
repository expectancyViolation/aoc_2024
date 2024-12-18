use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use crate::str_map::DIRECTIONS;

pub(crate) fn solve(data: &str) -> (i64, i64) {
    let mut p1 = 0;
    let mut p2 = (0, 0);
    let mut blocked = [[false; 71]; 71];
    data.lines().enumerate().for_each(|(j, line)| {
        let (X, Y) = line.split(",").map(|x| x.parse::<usize>().unwrap()).collect_tuple().unwrap();
        blocked[X][Y] = true;
        let mut frontier = HashSet::new();
        frontier.insert((0, 0));
        let mut distances = [[-1; 71]; 71];
        distances[0][0] = 0;
        for i in 1.. {
            if frontier.is_empty() {
                break;
            }
            let mut nf = HashSet::new();
            for (x, y) in frontier.iter() {
                for (dx, dy) in DIRECTIONS {
                    let (nx, ny) = (x + dx, y + dy);
                    if 0 <= nx && nx < 71 && 0 <= ny && ny < 71 {
                        if !blocked[nx as usize][ny as usize] && distances[nx as usize][ny as usize] == -1 {
                            distances[nx as usize][ny as usize] = i;
                            nf.insert((nx, ny));
                        }
                    }
                }
            }
            frontier = nf;
        }
        if j == 1024 {
            p1 = distances[70][70];
        }
        if distances[70][70] == -1 {
            if p2 == (0, 0) {
                p2 = (X, Y);
            }
        };
    });
    (p1, (p2.0 * 100 + p2.1) as i64)
}