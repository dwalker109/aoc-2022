use std::collections::{BinaryHeap, HashMap, HashSet};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1::<2000000>(INPUT), || part2(INPUT));
}

fn part1<const TARGET_Y: isize>(input: &'static str) -> usize {
    let space = Space::from(input);
    space
        .0
        .iter()
        .filter_map(|(_, d)| space.check_row(&TARGET_Y, d))
        .flatten()
        .collect::<HashSet<_>>()
        .len()
}

fn part2(input: &'static str) -> usize {
    todo!()
}

struct Space(HashMap<Xy, Device>);

impl From<&str> for Space {
    fn from(input: &str) -> Self {
        Self(
            input
                .lines()
                .flat_map(|l| {
                    let [sx, sy, bx, by] = <[isize; 4]>::try_from(
                        l.split_ascii_whitespace()
                            .filter_map(|w| match w.contains('=') {
                                true => Some(
                                    w.chars()
                                        .filter(|c| matches!(c, '0'..='9' | '-'))
                                        .collect::<String>()
                                        .parse::<isize>()
                                        .unwrap(),
                                ),
                                false => None,
                            })
                            .collect::<Vec<_>>(),
                    )
                    .unwrap();

                    let manhattan = sx.abs_diff(bx) + sy.abs_diff(by);

                    [
                        (
                            Xy(sx, sy),
                            Device::Sensor(Xy(sx, sy), isize::try_from(manhattan).unwrap()),
                        ),
                        (Xy(bx, by), Device::Beacon(Xy(bx, by))),
                    ]
                })
                .collect(),
        )
    }
}

impl Space {
    fn check_row(&self, row_y: &isize, device: &Device) -> Option<Vec<isize>> {
        match device {
            Device::Sensor(Xy(_, sy), manhattan) => {
                if sy > &(row_y + manhattan) || sy < &(row_y - manhattan) {
                    return None;
                }

                Some(
                    device
                        .scope(row_y)
                        .iter()
                        .filter_map(|Xy(sx, sy)| (sy == row_y).then_some(sx))
                        .filter_map(|x| (!self.0.contains_key(&Xy(*x, *row_y))).then_some(x))
                        .copied()
                        .collect(),
                )
            }
            Device::Beacon(_) => None,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Xy(isize, isize);

#[derive(Clone, Copy, Debug)]
enum Device {
    Sensor(Xy, isize),
    Beacon(Xy),
}

impl Device {
    fn scope(&self, row_y: &isize) -> Vec<Xy> {
        match self {
            Device::Sensor(Xy(sx, sy), manhattan) => {
                let mut scope = (sx - manhattan..=sx + manhattan)
                    .filter(|_| sy==row_y)
                    .map(|x| Xy(x, *sy))
                    .collect::<Vec<_>>();

                for y_offset in 1..=*manhattan {
                    let mut above = (sx - manhattan + y_offset..=sx + manhattan - y_offset)
                        .map(|x| Xy(x, sy + y_offset))
                        .filter(|Xy(_, y)| y==row_y)
                        .collect::<Vec<_>>();
                    let mut below = (sx - manhattan + y_offset..=sx + manhattan - y_offset)
                        .map(|x| Xy(x, sy - y_offset))
                        .filter(|Xy(_, y)| y==row_y)
                        .collect::<Vec<_>>();

                    scope.append(&mut above);
                    scope.append(&mut below);
                }

                scope
            }
            Device::Beacon(_) => panic!("beacons have no scope"),
        }
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1::<10>(INPUT), 26)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 0)
    }
}
