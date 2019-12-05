use std::io::{self, BufReader, Read, BufRead};

enum OpCode {
    Add = 1,
    Multiply,
    ReadInput,
    PrintAddress,
    Halt = 99
}

enum ParamModes {
    PositionMode,
    ImmediateMode
}

struct Instruction {
    op_code: OpCode,
    size: i32,
    param_modes: Vec<ParamModes>
}

fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let input = buffer.lines().next().unwrap(); // Reads the first and only line... let's break it!

    let mut value_vec: Vec<i32> = input?.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    // Lets "Read" the input and set the value... which is always 1 lol
    let first_pointer = value_vec[1];
    value_vec[first_pointer as usize] = 1;
    // Done! LOL!

    let mut index: usize = 2;

    loop {
        let instr = get_instruction_from_code(value_vec[index]);

        if !perform_instruction(&instr, &mut value_vec, index as i32) {
            break;
        }

        index += instr.size as usize;
    }

    Ok(())
}

fn perform_instruction(inst: &Instruction, memory: &mut Vec<i32>, instruction_pointer: i32) -> bool {
    match inst.op_code {
        OpCode::Add => perform_sum(memory, &inst.param_modes, instruction_pointer),
        OpCode::Multiply => perform_multiplication(memory, &inst.param_modes, instruction_pointer),
        OpCode::ReadInput => perform_read_input(memory, &inst.param_modes, instruction_pointer),
        OpCode::PrintAddress => perform_print_addr(memory, &inst.param_modes, instruction_pointer),
        OpCode::Halt => false
    }
}

fn perform_sum(memory: &mut Vec<i32>, param_modes: &Vec<ParamModes>, instruction_pointer: i32) -> bool {
    //get operands and calculate result

    let mut sum = 0;
    for idx in 1..=2 {
        sum = sum + get_value(memory, instruction_pointer + idx, &param_modes[idx as usize - 1])
    }

    // store it in third argument position
    store_value(memory, instruction_pointer + 3, sum, &param_modes[2]);
    true
}

fn perform_multiplication(memory: &mut Vec<i32>, param_modes: &Vec<ParamModes>, instruction_pointer: i32) -> bool {
    //get operands and calculate result

    let mut acc = 1;
    for idx in 1..=2 {
        acc = acc * get_value(memory, instruction_pointer + idx, &param_modes[idx as usize - 1])
    }

    // store it in third argument position
    store_value(memory, instruction_pointer + 3, acc, &param_modes[2]);
    true
}

fn perform_read_input(memory: &mut Vec<i32>, param_modes: &Vec<ParamModes>, instruction_pointer: i32) -> bool {
    print!("provide input!!! ");
    let buffer = BufReader::new(io::stdin());
    let input_value = buffer.lines().next().unwrap().unwrap();
    let addr = input_value.parse::<i32>().unwrap();

    store_value(memory, instruction_pointer + 1, addr, &param_modes[0]);
    true
}

fn perform_print_addr(memory: &mut Vec<i32>, param_modes: &Vec<ParamModes>, instruction_pointer: i32) -> bool {
    let print_value = get_value(memory, instruction_pointer + 1, &param_modes[0]);
    println!("Print Operation value: {}", print_value);
    true
}


fn get_value(memory: &mut Vec<i32>, val: i32, param_mode: &ParamModes) -> i32 {
    match param_mode {
        ParamModes::PositionMode => {
            let pos = memory[val as usize];
            memory[pos as usize]
        },
        _ => memory[val as usize]
    }
}

fn store_value(memory: &mut Vec<i32>, argument: i32, result: i32, param_mode: &ParamModes) {
    match param_mode {
        ParamModes::PositionMode => {
            let pos = memory[argument as usize];
            memory[pos as usize] = result;
        },
        _ => memory[argument as usize] = result
    }
}


fn get_instruction_from_code(code: i32) -> Instruction {
    let op_code = get_op_code(code % 100);
    let mut mode_codes = code / 100;

    let iter_count = get_number_parameters(&op_code);

    // Read the parameters modes
    let mut param_modes: Vec<ParamModes> = Vec::new();
    for _i in 1..=iter_count {
        if mode_codes == 0 {
            param_modes.push(ParamModes::PositionMode);
        } else {
            let mode = get_param_mode(mode_codes % 10);
            param_modes.push(mode);
            mode_codes = mode_codes / 10; // read the next parameter mode
        }
    }

    let size = get_increment_per_operation(&op_code);
    Instruction {
        op_code,
        size,
        param_modes
    }
}

fn get_op_code(value: i32) -> OpCode {
    match value {
        1 => OpCode::Add,
        2 => OpCode::Multiply,
        3 => OpCode::ReadInput,
        4 => OpCode::PrintAddress,
        _ => OpCode::Halt
    }
}

fn get_param_mode(value: i32) -> ParamModes {
    match value {
        0 => ParamModes::PositionMode,
        _ => ParamModes::ImmediateMode
    }
}

fn get_increment_per_operation(opcode: &OpCode) -> i32 {
    match opcode {
        OpCode::Add => 4,
        OpCode::Multiply => 4,
        OpCode::ReadInput => 2,
        OpCode::PrintAddress => 2,
        OpCode::Halt => 1
    }
}

fn get_number_parameters(opcode: &OpCode) -> i32 {
    match opcode {
        OpCode::Add => 3,
        OpCode::Multiply => 3,
        OpCode::ReadInput => 1,
        OpCode::PrintAddress => 1,
        OpCode::Halt => 0
    }
}
