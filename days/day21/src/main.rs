use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let m = Monkeys::from(input);
    m.resolve("root")
}

fn part2(input: &'static str) -> usize {
    let mut m = Monkeys::from(input);

    m.0.insert("root", input.lines().filter_map(|l| {
        l.starts_with("root").then(|| Monkey::Eql(&l[6..10], &l[13..17]))
    }).next().unwrap());

    m.0.insert("humn", Monkey::Unknown);

    todo!()
}

type Id = &'static str;

enum Monkey {
    Resolved(usize),
    Unknown,
    Eql(Id, Id),
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
            Monkey::Add(a, b) => self.resolve(a) + self.resolve(b),
            Monkey::Sub(a, b) => self.resolve(a) - self.resolve(b),
            Monkey::Mul(a, b) => self.resolve(a) * self.resolve(b),
            Monkey::Div(a, b) => self.resolve(a) / self.resolve(b),
            _ => unimplemented!(),
        }
    }

    fn route(&self, m: Id) -> Vec<Id> {
        match self.0.get(m).unwrap() {
            Monkey::Resolved(..) | Monkey::Unknown => vec![vec![m]],
            Monkey::Eql(l, r) => vec![self.route(l), self.route(r)],
            Monkey::Add(l, r) => vec![self.route(l), self.route(r)],
            Monkey::Sub(l, r) => vec![self.route(l), self.route(r)],
            Monkey::Mul(l, r) => vec![self.route(l), self.route(r)],
            Monkey::Div(l, r) => vec![self.route(l), self.route(r)],
        }.into_iter().flatten().collect()
    }

    fn divine(&self) -> [(Id, usize); 2] {
        let Some(Monkey::Eql(l, r)) = self.0.get("root") else {
            unimplemented!();
        };

        let (target, search) = match self.route(l).contains(&"humn") {
            true => (self.resolve(r), l),
            false => (self.resolve(l), r),
        };

        todo!()
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
