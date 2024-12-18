use std::cmp::min;

fn area_checksum(file_id: i64, start_pos: i64, len: i64) -> i64 {
    file_id * ((start_pos * len) + (len * (len - 1)) / 2)
}

fn part1(data: &str) -> i64 {
    let mut data = data[..data.len() - 1]
        .bytes()
        .map(|x| (x - '0' as u8) as i64)
        .collect::<Vec<i64>>();
    let mut res = 0;
    let mut file_blocks: i64 = data.iter().step_by(2).sum();
    let mut pos = 0;
    let mut i: usize = 0;
    let mut j: usize = data.len() - 1;
    while file_blocks > 0 {
        if i % 2 == 0 {
            // file
            let file_id = i / 2;
            let file_size = data[i];
            file_blocks -= file_size;
            res += area_checksum(file_id as i64, pos, file_size);
            pos += data[i];
        } else {
            // fill gaps from back
            let mut gap_size = min(data[i], file_blocks);
            file_blocks -= gap_size;
            while gap_size >= data[j] {
                // move whole block
                let file_id = j / 2;
                res += area_checksum(file_id as i64, pos, data[j]);
                pos += data[j];
                gap_size -= data[j];
                j -= 2;
            }
            // move partial block
            data[j] -= gap_size;
            let file_id = j / 2;
            res += area_checksum(file_id as i64, pos, gap_size);
            pos += gap_size;
        }
        i += 1;
    }
    res
}

fn part2(data: &str) -> i64 {
    let mut data = data[..data.len() - 1]
        .bytes()
        .map(|x| (x - '0' as u8) as usize)
        .collect::<Vec<usize>>();
    let mut positions = vec![0];
    positions.extend(data.iter().scan(0, |acc, &x| {
        *acc += x;
        Some(*acc)
    }));
    // i-th entry is index of available gap with size at least i
    let mut gap_indicies = [1; 10];
    let advance = |i, data: &Vec<usize>, gap_inds: &mut [usize; 10]| {
        while gap_inds[i] < data.len() && (data[gap_inds[i]] < i) {
            gap_inds[i] += 2;
        }
    };
    (0..10).for_each(|i| advance(i, &data, &mut gap_indicies));
    let mut final_file_positions = (0..data.len())
        .step_by(2)
        .map(|i| positions[i])
        .collect::<Vec<usize>>();
    let mut j = (data.len() - 1) as i64;
    while j >= 0 {
        let jj = j as usize;
        let file_size = data[jj];
        let file_id = jj / 2;
        let best_gap = gap_indicies
            .iter()
            .enumerate()
            .filter(|&(gap_size, &gap_ind)| (gap_size >= file_size) && (gap_ind < jj))
            .next();
        let bg = best_gap.clone();
        match bg {
            None => { /*cannot find suitable gap*/ }
            Some((_gap_size, &gap_ind)) => {
                final_file_positions[file_id] = positions[gap_ind];
                data[gap_ind] -= file_size;
                positions[gap_ind] += file_size;

                (0..10).for_each(|i| advance(i, &data, &mut gap_indicies));
            }
        }
        j -= 2;
    }

    final_file_positions
        .iter()
        .enumerate()
        .map(|(file_id, &x)| {
            let file_size = data[2 * file_id] as i64;
            area_checksum(file_id as i64, x as i64, file_size)
        })
        .sum()
}

pub(crate) fn solve(data: &str) -> (String, String) {
    (part1(data).to_string(), part2(data).to_string())
}
