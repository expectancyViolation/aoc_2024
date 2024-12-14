use std::cmp::min;
use std::collections::{HashMap, HashSet};
use crate::day11::n_digs;

fn step(n: i128) -> (i128, i128) {
    if n == 0 {
        return (1, -1);
    };
    let digs = n_digs(n);
    match digs % 2 {
        0 => {
            let ten_pow = 10_i128.pow((digs / 2) as u32);
            (n / ten_pow, n % ten_pow)
        }
        _ => (n.checked_mul(2024).unwrap(), -1)
    }
}


// this abomination is a non-recursive version of tarjans scc algorithm
// at least it's reasonably fast
pub(crate) fn tarjan(n: i128) {
    let mut components: HashMap<i128, usize> = HashMap::new();
    let mut component_count = 0;
    let mut call_stack = Vec::new();
    let mut call_stack_pos = Vec::new();
    let mut call_stack_did_recurse = Vec::new();
    let mut tarjan_stack = Vec::new();
    let mut index = 0;

    let mut indices: HashMap<i128, i32> = HashMap::new();
    let mut onstack: HashSet<i128> = HashSet::new();
    let mut lowlinks: HashMap<i128, i32> = HashMap::new();
    for i in 0..n {
        if indices.contains_key(&i) {
            continue;
        }
        onstack.clear();
        lowlinks.clear();
        call_stack.push(i);
        call_stack_pos.push(0);
        call_stack_did_recurse.push(false);
        while call_stack.len() > 0 {
            let l = call_stack.len();
            let j = call_stack[l - 1];
            let cp = call_stack_pos[l - 1].clone();


            call_stack_pos[l - 1] += 1;
            if cp == 0 {
                indices.insert(j, index);
                lowlinks.insert(j, index);
                index += 1;
                tarjan_stack.push(j);
                onstack.insert(j);
            }
            if cp == 1 || cp == 2 {
                let k = if cp == 1 { step(j).0 } else { step(j).1 };
                if k >= 0 {
                    if call_stack_did_recurse[l - 1] {
                        lowlinks.insert(j, min(lowlinks[&j], lowlinks[&k]));
                    } else {
                        if onstack.contains(&k) {
                            lowlinks.insert(j, min(lowlinks[&j], indices[&k]));
                        }
                    }
                }
            }
            if cp < 2 {
                let k = if cp == 0 { step(j).0 } else { step(j).1 };
                call_stack_did_recurse[l - 1] = false;
                if (k >= 0) && !indices.contains_key(&k) {
                    call_stack_did_recurse[l - 1] = true;
                    call_stack.push(k);
                    call_stack_pos.push(0);
                    call_stack_did_recurse.push(false);
                }
            } else {
                if lowlinks[&j] == indices[&j] {
                    component_count += 1;
                    let mut css = 0;
                    let mut component =HashSet::new();
                    loop {
                        let w = tarjan_stack.pop().unwrap();
                        component.insert(w);
                        css += 1;
                        onstack.remove(&w);
                        components.insert(w, component_count);
                        if w == j {
                            break;
                        }
                    }
                    if css>1 {
                        println!("new comp {} {:?}", css,component);
                    }
                }
                call_stack.pop();
                call_stack_pos.pop();
                call_stack_did_recurse.pop();
            }
        }
    }
    //println!("components {:?}", components);
}


#[cfg(test)]
mod tests {
    use crate::day11_exploration::tarjan;

    #[test]
    fn can_tarjan() {
        tarjan(10_000_000);
    }
}