use std::io::{self, BufReader, Read, BufRead};
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;
use std::time::Duration;

const LIMIT: usize = 100;

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


struct Computer {
    memory: HashMap<i128, i128>,
    relative_base: i128,
    instruction_pointer: i128,
    input: VecDeque<i128>,
}

type Point = (i32, i32);
type Direction = i32;

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

    fn compute_position(point: &Point, dir: &Direction) -> Point {
        match dir {
            1 => (point.0, point.1 - 1),
            2 => (point.0, point.1 + 1),
            3 => (point.0 - 1, point.1),
            4 => (point.0 + 1, point.1),
            _ => panic!("Wat")
        }
    }

    fn next_direction(point: &Point, grid: &HashMap<Point, i32>) -> Option<Direction> {
        (1..=4)
            .filter(|dir| !grid.contains_key(&Computer::compute_position(point, dir)))
            .next()
    }

    fn run(&mut self) -> HashMap<Point, i32> {
        let mut path: Vec<Direction> = Vec::new();
        let mut curr_pos: Point = (0, 0);
        let mut oxygen_grid: HashMap<Point, i32> = HashMap::new(); // We'll build the whole grid and then compute. Easier this way.
        let mut last_dir: Direction = 1;

        oxygen_grid.insert(curr_pos.clone(), 1);

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
                    if let Some(dir) = Computer::next_direction(&curr_pos, &oxygen_grid) {
                        self.push_input(dir as i128);
                        last_dir = dir;
                        println!("Input provided {} at point {:?}", dir, curr_pos);
                    } else {
                        if path.is_empty() {
                            // Return, nothing else to do here. grid is filled
                            println!("DONE!");
                            break;
                        }

                        last_dir = path.pop().unwrap();
                        self.push_input( last_dir as i128);
                        println!("Input reverse provided {} at point {:?}", last_dir, curr_pos);
                    }


                    // will read single input
                    let input = self.input.pop_front().unwrap();
                    self.store_mem(self.instruction_pointer + 1, input, &instruction.param_modes[0]);
                    self.instruction_pointer += 2;

                    std::thread::sleep(Duration::from_millis(45));
                },
                OpCode::PrintAddress => {
                    let board_output = self.read_mem(self.instruction_pointer + 1, &instruction.param_modes[0]);
                    let new_position = Computer::compute_position(&curr_pos, &last_dir);
                    let visited = oxygen_grid.insert(new_position, board_output as i32).is_some(); // we just fill it with whatever the computer says

                    println!("Output for pos: {:?} is {}. New pos is {:?}", curr_pos, board_output, new_position);

                    if board_output > 0 {
                        curr_pos = new_position;
                        if !visited {
                            path.push(match last_dir {
                                    1 => 2,
                                    2 => 1,
                                    3 => 4,
                                    4 => 3,
                                    _ => panic!("wat2")
                                }
                            )
                        }
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
                    break;
                }
            }
        }

        return oxygen_grid;
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

    let mut value_vec: Vec<i128> = input?.split(",").map(|x| x.parse::<i128>().unwrap()).collect();

    let mut computer = Computer::new(value_vec.clone());
    let grid = computer.run();

    // fetch oxygen point
    let oxygen =
        *grid.iter()
            .filter(|(k, v)| **v == 2)
            .map(|(k, _v)| k).next().unwrap();

    let distances = compute_all_distance_from(&(0, 0), &grid);
    println!("Min Movements to oxygen: {}", distances.get(&oxygen).unwrap());

    Ok(())
}

fn compute_all_distance_from(origin: &Point, grid: &HashMap<Point, i32>) -> HashMap<Point, i32> {
    let mut distance: HashMap<Point, i32> = HashMap::new();
    distance.insert((origin.0, origin.1), 0);
    let mut adj = VecDeque::new();
    adj.push_back((origin.0, origin.1));

    while let Some(point) = adj.pop_front() {
        // for all possible neighbors of the point add one!
        let new_distance = *distance.get(&point).unwrap() + 1;
        let possible_visits: Vec<_> =
            (1..=4).
                map(|dir| Computer::compute_position(&point, &dir))
                .filter(|point| !distance.contains_key(point) && *grid.get(point).unwrap_or(&0) > 0)
                .collect();

        for next_point in possible_visits {
            distance.insert(next_point, new_distance);
            adj.push_back(next_point);
        }
    }

    distance
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
