/* --- Day 11: Dumbo Octopus --- */

use std::convert::TryInto;
use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;

struct Point(i8, i8);

#[derive(Debug)]
struct OctopusGrid {
    grid: [[u8; 10]; 10],
    flashes: i32,
}

impl From<Vec<String>> for OctopusGrid {
    fn from(v: Vec<String>) -> OctopusGrid {
        let a: [String; 10] = v.try_into().unwrap();
        OctopusGrid {
            grid: a.map(string_to_u8array),
            flashes: 0,
        }
    }
}

impl OctopusGrid {
    fn get_value(&self, p: &Point) -> u8 {
        self.grid[p.0 as usize][p.1 as usize]
    }

    fn stack_em(&self) -> Vec<Point> {
        let mut stack = vec![];
        for x in 0..10 as i8 {
            for y in 0..10 as i8 {
                stack.push(Point(x, y));
            }
        }
        stack
    }

    fn increment_octopus(&mut self, p: &Point) {
        self.grid[p.0 as usize][p.1 as usize] += 1;
    }

    fn reset_flashers(&mut self) {
        for x in 0..10 {
            for y in 0..10 {
                if self.get_value(&Point(x, y)) >= 10 {
                    self.grid[x as usize][y as usize] = 0;
                }
            }
        }
    }

    fn is_flashing(&self, p: &Point) -> bool {
        self.grid[p.0 as usize][p.1 as usize] == 10
    }

    fn step(&mut self) {
        let mut stack = self.stack_em();

        while !stack.is_empty() {
            let p = stack.pop().unwrap();
            self.increment_octopus(&p);
            if self.is_flashing(&p) {
                self.flashes += 1;
                stack.append(&mut get_neighbors(&p))
            }
        }

        self.reset_flashers()
    }

    fn step_2(&mut self) -> bool {
        let mut stack = self.stack_em();
        let mut flashes = 0;

        while !stack.is_empty() {
            let p = stack.pop().unwrap();
            self.increment_octopus(&p);
            if self.is_flashing(&p) {
                self.flashes += 1;
                flashes += 1;
                stack.append(&mut get_neighbors(&p))
            }
        }

        self.reset_flashers();

        flashes == 100
    }
}

fn string_to_u8array(s: String) -> [u8; 10] {
    s.chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}

fn get_neighbors(p: &Point) -> Vec<Point> {
    let mut neighbors: Vec<Point> = vec![
        Point(p.0 - 1, p.1 - 1),
        Point(p.0 - 1, p.1),
        Point(p.0 - 1, p.1 + 1),
        Point(p.0, p.1 - 1),
        Point(p.0, p.1 + 1),
        Point(p.0 + 1, p.1 - 1),
        Point(p.0 + 1, p.1),
        Point(p.0 + 1, p.1 + 1),
    ];
    neighbors.retain(|p| valid_coordinates(&p));
    neighbors
}

fn valid_coordinates(p: &Point) -> bool {
    !(p.0 < 0 || p.0 >= 10 || p.1 < 0 || p.1 >= 10)
}

pub fn part_1<R: Read>(inp: BufReader<R>) -> i32 {
    let mut grid = OctopusGrid::from(inp.lines().map(Result::unwrap).collect_vec());

    for _ in 0..100 {
        grid.step();
    }

    grid.flashes
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> i32 {
    let mut grid = OctopusGrid::from(inp.lines().map(Result::unwrap).collect_vec());

    for i in 0.. {
        if grid.step_2() {
            return i + 1;
        }
    }

    return 0;
}

#[cfg(test)]
mod test {
    use crate::day11::{part_1, part_2};
    use std::io::BufReader;
    use test::Bencher;

    const INP: &[u8] = include_str!("../test/day11.txt").as_bytes();

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(BufReader::new(INP)), 1656);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(BufReader::new(INP)), 195);
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
