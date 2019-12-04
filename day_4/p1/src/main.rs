use std::io::{self, BufRead, BufReader, Read};

//... yeah, there goes the unassigned integer assumption!!!
fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);

    // read full input
    let input = buffer.lines().next().unwrap().unwrap();
    let start_end_range: Vec<i32> = input
        .split("-")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let start_range = start_end_range[0];
    let end_range = start_end_range[1];

    println!("{:?} - {:?}", start_range, end_range);

    let combs = (start_range..=end_range)
        .filter(|val| check_valid_password(*val))
        .count();

    println!("Combinations: {}", combs);

    Ok(())
}
fn check_valid_password(value: i32) -> bool {
    let digit_vec: Vec<u32> = value
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut has_pair = false;
    for idx in 1..digit_vec.len() {
        if digit_vec[idx - 1] > digit_vec[idx] {
            return false;
        }

        if digit_vec[idx - 1] == digit_vec[idx] {
            has_pair = true;
        }
    }

    has_pair
}
