static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let mut mn = notes::MonkeyNotes::from(input);
    mn.nav();
    mn.password()
}

fn part2(_input: &'static str) -> usize {
    todo!()
}

mod space;

mod notes;

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 6032);
    }
}
