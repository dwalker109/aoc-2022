use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let m = Monkeys::from(input);
    m.resolve("root").unwrap()
}

fn part2(input: &'static str) -> usize {
    let mut m = Monkeys::from(input);

    m.0.insert(
        "root",
        input
            .lines()
            .find_map(|l| {
                l.starts_with("root")
                    .then(|| Monkey::Eql(&l[6..10], &l[13..17]))
            })
            .unwrap(),
    );

    m.0.insert("humn", Monkey::Unknown);

    m.divine("humn", "root", None)
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

impl Monkey {
    fn parts(&self) -> (Id, Id) {
        match self {
            Monkey::Eql(l, r)
            | Monkey::Add(l, r)
            | Monkey::Sub(l, r)
            | Monkey::Mul(l, r)
            | Monkey::Div(l, r) => (*l, *r),
            _ => unimplemented!(),
        }
    }
}

struct Monkeys(HashMap<Id, Monkey>);

impl From<&'static str> for Monkeys {
    fn from(value: &'static str) -> Self {
        Self(
            value
                .lines()
                .map(|l| {
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
                            _ => panic!(),
                        }
                    }
                })
                .collect(),
        )
    }
}

impl Monkeys {
    fn resolve(&self, m: Id) -> Option<usize> {
        match self.0.get(m).unwrap() {
            Monkey::Resolved(n) => Some(*n),
            Monkey::Unknown => None,
            Monkey::Add(a, b) => Some(self.resolve(a)? + self.resolve(b)?),
            Monkey::Sub(a, b) => Some(self.resolve(a)? - self.resolve(b)?),
            Monkey::Mul(a, b) => Some(self.resolve(a)? * self.resolve(b)?),
            Monkey::Div(a, b) => Some(self.resolve(a)? / self.resolve(b)?),
            _ => unimplemented!(),
        }
    }

    fn divine(&self, target: Id, entry: Id, hint: Option<usize>) -> usize {
        if entry == target {
            return hint.unwrap();
        }

        let curr = self.0.get(entry).unwrap();

        let (l_id, r_id) = curr.parts();
        let (l_maybe_val, r_maybe_val) = (self.resolve(l_id), self.resolve(r_id));

        let (val, needs_divination) = if l_maybe_val.is_some() {
            (l_maybe_val.unwrap(), r_id)
        } else {
            (r_maybe_val.unwrap(), l_id)
        };

        match *curr {
            Monkey::Eql(..) => self.divine(target, needs_divination, Some(val)),
            Monkey::Add(..) => self.divine(target, needs_divination, Some(hint.unwrap() - val)),
            Monkey::Sub(..) => {
                if needs_divination == l_id {
                    self.divine(target, needs_divination, Some(hint.unwrap() + val))
                } else {
                    self.divine(target, needs_divination, Some(val - hint.unwrap()))
                }
            }
            Monkey::Mul(..) => self.divine(target, needs_divination, Some(hint.unwrap() / val)),
            Monkey::Div(..) => {
                if needs_divination == l_id {
                    self.divine(target, needs_divination, Some(hint.unwrap() * val))
                } else {
                    self.divine(target, needs_divination, Some(val / hint.unwrap()))
                }
            }
            _ => unimplemented!(),
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
