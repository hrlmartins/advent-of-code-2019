use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, BufReader, Read};

struct Layer {
    zero_count: u32,
    one_count: u32,
    two_count: u32,
}

impl Layer {
    fn min(self, other: Layer) -> Layer {
        if other.zero_count < self.zero_count {
            other
        } else {
            self
        }
    }

    fn inc_zeros(&mut self) {
        self.zero_count += 1
    }

    fn inc_ones(&mut self) {
        self.one_count += 1
    }

    fn inc_twos(&mut self) {
        self.two_count += 1
    }
}

fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let input = buffer.lines().next().unwrap(); // Reads the first and only line... let's break it!

    let string_line = input.unwrap();
    println!("{:?}", string_line);
    let layer_count = 25 * 6;
    let mut min_zero_layer = Layer {
        zero_count: u32::max_value(),
        one_count: 0,
        two_count: 0,
    };

    let mut count = 0;
    let mut curr_layer = Layer {
        zero_count: 0,
        one_count: 0,
        two_count: 0,
    };

    for character in string_line.chars() {
        match character {
            '0' => curr_layer.inc_zeros(),
            '1' => curr_layer.inc_ones(),
            '2' => curr_layer.inc_twos(),
            _ => (), //don't care,
        }

        count += 1;

        if count % layer_count == 0 {
            min_zero_layer = min_zero_layer.min(curr_layer);
            curr_layer = Layer {
                zero_count: 0,
                one_count: 0,
                two_count: 0,
            }
        }
    }

    println!(
        "Result: {}",
        min_zero_layer.one_count * min_zero_layer.two_count
    );

    Ok(())
}
