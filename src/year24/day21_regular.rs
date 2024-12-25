use cached::proc_macro::cached;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::cmp::min;

type Keymap = Vec<Vec<char>>;

fn find_keymap(keymap: &Keymap, c: char) -> (i32, i32) {
    for (i, v) in keymap.iter().enumerate() {
        for (j, c_) in v.iter().enumerate() {
            if c == *c_ {
                return (i as i32, j as i32);
            }
        }
    };
    panic!("Keymap doesn't contain a character!");
}

type CharSeqIt<'a> = Box<dyn Iterator<Item=String> + 'a>;

#[derive(Debug, Copy, Clone)]
enum Directon {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

impl Directon {
    fn to_c(&self) -> char {
        match self {
            Directon::UP => { '^' }
            Directon::LEFT => { '<' }
            Directon::RIGHT => { '>' }
            Directon::DOWN => { 'v' }
        }
    }

    fn to_delta(&self) -> (i32, i32) {
        match self {
            Directon::UP => { (-1, 0) }
            Directon::LEFT => { (0, -1) }
            Directon::RIGHT => { (0, 1) }
            Directon::DOWN => { (1, 0) }
        }
    }
}

fn gen_poss_letter_seqs(letter: char, keymap: &Keymap, start: (i32, i32)) -> CharSeqIt {
    let (x, y) = start;
    let mut curr_moves = Vec::new();
    let (nx, ny) = find_keymap(&keymap, letter);
    let dx = nx - x;
    let dy = ny - y;
    if dy > 0 {
        for _ in 0..dy {
            curr_moves.push(Directon::RIGHT);
        }
    }
    if dx > 0 {
        for _ in 0..dx {
            curr_moves.push(Directon::DOWN);
        }
    }
    if dy < 0 {
        for _ in 0..(-dy) {
            curr_moves.push(Directon::LEFT);
        }
    }
    if dx < 0 {
        for _ in 0..(-dx) {
            curr_moves.push(Directon::UP);
        }
    }
    let l = curr_moves.len();
    let res = Box::new(curr_moves.into_iter().permutations(l).filter_map(move | m| {
        let (mut x, mut y) = start;
        let mut res = "".to_string();
        for d in m.iter() {
            let (dx, dy) = d.to_delta();
            x += dx;
            y += dy;
            let k = keymap[x as usize][y as usize];
            if k == 'X' {
                return None;
            }
            res.push(d.to_c())
        }
        res.push('A');
        Some(res)
    }));
    res
}

lazy_static!(
    static ref dir_keypad:Vec<Vec<char>>= vec![vec!['X', '^', 'A'], vec!['<', 'v', '>']];
    static ref  num_keypad:Vec<Vec<char>>=vec![vec!['7', '8', '9'], vec!['4', '5', '6'], vec!['1', '2', '3'], vec!['X', '0', 'A']];

);


#[cached(
    key = "String",
    convert = r#"{ format!("{}_{}",code,depth) }"#,
)]
pub fn solve_keycode(code: &str, depth: i32, initial: bool) -> i128 {
    if depth == 0 {
        return code.len() as i128;
    }
    let mut res = 0;
    let kp: &Keymap = if initial { &num_keypad } else { &dir_keypad };
    let mut curr_pos = find_keymap(kp, 'A');
    for c in code.chars() {
        let mut c_cost: i128 = i128::MAX;
        for seq in gen_poss_letter_seqs(c, kp, curr_pos) {
            let cost = solve_keycode(seq.as_str(), depth - 1, false);
            c_cost = min(c_cost, cost);
        }
        res += c_cost;
        curr_pos = find_keymap(kp, c);
    };
    res
}


pub fn solve(data: &str) -> (String, String) {
    let mut p1 = 0;
    let mut p2 = 0;
    for inp in data.lines() {
        let numval = inp[..3].parse::<i128>().unwrap();

        let p1_code_len = solve_keycode(inp, 3, true);
        let p2_code_len = solve_keycode(inp, 26, true);
        p1 += numval + p1_code_len;
        p2 += numval + p2_code_len;
    }
    (p1.to_string(), p2.to_string())
}