use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut horizontal_pos = 0;
    let mut depth = 0;

    for line in lines {
        if let Some((movement, str_value)) = line.split_once(" ") {
            let value = str_value.parse::<i32>()
                .expect("Failed to parse value");

            match movement {
                "forward" => horizontal_pos += value,
                "down" => depth += value,
                "up" => depth -= value,
                _ => panic!("Unknown movement: {}", movement),
            }
        }
    }
    horizontal_pos * depth
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines {
        if let Some((movement, str_value)) = line.split_once(" ") {
            let value = str_value.parse::<i32>()
                .expect("Failed to parse value");

            match movement {
                "forward" => {
                    horizontal_pos += value;
                    depth += aim * value;
                },
                "down" => {
                    aim += value;
                },
                "up" => {
                    aim -= value
                },
                _ => panic!("Unknown movement: {}", movement),
            }
        }
    }

    horizontal_pos * depth
}

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let reader = io::BufReader::new(file);
    reader.lines().map(|l| l.expect("Could not parse line"))
        .collect()
}
