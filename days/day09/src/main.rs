use std::collections::HashSet;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let steps = Steps::from(input);
    let mut h = Knot::new(steps.0.len());
    let mut t = Knot::new(steps.0.len());

    for step in steps.0 {
        h.mv(step);
        t.follow(&h);
    }

    t.unique_history()
}

fn part2(input: &'static str) -> usize {
    let steps = Steps::from(input);
    let mut h = Knot::new(steps.0.len());
    let mut k1 = Knot::new(steps.0.len());
    let mut k2 = Knot::new(steps.0.len());
    let mut k3 = Knot::new(steps.0.len());
    let mut k4 = Knot::new(steps.0.len());
    let mut k5 = Knot::new(steps.0.len());
    let mut k6 = Knot::new(steps.0.len());
    let mut k7 = Knot::new(steps.0.len());
    let mut k8 = Knot::new(steps.0.len());
    let mut k9 = Knot::new(steps.0.len());

    for step in steps.0 {
        h.mv(step);
        k1.follow(&h);
        k2.follow(&k1);
        k3.follow(&k2);
        k4.follow(&k3);
        k5.follow(&k4);
        k6.follow(&k5);
        k7.follow(&k6);
        k8.follow(&k7);
        k9.follow(&k8);
    }

    k9.unique_history()
}

#[derive(Clone, Copy)]
enum Move {
    L,
    R,
    U,
    D,
}

struct Steps(Vec<Move>);

impl From<&str> for Steps {
    fn from(input: &str) -> Self {
        Self(
            input
                .lines()
                .flat_map(|l| {
                    let (dir, q) = l.split_once(' ').unwrap();

                    let dir = match dir {
                        "L" => Move::L,
                        "R" => Move::R,
                        "U" => Move::U,
                        "D" => Move::D,
                        _ => panic!(),
                    };

                    (0..q.parse::<usize>().unwrap()).map(move |_| dir)
                })
                .collect(),
        )
    }
}

struct Knot {
    pos: (isize, isize),
    history: Vec<(isize, isize)>,
}

impl Knot {
    fn new(capacity: usize) -> Self {
        let mut k = Self {
            pos: (0, 0),
            history: Vec::with_capacity(capacity),
        };

        k.history.push((0, 0));

        k
    }

    fn mv(&mut self, step: Move) {
        match step {
            Move::L => self.pos.0 -= 1,
            Move::R => self.pos.0 += 1,
            Move::U => self.pos.1 += 1,
            Move::D => self.pos.1 -= 1,
        };
    }

    fn is_touching(&self, other: &Knot) -> bool {
        !(self.pos.0.abs_diff(other.pos.0) > 1 || self.pos.1.abs_diff(other.pos.1) > 1)
    }

    fn follow(&mut self, other: &Knot) {
        if self.is_touching(other) {
            return;
        }

        match self.pos.0.cmp(&other.pos.0) {
            std::cmp::Ordering::Equal => { /* nothing */ }
            std::cmp::Ordering::Less => {
                self.mv(Move::R);
            }
            std::cmp::Ordering::Greater => {
                self.mv(Move::L);
            }
        }

        match self.pos.1.cmp(&other.pos.1) {
            std::cmp::Ordering::Equal => { /* nothing */ }
            std::cmp::Ordering::Less => {
                self.mv(Move::U);
            }
            std::cmp::Ordering::Greater => {
                self.mv(Move::D);
            }
        }

        self.history.push(self.pos);
    }

    fn unique_history(&self) -> usize {
        self.history.iter().collect::<HashSet<_>>().len()
    }
}

#[cfg(test)]
mod tests {
    static INPUT_1: &str = include_str!("../input_test_1");
    static INPUT_2: &str = include_str!("../input_test_2");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT_1), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT_1), 1);
        assert_eq!(super::part2(INPUT_2), 36);
    }
}
