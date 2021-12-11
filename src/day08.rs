/* --- Day 8: Seven Segment Search --- */

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use itertools::Itertools;

type WireConfig = HashMap<char, char>;

fn deduce_config(inp: String) -> WireConfig {
    let mut map = WireConfig::new();

    // use unique occurences to find b, e, f
    for character in "abcdefg".chars() {
        match &inp.chars().filter(|&c| c == character).count() {
            4 => map.insert('e', character),
            6 => map.insert('b', character),
            9 => map.insert('f', character),
            _ => None,
        };
    }

    let mut inp = inp
        .split_whitespace()
        .into_iter()
        .sorted_by_key(|s| s.len());

    // use 1 to find c
    map.insert(
        'c',
        inp.next()
            .unwrap()
            .chars()
            .filter(|&c| c != map[&'f'])
            .next()
            .unwrap(),
    );

    // use 7 to find a
    map.insert(
        'a',
        inp.next()
            .unwrap()
            .chars()
            .filter(|&c| c != map[&'f'])
            .filter(|&c| c != map[&'c'])
            .next()
            .unwrap(),
    );

    // use 4 to find d
    map.insert(
        'd',
        inp.next()
            .unwrap()
            .chars()
            .filter(|&c| c != map[&'f'])
            .filter(|&c| c != map[&'c'])
            .filter(|&c| c != map[&'b'])
            .next()
            .unwrap(),
    );

    for c in "abcdefg".chars() {
        if !map.values().any(|&v| v == c) {
            map.insert('g', c);
            break;
        }
    }

    map
}

fn decode_value(inp: String, config: WireConfig) -> i32 {
    let reference: HashMap<String, &str> = HashMap::from([
        (String::from("cf"), "1"),
        (String::from("acdeg"), "2"),
        (String::from("acdfg"), "3"),
        (String::from("bcdf"), "4"),
        (String::from("abdfg"), "5"),
        (String::from("abdefg"), "6"),
        (String::from("acf"), "7"),
        (String::from("abcdefg"), "8"),
        (String::from("abcdfg"), "9"),
        (String::from("abcefg"), "0"),
    ]);

    inp.split_whitespace()
        .map(|s| {
            s.chars()
                .map(|c| get_key(&config, &c))
                .sorted()
                .collect::<String>()
        })
        .map(|s| reference[&s])
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

fn get_key(m: &HashMap<char, char>, val: &char) -> char {
    for k in m.keys() {
        if m[k] == *val {
            return *k;
        }
    }
    return '0';
}

pub fn part_1<R: Read>(inp: BufReader<R>) -> i32 {
    let mut c: i32 = 0;
    for line in inp.lines().map(Result::unwrap) {
        c += line
            .split("|")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .into_iter()
            .filter(|n| n.len() != 5 && n.len() != 6)
            .count() as i32;
    }
    c
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> i32 {
    let mut total: i32 = 0;
    for line in inp.lines().map(Result::unwrap) {
        let mut s = line.split("|");
        let config = deduce_config(s.next().unwrap().try_into().unwrap());
        total += decode_value(s.next().unwrap().try_into().unwrap(), config);
    }
    total
}

#[cfg(test)]
mod test {
    use crate::day08::{part_1, part_2};
    use std::io::BufReader;
    use test::Bencher;

    const INP: &[u8] = include_str!("../test/day8.txt").as_bytes();

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(BufReader::new(INP)), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(BufReader::new(INP)), 61229);
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
