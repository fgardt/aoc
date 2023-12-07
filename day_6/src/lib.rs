#[inline]
#[must_use]
pub fn part1(input: &str) -> usize {
    let data = input
        .lines()
        .map(|line| {
            line[11..]
                .split("  ")
                .filter_map(|x| x.trim().parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let time = data[0].clone();
    let dist = data[1].clone();

    time.iter()
        .zip(dist.iter())
        .map(|(t, d)| {
            let mut solutions = 0;

            for i in 0..=*t {
                if i * (t - i) > *d {
                    solutions += 1;
                }
            }

            solutions
        })
        .product()
}

#[inline]
#[must_use]
pub fn part2(input: &str) -> usize {
    let data = input
        .lines()
        .map(|line| line[11..].replace(' ', "").parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let time = data[0];
    let dist = data[1];

    let mut solutions = 0;
    for i in 0..=time {
        if i * (time - i) > dist {
            solutions += 1;
        }
    }

    solutions
}
