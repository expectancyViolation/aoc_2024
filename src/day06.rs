use std::collections::HashSet;
use itertools::iproduct;

const FACINGS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Day06Guard {
    pos: (i64, i64),
    facing_index: usize,
}

impl Day06Guard {
    fn step(&self, occupied: &HashSet<(i64, i64)>) -> Day06Guard {
        let (dx, dy) = FACINGS[self.facing_index];
        let (x, y) = self.pos;
        let (nx, ny) = (x + dx, y + dy);
        if occupied.contains(&(nx, ny)) {
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

    fn is_in_bounds(&self, bounds: &(i64, i64)) -> bool {
        (0 <= self.pos.0)
            && (self.pos.0 <= bounds.0)
            && (0 <= self.pos.1)
            && (self.pos.1 <= bounds.1)
    }

    fn simulate(&self, occupied: &HashSet<(i64, i64)>, bounds: &(i64, i64)) -> (i64, bool) {
        let mut curr_state = self.clone();
        let mut visited = HashSet::new();
        while (curr_state.is_in_bounds(&bounds) && !visited.contains(&curr_state)) {
            visited.insert(curr_state.clone());
            curr_state = curr_state.step(&occupied);
        };
        let visited_positions = visited.iter().map(|state| state.pos).collect::<HashSet<_>>();
        let looped = curr_state.is_in_bounds(&bounds);
        (visited_positions.len() as i64, looped)
    }
}


fn parse(data: &str) -> (Day06Guard, HashSet<(i64, i64)>, (i64, i64)) {
    let mut occupied = HashSet::new();
    let mut start_pos = (0, 0);
    let mut limits = (0, 0);
    data.lines().enumerate().for_each(|(x, line)| {
        for (y, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    occupied.insert((x as i64, y as i64));
                }
                '^' => { start_pos = (x as i64, y as i64) }
                _ => {}
            }
            limits = (x as i64, y as i64);
        }
    });
    (Day06Guard { pos: start_pos, facing_index: 0 }, occupied, limits)
}

pub fn solve(data: &str) -> (i64, i64) {
    let (mut state, mut occupied, limits) = parse(&data);

    let (p1, _looped) = state.simulate(&occupied, &limits);

    let mut p2 = 0;
    for (x, y) in iproduct!(0..limits.0+1,0..limits.1+1) {
        if !occupied.contains(&(x, y)) {
            occupied.insert((x, y));
            let (_, looped) = state.simulate(&occupied, &limits);
            occupied.remove(&(x, y));
            p2 += if looped { 1 } else { 0 };
        }
    }
    (p1, p2)
}