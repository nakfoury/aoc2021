use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use aoc2021::*;

fn get_input(day: i32) -> io::BufReader<File> {
    let p = format!("input/day{}.txt", day);
    let path = Path::new(&p);

    if !path.exists() {
        download_input(day).unwrap();
    }

    let file: File = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file: {:?}", e),
    };

    io::BufReader::new(file)
}

fn download_input(day: i32) -> Result<(), reqwest::Error> {
    println!("Making web request...");
    let client = reqwest::blocking::Client::new();

    let url = format!("https://adventofcode.com/2021/day/{}/input", day);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "content-type",
        reqwest::header::HeaderValue::from_static("text/plain"),
    );
    headers.insert(
        "Cookie",
        reqwest::header::HeaderValue::from_static(include_str!("cookie.txt")),
    );

    match client.get(&url).headers(headers).send() {
        Ok(res) => {
            let mut f = File::create(Path::new(&format!("input/day{}.txt", day))).unwrap();
            f.write_all(res.text().unwrap().as_bytes()).unwrap();
            Ok(())
        }
        Err(error) => Err(error),
    }
}

fn main() {
    println!("Day1 Part1: {:?}", day01::part_1(get_input(1)));
    println!("Day1 Part2: {:?}", day01::part_2(get_input(1)));
    println!("Day2 Part1: {:?}", day02::part_1(get_input(2)));
    println!("Day2 Part2: {:?}", day02::part_2(get_input(2)));
    println!("Day3 Part1: {:?}", day03::part_1(get_input(3)));
    println!("Day3 Part2: {:?}", day03::part_2(get_input(3)));
    println!("Day4 Part1: {:?}", day04::part_1(get_input(4)));
    println!("Day4 Part2: {:?}", day04::part_2(get_input(4)));
    println!("Day5 Part1: {:?}", day05::part_1(get_input(5)));
    println!("Day5 Part2: {:?}", day05::part_2(get_input(5)));
    println!("Day6 Part1: {:?}", day06::part_1(get_input(6)));
    println!("Day6 Part2: {:?}", day06::part_2(get_input(6)));
    println!("Day7 Part1: {:?}", day07::part_1(get_input(7)));
    println!("Day7 Part2: {:?}", day07::part_2(get_input(7)));
    println!("Day8 Part1: {:?}", day08::part_1(get_input(8)));
    println!("Day8 Part2: {:?}", day08::part_2(get_input(8)));
    println!("Day9 Part1: {:?}", day09::part_1(get_input(9)));
    println!("Day9 Part2: {:?}", day09::part_2(get_input(9)));
    println!("Day10 Part1: {:?}", day10::part_1(get_input(10)));
    println!("Day10 Part2: {:?}", day10::part_2(get_input(10)));
    println!("Day11 Part1: {:?}", day11::part_1(get_input(11)));
    println!("Day11 Part2: {:?}", day11::part_2(get_input(11)));
    println!("Day12 Part1: {:?}", day12::part_1(get_input(12)));
    println!("Day12 Part2: {:?}", day12::part_2(get_input(12)));
}
