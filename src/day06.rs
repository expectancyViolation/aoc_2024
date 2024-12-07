use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

const FACINGS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Day06Guard {
    x: i32,
    y: i32,
    facing_index: usize,
}

impl Day06Guard {
    fn step(&mut self, occupied: &Vec<Vec<bool>>, blocked: Option<(i32, i32)>) {
        let (dx, dy) = FACINGS[self.facing_index];
        let (nx, ny) = (self.x + dx, self.y + dy);

        if blocked.is_some_and(|t| t == (nx, ny))
            || (Self::is_in_bounds(nx, ny, &occupied) && occupied[nx as usize][ny as usize])
        {
            self.facing_index = (self.facing_index + 1) % 4;
        } else {
            self.x = nx;
            self.y = ny;
        }
    }

    fn is_in_bounds(x: i32, y: i32, occupied: &Vec<Vec<bool>>) -> bool {
        (0 <= x) && (x < occupied.len() as i32) && (0 <= y) && (y < occupied[0].len() as i32)
    }

    fn simulate(
        &mut self,
        occupied: &Vec<Vec<bool>>,
        blocked: Option<(i32, i32)>,
    ) -> (HashSet<(i32, i32)>, bool) {
        let mut visited = HashSet::new();
        while Self::is_in_bounds(self.x, self.y, &occupied) && !visited.contains(self) {
            visited.insert(self.clone());
            self.step(&occupied, blocked);
        }
        let visited_positions = visited
            .iter()
            .map(|state| (state.x, state.y))
            .collect::<HashSet<_>>();
        let looped = Self::is_in_bounds(self.x, self.y, &occupied);
        (visited_positions, looped)
    }
}

fn parse(data: &str) -> (Day06Guard, Vec<Vec<bool>>) {
    let mut start_pos = (0, 0);
    let h = data.lines().count();
    let w = data.lines().next().unwrap().len();
    let mut occupied = vec![vec![false; w]; h];
    data.lines().enumerate().for_each(|(x, line)| {
        for (y, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    occupied[x][y] = true;
                }
                '^' => start_pos = (x as i32, y as i32),
                _ => {}
            }
        }
    });
    (
        Day06Guard {
            x: start_pos.0,
            y: start_pos.1,
            facing_index: 0,
        },
        occupied,
    )
}

pub fn solve(data: &str) -> (i64, i64) {
    let (state, occupied) = parse(&data);

    let (p1, _looped) = state.clone().simulate(&occupied, None);
    let r1 = p1.len() as i64;
    let to_check: Vec<(i32, i32)> = p1.into_iter().collect_vec();
    let p2 = to_check
        .par_iter()
        .filter(|&&(x, y)| {
            let (_, looped) = state.clone().simulate(&occupied, Some((x, y)));
            looped
        }).count();
    (r1, p2 as i64)
}
