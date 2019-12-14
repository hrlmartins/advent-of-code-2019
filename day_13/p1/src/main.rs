use std::io::{self, BufReader, Read, BufRead};
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;
use crate::BoardPieces::{Empty, Block, Wall, Paddle, Ball};

const LIMIT: usize = 38;

#[derive(PartialEq, Debug)]
enum OpCode {
    Add = 1,
    Multiply,
    ReadInput,
    PrintAddress,
    JIfTrue,
    JIfFalse,
    Lt,
    Eq,
    SetRelOffset,
    Halt = 99
}

#[derive(PartialEq, Debug)]
enum ParamModes {
    PositionMode = 0,
    ImmediateMode,
    RelativeMode
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum BoardPieces {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

struct Instruction {
    op_code: OpCode,
    param_modes: Vec<ParamModes>
}

struct BoardGame {
    board: [[BoardPieces; LIMIT]; LIMIT],
    output_count: i32,
    x_param: usize,
    y_param: usize
}


struct Computer {
    memory: HashMap<i128, i128>,
    relative_base: i128,
    instruction_pointer: i128,
    input: VecDeque<i128>,
}

impl BoardGame {
    fn new() -> BoardGame {
        BoardGame {
            board: [[BoardPieces::Empty; LIMIT]; LIMIT],
            output_count: 0,
            x_param: 0,
            y_param: 0
        }
    }

    fn receive_output(&mut self, value: i128) {
        match self.output_count % 3 {
            0 => {
                // x coordinate
                self.x_param = value as usize;
            }

            1 => {
                // y coordinate
                self.y_param = value as usize;
            }

            2 => {
                // board piece
                self.board[self.x_param][self.y_param] = match value {
                    0 => Empty,
                    1 => Wall,
                    2 => Block,
                    3 => Paddle,
                    4 => Ball,
                    _ => panic!("Well... there is no piece with this value...")
                };
            }
            _ => panic!("This is impossible?")
        }

        self.output_count += 1;
    }

    fn count_piece(&self, piece: BoardPieces) -> i32 {
        let mut sum = 0;
        for x in 0..LIMIT {
            for y in 0..LIMIT {
                if self.board[y][x] == piece {
                    sum += 1;
                }
            }
        }

        sum
    }

    fn print(&self) {
        for x in 0..LIMIT {
            for y in 0..LIMIT {
                match self.board[y][x] {
                    BoardPieces::Empty => print!(" "),
                    BoardPieces::Wall => print!("#"),
                    BoardPieces::Block => print!("@"),
                    BoardPieces::Paddle => print!("_"),
                    BoardPieces::Ball => print!("O"),
                }
            }
            println!();
        }
    }
}

impl Computer {
    fn new(init_memory: Vec<i128>) -> Computer {
        let mut computer_memory = HashMap::new();
        for (idx, value) in init_memory.iter().enumerate() {
            computer_memory.insert(idx as i128, *value);
        }

        Computer {
            memory: computer_memory,
            relative_base: 0,
            instruction_pointer: 0,
            input: VecDeque::new()
        }
    }

    fn run(&mut self) {
        let mut board = BoardGame::new();

        loop {
            let next_code = self.read_from_pos(self.instruction_pointer);
            let instruction = self.get_instruction(next_code);

            match instruction.op_code {
                OpCode::Add | OpCode::Multiply => {
                    let op0 = self.read_mem(self.instruction_pointer + 1, &instruction.param_modes[0]);
                    let op1 = self.read_mem(self.instruction_pointer + 2, &instruction.param_modes[1]);

                    let result = match instruction.op_code {
                        OpCode::Add => op0 + op1,
                        OpCode::Multiply => op0 * op1,
                        _ => unreachable!(),
                    };

                    self.store_mem(self.instruction_pointer + 3, result, &instruction.param_modes[2]);
                    self.instruction_pointer += 4;
                },
                OpCode::ReadInput => {
                    // will read single input
                    let input = self.input.pop_front().unwrap();
                    self.store_mem(self.instruction_pointer + 1, input, &instruction.param_modes[0]);
                    self.instruction_pointer += 2;
                },
                OpCode::PrintAddress => {
                    let board_output = self.read_mem(self.instruction_pointer + 1, &instruction.param_modes[0]);
                    board.receive_output(board_output);
                    self.instruction_pointer += 2;
                },
                OpCode::JIfTrue => {
                    if self.read_mem(self.instruction_pointer + 1, &instruction.param_modes[0]) != 0 {
                        self.instruction_pointer = self.read_mem(self.instruction_pointer + 2, &instruction.param_modes[1]);
                    } else {
                        self.instruction_pointer += 3;
                    }
                },
                OpCode::JIfFalse => {
                    if self.read_mem(self.instruction_pointer + 1, &instruction.param_modes[0]) == 0 {
                        self.instruction_pointer = self.read_mem(self.instruction_pointer + 2, &instruction.param_modes[1]);
                    } else {
                        self.instruction_pointer += 3;
                    }
                },
                OpCode::Lt => {
                    let op0 = self.read_mem(self.instruction_pointer + 1, &instruction.param_modes[0]);
                    let op1 = self.read_mem(self.instruction_pointer + 2, &instruction.param_modes[1]);

                    if op0 < op1 {
                        self.store_mem(self.instruction_pointer + 3, 1, &instruction.param_modes[2]);
                    } else {
                        self.store_mem(self.instruction_pointer + 3, 0, &instruction.param_modes[2]);
                    }

                    self.instruction_pointer += 4
                }
                OpCode::Eq => {
                    let op0 = self.read_mem(self.instruction_pointer + 1, &instruction.param_modes[0]);
                    let op1 = self.read_mem(self.instruction_pointer + 2, &instruction.param_modes[1]);

                    if op0 == op1 {
                        self.store_mem(self.instruction_pointer + 3, 1, &instruction.param_modes[2]);
                    } else {
                        self.store_mem(self.instruction_pointer + 3, 0, &instruction.param_modes[2]);
                    }

                    self.instruction_pointer += 4
                },
                OpCode::SetRelOffset => {
                    let val = self.read_mem(self.instruction_pointer + 1, &instruction.param_modes[0]);
                    self.relative_base += val;

                    self.instruction_pointer += 2;
                },
                OpCode::Halt => {
                    board.print();
                    println!("Block Count: {}", board.count_piece(BoardPieces::Block));
                    break;
                }
            }
        }
    }

    fn read_mem(&mut self, pos: i128, param_mode: &ParamModes) -> i128 {
        match *param_mode {
            ParamModes::ImmediateMode => self.read_from_pos(pos),
            ParamModes::PositionMode => {
                let idx_value = self.read_from_pos(pos);
                self.read_from_pos(idx_value)
            },
            ParamModes::RelativeMode => {
                let idx_value = self.read_from_pos(pos);
                //println!("Relative idx_value: {} - final pos {}:", idx_value, self.relative_base);
                self.read_from_pos(idx_value + self.relative_base)
            }
        }
    }

    fn store_mem(&mut self, pos: i128, value: i128, param_mode: &ParamModes) {
        match *param_mode {
            ParamModes::ImmediateMode => self.store_in_pos(pos, value),
            ParamModes::PositionMode => {
                let idx_value = self.read_from_pos(pos);
                self.store_in_pos(idx_value, value)
            },
            ParamModes::RelativeMode => {
                let idx_value = self.read_from_pos(pos);
                self.store_in_pos(idx_value + self.relative_base, value)
            }
        };
    }

    fn read_from_pos(&mut self, pos: i128) -> i128 {
        if pos < 0 {
            panic!("Trying to read from negative position");
        }

        if !self.memory.contains_key(&pos) {
            // fill it with zero
            self.memory.insert(pos, 0);
        }

        *self.memory.get(&pos).unwrap()
    }

    fn store_in_pos(&mut self, pos: i128, value: i128) {
        if pos < 0 {
            panic!("Trying to write to negative position");
        }

        self.memory.insert(pos, value);
    }

    fn get_instruction(&self, code: i128) -> Instruction {
        let op_code = self.get_op_code(code % 100);
        let mut mode_codes = code / 100;

        let param_count = get_number_parameters(&op_code);

        // Read the parameters modes
        let mut param_modes: Vec<ParamModes> = Vec::new();
        for _i in 1..=param_count {
            if mode_codes == 0 {
                param_modes.push(ParamModes::PositionMode);
            } else {
                let mode = self.get_param_mode(mode_codes % 10);
                param_modes.push(mode);
                mode_codes = mode_codes / 10; // read the next parameter mode
            }
        }

        Instruction {
            op_code,
            param_modes
        }
    }

    fn get_op_code(&self, value: i128) -> OpCode {
        match value {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::ReadInput,
            4 => OpCode::PrintAddress,
            5 => OpCode::JIfTrue,
            6 => OpCode::JIfFalse,
            7 => OpCode::Lt,
            8 => OpCode::Eq,
            9 => OpCode::SetRelOffset,
            _ => OpCode::Halt
        }
    }

    fn get_param_mode(&self, value: i128) -> ParamModes {
        match value {
            0 => ParamModes::PositionMode,
            1 => ParamModes::ImmediateMode,
            _ => ParamModes::RelativeMode,
        }
    }

    fn push_input(&mut self, input: i128) {
        self.input.push_back(input);
    }
}

fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let input = buffer.lines().next().unwrap(); // Reads the first and only line... let's break it!

    let value_vec: Vec<i128> = input?.split(",").map(|x| x.parse::<i128>().unwrap()).collect();

    let mut computer = Computer::new(value_vec.clone());
    computer.run();

    Ok(())
}

fn get_number_parameters(opcode: &OpCode) -> i32 {
    match opcode {
        OpCode::Add => 3,
        OpCode::Multiply => 3,
        OpCode::ReadInput => 1,
        OpCode::PrintAddress => 1,
        OpCode::JIfTrue => 2,
        OpCode::JIfFalse => 2,
        OpCode::Lt => 3,
        OpCode::Eq => 3,
        OpCode::SetRelOffset => 1,
        OpCode::Halt => 0
    }
}
