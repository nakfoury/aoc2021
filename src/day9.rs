/* --- Day 9: Smoke Basin --- */

use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;

struct Cell {
    value: u8,
    visited: bool,
}

struct LavaMap2 {
    grid: Vec<Vec<Cell>>,
}

impl From<Vec<String>> for LavaMap2 {
    fn from(v: Vec<String>) -> Self {
        let grid = v
            .into_iter()
            .map(|s| {
                s.chars()
                    .map(|c| Cell {
                        value: c.to_string().parse::<u8>().unwrap(),
                        visited: false,
                    })
                    .collect_vec()
            })
            .collect_vec();

        LavaMap2 { grid }
    }
}

impl LavaMap2 {
    fn length(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn get_basin_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];

        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.length() - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.width() - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors.retain(|(x, y)| !self.grid[*x][*y].visited);
        neighbors.retain(|(x, y)| self.grid[*x][*y].value != 9);
        neighbors
    }

    fn expand_basin(&mut self, x: usize, y: usize) -> i32 {
        let mut basin_size = 0;
        let mut stack: Vec<(usize, usize)> = vec![(x, y)];
        self.grid[x][y].visited = true;
        while let Some((x, y)) = stack.pop() {
            basin_size += 1;
            for neighbor in self.get_basin_neighbors(x, y) {
                self.grid[neighbor.0][neighbor.1].visited = true;
                stack.push(neighbor);
            }
        }
        basin_size
    }

    fn survey_basins(&mut self) -> i32 {
        let mut sizes: Vec<i32> = vec![];
        for x in 0..self.length() {
            for y in 0..self.width() {
                if self.grid[x][y].value == 9 {
                    self.grid[x][y].visited = true;
                }
                if self.grid[x][y].visited {
                    continue;
                }

                sizes.push(self.expand_basin(x, y));
            }
        }

        sizes.sort();
        sizes.reverse();
        sizes[0..3].iter().product()
    }
}

struct LavaMap {
    map: Vec<Vec<u8>>,
}

impl LavaMap {
    pub fn new(map: Vec<Vec<u8>>) -> Self {
        LavaMap { map }
    }

    fn length(&self) -> usize {
        self.map.len()
    }

    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn get_neighboring_values(&self, x: usize, y: usize) -> Vec<u8> {
        let mut values = vec![];

        if x > 0 {
            values.push(self.map[x - 1][y])
        }
        if x < self.length() - 1 {
            values.push(self.map[x + 1][y])
        }
        if y > 0 {
            values.push(self.map[x][y - 1])
        }
        if y < self.width() - 1 {
            values.push(self.map[x][y + 1])
        }

        values
    }
}

pub fn part_1<R: Read>(inp: BufReader<R>) -> i32 {
    let mut lava_map: Vec<Vec<u8>> = vec![];
    for line in inp.lines() {
        lava_map.push(
            line.unwrap()
                .chars()
                .map(|c| (c as u8) - ('0' as u8))
                .collect(),
        )
    }

    let lava_map = LavaMap::new(lava_map);

    let mut risk_sum: i32 = 0;

    for i in 0..lava_map.length() {
        for j in 0..lava_map.width() {
            if lava_map
                .get_neighboring_values(i, j)
                .iter()
                .any(|v| v <= &lava_map.map[i][j])
            {
                continue;
            } else {
                risk_sum += lava_map.map[i][j] as i32 + 1;
            }
        }
    }

    risk_sum
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> i32 {
    let mut lava_map = LavaMap2::from(inp.lines().map(Result::unwrap).collect_vec());
    lava_map.survey_basins()
}

#[cfg(test)]
mod test {
    use crate::day9::{part_1, part_2};
    use std::io::BufReader;
    use test::Bencher;

    const INP: &[u8] = include_str!("../test/day9.txt").as_bytes();

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(BufReader::new(INP)), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(BufReader::new(INP)), 1134);
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
