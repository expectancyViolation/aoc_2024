use crate::str_map::DIRECTIONS;
use hashbrown::HashSet;
use itertools::Itertools;
use std::time::Instant;

const W: i32 = 71;

fn bfs(start: (i32, i32), target: (i32, i32), blocked: &Vec<Vec<bool>>) -> i32 {
    let mut distances = vec![vec![-1; W as usize]; W as usize];
    let mut frontier = vec![start];
    distances[start.0 as usize][start.1 as usize] = 0;
    loop {
        let mut nf = Vec::new();
        for (x, y) in frontier {
            let dist = distances[x as usize][y as usize];
            for (dx, dy) in DIRECTIONS {
                let (nx, ny) = (x + dx, y + dy);
                if !(0 <= nx && nx < W && 0 <= ny && ny < W) {
                    continue;
                }
                if distances[nx as usize][ny as usize] == -1 && !blocked[nx as usize][ny as usize] {
                    nf.push((nx, ny));
                    distances[nx as usize][ny as usize] = dist + 1;
                }
            }
        }
        if distances[target.0 as usize][target.1 as usize] != -1 {
            return distances[target.0 as usize][target.1 as usize];
        }
        frontier = nf;
    }
}

fn dfs(frontier: &mut Vec<(i32, i32)>, blocked: &Vec<Vec<bool>>, visited: &mut Vec<Vec<bool>>) {
    while !frontier.is_empty() {
        let (x, y) = frontier.pop().unwrap();
        visited[x as usize][y as usize] = true;
        for (dx, dy) in DIRECTIONS {
            let (nx, ny) = (x + dx, y + dy);
            if !(0 <= nx && nx < W && 0 <= ny && ny < W) {
                continue;
            }
            if !visited[nx as usize][ny as usize] && !blocked[nx as usize][ny as usize] {
                if visited[nx as usize][ny as usize] {
                    continue;
                }
                frontier.push((nx, ny));
            }
        }
    }
}

pub(crate) fn solve(data: &str) -> (i64, i64) {
    let mut blocked = vec![vec![false; W as usize]; W as usize];
    let blocks = data
        .lines()
        .map(|l| {
            l.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    for (x, y) in blocks[..1024].iter() {
        blocked[*x][*y] = true;
    }
    let p1 = bfs((0, 0), (W - 1, W - 1), &blocked);
    // let first_double_even = 1026 + blocks[1024..].iter().position(|(x, y)| (x % 2) == 0 && (y % 2) == 0).unwrap();
    for (x, y) in blocks[1024..].iter() {
        blocked[*x][*y] = true;
    }
    let mut visited = vec![vec![false; W as usize]; W as usize];

    let mut frontier = vec![(0, 0)];
    let mut prev_block = blocks[0];
    for remove_block in blocks.iter().rev() {
        let &(rx, ry) = remove_block;
        blocked[rx][ry] = false;
        dfs(&mut frontier, &mut blocked, &mut visited);
        for (dx, dy) in DIRECTIONS {
            let (nrx, nry) = (rx as i32 + dx, ry as i32 + dy);
            if !(0 <= nrx && nrx < W && 0 <= nry && nry < W) {
                continue;
            }
            if visited[nrx as usize][nry as usize] {
                frontier.push((rx as i32, ry as i32));
            }
        }
        if visited[W as usize - 1][W as usize - 1] {
            break;
        }
        prev_block = *remove_block;
    }

    (p1 as i64, (prev_block.0 * 10000 + prev_block.1) as i64)
}
