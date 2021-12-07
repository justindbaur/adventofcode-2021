use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let lines = read_lines("input1.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut totals = HashMap::<usize, i32>::new();

    for line in lines {
        for (i, c) in line.char_indices() {
            println!("Char: {}", c);
            match c {
                '0' => {
                    totals.entry(i)
                        .and_modify(|e| *e -= 1)
                        .or_insert(-1);
                },
                '1' => {
                    totals.entry(i)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                },
                _ => panic!("Invalid character: {}", c),
            }
        }
        println!("{}", line);
        for (_, v) in totals.iter() {
            print!("{} ", v);
        }
        println!();
    }

    let mut gamma_bin = String::new();
    let mut epsilon_bin = String::new();

    for (_, v) in totals.iter() {
        // println!("{}", v);
        if v > &0 {
            gamma_bin.push('1');
            epsilon_bin.push('0');
        } else {
            gamma_bin.push('0');
            epsilon_bin.push('1');
        }
    }

    println!("Gamma Binary: {}", gamma_bin);
    println!("Epsilon Binary: {}", epsilon_bin);

    let gamma = i32::from_str_radix(&gamma_bin, 2).unwrap();
    println!("Gamma: {}", gamma); 
    let epsilon = i32::from_str_radix(&epsilon_bin, 2).unwrap();
    println!("Epsilon: {}", epsilon);

    gamma * epsilon
}

fn part2(_lines: &Vec<String>) -> i32 {
    1
}

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let reader = io::BufReader::new(file);
    reader.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
