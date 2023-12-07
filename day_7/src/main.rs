mod lib;

fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", lib::part1(input));
    println!("Part 2: {}", lib::part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(lib::part1(input), 6440);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        assert_eq!(lib::part2(input), 5905);
    }
}
