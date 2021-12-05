use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use aoc2021::*;

fn main() {
    println!("Day1 Part1: {:?}", day1::part_1(get_input(1)));
    println!("Day1 Part2: {:?}", day1::part_2(get_input(1)));
    println!("Day2 Part1: {:?}", day2::part_1(get_input(2)));
    println!("Day2 Part2: {:?}", day2::part_2(get_input(2)));
    println!("Day3 Part1: {:?}", day3::part_1(get_input(3)));
    println!("Day3 Part2: {:?}", day3::part_2(get_input(3)));
    println!("Day4 Part1: {:?}", day4::part_1(get_input(4)));
    println!("Day4 Part2: {:?}", day4::part_2(get_input(4)));
}

fn get_input(day: i32) -> io::BufReader<File> {
    let p = format!("input/day{}.txt", day);
    let path = Path::new(&p);

    let file: File = match File::open(&path) {
        Err(_) => download_input(day).unwrap(),
        Ok(f) => f,
    };

    io::BufReader::new(file)
}

fn download_input(day: i32) -> Result<File, reqwest::Error> {
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
            Ok(f)
        }
        Err(error) => Err(error),
    }
}
