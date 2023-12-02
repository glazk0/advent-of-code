use std::{fs, time};

use regex::Regex;

fn benchmark(func: fn()) {
    let start = time::Instant::now();
    func();
    let end = time::Instant::now();
    println!("Time: {}ms", (end - start).as_millis());
}

fn part1() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut total = 0;

    let red = Regex::new(r"(\d+) red").unwrap();
    let green = Regex::new(r"(\d+) green").unwrap();
    let blue = Regex::new(r"(\d+) blue").unwrap();

    let id = Regex::new(r"Game (\d+)").unwrap();

    for line in input.lines() {
        let red_count = red
            .find_iter(line)
            .map(|color| {
                color
                    .as_str()
                    .split(" ")
                    .filter_map(|num| num.parse::<i32>().ok())
                    .nth(0)
                    .unwrap()
            })
            .max()
            .unwrap();

        let green_count = green
            .find_iter(line)
            .map(|color| {
                color
                    .as_str()
                    .split(" ")
                    .filter_map(|num| num.parse::<i32>().ok())
                    .nth(0)
                    .unwrap()
            })
            .max()
            .unwrap();

        let blue_count = blue
            .find_iter(line)
            .map(|color| {
                color
                    .as_str()
                    .split(" ")
                    .filter_map(|num| num.parse::<i32>().ok())
                    .nth(0)
                    .unwrap()
            })
            .max()
            .unwrap();

        let id = id
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i16>()
            .unwrap();

        if red_count <= 12 && green_count <= 13 && blue_count <= 14 {
            total += id;
        }
    }

    dbg!(total);
}

fn part2() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut total = 0;

    let red = Regex::new(r"(\d+) red").unwrap();
    let green = Regex::new(r"(\d+) green").unwrap();
    let blue = Regex::new(r"(\d+) blue").unwrap();

    for line in input.lines() {
        let red_count = red
            .find_iter(line)
            .map(|color| {
                color
                    .as_str()
                    .split(" ")
                    .filter_map(|num| num.parse::<i32>().ok())
                    .next()
                    .unwrap()
            })
            .max()
            .unwrap();

        let green_count = green
            .find_iter(line)
            .map(|color| {
                color
                    .as_str()
                    .split(" ")
                    .filter_map(|num| num.parse::<i32>().ok())
                    .next()
                    .unwrap()
            })
            .max()
            .unwrap();

        let blue_count = blue
            .find_iter(line)
            .map(|color| {
                color
                    .as_str()
                    .split(" ")
                    .filter_map(|num| num.parse::<i32>().ok())
                    .next()
                    .unwrap()
            })
            .max()
            .unwrap();

        total += red_count * green_count * blue_count;
    }

    dbg!(total);
}

fn main() {
    benchmark(part1);
    benchmark(part2);
}
