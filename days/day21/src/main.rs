use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let m = Monkeys::from(input);
    m.resolve("root")
}

fn part2(_input: &'static str) -> usize {
    todo!()
}

type Id = &'static str;

enum Monkey {
    Resolved(usize),
    Add(Id, Id),
    Sub(Id, Id),
    Mul(Id, Id),
    Div(Id, Id),
}

struct Monkeys(HashMap<Id, Monkey>);

impl From<&'static str> for Monkeys {
    fn from(value: &'static str) -> Self {
        Self(value.lines().map(|l| {
            let m = &l[0..4];
            let rest = &l[6..];

            if let Ok(n) = rest.parse::<usize>() {
                (m, Monkey::Resolved(n))
            } else {
                let a = &rest[0..4];
                let b = &rest[7..11];

                match rest.chars().nth(5).unwrap() {
                    '+' => (m, Monkey::Add(a, b)),
                    '-' => (m, Monkey::Sub(a, b)),
                    '*' => (m, Monkey::Mul(a, b)),
                    '/' => (m, Monkey::Div(a, b)),
                    _ => panic!()
                }
            }
        }).collect())
    }
}

impl Monkeys {
    fn resolve(&self, m: Id) -> usize {
        match self.0.get(m).unwrap() {
            Monkey::Resolved(n) => *n,
            Monkey::Add(a,b) => self.resolve(a) + self.resolve(b),
            Monkey::Sub(a,b) => self.resolve(a) - self.resolve(b),
            Monkey::Mul(a,b) => self.resolve(a) * self.resolve(b),
            Monkey::Div(a,b) => self.resolve(a) / self.resolve(b),
        }
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 152);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 301);
    }
}
