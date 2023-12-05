use std::collections::HashSet;

#[inline]
#[must_use]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut win_nums = HashSet::with_capacity(10);
            let mut matches = 0;

            for i in 0..10 {
                let pos = 10 + i * 3;
                win_nums.insert(line[pos..pos + 2].trim().parse::<u8>().unwrap());
            }

            for i in 0..25 {
                let pos = 42 + i * 3;
                if win_nums.contains(&line[pos..pos + 2].trim().parse::<u8>().unwrap()) {
                    matches += 1;
                }
            }

            if matches == 0 {
                matches
            } else {
                1 << (matches - 1)
            }
        })
        .sum()
}

#[inline]
#[must_use]
pub fn part2(input: &str) -> usize {
    const MAX_CARD: usize = 186;

    let card_matches = input
        .lines()
        .map(|line| {
            let card_num = line[5..8].trim().parse::<usize>().unwrap();

            let mut win_nums = HashSet::with_capacity(10);
            let mut matches = 0;

            for i in 0..10 {
                let pos = 10 + i * 3;
                win_nums.insert(line[pos..pos + 2].trim().parse::<u8>().unwrap());
            }

            for i in 0..25 {
                let pos = 42 + i * 3;
                if win_nums.contains(&line[pos..pos + 2].trim().parse::<u8>().unwrap()) {
                    matches += 1;
                }
            }

            if (card_num + matches) > MAX_CARD {
                matches = MAX_CARD - card_num;
            }

            matches
        })
        .collect::<Vec<_>>();

    let mut total_cards = vec![1; MAX_CARD];

    for (i, matches) in card_matches.iter().enumerate() {
        let factor = total_cards[i];

        for j in 1..=*matches {
            total_cards[i + j] += factor;
        }
    }

    total_cards.iter().sum()
}
