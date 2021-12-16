/* --- Day 14: Extended Polymerization --- */

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use itertools::Itertools;

#[derive(Debug)]
struct Polymerization {
    rules: HashMap<String, char>,
    template: Vec<char>,
}

impl<R: Read> From<BufReader<R>> for Polymerization {
    fn from(inp: BufReader<R>) -> Self {
        let mut lines = inp.lines().map(Result::unwrap);

        let template = lines.next().unwrap().chars().collect_vec();
        lines.next();

        let mut rules: HashMap<String, char> = HashMap::new();

        for line in lines {
            let mut split = line.split(" -> ");
            rules.insert(
                split.next().unwrap().to_string(),
                split.next().unwrap().chars().next().unwrap(),
            );
        }

        Polymerization { rules, template }
    }
}

impl Polymerization {
    fn step(&mut self) {
        let mut i = 0;
        while i < self.template.len() - 1 {
            let ngram = self.template[i..i + 2].iter().collect::<String>();
            match self.rules.get(&ngram) {
                Some(v) => {
                    self.template.insert(i + 1, *v);
                    i += 2
                }
                None => {}
            }
        }
    }

    fn most_common(&self) -> i64 {
        let mut sorted = self.template.clone();
        sorted.sort();

        let mut running_char = *sorted.first().unwrap();
        let mut current_run = 0;
        let mut best = 0;

        for c in sorted {
            if c == running_char {
                current_run += 1;
            } else {
                if current_run > best {
                    best = current_run;
                }
                current_run = 1;
                running_char = c;
            }
        }
        if current_run > best {
            best = current_run;
        }
        best
    }

    fn least_common(&self) -> i64 {
        let mut sorted = self.template.clone();
        sorted.sort();

        let mut running_char = *sorted.first().unwrap();
        let mut current_run = 0;
        let mut best = i64::MAX;

        for c in sorted {
            if c == running_char {
                current_run += 1;
            } else {
                if current_run < best {
                    best = current_run;
                }
                current_run = 1;
                running_char = c;
            }
        }
        if current_run < best {
            best = current_run;
        }
        best
    }
}

pub fn part_1<R: Read>(inp: BufReader<R>) -> i64 {
    let mut polymerization = Polymerization::from(inp);
    for _ in 0..10 {
        polymerization.step();
    }
    polymerization.most_common() - polymerization.least_common()
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> i64 {
    let mut polymerization = Polymerization::from(inp);
    for _ in 0..40 {
        polymerization.step();
    }
    polymerization.most_common() - polymerization.least_common()
}

#[cfg(test)]
mod test {
    use crate::day14::{part_1, part_2, Polymerization};
    use std::{collections::HashMap, io::BufReader};
    use test::Bencher;

    const INP: &[u8] = include_str!("../test/day14.txt").as_bytes();

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(BufReader::new(INP)), 1588);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(BufReader::new(INP)), 2188189693529);
    }

    #[test]
    fn test_most_common() {
        let p = Polymerization {
            template: vec!['a', 'a', 'a', 'a', 'b', 'c', 'c', 'c', 'c', 'c'],
            rules: HashMap::new(),
        };
        assert_eq!(5, p.most_common());
    }

    #[test]
    fn test_least_common() {
        let p = Polymerization {
            template: vec!['a', 'a', 'a', 'a', 'b', 'c', 'c', 'c', 'c', 'c'],
            rules: HashMap::new(),
        };
        assert_eq!(1, p.least_common());
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part_1(BufReader::new(INP)))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part_2(BufReader::new(INP)))
    }
}
