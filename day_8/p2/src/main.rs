use std::io::{self, BufRead, BufReader, Read};

struct Layer {
    row: usize,
    column: usize,
    grid: [[char; 25]; 6] // [['2'; 6]; 25];
}

impl Layer {
    fn new() -> Layer {
        Layer {
            row: 0,
            column: 0,
            grid: [['2'; 25]; 6]
        }
    }

    fn push_color(&mut self, pixel: char) {
        self.grid[self.row][self.column] = pixel;

        self.column += 1;
        if self.column == 25 {
            self.row += 1;
            self.column = 0;
        }
    }

    fn get_color_in(&self, row: usize, col: usize) -> char {
        self.grid[row][col]
    }

    fn print_layer(&self) {
        for r in 0..6 {
            for c in 0..25 {
                print!("{}", self.grid[r][c])
            }
            println!();
        }
    }
}

fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let input = buffer.lines().next().unwrap(); // Reads the first and only line... let's break it!

    let string_line = input.unwrap();
    let layer_count = 25 * 6;

    let mut layers = Vec::new();

    let mut count = 0;
    let mut curr_layer = Layer::new();
    for character in string_line.chars() {
        curr_layer.push_color(character);
        count += 1;

        if count % layer_count == 0 {
            layers.push(curr_layer);
            curr_layer = Layer::new();
        }
    }

    let mut final_layer = Layer::new();

    for row in 0..6 {
        for col in 0..25 {
            for layer in &layers {
                let color = layer.get_color_in(row, col);

                // remember... if your eyes are not trained as computers to read binary... just print something that stands out xD
                if color == '1' {
                    final_layer.push_color('#');
                    break;
                }

                if color == '0' {
                    final_layer.push_color(' ');
                    break;
                }
            }
        }
    }

    final_layer.print_layer();
    Ok(())
}
