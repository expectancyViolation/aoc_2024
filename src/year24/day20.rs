use crate::str_map::{StrMap, DIRECTIONS};
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::iter::IndexedParallelIterator;

fn bfs(start: (i32, i32), m: &StrMap) -> Vec<Vec<i32>> {
    let mut distances = vec![vec![-1; m.w as usize]; m.h as usize];
    let mut frontier = vec![start];
    distances[start.0 as usize][start.1 as usize] = 0;
    while !frontier.is_empty() {
        let mut nf = Vec::new();
        for (x, y) in frontier {
            for (dx, dy) in DIRECTIONS {
                let (nx, ny) = (x + dx, y + dy);
                let sym = m.get(nx, ny);
                if distances[nx as usize][ny as usize] == -1
                    && ((sym == b'.'))
                {
                    nf.push((nx, ny));
                    distances[nx as usize][ny as usize] = distances[x as usize][y as usize] + 1;
                }
            }
        }
        frontier = nf;
    }
    distances
}

pub(crate) fn solve(data: &str) -> (String, String) {
    let w = data.lines().next().unwrap().len() as i32;
    let h = data.len() as i32 /(w+1);
    let start = data.find('S').unwrap() as i32;
    let end = data.find('E').unwrap() as i32;
    let end_pos = (end / (w + 1), end % (w + 1));
    let start_pos = (start / (w + 1), start % (w + 1));
    let mut data = String::from(data).into_bytes();
    let mut m = StrMap {
        data: data.as_mut_slice(),
        w,
        h,
    };
    m.set(start_pos.0, start_pos.1, b'.');
    m.set(end_pos.0, end_pos.1, b'.');
    let d_start = bfs(start_pos, &m);

    let sols = d_start.par_iter().enumerate().map(|(px, v)| {
        let px = px as i32;
        let mut p1 = 0;
        let mut p2 = 0;
        v.iter().enumerate().for_each(|(py, &dist)| {
            if dist < 0 {
                return;
            }
            let py = py as i32;
            for dx in 0..21 {
                let ex = px + dx;
                if ex < 0 || ex >= h {
                    continue;
                }
                let mx = (px - ex).abs();
                let adx = dx.abs();
                let lower=if (dx>0) {-20+adx} else {0};
                for dy in lower..(21 - adx) {
                    let ey = py + dy;
                    if ey < 0 || ey >= w {
                        continue;
                    }
                    let e_dist = d_start[ex as usize][ey as usize];
                    if e_dist >= 0 {
                        let d = mx + (py - ey).abs();
                        let diff=(dist-e_dist).abs()-d;
                        if diff>=100 {
                            if d<=2 {
                                p1 += 1;
                            }
                            p2 += 1;
                        }
                    }
                }
            }
        });
        (p1, p2)
    }).collect::<Vec<_>>();

    let (res, res2) = sols
        .into_iter()
        .fold((0, 0), |(p11, p21), (p12, p22)| (p11 + p12, p21 + p22));

    (res.to_string(), res2.to_string())
}
