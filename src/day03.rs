/* --- Day 3: Binary Diagnostic --- */

use std::io::{BufRead, BufReader, Read};

pub fn part_1<R: Read>(inp: BufReader<R>) -> i32 {
    let lines: Vec<String> = inp.lines().map(|l| l.unwrap()).collect();

    let n_lines = lines.len();
    let n_cols = lines[0].len();

    let mut cols: Vec<Vec<char>> = vec![vec![]; n_cols];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            cols[i].push(c);
        }
    }

    let mut gamma: Vec<char> = vec!['0'; n_cols];
    let mut epsilon: Vec<char> = vec!['0'; n_cols];

    for i in 0..n_cols {
        let sum = cols[i].iter().filter(|&c| c == &'1').count();
        if sum * 2 > n_lines {
            gamma[i] = '1';
        } else {
            epsilon[i] = '1';
        }
    }

    bin_str_to_dec(gamma.iter().collect()) * bin_str_to_dec(epsilon.iter().collect())
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> i32 {
    let mut oxy_values: Vec<String> = inp.lines().map(|l| l.unwrap()).collect();
    let mut co2_values: Vec<String> = oxy_values.clone();

    let n_cols = oxy_values[0].len();

    for i in 0..n_cols {
        let oxy_len = oxy_values.len();
        let co2_len = co2_values.len();

        if oxy_len != 1 {
            if count_1s(&oxy_values, i) * 2 >= oxy_len {
                oxy_values = filter_by_char_in_pos(&oxy_values, i, '1')
            } else {
                oxy_values = filter_by_char_in_pos(&oxy_values, i, '0')
            }
        }

        if co2_len != 1 {
            if count_1s(&co2_values, i) * 2 < co2_len {
                co2_values = filter_by_char_in_pos(&co2_values, i, '1')
            } else {
                co2_values = filter_by_char_in_pos(&co2_values, i, '0')
            }
        }
    }

    bin_str_to_dec(oxy_values[0].to_string()) * bin_str_to_dec(co2_values[0].to_string())
}

fn count_1s(v: &Vec<String>, pos: usize) -> usize {
    v.iter()
        .filter(|&c| c.chars().nth(pos).unwrap() == '1')
        .count()
}

fn filter_by_char_in_pos(v: &Vec<String>, pos: usize, c: char) -> Vec<String> {
    let (part, _) = v
        .clone()
        .into_iter()
        .partition(|x| (x.chars().nth(pos).unwrap()) == c);
    part
}

fn bin_str_to_dec(s: String) -> i32 {
    isize::from_str_radix(&*s, 2).unwrap() as i32
}

#[cfg(test)]
mod unit_test {
    use crate::day03::{bin_str_to_dec, part_1, part_2};
    use std::io::BufReader;
    use test::Bencher;

    const INP: &[u8] = include_str!("../test/day3.txt").as_bytes();

    #[test]
    fn test_bin_str() {
        let inp = "0101";
        assert_eq!(bin_str_to_dec(String::from(inp)), 5);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(BufReader::new(INP)), 198);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(BufReader::new(INP)), 230);
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
