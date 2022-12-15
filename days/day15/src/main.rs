use std::{collections::HashSet, ops::Range};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1::<2000000>(INPUT), || part2::<0, 4000000>(INPUT));
}

fn part1<const Y: isize>(input: &'static str) -> usize {
    let caves = Caves::from(input);

    caves.x_bounds().fold(0, |mut acc, x| {
        if caves
            .sensors()
            .any(|device| device.in_range(&Xy(x, Y)) && !caves.occupied(&Xy(x, Y)))
        {
            acc += 1
        }

        acc
    })
}

fn part2<const MIN: isize, const MAX: isize>(input: &'static str) -> usize {
    let caves = Caves::from(input);

    let x_bounds = MIN..=MAX;
    let y_bounds = MIN..=MAX;

    let search_space = caves
        .sensors()
        .flat_map(|device| device.surrounding_points())
        .filter(|Xy(x, y)| x_bounds.contains(x) && y_bounds.contains(y));

    for Xy(x, y) in search_space {
        let search_pos = &Xy(x, y);

        if caves.sensors().all(|d| !d.in_range(search_pos)) {
            return usize::try_from((x * 4000000) + y).unwrap();
        }
    }

    unreachable!();
}

#[derive(Debug)]
struct Caves(Vec<Device>, HashSet<Xy>);

impl Caves {
    fn sensors(&self) -> impl Iterator<Item = &Device> {
        self.0
            .iter()
            .filter(|device| matches!(device, Device::Sensor(..)))
    }

    fn occupied(&self, xy: &Xy) -> bool {
        self.1.contains(xy)
    }

    fn x_bounds(&self) -> Range<isize> {
        let all_x_bounds = self
            .sensors()
            .map(|device| device.x_bounds())
            .collect::<Vec<_>>();

        (all_x_bounds.iter().min_by_key(|(l, _)| l).unwrap().0)
            ..(all_x_bounds.iter().max_by_key(|(_, r)| r).unwrap().1)
    }
}

impl From<&str> for Caves {
    fn from(input: &str) -> Self {
        let c = input
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
                    Device::Sensor(Xy(sx, sy), isize::try_from(manhattan).unwrap()),
                    Device::Beacon(Xy(bx, by)),
                ]
            })
            .collect::<Vec<_>>();

        let o = c
            .iter()
            .map(|device| match device {
                Device::Sensor(xy, _) => *xy,
                Device::Beacon(xy) => *xy,
            })
            .collect();

        Self(c, o)
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
    fn x_bounds(&self) -> (isize, isize) {
        match self {
            Device::Sensor(Xy(x, _), manhattan) => (x - manhattan - 1, x + manhattan + 1),
            Device::Beacon(_) => panic!("beacons have no x bounds"),
        }
    }

    fn in_range(&self, Xy(search_x, search_y): &Xy) -> bool {
        match self {
            Device::Sensor(Xy(x, y), manhattan) => {
                (x.abs_diff(*search_x) as isize + y.abs_diff(*search_y) as isize) <= *manhattan
            }
            Device::Beacon(_) => panic!("beacons have no range"),
        }
    }

    fn surrounding_points(&self) -> Vec<Xy> {
        match self {
            Device::Sensor(Xy(x, y), manhattan) => {
                let mut points = Vec::with_capacity((*manhattan as usize + 1) * 4);
                points.push(Xy(x - manhattan - 1, *y));
                points.push(Xy(x + manhattan + 1, *y));

                for y_offset in 1..=*manhattan {
                    points.push(Xy(x - 1 - manhattan + y_offset, y + y_offset));
                    points.push(Xy(x + 1 + manhattan - y_offset, y + y_offset));

                    points.push(Xy(x - 1 - manhattan + y_offset, y - y_offset));
                    points.push(Xy(x + 1 + manhattan - y_offset, y - y_offset));
                }

                points
            }
            Device::Beacon(_) => panic!("beacons have no adjacent points"),
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
        assert_eq!(super::part2::<0, 20>(INPUT), 56000011)
    }
}
