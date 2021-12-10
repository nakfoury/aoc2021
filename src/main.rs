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
    println!("Day1 Part1: {:?}", day1::part_1(get_input(1)));
    println!("Day1 Part2: {:?}", day1::part_2(get_input(1)));
    println!("Day2 Part1: {:?}", day2::part_1(get_input(2)));
    println!("Day2 Part2: {:?}", day2::part_2(get_input(2)));
    println!("Day3 Part1: {:?}", day3::part_1(get_input(3)));
    println!("Day3 Part2: {:?}", day3::part_2(get_input(3)));
    println!("Day4 Part1: {:?}", day4::part_1(get_input(4)));
    println!("Day4 Part2: {:?}", day4::part_2(get_input(4)));
    println!("Day5 Part1: {:?}", day5::part_1(get_input(5)));
    println!("Day5 Part2: {:?}", day5::part_2(get_input(5)));
    println!("Day6 Part1: {:?}", day6::part_1(get_input(6)));
    println!("Day6 Part2: {:?}", day6::part_2(get_input(6)));
    println!("Day7 Part1: {:?}", day7::part_1(get_input(7)));
    println!("Day7 Part2: {:?}", day7::part_2(get_input(7)));
    println!("Day8 Part1: {:?}", day8::part_1(get_input(8)));
    println!("Day8 Part2: {:?}", day8::part_2(get_input(8)));
    println!("Day9 Part1: {:?}", day9::part_1(get_input(9)));
    println!("Day9 Part2: {:?}", day9::part_2(get_input(9)));
    println!("Day10 Part1: {:?}", day10::part_1(get_input(10)));
    println!("Day10 Part2: {:?}", day10::part_2(get_input(10)));
}
