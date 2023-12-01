use std::{collections::HashMap, fs, time};

fn benchmark(func: fn()) {
    let start = time::Instant::now();
    func();
    let end = time::Instant::now();
    println!("Time: {}ms", (end - start).as_millis());
}

fn part1() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut total = 0;

    for line in input.lines() {
        let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        total += digits.get(0).unwrap() * 10u32 + digits.get(digits.len() - 1).unwrap();
    }

    dbg!(total);
}

fn part2() {
    let input = fs::read_to_string("input.txt").unwrap();

    let numbers: HashMap<&str, char> = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("zero", '0'),
    ]
    .iter()
    .cloned()
    .collect();

    let mut total = 0;

    for line in input.lines() {
        let mut line = line.to_owned();
        let mut digits = Vec::new();

        while line.len() > 0 {
            if line.chars().nth(0).unwrap().is_digit(10) {
                digits.push(line.chars().nth(0).unwrap());
            } else {
                for number in numbers.keys() {
                    if line.starts_with(number) {
                        digits.push(numbers.get(number).unwrap().clone());
                        break;
                    }
                }
            }
            line.remove(0);
        }
        total += digits.get(0).unwrap().to_digit(10).unwrap() * 10u32
            + digits.get(digits.len() - 1).unwrap().to_digit(10).unwrap();
    }

    dbg!(total);
}

fn main() {
    benchmark(part1);
    benchmark(part2);
}
