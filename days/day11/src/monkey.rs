use std::{collections::VecDeque, fmt::Display};

pub type Id = u8;
type OperationFn = dyn Fn(usize) -> usize;
type TestFn = dyn Fn(usize) -> Id;
pub struct Monkey {
    pub id: u8,
    items: VecDeque<usize>,
    pub inspected: usize,
    operation: Box<OperationFn>,
    test: Box<TestFn>,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Monkey {}, Items {:?}, Inspected: {}",
            self.id, self.items, self.inspected
        )
    }
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let mut attr_raw = input.lines().map(|l| l.trim());

        let raw = attr_raw.next().unwrap();
        let id = raw[7..8].parse::<u8>().unwrap();

        let raw = attr_raw.next().unwrap();
        let items: VecDeque<usize> = raw[16..]
            .split(", ")
            .filter_map(|i| i.parse::<usize>().ok())
            .collect();

        let raw = attr_raw.next().unwrap();
        let operation = {
            let (operation, on) = raw[21..].split_once(' ').unwrap();
            let maybe_literal = on.parse::<usize>();

            let f: Box<OperationFn> = match operation {
                "+" => Box::new(move |x| x + maybe_literal.as_ref().unwrap_or(&x)),
                "*" => Box::new(move |x| x * maybe_literal.as_ref().unwrap_or(&x)),
                _ => panic!(),
            };

            f
        };

        let [by, id_true, id_false] = <[u8; 3]>::try_from(
            attr_raw
                .take(3)
                .filter_map(|y| {
                    y.split_ascii_whitespace()
                        .last()
                        .unwrap()
                        .parse::<u8>()
                        .ok()
                })
                .collect::<Vec<_>>(),
        )
        .unwrap();
        let test: Box<TestFn> = Box::new(move |x| {
            if x % by as usize == 0 {
                id_true
            } else {
                id_false
            }
        });

        Self {
            id,
            items,
            inspected: 0,
            operation,
            test,
        }
    }
}

pub enum MonkeyReduce {
    DivThree,
    Lcm(usize),
}

impl Monkey {
    pub fn throw(&mut self, mr: &MonkeyReduce) -> Option<(Id, usize)> {
        if let Some(wl) = self.items.pop_front() {
            self.inspected += 1;

            let mut wl = (self.operation)(wl);
            match mr {
                MonkeyReduce::DivThree => wl /= 3,
                MonkeyReduce::Lcm(n) => wl %= n,
            }
            let to = (self.test)(wl);

            return Some((to, wl));
        }

        None
    }

    pub fn catch(&mut self, item: usize) {
        self.items.push_back(item);
    }
}
