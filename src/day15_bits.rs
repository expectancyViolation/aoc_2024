use bnum::{BUint};

type UI128 = BUint<2>; // 94*64=6016 bit


const N_ROWS: usize = 100;
type Bitmap = [UI128; N_ROWS];

struct CollisionBitFront {
    curr_mask: UI128,
    curr_row: usize,
}


fn step_frontier(
    blocked: &Bitmap,
    boxes: &Bitmap,
    bitfront: &mut CollisionBitFront,
    up: bool,
    part2: bool,
) -> bool {
    bitfront.curr_row = if up { bitfront.curr_row - 1 } else { bitfront.curr_row + 1 };
    let res = (blocked[bitfront.curr_row] & bitfront.curr_mask) == UI128::ZERO;
    let collided =
        if part2 {
            boxes[bitfront.curr_row] & (bitfront.curr_mask | (bitfront.curr_mask >> 1))
        } else {
            boxes[bitfront.curr_row] & bitfront.curr_mask
        };
    bitfront.curr_mask = if part2 { collided | (collided << 1) } else { collided };
    res
}

fn can_move_vertical(blocked: &Bitmap, boxes: &Bitmap, robot: &(i32, i32), up: bool, part2: bool) -> bool {
    let mut curr_row = robot.0 as usize;
    let mut curr_mask = UI128::ONE << robot.1;
    let mut bf = CollisionBitFront {
        curr_mask,
        curr_row,
    };
    while bf.curr_mask != UI128::ZERO {
        if !step_frontier(blocked, &boxes, &mut bf, up, part2) {
            return false;
        }
    }
    true
}

fn move_left(blocked: &Bitmap, boxes: &mut Bitmap, robot: &mut (i32, i32), part2: bool) {
    let mut row_boxes = boxes[robot.0 as usize];
    let mut j = UI128::ONE << robot.1;
    // TODO: replace by bit-twiddling "next unset bit"?
    loop {
        j >>= 1;
        let collision = (j & row_boxes != UI128::ZERO) || (part2 && (j & (row_boxes << 1) != UI128::ZERO));
        if !collision {
            break;
        }
    }
    if blocked[robot.0 as usize] & j != UI128::ZERO {
        return;
    }
    robot.1 = robot.1 - 1;
    let move_mask = ((UI128::ONE << (robot.1 + 1)) - UI128::ONE) ^ (j - UI128::ONE);
    let moving_boxes = move_mask & boxes[robot.0 as usize];
    boxes[robot.0 as usize] &= !move_mask;
    boxes[robot.0 as usize] |= moving_boxes >> 1;
}

fn move_right(blocked: &Bitmap, boxes: &mut Bitmap, robot: &mut (i32, i32), part2: bool) {
    let mut row_boxes = boxes[robot.0 as usize];
    let mut j = UI128::ONE << robot.1;

    // TODO: replace by bit-twiddling "next unset bit"?
    loop {
        j <<= 1;
        let collision = (j & row_boxes != UI128::ZERO) || (part2 && (j & (row_boxes << 1) != UI128::ZERO));
        if !collision {
            break;
        }
    }
    if blocked[robot.0 as usize] & j != UI128::ZERO {
        return;
    }
    robot.1 = robot.1 + 1;
    let move_mask: UI128 = ((j << 1) - UI128::ONE) ^ ((UI128::ONE << robot.1) - UI128::ONE);
    let moving_boxes = move_mask & boxes[robot.0 as usize];
    boxes[robot.0 as usize] &= !move_mask;
    boxes[robot.0 as usize] |= moving_boxes << 1;
}

fn move_vertical(
    blocked: &Bitmap,
    boxes: &mut Bitmap,
    robot: &mut (i32, i32),
    up: bool,
    part2: bool,
) {
    if !can_move_vertical(blocked, boxes, robot, up, part2) {
        return;
    }
    let curr_row = robot.0 as usize;
    let curr_mask = UI128::ONE << robot.1;
    let mut bf = CollisionBitFront {
        curr_mask,
        curr_row,
    };
    let mut prev_boxes = UI128::ZERO;
    while bf.curr_mask != UI128::ZERO {
        let prev_mask = bf.curr_mask;
        step_frontier(blocked, &boxes, &mut bf, up, part2);
        let old_boxes = prev_boxes;
        prev_boxes = boxes[bf.curr_row];
        boxes[bf.curr_row] &= !bf.curr_mask;
        boxes[bf.curr_row] |= old_boxes & prev_mask;
    }
    robot.0 += if up { -1 } else { 1 };
}

fn solve_map(
    box_locations: &mut Bitmap,
    blocked_locations: &Bitmap,
    robot: (i32, i32),
    moves: &[u8],
    part2: bool,
) -> i64 {
    let mut robot = robot;

    for &m_ in moves {
        match m_ as char {
            '^' => move_vertical(blocked_locations, box_locations, &mut robot, true, part2),
            '>' => move_right(blocked_locations, box_locations, &mut robot, part2),
            'v' => move_vertical(blocked_locations, box_locations, &mut robot, false, part2),
            '<' => move_left(blocked_locations, box_locations, &mut robot, part2),
            _ => continue,
        };
        //draw(&blocked_locations, &mut box_locations, &robot, part2);
    }

    let mut res = 0;
    for (i, row) in box_locations.iter().enumerate() {
        for j in 0..128 {
            if row & (UI128::ONE << j) != UI128::ZERO {
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

    let mut box_locations: Bitmap = [UI128::ZERO; N_ROWS];
    let mut blocked_locations: Bitmap = [UI128::ZERO; N_ROWS];

    let mut box2_locations: Bitmap = [UI128::ZERO; N_ROWS];
    let mut blocked2_locations: Bitmap = [UI128::ZERO; N_ROWS];

    let mut robot1 = (0, 0);
    let mut robot2 = (0, 0);

    data.lines().take(height).enumerate().for_each(|(i, line)| {
        for (j, c) in line.chars().enumerate() {
            match c {
                'O' => {
                    box_locations[i] |= UI128::ONE << j;
                    box2_locations[i] |= UI128::ONE << (2 * j);
                }
                '#' => {
                    blocked_locations[i] |= UI128::ONE << j;
                    blocked2_locations[i] |= UI128::ONE << (2 * j);
                    blocked2_locations[i] |= UI128::ONE << (2 * j + 1);
                }
                '@' => {
                    robot1 = (i as i32, j as i32);
                    robot2 = (i as i32, 2 * j as i32);
                }
                _ => {}
            }
        }
    });
    let p1 = solve_map(
        &mut box_locations,
        &mut blocked_locations,
        robot1,
        data[split..].as_bytes(),
        false,
    );
    let p2 = solve_map(
        &mut box2_locations,
        &mut blocked2_locations,
        robot2,
        data[split..].as_bytes(),
        true,
    );
    (p1, p2)
}
