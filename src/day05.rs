/* --- Day 5: Hydrothermal Venture --- */

use std::io::{BufRead, BufReader, Read};

#[derive(Debug, PartialEq, Eq)]
struct Point(i32, i32);
struct Line(Point, Point);

impl FromIterator<i32> for Point {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Point(iter.next().unwrap(), iter.next().unwrap())
    }
}

impl FromIterator<Point> for Line {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Line(iter.next().unwrap(), iter.next().unwrap())
    }
}

fn is_horizontal_or_vertical(ln: &Line) -> bool {
    ln.0 .0 == ln.1 .0 || ln.0 .1 == ln.1 .1
}

fn points_in_line(ln: &Line) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];

    let mut x = vec![ln.0 .0, ln.1 .0];
    let mut y = vec![ln.0 .1, ln.1 .1];
    x.sort_unstable();
    y.sort_unstable();

    for x in x[0]..x[1] + 1 {
        for y in y[0]..y[1] + 1 {
            points.push(Point(x, y))
        }
    }
    points
}

fn is_diagonal(ln: &Line) -> bool {
    (ln.0 .0 - ln.1 .0).abs() == (ln.0 .1 - ln.1 .1).abs()
}

fn points_in_diagonal_line(ln: &Line) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];

    let x_vec;
    let y_vec;

    if ln.0 .0 > ln.1 .0 {
        x_vec = -1;
    } else {
        x_vec = 1;
    }

    if ln.0 .1 > ln.1 .1 {
        y_vec = -1;
    } else {
        y_vec = 1;
    }

    for i in 0..((ln.1 .0 - ln.0 .0).abs()) + 1 {
        points.push(Point(ln.0 .0 + (i * x_vec), ln.0 .1 + (i * y_vec)))
    }

    points
}

pub fn part_1<R: Read>(inp: BufReader<R>) -> i32 {
    let mut ocean_floor = vec![vec![0; 1000]; 1000];

    for line in inp.lines().map(|l| l.unwrap()).collect::<Vec<String>>() {
        let ln: Line = line
            .split(" -> ")
            .map(|p| {
                p.split(',')
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect::<Point>()
            })
            .collect::<Line>();

        if is_horizontal_or_vertical(&ln) {
            for p in points_in_line(&ln) {
                ocean_floor[p.0 as usize][p.1 as usize] += 1;
            }
        }
    }
    let mut overlaps: i32 = 0;

    for row in ocean_floor {
        for j in row {
            if j >= 2 {
                overlaps += 1;
            }
        }
    }
    overlaps
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> i32 {
    let mut ocean_floor = vec![vec![0; 1000]; 1000];

    for line in inp.lines().map(|l| l.unwrap()).collect::<Vec<String>>() {
        let ln: Line = line
            .split(" -> ")
            .map(|p| {
                p.split(',')
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect::<Point>()
            })
            .collect::<Line>();

        if is_horizontal_or_vertical(&ln) {
            for p in points_in_line(&ln) {
                ocean_floor[p.0 as usize][p.1 as usize] += 1;
            }
        } else if is_diagonal(&ln) {
            for p in points_in_diagonal_line(&ln) {
                ocean_floor[p.0 as usize][p.1 as usize] += 1;
            }
        }
    }

    let mut overlaps: i32 = 0;

    for row in ocean_floor {
        for j in row {
            if j >= 2 {
                overlaps += 1;
            }
        }
    }
    overlaps
}

#[cfg(test)]
mod test {
    use crate::day05::{
        is_diagonal, is_horizontal_or_vertical, part_1, part_2, points_in_diagonal_line,
        points_in_line, Line, Point,
    };
    use std::io::BufReader;
    use test::Bencher;

    const INP: &[u8] = include_str!("../test/day5.txt").as_bytes();

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(BufReader::new(INP)), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(BufReader::new(INP)), 12);
    }

    #[test]
    fn test_points_in_line() {
        let ln = Line(Point(0, 5), Point(3, 5));
        let mut expected: Vec<Point> = vec![];

        expected.push(Point(0, 5));
        expected.push(Point(1, 5));
        expected.push(Point(2, 5));
        expected.push(Point(3, 5));

        assert_eq!(points_in_line(&ln), expected)
    }

    #[test]
    fn test_points_in_diagonal_line() {
        let ln = Line(Point(1, 1), Point(4, 4));
        let mut expected: Vec<Point> = vec![];

        expected.push(Point(1, 1));
        expected.push(Point(2, 2));
        expected.push(Point(3, 3));
        expected.push(Point(4, 4));

        assert_eq!(points_in_diagonal_line(&ln), expected)
    }

    #[test]
    fn test_points_in_diagonal_line_2() {
        let ln = Line(Point(4, 4), Point(1, 1));
        let mut expected: Vec<Point> = vec![];

        expected.push(Point(4, 4));
        expected.push(Point(3, 3));
        expected.push(Point(2, 2));
        expected.push(Point(1, 1));

        assert_eq!(points_in_diagonal_line(&ln), expected)
    }

    #[test]
    fn test_is_horizontal() {
        let ln = Line(Point(0, 5), Point(3, 5));

        assert_eq!(is_horizontal_or_vertical(&ln), true)
    }

    #[test]
    fn test_is_vertical() {
        let ln = Line(Point(3, 7), Point(3, 5));

        assert_eq!(is_horizontal_or_vertical(&ln), true)
    }

    #[test]
    fn test_is_diagonal() {
        let ln = Line(Point(1, 1), Point(4, 4));

        assert_eq!(is_diagonal(&ln), true)
    }

    #[test]
    fn test_is_diagonal_2() {
        let ln = Line(Point(77, 29), Point(75, 27));

        assert_eq!(is_diagonal(&ln), true)
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
