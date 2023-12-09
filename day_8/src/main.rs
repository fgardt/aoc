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
    fn test_part1_1() {
        let input = include_str!("../test1.txt");
        assert_eq!(lib::part1(input), 2);
    }

    #[test]
    fn test_part1_2() {
        let input = include_str!("../test2.txt");
        assert_eq!(lib::part1(input), 6);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test3.txt");
        assert_eq!(lib::part2(input), 6);
    }
}
