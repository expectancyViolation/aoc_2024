use crate::str_map::DIRECTIONS;
use hashbrown::HashMap;
use itertools::Itertools;

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

fn dfs(start: (i32, i32), target: (i32, i32), blocked: &Vec<Vec<bool>>) -> HashMap<(i32, i32), (i32, i32)> {
    let mut visited = vec![vec![false; W as usize]; W as usize];
    let mut preds = HashMap::new();
    let mut frontier = vec![start];
    while !frontier.is_empty() {
        let (x, y) = frontier.pop().unwrap();
        visited[x as usize][y as usize] = true;
        for (dx, dy) in DIRECTIONS {
            let (nx, ny) = (x + dx, y + dy);
            if !(0 <= nx && nx < W && 0 <= ny && ny < W) {
                continue;
            }
            if !visited[nx as usize][ny as usize] && !blocked[nx as usize][ny as usize] {
                preds.insert((nx, ny), (x, y));
                if (nx, ny) == target {
                    return preds;
                }
                frontier.push((nx, ny));
            }
        }
    }
    preds
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
    let first_double_even = 1024 + blocks[1024..].iter().position(|(x, y)| (x % 2) == 0 && (y % 2) == 0).unwrap();
    for (x, y) in blocks[1024..first_double_even].iter() {
        blocked[*x][*y] = true;
    }

    let preds = dfs((0, 0), (W - 1, W - 1), &blocked);
    let mut path = Vec::new();
    let mut p = preds.get(&(W - 1, W - 1));
    while p.is_some() {
        path.push(p.unwrap());
        p = preds.get(p.unwrap());
    }

    let p2=blocks[first_double_even..].iter().find(|(x, y)| {
        path.contains(&&(*x as i32, *y as i32))
    }).unwrap();

    (p1 as i64, (p2.0 * 10000 + p2.1) as i64)
}
