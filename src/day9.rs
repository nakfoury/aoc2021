use std::io::{BufRead, BufReader, Read};

/* --- Day 9: Smoke Basin --- */

// type LavaMap = Vec<Vec<u8>>;
struct LavaMap {
    map: Vec<Vec<u8>>,
    length: usize,
    width: usize,
}

impl LavaMap {
    pub fn new(map: Vec<Vec<u8>>) -> Self {
        let length = map.len();
        let width = map[0].len();
        LavaMap { map, length, width }
    }

    fn get_neighboring_values(&self, x: usize, y: usize) -> Vec<u8> {
        let mut values = vec![];

        if x > 0 {
            values.push(self.map[x - 1][y])
        }
        if x < self.length - 1 {
            values.push(self.map[x + 1][y])
        }
        if y > 0 {
            values.push(self.map[x][y - 1])
        }
        if y < self.width - 1 {
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

    for i in 0..lava_map.length {
        for j in 0..lava_map.width {
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

pub fn part_2<R: Read>(_inp: BufReader<R>) -> i32 {
    0
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
