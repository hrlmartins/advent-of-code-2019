use std::io;
use std::io::{Read, BufReader, BufRead};
use std::iter::FromIterator;

fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let input = buffer.lines().next().unwrap(); // Reads the first and only line... let's break it!

    let mut value_vec: Vec<i32> = input?.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();

    // 100 phaes
    for phase in 1..=100 {
        value_vec = compute_phase(&value_vec, value_vec.len() as i32, vec![0, 1, 0, -1]);
    }

    println!("{:?}", value_vec);

    let mut multiplier = 1;
    let mut res = 0;
    for pos in (0..8).rev() {
        res += value_vec[pos] * multiplier;
        multiplier *= 10;
    }

    println!("first 8 digits: {}", res);

    Ok(())
}

fn compute_phase(input: &Vec<i32>, size_sequence: i32, base_pattern: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    for sequence in 1..=size_sequence { // number of times to generate the next index
        let mut acc = 0;
        let mut pattern =  build_pattern(&base_pattern, sequence as i32);

        let mut pattern_idx = 1;
        for (idx, value) in input.iter().enumerate() {
            acc += value * pattern[pattern_idx];
            pattern_idx = (pattern_idx + 1) % pattern.len();
        }

        //println!("pattern: {:?}", pattern);
        //println!(" acc {}  res {} sequence: {}", acc, acc.abs() % 10, sequence);
        result.push(acc.abs() % 10)
    }

    result
}

fn build_pattern(pattern: &Vec<i32>, sequence: i32) -> Vec<i32> {
    let mut result = Vec::new();
    for (idx, val) in pattern.iter().enumerate() {
        for _rep in 1..=sequence {
            result.push(*val);
        }
    }

    result
}