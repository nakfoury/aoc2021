/* --- Day 7: The Treachery of Whales --- */

use std::io::{BufRead, BufReader, Read};

fn this_much_fuel(n: i32) -> i32 {
    n * (n + 1) / 2
}

pub fn part_1<R: Read>(inp: BufReader<R>) -> i32 {
    let mut crab_subs = inp
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<i32>>();

    crab_subs.sort();

    let median = crab_subs[crab_subs.len() / 2];

    crab_subs.iter().map(|p| (median - p).abs()).sum::<i32>()
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> i32 {
    let mut crab_subs = inp
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<i32>>();

    crab_subs.sort();
    let mut best = i32::MAX;

    for i in 0..crab_subs.last().unwrap().clone() {
        best = std::cmp::min(
            best,
            crab_subs
                .clone()
                .into_iter()
                .map(|c| this_much_fuel((i - c).abs()))
                .sum(),
        );
    }
    best
}

#[cfg(test)]
mod test {
    use crate::day7::{part_1, part_2};
    use std::io::BufReader;
    use test::Bencher;

    const INP: &[u8] = include_str!("../test/day7.txt").as_bytes();

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(BufReader::new(INP)), 37);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(BufReader::new(INP)), 168);
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
