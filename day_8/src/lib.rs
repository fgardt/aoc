use std::collections::HashMap;

#[inline]
#[must_use]
pub fn part1(input: &str) -> usize {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let direction = parts[0].chars().collect::<Vec<_>>();
    let nodes = parts[1]
        .lines()
        .map(|line| {
            let id = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];

            (id, (left, right))
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    let mut current = "AAA";
    let mut steps = 0;

    while current != "ZZZ" {
        let dir = direction[steps % direction.len()];
        let (left, right) = nodes.get(current).unwrap();

        current = match dir {
            'L' => left,
            'R' => right,
            _ => unreachable!(),
        };

        steps += 1;
    }

    steps
}

#[inline]
#[must_use]
pub fn part2(input: &str) -> usize {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let direction = parts[0].chars().collect::<Vec<_>>();
    let nodes = parts[1]
        .lines()
        .map(|line| {
            let id = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];

            (id, (left, right))
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    let mut current = nodes
        .keys()
        .filter_map(|k| if &k[2..3] == "A" { Some(*k) } else { None })
        .collect::<Vec<_>>();

    let mut steps = 0;
    let mut loop_end = HashMap::<&str, usize>::new();

    loop {
        let dir = direction[steps % direction.len()];

        for c in &mut current {
            if &c[2..3] == "Z" {
                loop_end.entry(c).or_insert(steps);
                continue;
            }

            let (left, right) = nodes.get(c).unwrap();

            *c = match dir {
                'L' => *left,
                'R' => *right,
                _ => unreachable!(),
            };
        }

        if loop_end.len() == current.len() {
            break;
        }

        steps += 1;
    }

    let mut it = loop_end.values();
    let mut lcm = *it.next().unwrap();

    for len in it {
        lcm = num::integer::lcm(lcm, *len);
    }

    lcm
}
