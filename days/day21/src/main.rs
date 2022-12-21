use std::{collections::HashMap, sync::mpsc::channel, thread};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> isize {
    let root_receiver = start_threads(input);
    root_receiver.recv().unwrap()
}

fn part2(input: &'static str) -> isize {
    todo!()
}

fn start_threads(input: &str) -> std::sync::mpsc::Receiver<isize> {
    let mut senders = HashMap::new();
    let mut receivers = HashMap::new();

    for l in input.lines() {
        let id = &l[0..4].to_string();
        let (sender, receiver) = channel::<isize>();

        senders.insert(id.clone(), sender);
        receivers.insert(id.clone(), receiver);
    }

    for l in input.lines() {
        let id = &l[0..4];
        let instr = &l[6..];

        // Monkey shouts
        if let Ok(val) = instr.parse::<isize>() {
            let sender = senders.remove(id).unwrap();
            thread::spawn(move || {
                sender.send(val).unwrap();
            });

            continue;
        }

        // Monkey await calcs, then shouts
        let subj_a = &instr[0..4];
        let subj_b = &instr[7..11];
        let op = instr.chars().nth(5).unwrap();

        let sender = senders.remove(id).unwrap();
        let recv_a = receivers.remove(subj_a).unwrap();
        let recv_b = receivers.remove(subj_b).unwrap();

        thread::spawn(move || {
            let val_a = recv_a.recv().unwrap();
            let val_b = recv_b.recv().unwrap();

            let val = match op {
                '+' => val_a + val_b,
                '-' => val_a - val_b,
                '*' => val_a * val_b,
                '/' => val_a / val_b,
                _ => unimplemented!(),
            };

            sender.send(val).unwrap();
        });
    }

    receivers.remove("root").unwrap()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 152);
    }
}
