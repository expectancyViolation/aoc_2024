use crate::str_map::StrMap;
use std::cmp::PartialEq;
use std::time::Instant;

type Bitmap = [u128; 128];

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct CollisionBitFront {
    curr_mask: u128,
    curr_row: i32,
}

fn step_frontier(
    blocked: &Bitmap,
    boxes: &Bitmap,
    bitfront: &mut CollisionBitFront,
    up: bool,
    part2: bool,
) -> bool {
    bitfront.curr_row += if up { -1 } else { 1 };
    let res = (blocked[bitfront.curr_row as usize] & bitfront.curr_mask) == 0;
    let mut collided = (boxes[bitfront.curr_row as usize] & bitfront.curr_mask);
    if part2 {
        collided |= boxes[bitfront.curr_row as usize] & (bitfront.curr_mask >> 1);
    }
    bitfront.curr_mask = collided;
    if part2 {
        bitfront.curr_mask |= (collided << 1);
    }
    res
}

fn can_move(
    blocked: &Bitmap,
    boxes: &Bitmap,
    robot: &(i32, i32),
    d: Direction,
    part2: bool,
) -> bool {
    let mut curr_row = robot.0;
    let mut curr_mask = 1 << robot.1;
    let mut bf = CollisionBitFront {
        curr_mask,
        curr_row,
    };
    while bf.curr_mask != 0 {
        if !step_frontier(blocked, &boxes, &mut bf, d == Direction::UP, part2) {
            return false;
        }
    }
    true
}

fn move_left(blocked: &Bitmap, boxes: &mut Bitmap, robot: &mut (i32, i32), part2: bool) {
    let mut row_boxes = boxes[robot.0 as usize];
    let mut j = 1 << robot.1;
    // TODO: replace by bit-twiddling "next unset bit"?
    loop {
        j >>= 1;
        let collision = (j & row_boxes != 0) || (part2 && (j & (row_boxes << 1) != 0));
        if !collision {
            break;
        }
    }
    if blocked[robot.0 as usize] & j != 0 {
        return;
    }
    robot.1 = robot.1 - 1;
    let move_mask = ((1 << (robot.1 + 1)) - 1) ^ (j - 1);
    let moving_boxes = move_mask & boxes[robot.0 as usize];
    boxes[robot.0 as usize] &= !move_mask;
    boxes[robot.0 as usize] |= moving_boxes >> 1;
}

fn move_right(blocked: &Bitmap, boxes: &mut Bitmap, robot: &mut (i32, i32), part2: bool) {
    let mut row_boxes = boxes[robot.0 as usize];
    let mut j = 1 << robot.1;

    // TODO: replace by bit-twiddling "next unset bit"?
    loop {
        j <<= 1;
        let collision = (j & row_boxes != 0) || (part2 && (j & (row_boxes << 1) != 0));
        if !collision {
            break;
        }
    }
    if blocked[robot.0 as usize] & j != 0 {
        return;
    }
    robot.1 = robot.1 + 1;
    let move_mask = ((j << 1) - 1) ^ ((1 << robot.1) - 1);
    let moving_boxes = move_mask & boxes[robot.0 as usize];
    boxes[robot.0 as usize] &= !move_mask;
    boxes[robot.0 as usize] |= moving_boxes << 1;
}

fn move_(blocked: &Bitmap, boxes: &mut Bitmap, robot: &mut (i32, i32), d: Direction, part2: bool) {
    if d == Direction::LEFT {
        move_left(blocked, boxes, robot, part2);
        return;
    }
    if d == Direction::RIGHT {
        move_right(blocked, boxes, robot, part2);
        return;
    }
    if !can_move(blocked, boxes, robot, d, part2) {
        return;
    }
    let curr_row = robot.0;
    let curr_mask = 1 << robot.1;
    let mut bf = CollisionBitFront {
        curr_mask,
        curr_row,
    };
    let mut prev_boxes = 0;
    while bf.curr_mask != 0 {
        let prev_mask = bf.curr_mask;
        step_frontier(blocked, &boxes, &mut bf, d == Direction::UP, part2);
        let old_boxes = prev_boxes;
        prev_boxes = boxes[bf.curr_row as usize];
        boxes[bf.curr_row as usize] &= !bf.curr_mask;
        boxes[bf.curr_row as usize] |= old_boxes & prev_mask;
    }
    match d {
        Direction::UP => {
            robot.0 -= 1;
        }
        Direction::DOWN => {
            robot.0 += 1;
        }
        Direction::LEFT => {
            robot.1 -= 1;
        }
        Direction::RIGHT => {
            robot.1 += 1;
        }
    }
}


fn solve_map(m: &StrMap, moves: &[u8], part2: bool) -> i64 {
    let mut box_locations = [0u128; 128];
    let mut blocked_locations = [0u128; 128];
    let mut robot = (0, 0);
    for i in 0..m.h {
        for j in 0..m.w {
            match m.get(i, j) as char {
                '#' => {
                    blocked_locations[i as usize] |= (1 << j);
                }
                'O' | '[' => {
                    box_locations[i as usize] |= (1 << j);
                }
                '@' => {
                    robot = (i, j);
                }
                _ => {}
            }
        }
    }

    for &m_ in moves {
        let d = match (m_ as char) {
            '^' => Direction::UP,
            '>' => Direction::RIGHT,
            'v' => Direction::DOWN,
            '<' => Direction::LEFT,
            _ => continue,
        };
        move_(&blocked_locations, &mut box_locations, &mut robot, d, part2);
        //draw(&blocked_locations, &mut box_locations, &robot, part2);
    }

    let mut res = 0;
    for (i, row) in box_locations.iter().enumerate() {
        for j in 0..128 {
            if row & (1 << j) != 0 {
                res += 100 * i + j;
            }
        }
    }
    res as i64
}

pub(crate) fn solve(data: &str) -> (i64, i64) {
    let split = data.find("\n\n").unwrap();
    let width = data.lines().next().unwrap().len();
    let height = (split) / width;

    let mut p1_data = data[..split + 2].to_owned().into_bytes();
    let mut p2_data = p1_data
        .iter()
        .flat_map(|&x| {
            let res = match x as char {
                '.' => "..",
                'O' => "[]",
                '#' => "##",
                '@' => "@.",
                '\n' => "\n",
                _ => "",
            };
            res.as_bytes().iter().cloned()
        })
        .collect::<Vec<u8>>();

    let m1 = StrMap {
        data: p1_data.as_mut_slice(),
        h: height as i32,
        w: (width) as i32,
    };

    let m2 = StrMap {
        data: p2_data.as_mut_slice(),
        h: height as i32,
        w: (width * 2) as i32,
    };
    let p1 = solve_map(&m1, data[split..].as_bytes(), false);
    let p2 = solve_map(&m2, data[split..].as_bytes(), true);

    (p1, p2)
}

// fn draw(blocked: &Bitmap, boxes: &Bitmap, robot: &(i32, i32), part2: bool) {
//     let mut cmap = [['.'; 128]; 128];
//     for (i, (bl, bo)) in blocked.iter().zip(boxes.iter()).enumerate() {
//         assert_eq!(bl & bo, 0);
//         for j in 0..128 {
//             if bl & (1 << j) != 0 {
//                 cmap[i][j] = '#';
//             }
//             if bo & (1 << j) != 0 {
//                 if part2 {
//                     cmap[i][j] = '[';
//                     cmap[i][j + 1] = ']';
//                 } else {
//                     cmap[i][j] = 'O';
//                 }
//             }
//         }
//     }
//     cmap[robot.0 as usize][robot.1 as usize] = '@';
//     for row in cmap[..20].iter() {
//         println!("{}", row[..20].iter().collect::<String>());
//     }
// }
