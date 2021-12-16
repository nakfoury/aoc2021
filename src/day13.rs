/* --- Day 13: Transparent Origami --- */

use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;

type Point = (u32, u32);
type Instruction = (bool, u32);

struct TransparentPaper {
    points: Vec<Point>,
    instructions: Vec<Instruction>,
}

impl<R: Read> From<BufReader<R>> for TransparentPaper {
    fn from(inp: BufReader<R>) -> Self {
        let inp = inp.lines().into_iter().map(Result::unwrap).collect_vec();

        let points: Vec<Point> = inp
            .clone()
            .into_iter()
            .filter(|l| l.contains(","))
            .map(|s| {
                Point::from(
                    s.split_once(",")
                        .map(|(x, y)| {
                            (
                                x.to_string().parse::<u32>().unwrap(),
                                y.to_string().parse::<u32>().unwrap(),
                            )
                        })
                        .unwrap(),
                )
            })
            .collect_vec();

        let instructions: Vec<Instruction> = inp
            .into_iter()
            .filter(|l| l.contains("="))
            .map(|s| {
                Instruction::from(
                    s.split_once("=")
                        .map(|(a, b)| (a.contains("x"), b.to_string().parse::<u32>().unwrap()))
                        .unwrap(),
                )
            })
            .collect_vec();

        TransparentPaper {
            points,
            instructions,
        }
    }
}

impl TransparentPaper {
    fn fold(&mut self, inst: Instruction) {
        let (vertical, fold_pos) = inst;
        for point in &mut self.points {
            if vertical {
                if point.0 > fold_pos {
                    point.0 = 2 * fold_pos - point.0;
                }
            } else {
                if point.1 > fold_pos {
                    point.1 = 2 * fold_pos - point.1;
                }
            }
        }
        self.points.sort();
        self.points.dedup();
    }

    fn instruction_list(&self) -> Vec<Instruction> {
        self.instructions.clone()
    }

    fn count(&self) -> i32 {
        self.points.len() as i32
    }

    fn plot(&self) {
        let mut plot = [[' '; 40]; 6];
        for pt in &self.points {
            plot[pt.1 as usize][pt.0 as usize] = '#';
        }
        for row in plot {
            println!("{:?}", row.iter().collect::<String>());
        }
    }
}

pub fn part_1<R: Read>(inp: BufReader<R>) -> i32 {
    let mut paper = TransparentPaper::from(inp);
    paper.fold(*paper.instructions.first().unwrap());
    paper.count()
}

pub fn part_2<R: Read>(inp: BufReader<R>) -> i32 {
    let mut paper = TransparentPaper::from(inp);
    for inst in paper.instruction_list() {
        paper.fold(inst);
    }
    paper.plot();
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
        assert_eq!(part_1(BufReader::new(INP)), 17);
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
