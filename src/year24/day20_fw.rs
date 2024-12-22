use std::collections::BTreeSet;
use crate::str_map::{StrMap, DIRECTIONS};
use crate::util::FenwickTree;
use itertools::Itertools;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

const PADDING: i32 = 30;

const P2_DIST: i32 = 20;

fn bfs(start: (i32, i32), m: &StrMap) -> Vec<Vec<i32>> {
    let mut distances = vec![vec![-1; (2 * PADDING + m.w) as usize]; (2 * PADDING + m.h) as usize];
    let mut frontier = vec![start];
    distances[(PADDING + start.0) as usize][(PADDING + start.1) as usize] = 0;
    while !frontier.is_empty() {
        let mut nf = Vec::new();
        for (x, y) in frontier {
            for (dx, dy) in DIRECTIONS {
                let (nx, ny) = (x + dx, y + dy);
                let sym = m.get(nx, ny);
                if distances[(PADDING + nx) as usize][(PADDING + ny) as usize] == -1
                    && (sym == b'.')
                {
                    nf.push((nx, ny));
                    distances[(PADDING + nx) as usize][(PADDING + ny) as usize] =
                        distances[(PADDING + x) as usize][(PADDING + y) as usize] + 1;
                }
            }
        }
        frontier = nf;
    }
    distances
}

/*
pub(crate) fn solve(data: &str) -> (String, String) {
    let w = data.lines().next().unwrap().len() as i32;
    let h = data.len() as i32 / (w + 1);
    println!("{} {}", w, h);
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
    let max_dist = d_start.iter().flatten().max().unwrap();


    const POT: i32 = 300;
    let mut res = 0;
    for x in PADDING..(w + PADDING) {
        //println!("x={}",x);
        let x = x as i32;
        // let mut fw = FenwickTree::new((POT + max_dist + 10) as usize);
        let mut in_diamond = vec![];
        for y in PADDING..w + PADDING {
            //println!("y={}",y);
            // add new frontier
            for dx in -P2_DIST..P2_DIST+1 {
                let pdist = d_start[(x + dx) as usize][y as usize];
                if pdist >= 0 {
                    in_diamond.push(pdist - y+dx.abs());
                }
            }
            // remove old frontier
            for dx in -P2_DIST..P2_DIST+1 {
                let dy = P2_DIST + 1 - dx.abs();
                let pdist = d_start[(x + dx) as usize][(y - dy) as usize];
                if pdist >= 0 {
                    in_diamond.remove(in_diamond.iter().position(|x| *x == pdist - y + dy + dx.abs()).expect("needle not found"));
                }
            }
            let cdist = d_start[x as usize][y as usize]-y;
            let saving = in_diamond.iter().filter(|&&x| (cdist-x).abs()>=100).count();
            //let saving=0;
            res += saving;
        }
    }

    ("".to_string(),res.to_string())
}
*/