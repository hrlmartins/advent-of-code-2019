use std::io::{self, BufReader, Read, BufRead};

//... yeah, there goes the unassigned integer assumption!!! 
fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let mut accumulator: i32 = 0;
    for line in buffer.lines() {
        let mass = line?.parse::<i32>().unwrap();
        accumulator = accumulator + compute_fuel(mass);
    }

    println!("{}", accumulator);

    Ok(())
}

fn compute_fuel(mass: i32) -> i32 {
    let magic_result = magic_formula(mass);
    if magic_result <= 0 {
        0
    } else {
        magic_result + compute_fuel(magic_result)
    }
}

fn magic_formula(value: i32) -> i32 {
    (value / 3) - 2
}
