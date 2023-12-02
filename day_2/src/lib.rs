#[inline]
#[must_use]
pub fn part1(input: &str) -> usize {
    const MAX_RED: usize = 12;
    const MAX_BLUE: usize = 14;
    const MAX_GREEN: usize = 13;

    input
        .lines()
        .filter_map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();
            let id = parts
                .first()
                .unwrap()
                .replace("Game ", "")
                .parse::<usize>()
                .unwrap();

            if parts.last().unwrap().split("; ").any(|set| {
                set.split(", ").any(|set_entry| {
                    let num_color = set_entry.split(' ').collect::<Vec<_>>();

                    let count = num_color.first().unwrap().parse::<usize>().unwrap();

                    match *num_color.last().unwrap() {
                        "red" => count > MAX_RED,
                        "blue" => count > MAX_BLUE,
                        "green" => count > MAX_GREEN,
                        _ => unreachable!("invalid color: {}", num_color.last().unwrap()),
                    }
                })
            }) {
                None
            } else {
                Some(id)
            }
        })
        .sum()
}

#[inline]
#[must_use]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();

            let mut min_red = 0;
            let mut min_blue = 0;
            let mut min_green = 0;

            for set in parts.last().unwrap().split("; ") {
                for set_entry in set.split(", ") {
                    let num_color = set_entry.split(' ').collect::<Vec<_>>();

                    let count = num_color.first().unwrap().parse::<usize>().unwrap();

                    match *num_color.last().unwrap() {
                        "red" => {
                            if min_red < count {
                                min_red = count;
                            }
                        }
                        "blue" => {
                            if min_blue < count {
                                min_blue = count;
                            }
                        }
                        "green" => {
                            if min_green < count {
                                min_green = count;
                            }
                        }
                        _ => unreachable!("invalid color: {}", num_color.last().unwrap()),
                    }
                }
            }

            min_red * min_blue * min_green
        })
        .sum()
}
