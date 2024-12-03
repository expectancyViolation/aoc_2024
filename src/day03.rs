use regex::Regex;

struct Day03State {
    disabled_sum: i64,
    enabled_sum: i64,
    is_enabled: bool,
}

impl Default for Day03State {
    fn default() -> Self {
        Day03State { disabled_sum: 0, enabled_sum: 0, is_enabled: true }
    }
}

pub fn solve(input: &str) -> (i64, i64) {
    let mut state: Day03State = Default::default();
    let reg = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();
    reg.captures_iter(input).for_each(|cap| {
        let instr = cap.get(0).unwrap().as_str();
        match instr {
            "do()" => state.is_enabled = true,
            "don't()" => state.is_enabled = false,
            _ => {
                let x = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
                let y = cap.get(3).unwrap().as_str().parse::<i64>().unwrap();
                match state.is_enabled {
                    true => state.enabled_sum = state.enabled_sum + x * y,
                    false => state.disabled_sum = state.disabled_sum + x * y,
                }
            }
        }
    });
    (state.disabled_sum + state.enabled_sum, state.enabled_sum)
}
