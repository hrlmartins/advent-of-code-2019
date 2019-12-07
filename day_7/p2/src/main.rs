use std::collections::{HashSet, HashMap};
use std::io::{self, BufRead, BufReader, Read};
use std::hash::Hash;

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

#[derive(Clone, Debug)]
pub struct Amp {
    memory: Vec<i32>,
    inst_pointer: i32
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


    let first_amp = Amp {
        memory: value_vec.clone(),
        inst_pointer: 0,
    };

    let mut amps: Vec<Amp> = vec![first_amp.clone(); 5];
    dbg!(amps.clone());
    let mut used: [bool; 10] = [false; 10];
    let mut sequence: [i32; 5] = [0; 5];
    let mut max_value = i32::min_value();

    generate_phases(0, &mut sequence, &mut used, &mut amps, &mut max_value);

    println!("Is this armaggedon? Full thruster power: {}", max_value);

    Ok(())
}

fn generate_phases(position: usize, sequence: &mut [i32; 5], used: &mut [bool; 10], amps: &mut Vec<Amp>, curr_max: &mut i32) {
    if position == 5 {
        // Run computation with generated sequence so far
        println!("{:?}", sequence);
        let mut reset_state = amps.clone();
        *curr_max = run_sequence(sequence, &mut reset_state).max(*curr_max);
        return;
    }

    for idx in 5..=9 {
        if !used[idx] {
            used[idx] = true;
            sequence[position] = idx as i32;
            generate_phases(position + 1, sequence, used, amps, curr_max);
            used[idx] = false;
        }
    }
}

fn run_sequence(sequence: &[i32; 5], amps: &mut Vec<Amp>) -> i32 {
    let mut signal = 0;
    let mut was_phase_read: [bool; 5] = [false; 5];
    let mut amp_id = 0;
    'feedback: loop {
        for (amp, &phase) in amps.iter_mut().zip(sequence) {
            println!("Switching {:?} - {:?}", amp, phase);
            loop {
                let instruction = get_instruction_from_code(amp.memory[amp.inst_pointer as usize]);

                println!("Processing {:?}", instruction.op_code);

                let (new_pointer, output) = if instruction.op_code == OpCode::ReadInput && !was_phase_read[amp_id as usize] {
                    was_phase_read[amp_id as usize] = true;
                    perform_instruction(&instruction, &mut amp.memory, amp.inst_pointer, phase)
                } else {
                    perform_instruction(&instruction, &mut amp.memory, amp.inst_pointer, signal)
                };

                if instruction.op_code == OpCode::PrintAddress {
                    println!("After OP Print: {:?} - {:?}", amp, phase);
                    signal = output;
                    amp.inst_pointer = new_pointer;
                    break;
                }

                if instruction.op_code == OpCode::Halt {
                    println!("After OP halting: {:?} - {:?}", amp, phase);
                    amp.inst_pointer = new_pointer;
                    break 'feedback;
                }

                amp.inst_pointer = new_pointer;
                println!("After OP: {:?} - {:?}", amp, phase);
            }

            amp_id = (amp_id + 1) % 5;
        }
    }

    signal
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
    println!("provide input!!! ");
    store_value(memory, instruction_pointer + 1, input, &param_modes[0]);
    instruction_pointer + get_increment_per_operation(&OpCode::ReadInput)
}

fn perform_print_addr(
    memory: &mut Vec<i32>,
    param_modes: &Vec<ParamModes>,
    instruction_pointer: i32,
) -> (i32, i32) {
    let print_value = get_value(memory, instruction_pointer + 1, &param_modes[0]);
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
