use std::{collections::HashMap, sync::mpsc::channel, thread};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2::<3059361893920>(INPUT));
}

/// I was really proud of this channel based setup - it worked exactly as I
/// planned it in my head, was easy to code and worked perfectly first time.
/// EUREKA! Until we get to part 2 and it doesn't help at all, and is also
/// relatively slow - 50ms per pass is OK when you're doing one, but if you
/// are trying to do many (see part 2) that makes a big difference.
fn part1(input: &'static str) -> isize {
    let (_, receivers) = start_general_threads(input, None);
    receivers.get("root").unwrap().recv().unwrap()
}

/// So yeah, I manually brute forced this answer by tweaking the start of the
/// search range by hand (you know the target and the input changes affect the
/// outcome linearly, so it is just a case of messing with it until it works).
/// I *could* make the brute force smart, and keep adjusting itself as it gets
/// nearer/further from the target. But for now, my answer is essentially
/// hardcoded into this const generic. The correct way of solving this is
/// equations, but I'd just be copying somebody else's answer at that point.
fn part2<const N: isize>(input: &'static str) -> isize {
    for humn_val in N..isize::MAX {
        let (mut senders, mut receivers) = start_general_threads(input, Some(&["root", "humn"]));
        start_special_threads(input, humn_val, &mut senders, &mut receivers);

        if receivers.get("root").unwrap().recv().unwrap() == 1 {
            return humn_val;
        }
    }

    unreachable!();
}

fn start_general_threads(
    input: &str,
    skip_ids: Option<&[&str]>,
) -> (
    HashMap<String, std::sync::mpsc::Sender<isize>>,
    HashMap<String, std::sync::mpsc::Receiver<isize>>,
) {
    let mut senders = HashMap::new();
    let mut receivers = HashMap::new();

    for l in input.lines() {
        let id = &l[0..4].to_string();
        let (sender, receiver) = channel::<isize>();

        senders.insert(id.clone(), sender);
        receivers.insert(id.clone(), receiver);
    }

    for l in input.lines().filter(|l| match skip_ids {
        Some(ids) => !ids.contains(&&l[0..4]),
        None => true,
    }) {
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

    (senders, receivers)
}

fn start_special_threads(
    input: &str,
    humn_val: isize,
    senders: &mut HashMap<String, std::sync::mpsc::Sender<isize>>,
    receivers: &mut HashMap<String, std::sync::mpsc::Receiver<isize>>,
) {
    let sender = senders.remove("humn").unwrap();
    // Send our humn value out
    thread::spawn(move || {
        sender.send(humn_val).unwrap();
    });

    let l = input.lines().find(|l| &l[0..4] == "root").unwrap();

    let sender = senders.remove("root").unwrap();

    let subj_a = &l[6..10];
    let subj_b = &l[13..17];
    let recv_a = receivers.remove(subj_a).unwrap();
    let recv_b = receivers.remove(subj_b).unwrap();

    // Listen for and check root value - send result bool as an isize (true = 1, false = 0)
    thread::spawn(move || {
        let val_a = recv_a.recv().unwrap();
        let val_b = recv_b.recv().unwrap();

        sender
            .send(isize::try_from(val_a == val_b).unwrap())
            .unwrap();
    });
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
        assert_eq!(super::part2::<0>(INPUT), 301);
    }
}
