use std::io::{self, BufReader, Read, BufRead};

fn main() {
   read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let mut accumulator: u32 = 0;
    for line in buffer.lines() {
        // Each line is an integer... no limits were specifiec assuming unassigned 32 bit
        let res = (line?.parse::<u32>().unwrap() / 3) - 2;
        accumulator = accumulator + res;
    }

    println!("{}", accumulator);

    Ok(())
}
