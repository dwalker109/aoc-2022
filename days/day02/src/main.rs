static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    let r1 = part1(INPUT);
    let r2 = part2(INPUT);

    println!("Part 1: {r1}");
    println!("Part 2: {r2}");
}

#[logging_timer::time]
fn part1(input: &'static str) -> usize {
    input
        .lines()
        .map(|l| {
            let b = l.as_bytes();
            match (b[0], b[2]) {
                (b'A', b'X') => 1 + 3,
                (b'A', b'Y') => 2 + 6,
                (b'A', b'Z') => 3 + 0,
                (b'B', b'X') => 1 + 0,
                (b'B', b'Y') => 2 + 3,
                (b'B', b'Z') => 3 + 6,
                (b'C', b'X') => 1 + 6,
                (b'C', b'Y') => 2 + 0,
                (b'C', b'Z') => 3 + 3,
                _ => panic!(),
            }
        })
        .sum()
}

#[logging_timer::time]
fn part2(input: &'static str) -> usize {
    input
        .lines()
        .map(|l| {
            let b = l.as_bytes();
            match (b[0], b[2]) {
                (b'A', b'X') => 3 + 0,
                (b'A', b'Y') => 1 + 3,
                (b'A', b'Z') => 2 + 6,
                (b'B', b'X') => 1 + 0,
                (b'B', b'Y') => 2 + 3,
                (b'B', b'Z') => 3 + 6,
                (b'C', b'X') => 2 + 0,
                (b'C', b'Y') => 3 + 3,
                (b'C', b'Z') => 1 + 6,
                _ => panic!(),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 12);
    }
}
