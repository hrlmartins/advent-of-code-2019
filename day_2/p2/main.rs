use std::io::{self, BufRead, BufReader, Read};

//... yeah, there goes the unassigned integer assumption!!!
fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let input = buffer.lines().next().unwrap(); // Reads the first and only line... let's break it!

    let real_value_vec: Vec<usize> = input?
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut value_vec: Vec<usize> = real_value_vec.clone();
    // Pre_computation as stated on the problem statement...but now BRUTE FORCE IT BABY LOOOL
    'outer: for noun in 0..=99 {
        for verb in 0..=99 {
            value_vec[1] = noun;
            value_vec[2] = verb;

            let mut index: usize = 0;
            loop {
                match value_vec[index] {
                    1 => {
                        // TODO MOVE THESE OUT TO COMMON PART
                        let final_pos = value_vec[index + 3];
                        let pos1 = value_vec[index + 1];
                        let pos2 = value_vec[index + 2];
                        value_vec[final_pos] = value_vec[pos1] + value_vec[pos2]
                    }
                    2 => {
                        let final_pos = value_vec[index + 3];
                        let pos1 = value_vec[index + 1];
                        let pos2 = value_vec[index + 2];
                        value_vec[final_pos] = value_vec[pos1] * value_vec[pos2]
                    }
                    _ => break,
                }

                index += 4;
            }

            if value_vec[0] == 19690720 {
                println!("Noun: {}, Verb: {}", noun, verb);
                println!("final Result: {}", 100 * noun + verb);
                break 'outer;
            } else {
                value_vec = real_value_vec.clone();
            }
        }
    }

    Ok(())
}
