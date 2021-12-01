use std::{
    fs,
    io::{BufRead, BufReader, Read},
};

use test::Bencher;

pub fn part_1<R: Read>(inp: BufReader<R>) -> u32 {
    let mut result = 0;
    let mut prev = u32::MAX;

    for l in inp.lines() {
        let cur = l.unwrap().parse::<u32>().unwrap();
        if cur > prev {
            result += 1;
        }
        prev = cur;
    }
    result
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> u32 {
    let mut result: u32 = 0;

    let mut win_a: Window = Window {
        next: 0,
        numbers: [0, 0, 0],
    };

    let mut win_b: Window = Window {
        next: 0,
        numbers: [0, 0, 0],
    };

    let mut prev = u32::MAX;

    for (i, line) in inp.lines().enumerate() {
        let cur = line.unwrap().parse::<u32>().unwrap();
        match i {
            0 => win_a.rotate(cur),
            1 | 2 => {
                win_a.rotate(cur);
                win_b.rotate(cur);
            }
            3 => {
                win_b.rotate(cur);
                if win_b.sum() > win_a.sum() {
                    result += 1;
                }
            }
            _ => {
                win_a.rotate(prev);
                win_b.rotate(cur);
                if win_b.sum() > win_a.sum() {
                    result += 1;
                }
            }
        }
        prev = cur;
    }
    result
}

#[derive(Clone, Copy)]
struct Window {
    next: usize,
    numbers: [u32; 3],
}

impl Window {
    fn sum(self) -> u32 {
        self.numbers[0] + self.numbers[1] + self.numbers[2]
    }

    fn rotate(&mut self, num: u32) {
        self.numbers[self.next % 3] = num;
        self.next += 1;
    }
}

#[bench]
fn bench_part1(b: &mut Bencher) {
    let input = include_str!("../input/day1.txt");
    let input = input.as_bytes();
    b.iter(|| part_1(BufReader::new(input)))
}

#[bench]
fn bench_part2(b: &mut Bencher) {
    let input = include_str!("../input/day1.txt");
    let input = input.as_bytes();
    b.iter(|| part_2(BufReader::new(input)))
}
