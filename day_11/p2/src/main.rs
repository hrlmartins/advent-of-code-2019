use std::io::{self, BufReader, Read, BufRead};
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

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

struct Instruction {
    op_code: OpCode,
    param_modes: Vec<ParamModes>
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

enum OutputState {
    Paint,
    Move
}

struct Robot {
    position: (i128, i128),
    painted: HashMap<(i128, i128), i128>, // coordinate will be painted black or white. No coordinate means black,
    direction: Direction,
    output_state: OutputState
}


impl Robot {
    fn new() -> Robot {
        Robot {
            position: (20, 20),
            painted: HashMap::new(),
            direction: Direction::UP,
            output_state: OutputState::Paint
        }
    }

    fn turn_bot_and_move(&mut self, turn: i128) {
        if turn == 0 { // turn 90 degrees left
            match self.direction {
                Direction::UP => {
                    self.direction = Direction::LEFT
                },
                Direction::DOWN => {
                    self.direction = Direction::RIGHT
                },
                Direction::LEFT => {
                    self.direction = Direction::DOWN
                },
                Direction::RIGHT => {
                    self.direction = Direction::UP
                },
            }
        } else {
            // its turn right 90
            match self.direction {
                Direction::UP => {
                    self.direction = Direction::RIGHT
                },
                Direction::DOWN => {
                    self.direction = Direction::LEFT
                },
                Direction::LEFT => {
                    self.direction = Direction::UP
                },
                Direction::RIGHT => {
                    self.direction = Direction::DOWN
                },
            }
        }

        self.move_bot();
        // back to painting state
        self.output_state = OutputState::Paint;
    }

    fn move_bot(&mut self) {
        let (curr_x, curr_y) = self.position;
        match self.direction {
            Direction::UP => {
                self.position = (curr_x, curr_y + 1)
            },
            Direction::DOWN => {
                self.position = (curr_x, curr_y - 1)
            },
            Direction::LEFT => {
                self.position = (curr_x - 1, curr_y)
            },
            Direction::RIGHT => {
                self.position = (curr_x + 1, curr_y)
            },
        }
    }

    fn get_input(&self) -> i128 {
        if self.painted.contains_key(&self.position) {
            *self.painted.get(&self.position).unwrap()
        } else {
            0_i128
        }
    }

    fn paint_position(&mut self, paint_value: i128) {
        self.painted.insert((self.position.0, self.position.1), paint_value);
        // after painting back to move output state
        self.output_state = OutputState::Move;
    }

    fn count_painted(&self) -> usize {
        self.painted.len()
    }
}

struct Computer {
    memory: HashMap<i128, i128>,
    relative_base: i128,
    instruction_pointer: i128,
    input: VecDeque<i128>,
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
        let mut robot = Robot::new();

        loop {
            //println!("{:?}", self.memory);
            let next_code = self.read_from_pos(self.instruction_pointer);
            let instruction = self.get_instruction(next_code);
            //println!("Processing OP: {:?}",  instruction.op_code);
            // println!("Param modes {:?}", instruction.param_modes);

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
                    let output = self.read_mem(self.instruction_pointer + 1, &instruction.param_modes[0]);
                    //println!("Output: {}", output);

                    match robot.output_state {
                        OutputState::Paint => {
                            robot.paint_position(output);
                        },
                        OutputState::Move => {
                            robot.turn_bot_and_move(output);

                            // read new input value and input
                            self.input.push_back(robot.get_input());
                        },
                    }

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
                    create_board_and_print(&robot.painted);
                    //println!("{:?}", robot.painted);
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
    computer.push_input(1); // push 1 as first input white at position 0,0
    computer.run();

    Ok(())
}

fn create_board_and_print(painted: &HashMap<(i128, i128), i128>) -> [[i128; 100]; 100] {
    let mut board = [[0_i128; 100]; 100];

    painted.iter().for_each(|(&(x, y), &v)| {
        board[x as usize][y as usize] = v;
    });

    for x in 0..100 {
        for y in 0..100 {
            if board[x][y] != 0 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    board
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
