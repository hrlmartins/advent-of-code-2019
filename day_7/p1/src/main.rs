use std::collections::HashSet;
use std::io::{self, BufRead, BufReader, Read};

#[derive(PartialEq)]
enum OpCode {
    Add = 1,
    Multiply,
    ReadInput,
    PrintAddress,
    JIfTrue,
    JIfFalse,
    Lt,
    Eq,
    Halt = 99,
}

enum ParamModes {
    PositionMode,
    ImmediateMode,
}

struct Instruction {
    op_code: OpCode,
    param_modes: Vec<ParamModes>,
}

fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let input = buffer.lines().next().unwrap(); // Reads the first and only line... let's break it!

    let value_vec: Vec<i32> = input?
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut max_value = i32::min_value();
    let mut set = HashSet::new();
    try_comb(&value_vec, 0, 0, &mut set, &mut max_value);

    println!("Is this armaggedon? Full thruster power: {}", max_value);

    Ok(())
}

fn try_comb(
    memory: &Vec<i32>,
    pos: i32,
    previous_output_signal: i32,
    used_values: &mut HashSet<i32>,
    max_value: &mut i32,
) {
    if pos == 5 {
        *max_value = previous_output_signal.max(*max_value);
        return;
    }

    // first input phase setting, next input signal input
    for phase in 0..=4 {
        let mut mut_memory = memory.clone();
        if used_values.contains(&phase) {
            // tough luck already in use...next
            continue;
        }

        //this is the phase setting
        let phase_pointer = mut_memory[1];
        mut_memory[phase_pointer as usize] = phase;
        //mark it as used
        used_values.insert(phase);

        //then read the input signal from last amp
        let signal_pointer = mut_memory[3];
        mut_memory[signal_pointer as usize] = previous_output_signal;

        let mut mem_pointer: i32 = 0;
        let mut output = -9999;
        let mut phase_read: bool = false;
        loop {
            let instr = get_instruction_from_code(mut_memory[mem_pointer as usize]);

            if instr.op_code == OpCode::Halt {
                break;
            }

            if instr.op_code == OpCode::ReadInput && !phase_read {
                // Reading the phase
                phase_read = true;
                let (pointer, tmp_out) =
                    perform_instruction(&instr, &mut mut_memory, mem_pointer, phase);
                mem_pointer = pointer;
                output = tmp_out;
            } else {
                let (pointer, tmp_out) = perform_instruction(
                    &instr,
                    &mut mut_memory,
                    mem_pointer,
                    previous_output_signal,
                );
                mem_pointer = pointer;
                output = tmp_out;
            }
        }

        try_comb(memory, pos + 1, output, used_values, max_value);
        // clear phase code from set
        used_values.remove(&phase);
    }
}

fn perform_instruction(
    inst: &Instruction,
    memory: &mut Vec<i32>,
    instruction_pointer: i32,
    input_val: i32,
) -> (i32, i32) {
    match inst.op_code {
        OpCode::Add => (
            perform_sum(memory, &inst.param_modes, instruction_pointer),
            0,
        ),
        OpCode::Multiply => (
            perform_multiplication(memory, &inst.param_modes, instruction_pointer),
            0,
        ),
        OpCode::ReadInput => (
            perform_read_input(memory, &inst.param_modes, instruction_pointer, input_val),
            0,
        ),
        OpCode::PrintAddress => perform_print_addr(memory, &inst.param_modes, instruction_pointer),
        OpCode::JIfTrue => (
            perform_jump_if_true(memory, &inst.param_modes, instruction_pointer),
            0,
        ),
        OpCode::JIfFalse => (
            perform_jump_if_false(memory, &inst.param_modes, instruction_pointer),
            0,
        ),
        OpCode::Lt => (
            perform_less_than(memory, &inst.param_modes, instruction_pointer),
            0,
        ),
        OpCode::Eq => (
            perform_eq(memory, &inst.param_modes, instruction_pointer),
            0,
        ),
        OpCode::Halt => (
            instruction_pointer + get_increment_per_operation(&OpCode::Halt),
            0,
        ),
    }
}

fn perform_eq(
    memory: &mut Vec<i32>,
    param_modes: &Vec<ParamModes>,
    instruction_pointer: i32,
) -> i32 {
    let first_value = get_value(memory, instruction_pointer + 1, &param_modes[0]);
    let second_value = get_value(memory, instruction_pointer + 2, &param_modes[1]);

    if first_value == second_value {
        store_value(memory, instruction_pointer + 3, 1, &param_modes[2])
    } else {
        store_value(memory, instruction_pointer + 3, 0, &param_modes[2])
    }

    instruction_pointer + get_increment_per_operation(&OpCode::Eq)
}

fn perform_less_than(
    memory: &mut Vec<i32>,
    param_modes: &Vec<ParamModes>,
    instruction_pointer: i32,
) -> i32 {
    let first_value = get_value(memory, instruction_pointer + 1, &param_modes[0]);
    let second_value = get_value(memory, instruction_pointer + 2, &param_modes[1]);

    if first_value < second_value {
        store_value(memory, instruction_pointer + 3, 1, &param_modes[2])
    } else {
        store_value(memory, instruction_pointer + 3, 0, &param_modes[2])
    }

    instruction_pointer + get_increment_per_operation(&OpCode::Lt)
}

fn perform_jump_if_true(
    memory: &mut Vec<i32>,
    param_modes: &Vec<ParamModes>,
    instruction_pointer: i32,
) -> i32 {
    let first_value = get_value(memory, instruction_pointer + 1, &param_modes[0]);

    if first_value != 0 {
        let second_value = get_value(memory, instruction_pointer + 2, &param_modes[1]);
        second_value
    } else {
        instruction_pointer + get_increment_per_operation(&OpCode::JIfTrue)
    }
}

fn perform_jump_if_false(
    memory: &mut Vec<i32>,
    param_modes: &Vec<ParamModes>,
    instruction_pointer: i32,
) -> i32 {
    let first_value = get_value(memory, instruction_pointer + 1, &param_modes[0]);

    if first_value == 0 {
        let second_value = get_value(memory, instruction_pointer + 2, &param_modes[1]);
        second_value
    } else {
        instruction_pointer + get_increment_per_operation(&OpCode::JIfFalse)
    }
}

fn perform_sum(
    memory: &mut Vec<i32>,
    param_modes: &Vec<ParamModes>,
    instruction_pointer: i32,
) -> i32 {
    //get operands and calculate result

    let mut sum = 0;
    for idx in 1..=2 {
        sum = sum
            + get_value(
                memory,
                instruction_pointer + idx,
                &param_modes[idx as usize - 1],
            )
    }

    // store it in third argument position
    store_value(memory, instruction_pointer + 3, sum, &param_modes[2]);
    instruction_pointer + get_increment_per_operation(&OpCode::Add)
}

fn perform_multiplication(
    memory: &mut Vec<i32>,
    param_modes: &Vec<ParamModes>,
    instruction_pointer: i32,
) -> i32 {
    //get operands and calculate result

    let mut acc = 1;
    for idx in 1..=2 {
        acc = acc
            * get_value(
                memory,
                instruction_pointer + idx,
                &param_modes[idx as usize - 1],
            )
    }

    // store it in third argument position
    store_value(memory, instruction_pointer + 3, acc, &param_modes[2]);
    instruction_pointer + get_increment_per_operation(&OpCode::Multiply)
}

fn perform_read_input(
    memory: &mut Vec<i32>,
    param_modes: &Vec<ParamModes>,
    instruction_pointer: i32,
    input: i32
) -> i32 {
    print!("provide input!!! ");
    store_value(memory, instruction_pointer + 1, input, &param_modes[0]);
    instruction_pointer + get_increment_per_operation(&OpCode::ReadInput)
}

fn perform_print_addr(
    memory: &mut Vec<i32>,
    param_modes: &Vec<ParamModes>,
    instruction_pointer: i32,
) -> (i32, i32) {
    let print_value = get_value(memory, instruction_pointer + 1, &param_modes[0]);
    println!("Print Operation value: {}", print_value);
    (
        instruction_pointer + get_increment_per_operation(&OpCode::PrintAddress),
        print_value,
    )
}

fn get_value(memory: &mut Vec<i32>, val: i32, param_mode: &ParamModes) -> i32 {
    match param_mode {
        ParamModes::PositionMode => {
            let pos = memory[val as usize];
            memory[pos as usize]
        }
        _ => memory[val as usize],
    }
}

fn store_value(memory: &mut Vec<i32>, argument: i32, result: i32, param_mode: &ParamModes) {
    match param_mode {
        ParamModes::PositionMode => {
            let pos = memory[argument as usize];
            memory[pos as usize] = result;
        }
        _ => memory[argument as usize] = result,
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

    Instruction {
        op_code,
        param_modes,
    }
}

fn get_op_code(value: i32) -> OpCode {
    match value {
        1 => OpCode::Add,
        2 => OpCode::Multiply,
        3 => OpCode::ReadInput,
        4 => OpCode::PrintAddress,
        5 => OpCode::JIfTrue,
        6 => OpCode::JIfFalse,
        7 => OpCode::Lt,
        8 => OpCode::Eq,
        _ => OpCode::Halt,
    }
}

fn get_param_mode(value: i32) -> ParamModes {
    match value {
        0 => ParamModes::PositionMode,
        _ => ParamModes::ImmediateMode,
    }
}

fn get_increment_per_operation(opcode: &OpCode) -> i32 {
    match opcode {
        OpCode::Add => 4,
        OpCode::Multiply => 4,
        OpCode::ReadInput => 2,
        OpCode::PrintAddress => 2,
        OpCode::JIfTrue => 3, // When it is false... the cursor has to move right? :)
        OpCode::JIfFalse => 3,
        OpCode::Lt => 4,
        OpCode::Eq => 4,
        OpCode::Halt => 1,
    }
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
        OpCode::Halt => 0,
    }
}
