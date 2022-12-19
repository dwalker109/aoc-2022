use lava::Droplet;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    Droplet::from(input).surface_area()
}

fn part2(input: &'static str) -> usize {
    todo!()
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
    use std::collections::HashSet;

    #[derive(Debug)]
    pub struct Droplet {
        scan: HashSet<Voxel>,
    }

    impl Droplet {
        pub fn surface_area(&self) -> usize {
            self.scan.iter().fold(0, |acc, v| {
                acc + (FACES as usize
                    - HashSet::from(v.adjacent()).intersection(&self.scan).count())
            })
        }
    }

    impl From<&str> for Droplet {
        fn from(value: &str) -> Self {
            let scan = value.lines().map(Voxel::from).collect();

            Self { scan }
        }
    }
}
