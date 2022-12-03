#![feature(iter_array_chunks)]

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.as_bytes().split_at(l.len() / 2);
            x.iter().find(|a| y.contains(a)).unwrap()
        })
        .map(to_priority)
        .sum::<usize>()
}

fn part2(input: &'static str) -> usize {
    input
        .lines()
        .array_chunks::<3>()
        .map(|[x, y, z]| {
            let [x, y, z] = [x.as_bytes(), y.as_bytes(), z.as_bytes()];
            x.iter().find(|a| y.contains(a) && z.contains(a)).unwrap()
        })
        .map(to_priority)
        .sum::<usize>()
}

fn to_priority(x: &u8) -> usize {
    usize::try_from(match x >= &96 {
        true => x - 96,
        false => x - 38,
    })
    .unwrap()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 157);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 70);
    }
}
