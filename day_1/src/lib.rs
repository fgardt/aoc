#[inline]
#[must_use]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digs = line
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<Vec<_>>();

            // char code for 0 is 48
            let l = *digs.first().unwrap() as u32 - 48;
            let r = *digs.last().unwrap() as u32 - 48;

            l * 10 + r
        })
        .sum()
}

#[inline]
#[must_use]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line = line
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");

            let digs = line
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<Vec<_>>();

            // char code for 0 is 48
            let l = *digs.first().unwrap() as u32 - 48;
            let r = *digs.last().unwrap() as u32 - 48;

            l * 10 + r
        })
        .sum()
}
