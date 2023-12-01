mod lib;

fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", lib::part1(input));
    println!("Part 2: {}", lib::part2(input));
}
