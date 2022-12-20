use std::ops::{Add, Sub};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> isize {
    let mut ch = chamber::Chamber::default();
    let mut r = rock::generator();
    let mut j = chamber::jetstreams(input);

    ch.run::<2022>(&mut r, &mut j);

    ch.height()
}

fn part2(input: &'static str) -> isize {
    let mut ch = chamber::Chamber::default();
    let mut r = rock::generator();
    let mut j = chamber::jetstreams(input);

    ch.run::<1000000000000>(&mut r, &mut j);

    ch.height()
}

mod chamber;
mod rock;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, PartialOrd, Ord)]
pub struct Xy(isize, isize);

impl Xy {
    pub fn x(&self) -> isize {
        self.0
    }

    pub fn y(&self) -> isize {
        self.1
    }
}

impl Add for Xy {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Xy(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Xy {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Xy(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 3068);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 1514285714288);
    }
}
