use std::io::{self, BufReader, Read, BufRead};

//... yeah, there goes the unassigned integer assumption!!! 
fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let input = buffer.lines().next().unwrap(); // Reads the first and only line... let's break it!

    let mut value_vec: Vec<usize> = input?.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    // Pre_computation as stated on the problem statement
    value_vec[1] = 12;
    value_vec[2] = 2;

    let mut index: usize = 0;
    loop {
        match value_vec[index] {
            1 => {
                let final_pos = value_vec[index + 3];
                let pos1 = value_vec[index + 1];
                let pos2 = value_vec[index + 2];
                value_vec[final_pos] = value_vec[pos1] + value_vec[pos2] 
            },
            2 => {
                let final_pos = value_vec[index + 3];
                let pos1 = value_vec[index + 1];
                let pos2 = value_vec[index + 2];
                value_vec[final_pos] = value_vec[pos1] * value_vec[pos2] 
            },
            _ => break,
        }

        index += 4;
    }

    println!("{}", value_vec[0]);

    Ok(())
}
