/* --- Day 2: Dive! --- */

use std::io::{BufRead, BufReader, Read};

pub fn part_1<R: Read>(inp: BufReader<R>) -> i32 {
    let mut h_pos: i32 = 0;
    let mut v_pos: i32 = 0;

    for line in inp.lines() {
        let l = line.unwrap();
        let mut instruction = l.split_whitespace();
        match instruction.next().unwrap() {
            "forward" => h_pos += instruction.next().unwrap().parse::<i32>().unwrap(),
            "down" => v_pos += instruction.next().unwrap().parse::<i32>().unwrap(),
            "up" => v_pos -= instruction.next().unwrap().parse::<i32>().unwrap(),
            _ => (),
        }
    }
    v_pos * h_pos
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> i32 {
    let mut h_pos: i32 = 0;
    let mut v_pos: i32 = 0;
    let mut aim: i32 = 0;

    for line in inp.lines() {
        let l = line.unwrap();
        let mut instruction = l.split_whitespace();
        match instruction.next().unwrap() {
            "forward" => {
                let t: i32 = instruction.next().unwrap().parse::<i32>().unwrap();
                h_pos += t;
                v_pos += t * aim;
            }
            "down" => aim += instruction.next().unwrap().parse::<i32>().unwrap(),
            "up" => aim -= instruction.next().unwrap().parse::<i32>().unwrap(),
            _ => (),
        }
    }
    v_pos * h_pos
}

#[cfg(test)]
mod unit_test {
    use crate::day2::{part_1, part_2};
    use std::io::BufReader;
    use test::Bencher;

    const INP: &[u8] = include_str!("../test/day2.txt").as_bytes();

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(BufReader::new(INP)), 150);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(BufReader::new(INP)), 900);
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
