use std::{cmp::Ordering, collections::VecDeque};

#[derive(Debug, Clone)]
enum Token {
    StartList,
    EndList,
    Comma,
    Num(Vec<char>),
}

#[derive(Debug)]
struct RawPacket(VecDeque<Token>);

impl From<&str> for RawPacket {
    fn from(value: &str) -> Self {
        let mut rp = Vec::new();

        for c in value.chars() {
            match c {
                '[' => rp.push(Token::StartList),
                ']' => rp.push(Token::EndList),
                ',' => rp.push(Token::Comma),
                '0'..='9' => {
                    if let Some(Token::Num(x)) = rp.last_mut() {
                        x.push(c);
                    } else {
                        rp.push(Token::Num(vec![c]));
                    }
                }
                _ => panic!("bad token"),
            }
        }

        Self(rp.into_iter().collect())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

impl From<&str> for Packet {
    fn from(value: &str) -> Self {
        Self::from(RawPacket::from(value))
    }
}

impl From<RawPacket> for Packet {
    fn from(mut raw: RawPacket) -> Self {
        Self::from_tokens(&mut raw.0)
    }
}

impl Packet {
    fn from_tokens(raw: &mut VecDeque<Token>) -> Packet {
        let mut list = matches!(raw.pop_front().unwrap(), Token::StartList)
            .then(Vec::<Packet>::new)
            .unwrap();

        loop {
            match raw.get(0).cloned().unwrap() {
                Token::StartList => list.push(Self::from_tokens(raw)),
                Token::EndList => {
                    raw.pop_front();
                    return Packet::List(list);
                }
                Token::Comma => {
                    raw.pop_front();
                }
                Token::Num(chars) => {
                    raw.pop_front();
                    list.push(Packet::Int(
                        chars.iter().collect::<String>().parse::<u8>().unwrap(),
                    ));
                }
            }
        }
    }

    fn is_list(&self) -> bool {
        matches!(&self, Self::List(_))
    }

    fn as_list(&self) -> &Vec<Packet> {
        match &self {
            Packet::Int(_) => panic!("not a list"),
            Packet::List(v) => v,
        }
    }

    fn is_int(&self) -> bool {
        matches!(&self, Self::Int(_))
    }

    fn as_int(&self) -> &u8 {
        match &self {
            Packet::Int(v) => v,
            Packet::List(_) => panic!("not an int"),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_list() && other.is_list() {
            let ll = self.as_list();
            let rl = other.as_list();

            for i in 0..std::cmp::max(ll.len(), rl.len()) {
                let (lv, rv) = (ll.get(i), rl.get(i));

                if rv.is_none() {
                    return Ordering::Greater;
                }

                if lv.is_none() {
                    return Ordering::Less;
                }

                let next = Self::cmp(lv.unwrap(), rv.unwrap());

                if matches!(next, Ordering::Equal) {
                    continue;
                }

                return next;
            }

            return Ordering::Equal;
        }

        if self.is_int() && other.is_int() {
            return self.as_int().cmp(other.as_int());
        }

        if self.is_int() && other.is_list() {
            return Self::cmp(&Self::List(vec![self.clone()]), other);
        }

        if self.is_list() && other.is_int() {
            return Self::cmp(self, &Self::List(vec![other.clone()]));
        }

        unreachable!();
    }
}
