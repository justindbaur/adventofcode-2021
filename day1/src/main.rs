use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "./input.txt";
    let numbers = read_lines(filename);
    println!("{}", part1(&numbers));
    println!("{}", part2(&numbers));
}

fn part1(numbers: &Vec<i32>) -> i32 {
    let mut increase_count = 0;
    let mut prev_value = numbers[0];
    for i in 1..numbers.len() {
        let current_value = numbers[i];
        if current_value > prev_value {
            increase_count += 1;
        }
        prev_value = current_value;
    }
    increase_count
}

fn part2(numbers: &Vec<i32>) -> i32 {
    let mut increase_count = 0;
    let mut prev_value = numbers[0] + numbers[1] + numbers[2];
    for i in 3..numbers.len() {
        let current_value = numbers[i] + numbers[i - 1] + numbers[i - 2];
        if current_value > prev_value {
            increase_count += 1;
        }
        prev_value = current_value;
    }
    increase_count
}

fn read_lines(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename)
        .expect("Failed to open file");

    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| l.parse::<i32>().expect("Could not parse line"))
        .collect()
}
