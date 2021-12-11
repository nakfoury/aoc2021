/* --- Day 4: Giant Squid --- */

use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;

type Board = [Option<i32>; 25];

fn mark_number(b: &mut Board, num: i32) -> bool {
    match b.iter().position(|n| *n == Some(num)) {
        Some(n) => {
            b[n] = None;
            true
        }
        None => false,
    }
}

fn is_win(b: &Board) -> bool {
    is_win_by_col(b) || is_win_by_row(b)
}

fn is_win_by_row(b: &Board) -> bool {
    for mut row in &b.iter().chunks(5) {
        if row.all_equal() {
            return true;
        }
    }
    false
}

fn is_win_by_col(b: &Board) -> bool {
    for i in 0..5 {
        if b[i..].iter().step_by(5).all_equal() {
            return true;
        }
    }
    false
}

fn score_board(b: &Board) -> i32 {
    b.iter()
        .filter_map(|x| *x)
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

pub fn part_1<R: Read>(inp: BufReader<R>) -> i32 {
    let mut lines = inp.lines();

    let bingo_numbers = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>();

    let mut bingo_boards: Vec<Board> = vec![];

    for board_chunk in &lines.chunks(6) {
        bingo_boards.push(
            board_chunk
                .map(|l| l.unwrap())
                .collect::<Vec<String>>()
                .join(" ")
                .split_whitespace()
                .map(|n| Some(n.parse().unwrap()))
                .collect::<Vec<Option<i32>>>()
                .try_into()
                .unwrap(),
        );
    }

    for num in bingo_numbers {
        for board in &mut bingo_boards {
            if mark_number(board, num) && is_win(board) {
                return num * score_board(board);
            }
        }
    }

    0
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> i32 {
    let mut lines = inp.lines();

    let bingo_numbers = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>();

    let mut bingo_boards: Vec<Board> = vec![];

    for board_chunk in &lines.chunks(6) {
        bingo_boards.push(
            board_chunk
                .map(|l| l.unwrap())
                .collect::<Vec<String>>()
                .join(" ")
                .split_whitespace()
                .map(|n| Some(n.parse().unwrap()))
                .collect::<Vec<Option<i32>>>()
                .try_into()
                .unwrap(),
        );
    }

    let mut score_moves_tuple_list: Vec<(usize, i32)> = vec![];

    for mut board in bingo_boards {
        for (i, num) in bingo_numbers.iter().enumerate() {
            if mark_number(&mut board, *num) && is_win(&board) {
                score_moves_tuple_list.push((i, num * score_board(&board)));
                break;
            }
        }
    }

    score_moves_tuple_list.sort_by_key(|t| t.0);
    score_moves_tuple_list.last().unwrap().1
}

#[cfg(test)]
mod test {
    use crate::day04::{is_win_by_col, is_win_by_row, part_1, part_2, score_board};
    use std::io::BufReader;
    use test::Bencher;

    const INP: &[u8] = include_str!("../test/day4.txt").as_bytes();

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(BufReader::new(INP)), 4512);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(BufReader::new(INP)), 1924);
    }

    #[test]
    fn test_score_board() {
        let inp = [
            None,
            None,
            None,
            None,
            None,
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11),
            Some(12),
            Some(13),
            Some(14),
            Some(15),
            Some(16),
            Some(17),
            Some(18),
            Some(19),
            Some(20),
        ];
        assert_eq!(score_board(&inp), 210);
    }

    #[test]
    fn test_is_win_by_col() {
        let inp = [
            Some(1),
            None,
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            None,
            Some(10),
            Some(11),
            Some(12),
            Some(13),
            None,
            Some(14),
            Some(15),
            Some(16),
            Some(17),
            None,
            Some(18),
            Some(19),
            Some(20),
        ];
        assert_eq!(is_win_by_col(&inp), true);
    }

    #[test]
    fn test_is_win_by_col_fail() {
        let inp = [
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11),
            Some(12),
            Some(13),
            Some(14),
            Some(15),
            Some(16),
            Some(17),
            Some(18),
            Some(19),
            Some(20),
            Some(21),
            Some(22),
            Some(23),
            Some(24),
            Some(25),
        ];
        assert_eq!(is_win_by_col(&inp), false);
    }

    #[test]
    fn test_is_win_by_row() {
        let inp = [
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            None,
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
        ];
        assert_eq!(is_win_by_row(&inp), true);
    }

    #[test]
    fn test_is_win_by_row_fail() {
        let inp = [
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11),
            Some(12),
            Some(13),
            Some(14),
            Some(15),
            Some(16),
            Some(17),
            Some(18),
            Some(19),
            Some(20),
            Some(21),
            Some(22),
            Some(23),
            Some(24),
            Some(25),
        ];
        assert_eq!(is_win_by_row(&inp), false);
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
