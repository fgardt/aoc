use std::collections::HashSet;

#[inline]
#[must_use]
pub fn part1(input: &str) -> usize {
    let mut res = 0;

    let lines = input.lines().collect::<Vec<_>>();

    for (l_idx, line) in lines.iter().enumerate() {
        for (c_idx, c) in line.chars().enumerate() {
            if c == '.' || c.is_ascii_digit() {
                continue;
            }

            // found a symbol

            for sl_idx in l_idx as i64 - 1..=l_idx as i64 + 1 {
                if sl_idx < 0 || sl_idx >= lines.len() as i64 {
                    continue;
                }

                let sl_idx = sl_idx as usize;
                let line = &lines[sl_idx];
                let line_c = line.chars().collect::<Vec<_>>();

                let mut used_dig_idx = HashSet::new();

                for sc_idx in c_idx as i64 - 1..=c_idx as i64 + 1 {
                    if sc_idx < 0 || sc_idx >= line.len() as i64 {
                        continue;
                    }

                    let sc_idx = sc_idx as usize;

                    if sl_idx == l_idx && sc_idx == c_idx {
                        continue;
                    }

                    if !line_c[sc_idx].is_ascii_digit() {
                        continue;
                    }

                    if used_dig_idx.contains(&sc_idx) {
                        continue;
                    }

                    // found a digit around the symbol
                    let mut dig_min_idx = sc_idx;
                    let mut dig_max_idx = sc_idx;

                    for dig_idx in (0..sc_idx).rev() {
                        if !line_c[dig_idx].is_ascii_digit() {
                            break;
                        }

                        dig_min_idx = dig_idx;
                    }

                    for (dig_idx, c) in line_c.iter().enumerate().skip(sc_idx + 1) {
                        if !c.is_ascii_digit() {
                            break;
                        }

                        dig_max_idx = dig_idx;
                    }

                    for dig_idx in dig_min_idx..=dig_max_idx {
                        used_dig_idx.insert(dig_idx);
                    }

                    res += line[dig_min_idx..=dig_max_idx].parse::<usize>().unwrap();
                }
            }
        }
    }

    res
}

#[inline]
#[must_use]
pub fn part2(input: &str) -> usize {
    let mut res = 0;

    let lines = input.lines().collect::<Vec<_>>();

    for (l_idx, line) in lines.iter().enumerate() {
        'char: for (c_idx, c) in line.chars().enumerate() {
            if c != '*' {
                continue;
            }

            // found a potential gear

            let mut ratio = 1;
            let mut part_numbers = 0;

            for sl_idx in l_idx as i64 - 1..=l_idx as i64 + 1 {
                if sl_idx < 0 || sl_idx >= lines.len() as i64 {
                    continue;
                }

                let sl_idx = sl_idx as usize;
                let line = &lines[sl_idx];
                let line_c = line.chars().collect::<Vec<_>>();

                let mut used_dig_idx = HashSet::new();

                for sc_idx in c_idx as i64 - 1..=c_idx as i64 + 1 {
                    if sc_idx < 0 || sc_idx >= line.len() as i64 {
                        continue;
                    }

                    let sc_idx = sc_idx as usize;

                    if sl_idx == l_idx && sc_idx == c_idx {
                        continue;
                    }

                    if !line_c[sc_idx].is_ascii_digit() {
                        continue;
                    }

                    if used_dig_idx.contains(&sc_idx) {
                        continue;
                    }

                    // found a digit around the symbol

                    if part_numbers == 2 {
                        continue 'char;
                    }

                    let mut dig_min_idx = sc_idx;
                    let mut dig_max_idx = sc_idx;

                    for dig_idx in (0..sc_idx).rev() {
                        if !line_c[dig_idx].is_ascii_digit() {
                            break;
                        }

                        dig_min_idx = dig_idx;
                    }

                    for (dig_idx, c) in line_c.iter().enumerate().skip(sc_idx + 1) {
                        if !c.is_ascii_digit() {
                            break;
                        }

                        dig_max_idx = dig_idx;
                    }

                    for dig_idx in dig_min_idx..=dig_max_idx {
                        used_dig_idx.insert(dig_idx);
                    }

                    part_numbers += 1;
                    ratio *= line[dig_min_idx..=dig_max_idx].parse::<usize>().unwrap();
                }
            }

            if part_numbers != 2 {
                continue 'char;
            }

            res += ratio;
        }
    }

    res
}
