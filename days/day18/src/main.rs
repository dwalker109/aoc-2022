use lava::Droplet;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    Droplet::from(input).surface_area()
}

fn part2(input: &'static str) -> usize {
    Droplet::from(input).external_only_surface_area()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 64);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 58);
    }
}

mod voxel {
    use std::ops::{Add, Sub};

    pub const FACES: i8 = 6;

    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    pub struct Voxel(i8, i8, i8);

    impl Voxel {
        pub fn new(x: i8, y: i8, z: i8) -> Self {
            Self(x, y, z)
        }

        pub fn x(&self) -> i8 {
            self.0
        }

        pub fn y(&self) -> i8 {
            self.1
        }

        pub fn z(&self) -> i8 {
            self.2
        }

        pub fn adjacent(&self) -> [Self; 6] {
            let x_offset = Self(1, 0, 0);
            let y_offset = Self(0, 1, 0);
            let z_offset = Self(0, 0, 1);

            [
                *self - x_offset,
                *self + x_offset,
                *self - y_offset,
                *self + y_offset,
                *self - z_offset,
                *self + z_offset,
            ]
        }

        pub fn within(&self, min: Self, max: Self) -> bool {
            self.x() >= min.x()
                && self.y() >= min.y()
                && self.z() >= min.z()
                && self.x() <= max.x()
                && self.y() <= max.y()
                && self.z() <= max.z()
        }
    }

    impl From<&str> for Voxel {
        fn from(value: &str) -> Self {
            let [x, y, z] = <[i8; 3]>::try_from(
                value
                    .split(',')
                    .map(|w| w.parse().unwrap())
                    .collect::<Vec<_>>(),
            )
            .unwrap();

            Self(x, y, z)
        }
    }

    impl Add for Voxel {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
        }
    }

    impl Sub for Voxel {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
        }
    }
}

mod lava {
    use crate::voxel::{Voxel, FACES};
    use std::collections::{HashSet, VecDeque};

    #[derive(Debug)]
    pub struct Droplet {
        scan: HashSet<Voxel>,
        limit: (Voxel, Voxel),
    }

    impl Droplet {
        pub fn surface_area(&self) -> usize {
            self.scan.iter().fold(0, |acc, v| {
                acc + (FACES as usize
                    - HashSet::from(v.adjacent()).intersection(&self.scan).count())
            })
        }

        pub fn external_only_surface_area(&self) -> usize {
            let flood = self.flood_outer();

            self.scan
                .iter()
                .map(|v| HashSet::from(v.adjacent()).intersection(&flood).count())
                .sum()
        }

        fn flood_outer(&self) -> HashSet<Voxel> {
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            let mut flood = HashSet::new();

            let (min, max) = self.limit;
            queue.push_back(min);
            queue.push_back(max);

            while let Some(v) = queue.pop_front() {
                flood.insert(v);

                for adj in v.adjacent() {
                    if adj.within(min, max) && !visited.contains(&adj) && !self.scan.contains(&adj)
                    {
                        visited.insert(adj);
                        queue.push_front(adj);
                    }
                }
            }

            flood
        }
    }

    impl From<&str> for Droplet {
        fn from(value: &str) -> Self {
            let scan = value.lines().map(Voxel::from).collect::<HashSet<_>>();

            let min = Voxel::new(-1, -1, -1);
            let max = Voxel::new(25, 25, 25);

            Self {
                scan,
                limit: (min, max),
            }
        }
    }
}
