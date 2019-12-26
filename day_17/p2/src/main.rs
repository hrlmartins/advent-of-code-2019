use std::io::{self, BufReader, Read, BufRead};

mod intcode;

#[derive(PartialEq, Debug, Copy, Clone)]
struct Block {
    alignment_param: i32,
    display: char
}

impl Block {
    fn new(display: char) -> Block {
        Block {
            alignment_param: 0,
            display
        }
    }

    fn new_ali(display: char, alignment_param: i32) -> Block {
        Block {
            alignment_param,
            display
        }
    }
}

struct Board {
    board: [[Block; 61]; 61],
    row: i32,
    col: i32
}


impl Board {
    fn new() -> Board {
        Board {
            board: [[Block::new('d'); 61];  61],
            row: 0,
            col: 0
        }
    }

    fn set_char(&mut self, block: char) {
        self.board[self.row as usize][self.col as usize] = match block {
            '#' => Block::new_ali(block, self.col * self.row),
            _ => Block::new(block)
        };
        self.col += 1
    }

    fn new_row(&mut self) {
        self.row += 1;
        self.col = 0;
    }

    fn calc_align_sum(&self) -> i32 {
        let mut sum = 0;
        for y in 0..61 {
            for x in 0..61 {
                if self.board[y][x].display == '#' && self.is_intersection(y  as i32, x as i32) {
                    sum += self.board[y][x].alignment_param;
                }
            }
        }

        sum
    }

    fn is_intersection(&self, row: i32, col: i32) -> bool {
        let neighbors = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

        for n in neighbors {
            let new_row = row + n.0;
            let new_col = col + n.1;

            if new_row >= 0 && new_col >=0 && self.board[new_row as usize][new_col as usize].display != '#' {
                return false;
            }
        }

        true
    }

    fn print(&self) {
        for y in 0..61 {
            for x in 0..61 {
                print!("{}", self.board[y][x].display)
            }
            println!()
        }
    }
}

fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let input = buffer.lines().next().unwrap(); // Reads the first and only line... let's break it!

    let value_vec: Vec<i128> = input?.split(",").map(|x| x.parse::<i128>().unwrap()).collect();

    // The board is 61 by 61 matrix
    let mut computer = intcode::Computer::new(value_vec.clone());
    computer.run();

    let mut board = Board::new();

    while let Some(out) = computer.read_output() {
        let character = out as u8 as char;
        match character {
            '\n' => board.new_row(),
            _ => board.set_char(character)
        }
    }

    board.print();

    Ok(())
}
