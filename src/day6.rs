/* --- Day 6: Lanternfish --- */

use std::io::{BufRead, BufReader, Read};

fn count_children(days_to_live: i64) -> i64 {
    if days_to_live < 0 {
        0
    } else if days_to_live < 7 {
        1
    } else {
        (0..days_to_live + 1)
            .rev()
            .step_by(7)
            .map(|t| 1 + count_children(t - 9))
            .sum()
    }
}

pub fn part_1<R: Read>(inp: BufReader<R>) -> i64 {
    let school = inp.lines().next().unwrap().unwrap();
    let school = school.split(',').map(str::parse::<i64>).map(Result::unwrap);

    school.map(|t| 1 + count_children(79 - t)).sum()
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> i64 {
    let mut school: [i64; 9] = [0; 9];
    for f in inp
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
    {
        school[f] += 1
    }

    for _ in 0..256 {
        let mut new_school: [i64; 9] = [0; 9];
        for i in 0..8 {
            new_school[i] = school[i + 1];
        }
        new_school[8] = school[0];
        new_school[6] += school[0];
        school = new_school
    }

    school.iter().sum::<i64>()
}

#[cfg(test)]
mod test {
    use crate::day6::{count_children, part_1, part_2};
    use std::io::BufReader;
    use test::Bencher;

    const INP: &[u8] = include_str!("../test/day6.txt").as_bytes();

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(BufReader::new(INP)), 5934);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(BufReader::new(INP)), 26984457539);
    }

    #[test]
    fn test_count_children_0() {
        assert_eq!(2, count_children(7)) // 7, 0
    }

    #[test]
    fn test_count_children_1() {
        assert_eq!(1, count_children(6)) // 6
    }

    #[test]
    fn test_count_children_2() {
        assert_eq!(4, count_children(14)) // 14, 7, 0; 5
    }

    #[test]
    fn test_count_children_3() {
        assert_eq!(8, count_children(21)) // 21, 14, 7, 0; 12, 5; 3; 5
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
