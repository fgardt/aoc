mod lib;

fn main() {
    let input = include_str!("../input.txt");

    let time = std::time::Instant::now();
    println!("Part 1: {}", lib::part1(input));
    println!("Time: {}µs", time.elapsed().as_micros());

    let time = std::time::Instant::now();
    println!("Part 2: {}", lib::part2(input));
    println!("Time: {}ms", time.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(lib::part1(input), 35);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        assert_eq!(lib::part2(input), 46);
    }
}
