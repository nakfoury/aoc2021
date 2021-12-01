use std::{
    fs::File,
    io::{self, BufRead, Lines, Write},
    path::Path,
};

use aoc2021::*;

extern crate reqwest;

fn main() {
    println!("Day1 Part1: {:?}", day1::part_1(get_input(1)));
    println!("Day1 Part2: {:?}", day1::part_2(get_input(1)));
}

fn get_input(day: usize) -> io::BufReader<File> {
    let p = format!("input/day{}.txt", day);
    let path = Path::new(&p);

    let file: File = match File::open(&path) {
        Err(_) => download_input(day).unwrap(),
        Ok(f) => f,
    };

    io::BufReader::new(file)
}

fn download_input(day: usize) -> Result<File, reqwest::Error> {
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