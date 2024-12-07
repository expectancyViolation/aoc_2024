use std::collections::HashSet;

const FACINGS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Day06Guard {
    pos: (i32, i32),
    facing_index: usize,
}

impl Day06Guard {
    fn step(&self, occupied: &Vec<Vec<bool>>) -> Day06Guard {
        let (dx, dy) = FACINGS[self.facing_index];
        let (x, y) = self.pos;
        let (nx, ny) = (x + dx, y + dy);

        if Self::is_in_bounds((nx, ny), &occupied) && occupied[nx as usize][ny as usize] {
            Day06Guard {
                pos: self.pos,
                facing_index: (self.facing_index + 1) % 4,
            }
        } else {
            Day06Guard {
                pos: (nx, ny),
                facing_index: self.facing_index,
            }
        }
    }

    fn is_in_bounds(pos: (i32, i32), occupied: &Vec<Vec<bool>>) -> bool {
        (0 <= pos.0)
            && (pos.0 < occupied.len() as i32)
            && (0 <= pos.1)
            && (pos.1 < occupied[0].len() as i32)
    }

    fn simulate(&self, occupied: &Vec<Vec<bool>>) -> (HashSet<(i32, i32)>, bool) {
        let mut curr_state = self.clone();
        let mut visited = HashSet::new();
        while Self::is_in_bounds(curr_state.pos, &occupied) && !visited.contains(&curr_state) {
            visited.insert(curr_state.clone());
            curr_state = curr_state.step(&occupied);
        }
        let visited_positions = visited
            .iter()
            .map(|state| state.pos)
            .collect::<HashSet<_>>();
        let looped = Self::is_in_bounds(curr_state.pos, &occupied);
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
            pos: start_pos,
            facing_index: 0,
        },
        occupied,
    )
}

pub fn solve(data: &str) -> (i64, i64) {
    let (state, mut occupied) = parse(&data);

    let (p1, _looped) = state.simulate(&occupied);

    let mut p2 = 0;
    for &(x, y) in p1.iter() {
        let x = x as usize;
        let y = y as usize;
        if !occupied[x][y] {
            occupied[x][y] = true;
            let (_, looped) = state.simulate(&occupied);
            occupied[x][y] = false;
            p2 += if looped { 1 } else { 0 };
        }
    }
    (p1.len() as i64, p2)
}
