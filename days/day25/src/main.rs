use std::fmt::{Display, Formatter};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || Snafu::new("n/a"));
}

fn part1(input: &'static str) -> Snafu {
    input
        .lines()
        .map(Snafu::new)
        .map(isize::from)
        .sum::<isize>()
        .into()
}

struct Snafu(String);

impl Snafu {
    fn new(inner: &str) -> Self {
        Self(inner.into())
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Snafu> for isize {
    fn from(value: Snafu) -> Self {
        value.0.chars().rev().enumerate().fold(0, |acc, (exp, d)| {
            let d = match d {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unimplemented!(),
            };

            acc + d * 5isize.pow(exp.try_into().unwrap())
        })
    }
}

impl From<isize> for Snafu {
    fn from(mut value: isize) -> Self {
        let mut base5 = Vec::new();

        while value > 0 {
            let (next_value, rem) = (value / 5, value % 5);
            value = next_value;
            base5.push(rem);
        }

        let (_, balanced_base5) = base5.iter().fold((0, Vec::new()), |(carry, mut acc), d| {
            let sum = carry + d;

            let (carry, d) = match sum {
                _ if sum > 2 => (1, -5 + sum),
                _ => (0, sum),
            };

            acc.push(d);

            (carry, acc)
        });

        Snafu(
            balanced_base5
                .into_iter()
                .rev()
                .map(|n| match n {
                    -2 => '=',
                    -1 => '-',
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    _ => unimplemented!(),
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT).0, "2=-1=0");
    }
}
