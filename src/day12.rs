/* --- Day 12: Passage Pathing --- */

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    slice::SliceIndex,
};

use itertools::{concat, Itertools};

#[derive(Debug)]
struct CaveSystem(HashMap<String, Vec<String>>);

impl<R: Read> From<BufReader<R>> for CaveSystem {
    fn from(inp: BufReader<R>) -> Self {
        let mut system: HashMap<String, Vec<String>> = HashMap::new();

        // Process input.
        let edges = inp
            .lines()
            .map(Result::unwrap)
            .map(|s| {
                s.split_once('-')
                    .map(|(a, b)| (a.to_string(), b.to_string()))
            })
            .map(Option::unwrap)
            .collect_vec();

        // Initialize cave system.
        for edge in edges {
            match system.get_mut(&edge.0) {
                Some(v) => v.push(edge.1.clone()),
                None => {
                    system.insert(edge.0.clone(), vec![edge.1.clone()]);
                }
            }
            match system.get_mut(&edge.1) {
                Some(v) => v.push(edge.0.clone()),
                None => {
                    system.insert(edge.1.clone(), vec![edge.0.clone()]);
                }
            }
        }

        CaveSystem(system)
    }
}

impl CaveSystem {
    fn get_neighbors(&self, cave: &String) -> &Vec<String> {
        self.0.get(cave).unwrap()
    }

    fn explore(&self, cave: &String, path: Vec<String>) -> i32 {
        if cave == "end" {
            return 1;
        }

        if path.contains(cave) {
            if cave.chars().any(|c| (c as u8) < 65 || (c as u8) > 90) {
                return 0;
            }
        }

        let mut path = path;
        path.push(cave.to_string());

        self.get_neighbors(cave)
            .iter()
            .map(|n| self.explore(n, path.clone()))
            .sum()
    }

    fn explore_2(&self, cave: &String, path: Vec<String>, spare_time: bool) -> i32 {
        if cave == "end" {
            return 1;
        }

        let mut spare_time = spare_time;

        if path.contains(cave) {
            if cave == &String::from("start") {
                return 0;
            }
            if cave.chars().any(|c| (c as u8) < 65 || (c as u8) > 90) {
                if spare_time {
                    spare_time = false;
                } else {
                    return 0;
                }
            }
        }

        let mut path = path;
        path.push(cave.to_string());

        self.get_neighbors(cave)
            .iter()
            .map(|n| self.explore_2(n, path.clone(), spare_time.clone()))
            .sum()
    }
}

pub fn part_1<R: Read>(inp: BufReader<R>) -> i32 {
    let cave_system = CaveSystem::from(inp);
    cave_system.explore(&String::from("start"), vec![])
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> i32 {
    let cave_system = CaveSystem::from(inp);
    cave_system.explore_2(&String::from("start"), vec![], true)
}

#[cfg(test)]
mod test {
    use crate::day12::{part_1, part_2};
    use std::io::BufReader;
    use test::Bencher;

    const INP: &[u8] = include_str!("../test/day12.txt").as_bytes();

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(BufReader::new(INP)), 10);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(BufReader::new(INP)), 36);
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
