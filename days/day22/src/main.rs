use notes::{cube::CubeNet, plane};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2::<50>(INPUT, CubeNet::Type2));
}

fn part1(input: &'static str) -> usize {
    let mut mn = notes::MonkeyNotes::from(input);
    plane::build_portals(&mut mn);
    mn.nav();
    mn.password()
}

fn part2<const N: usize>(input: &'static str, cube_net: CubeNet<N>) -> usize {
    let mut mn = notes::MonkeyNotes::from(input);
    cube_net.build_portals(&mut mn);
    mn.nav();
    mn.password()
}

mod notes;
mod space;

#[cfg(test)]
mod tests {
    use crate::notes::cube::CubeNet;

    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 6032);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2::<4>(INPUT, CubeNet::Type1), 5031);
    }
}
