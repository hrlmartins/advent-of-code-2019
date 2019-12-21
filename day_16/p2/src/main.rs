use std::io;
use std::io::{Read, BufReader, BufRead};
use std::iter::FromIterator;

fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let input = buffer.lines().next().unwrap(); // Reads the first and only line... let's break it!

    let mut value_vec: Vec<i128> = input?.chars().map(|x| x.to_digit(10).unwrap() as i128).collect();

    // Produce 10_000 fold input
    let increased_input: Vec<i128> = value_vec.iter().cycle().take(value_vec.len() * 10_000).map(|x| *x).collect();

    // Extract the offset (first 7 digits)
    // This actually means that we won't need to calculate the positions up to the offset. This includes the pattern
    // We know that at the ith position of the sequence we will have i 0s followed by i 1s and so on and so forth on the base pattern
    // From pre processing I know my input is 6_500_000 long and the offset is 5_976_733.
    // Results are 8 digits starting from index 5_976_734 (so... we will have 5_976_733 0s to start the sequence followed by 5_976_734 1s)
    //... lol doesn't this actually mean that the processing is just summing values after the offset? xD

    let mut relevant_input = Vec::new();
    let offset = to_number(&increased_input, 7);
    println!("offset {:?}", offset);
    increased_input[offset as usize..].iter().for_each(|elem| relevant_input.push(*elem));

    println!("{:?}", relevant_input.len());

    // well I guess just summing is not efficient enough (and after a couple of Hours).... looking at more patterns you can see that the last position
    // is always itself. The previous position to that is itself plus the result at the next position...
    // So if we start summing from the end we can use already calculated values....
    for phase in 0..100 {
        for idx in (1..relevant_input.len()).rev() {
            relevant_input[idx - 1] = (relevant_input[idx - 1] + relevant_input[idx]) % 10;
        }
    }

    println!("Final numba {}", to_number(&relevant_input, 8));

    Ok(())
}

fn to_number(list: &Vec<i128>, digits: i128) -> i128 {
    list.iter().take(digits as usize).fold(0, |acc, val| val + acc * 10)
}