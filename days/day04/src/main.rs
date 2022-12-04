use std::ops::RangeInclusive;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    parse(input)
        .filter(|(xr, yr)| {
            xr.contains(yr.start()) && xr.contains(yr.end())
                || yr.contains(xr.start()) && yr.contains(xr.end())
        })
        .count()
}

fn part2(input: &'static str) -> usize {
    parse(input)
        .filter(|(xr, yr)| {
            xr.contains(yr.start())
                || xr.contains(yr.end())
                || yr.contains(xr.start())
                || yr.contains(xr.end())
        })
        .count()
}

fn parse(
    input: &'static str,
) -> impl Iterator<Item = (RangeInclusive<usize>, RangeInclusive<usize>)> {
    let to_range = |a: &str| {
        let (lower, upper) = a.split_once('-').unwrap();
        lower.parse::<usize>().unwrap()..=upper.parse::<usize>().unwrap()
    };

    input.lines().map(move |l| {
        let (x, y) = l.split_once(',').unwrap();

        (to_range(x), to_range(y))
    })
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 4);
    }
}
