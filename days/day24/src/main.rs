use crate::valley::Valley;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let mut valley = Valley::from(input);

    valley.nav(&valley.entrance_pos(), &valley.exit_pos())
}

fn part2(input: &'static str) -> usize {
    let mut valley = Valley::from(input);

    let l1 = valley.nav(&valley.entrance_pos(), &valley.exit_pos());
    let l2 = valley.nav(&valley.exit_pos(), &valley.entrance_pos());
    let l3 = valley.nav(&valley.entrance_pos(), &valley.exit_pos());

    l1 + l2 + l3
}

mod coords;
mod matter;
mod valley;

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 18);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 54);
    }
}
