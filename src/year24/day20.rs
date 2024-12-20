use crate::str_map::{StrMap, DIRECTIONS};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

fn bfs(start: (i32, i32), m: &StrMap) -> HashMap<(i32, i32), i32> {
    let mut distances = HashMap::new();
    let mut frontier = HashSet::new();
    frontier.insert(start);
    distances.insert(start, 0);
    while !frontier.is_empty() {
        let mut nf = HashSet::new();
        for (x, y) in frontier {
            let dist = distances[&(x, y)];
            for (dx, dy) in DIRECTIONS {
                let (nx, ny) = (x + dx, y + dy);
                let sym = m.get(nx, ny);
                if !distances.contains_key(&(nx, ny))
                    && ((sym == b'.') || (sym == b'S') || (sym == b'E'))
                {
                    nf.insert((nx, ny));
                    distances.insert((nx, ny), dist + 1);
                }
            }
        }
        frontier = nf;
    }
    distances
}

pub(crate) fn solve(data: &str) -> (String, String) {
    let w = data.lines().next().unwrap().len() as i32;
    let h = data.lines().count() as i32;
    let start = data.find('S').unwrap() as i32;
    let end = data.find('E').unwrap() as i32;
    let end_pos = (end / (w + 1), end % (w + 1));
    let start_pos = (start / (w + 1), start % (w + 1));
    let mut data = String::from(data).into_bytes();
    let m = StrMap {
        data: data.as_mut_slice(),
        w,
        h,
    };
    let d_start = bfs(start_pos, &m);
    let d_end = bfs(end_pos, &m);

    let total_dist = d_start[&end_pos];
    let to_test = d_start.iter().collect_vec();

    let sols: Vec<(i32, i32)> = to_test
        .par_iter()
        .map(|((px, py), &dist)| {
            let mut p1 = 0;
            let mut p2 = 0;
            for dx in -20..21 {
                let adx = (dx as i32).abs();
                for dy in (-20 + adx)..(21 - adx) {
                    let (ex, ey) = (px + dx, py + dy);
                    d_end.get(&(ex, ey)).map(|e_dist| {
                        ;
                        let d = (px - ex).abs() + (py - ey).abs();
                        let curr_saved = total_dist - dist - e_dist - d;
                        if curr_saved >= 100 {
                            if d <= 2 {
                                p1 += 1;
                            }
                            if d <= 20 {
                                p2 += 1;
                            }
                        }
                    });
                }
            }
            (p1, p2)
        })
        .collect();

    let (res, res2) = sols
        .into_iter()
        .fold((0, 0), |(p11, p21), (p12, p22)| (p11 + p12, p21 + p22));

    (res.to_string(), res2.to_string())
}
