use std::collections::{HashMap, VecDeque};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let hill = Hill::from(input);
    march(hill)
}

fn part2(input: &'static str) -> usize {
    let hill = Hill::from(input);

    hill.remaining
        .iter()
        .filter_map(|(xy, h)| match h {
            0 => {
                let mut hill = hill.clone();
                hill.start = *xy;
                Some(march(hill))
            }
            _ => None,
        })
        .min()
        .unwrap()
}

fn march(mut hill: Hill) -> usize {
    let mut results = Vec::new();
    let mut queue = vec![(hill.start, 0)].into_iter().collect::<VecDeque<_>>();

    while let Some((curr_xy, steps)) = queue.pop_front() {
        if curr_xy == hill.finish {
            results.push(steps);
        }

        if let Some(curr_h) = hill.remaining.remove(&curr_xy) {
            for adj in hill.adjacent(&curr_xy) {
                if let Some((&adj_xy, &adj_h)) = adj {
                    if Hill::climbable(curr_h, adj_h) {
                        queue.push_back((adj_xy, steps + 1));
                    }
                }
            }
        }
    }

    *results.iter().min().unwrap_or(&usize::MAX)
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Xy(i8, i8);

#[derive(Clone, Debug)]
struct Hill {
    remaining: HashMap<Xy, u8>,
    start: Xy,
    finish: Xy,
}

impl From<&str> for Hill {
    fn from(input: &str) -> Self {
        let mut remaining = HashMap::with_capacity(input.lines().count() * input.lines().count());
        let mut start = Xy(0, 0);
        let mut finish = Xy(0, 0);

        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                let xy = Xy(x as i8, y as i8);
                remaining.insert(
                    xy,
                    match c {
                        'a'..='z' => c as u8 - b'a',
                        'S' => {
                            start = xy;
                            'a' as u8 - b'a'
                        }
                        'E' => {
                            finish = xy;
                            'z' as u8 - b'a'
                        }
                        _ => panic!("invalid input char"),
                    },
                );
            }
        }

        Self {
            remaining,
            start,
            finish,
        }
    }
}

impl Hill {
    fn adjacent(&self, Xy(x, y): &Xy) -> [Option<(&Xy, &u8)>; 4] {
        let (l, r, u, d) = (Xy(x - 1, *y), Xy(x + 1, *y), Xy(*x, y - 1), Xy(*x, y + 1));

        [
            self.remaining.get_key_value(&l),
            self.remaining.get_key_value(&r),
            self.remaining.get_key_value(&u),
            self.remaining.get_key_value(&d),
        ]
    }

    fn climbable(a: u8, b: u8) -> bool {
        b <= a + 1
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 31);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 29);
    }
}
