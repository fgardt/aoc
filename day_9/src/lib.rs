#[inline]
#[must_use]
pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let mut layers = vec![nums];

            while layers.last().unwrap().iter().any(|num| *num != 0) {
                let layer = layers.last().unwrap();
                let it0 = layer.iter();
                let it1 = layer.iter().skip(1);

                layers.push(it0.zip(it1).map(|(a, b)| b - a).collect::<Vec<_>>());
            }

            for i in (0..layers.len() - 1).rev() {
                let num = layers[i].last().unwrap() + layers[i + 1].last().unwrap();
                layers[i].push(num);
            }

            *layers[0].last().unwrap()
        })
        .sum()
}

#[inline]
#[must_use]
pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let mut layers = vec![nums];

            while layers.last().unwrap().iter().any(|num| *num != 0) {
                let layer = layers.last().unwrap();
                let it0 = layer.iter();
                let it1 = layer.iter().skip(1);

                layers.push(it0.zip(it1).map(|(a, b)| b - a).collect::<Vec<_>>());
            }

            for i in (0..layers.len() - 1).rev() {
                let num = layers[i].first().unwrap() - layers[i + 1].first().unwrap();
                layers[i].insert(0, num);
            }

            *layers[0].first().unwrap()
        })
        .sum()
}
