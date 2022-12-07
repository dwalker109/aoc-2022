#![feature(box_into_inner)]

use std::collections::VecDeque;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let candidates = get_candidates(input);
    candidates.iter().filter(|c| **c <= 100000).sum::<usize>()
}

fn part2(input: &'static str) -> usize {
    let candidates = get_candidates(input);

    let tot = 70000000;
    let req = 30000000;
    let unused = tot - candidates.last().unwrap();
    let target = req - unused;

    *candidates.iter().find(|s| **s >= target).unwrap()
}

fn get_candidates(input: &'static str) -> Vec<usize> {
    let root = Box::new(Fs::from(input));

    let mut candidates = Vec::new();
    descend(&root, &mut candidates);
    candidates.sort();

    candidates
}

fn descend(node: &Box<Fs>, candidates: &mut Vec<usize>) {
    candidates.push(node.size());

    for subnode in node.subdirectories().unwrap() {
        descend(subnode, candidates);
    }
}

#[derive(Debug)]
enum Cmd {
    Cd(&'static str),
    Ls(Vec<&'static str>),
}

#[derive(Debug)]
enum Fs {
    Dir(Vec<Box<Fs>>),
    File(usize),
}

impl From<&'static str> for Fs {
    fn from(listing: &'static str) -> Self {
        let mut commands = listing
            .split("$ ")
            .skip(1)
            .map(|l| match &l[..=1] {
                "cd" => Cmd::Cd(l.split_whitespace().last().unwrap()),
                "ls" => Cmd::Ls(l.lines().skip(1).collect()),
                _ => panic!(),
            })
            .collect::<VecDeque<_>>();

        fn shell_exec(commands: &mut VecDeque<Cmd>) -> Vec<Box<Fs>> {
            let mut cwd = Vec::new();

            while let Some(cmd) = commands.pop_front() {
                match cmd {
                    Cmd::Cd(path) if path == ".." => return cwd,
                    Cmd::Cd(_) => cwd.push(Box::new(Fs::Dir(shell_exec(commands)))),
                    Cmd::Ls(listing) => {
                        for file in listing.iter().filter_map(|l| {
                            let (a, _) = l.split_once(' ').unwrap();
                            if let Ok(bytes) = a.parse::<usize>() {
                                Some(Fs::File(bytes)) // is a file
                            } else {
                                None // is a dir, ignore
                            }
                        }) {
                            cwd.push(Box::new(file));
                        }
                    }
                }
            }

            cwd
        }

        let mut root = shell_exec(&mut commands);
        Box::into_inner(root.remove(0))
    }
}

impl Fs {
    fn size(&self) -> usize {
        match self {
            Fs::Dir(contents) => contents.iter().map(|e| e.size()).sum::<usize>(),
            Fs::File(s) => *s,
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            Fs::Dir(_) => true,
            Fs::File(_) => false,
        }
    }

    fn subdirectories(&self) -> Option<Vec<&Box<Fs>>> {
        match self {
            Fs::Dir(contents) => Some(contents.iter().filter(|e| e.is_dir()).collect()),
            Fs::File(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 95437);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 24933642);
    }
}
