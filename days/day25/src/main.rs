static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || "".into());
}

fn part1(input: &'static str) -> String {
    todo!()
}

#[repr(transparent)]
struct Snafu(String);

impl From<Snafu> for isize {
    fn from(value: Snafu) -> Self {
        value.0.chars().rev().enumerate().fold(0, |acc, (y, x)| {
            let x = match x {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unimplemented!(),
            };

            acc + x * 5isize.pow(y.try_into().unwrap())
        })
    }
}

impl From<isize> for Snafu {
    fn from(value: isize) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Snafu;

    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), "2=-1=0");
    }

    #[test]
    fn snafu_to_dec() {
        let results: Vec<isize> = INPUT.lines().map(|l| Snafu(l.into()).into()).collect();

        assert_eq!(
            &results,
            &[1747, 906, 198, 11, 201, 31, 1257, 32, 353, 107, 7, 3, 37,]
        );
    }
}
