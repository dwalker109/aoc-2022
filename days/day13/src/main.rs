use packet::Packet;

mod packet;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let pairs = parse_pairs(input);

    pairs
        .enumerate()
        .filter_map(|(i, [l, r])| l.lt(&r).then_some(i + 1))
        .sum()
}

fn part2(input: &'static str) -> usize {
    let dividers = [Packet::from("[[2]]"), Packet::from("[[6]]")];

    let mut pairs = parse_pairs(input)
        .flatten()
        .chain(dividers.clone().into_iter())
        .collect::<Vec<_>>();

    pairs.sort();

    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, v)| dividers.contains(v).then_some(i + 1))
        .product()
}

fn parse_pairs(input: &'static str) -> impl Iterator<Item = [Packet; 2]> {
    input
        .split("\n\n")
        .map(|p| <[Packet; 2]>::try_from(p.lines().map(Packet::from).collect::<Vec<_>>()).unwrap())
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 140);
    }
}
