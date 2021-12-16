/*  */

use std::io::{BufRead, BufReader, Read};

pub fn part_1<R: Read>(inp: BufReader<R>) -> i32 {
    0
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use crate::day13::{part_1, part_2};
    use std::io::BufReader;
    use test::Bencher;

    const INP: &[u8] = include_str!("../test/day13.txt").as_bytes();

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
