/* --- Day 10: Syntax Scoring --- */

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use itertools::Itertools;

pub fn part_1<R: Read>(inp: BufReader<R>) -> i32 {
    let closing = HashMap::from([('[', ']'), ('{', '}'), ('(', ')'), ('<', '>')]);
    let scores = HashMap::from([(']', 57), ('}', 1197), (')', 3), ('>', 25137)]);

    let mut total_score = 0;

    for line in inp.lines().map(Result::unwrap) {
        let mut stack: Vec<char> = vec![];
        for c in line.chars() {
            if c == '<' || c == '[' || c == '{' || c == '(' {
                stack.push(c);
                continue;
            }

            match stack.last() {
                Some(prev_c) => {
                    if closing[prev_c] != c {
                        total_score += scores[&c];
                        break;
                    } else {
                        stack.pop();
                    }
                }
                None => {
                    total_score += scores[&c];
                    break;
                }
            }
        }
    }

    total_score
}

fn get_score(line: String) -> Option<i64> {
    let closing = HashMap::from([('[', ']'), ('{', '}'), ('(', ')'), ('<', '>')]);
    let scores = HashMap::from([(']', 2), ('}', 3), (')', 1), ('>', 4)]);

    let mut stack: Vec<char> = vec![];
    for c in line.chars() {
        if c == '<' || c == '[' || c == '{' || c == '(' {
            stack.push(c);
            continue;
        }

        match stack.last() {
            Some(prev_c) => {
                if closing[prev_c] != c {
                    return None;
                } else {
                    stack.pop();
                }
            }
            None => {
                return None;
            }
        }
    }

    stack.reverse();
    let mut score = 0;
    for c in stack {
        score *= 5;
        score += scores[&closing[&c]];
    }

    Some(score)
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> i64 {
    let scores: Vec<i64> = inp
        .lines()
        .map(Result::unwrap)
        .filter_map(|x| get_score(x))
        .sorted()
        .collect();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
    use crate::day10::{get_score, part_1, part_2};
    use std::io::BufReader;
    use test::Bencher;

    const INP: &[u8] = include_str!("../test/day10.txt").as_bytes();

    #[test]
    fn test_scoring() {
        let line = "[({(<(())[]>[[{[]{<()<>>";

        assert_eq!(get_score(line.to_string()).unwrap(), 288957);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(BufReader::new(INP)), 26397);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(BufReader::new(INP)), 288957);
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
