use std::collections::HashMap;
use regex::Regex;

/// Defines the different instructions and their variants based on operand types.
/// Possible instructions are: INPUT, AND, OR, LSHIFT, RSHIFT, NOT.
#[derive(Debug)]
enum Instruction {
    InputWire{wire_id: String},
    InputValue{input_value: u16},
    AndWireInput{wire_id: String, input_value: u16},
    AndWires{l_wire_id: String, r_wire_id: String},
    AndInputs{l_input_value: u16, r_input_value: u16},
    OrWireInput{wire_id: String, input_value: u16},
    OrWires{l_wire_id: String, r_wire_id: String},
    OrInputs{l_input_value: u16, r_input_value: u16},
    LeftshiftWire{wire_id: String, shift_value: u16},
    LeftshiftInput{input_value: u16, shift_value: u16},
    RightshiftWire{wire_id: String, shift_value: u16},
    RightshiftInput{input_value: u16, shift_value: u16},
    NotWire{wire_id: String},
    NotInput{input_value: u16},
}

#[aoc_generator(day7)]
fn generate_input(raw_input: &str) -> HashMap<String, Instruction> {
    // Record the input being provided to each wire - ASSUMED that each wire has only one input!
    let mut circuit_wires: HashMap<String, Instruction> = HashMap::new();
    // Define regexes to match each of the instruction variants
    let input_value_regex = Regex::new(r"^(\d+) -> ([[:alpha:]]+)$").unwrap();
    let input_wire_regex = Regex::new(r"^([[:alpha:]]+) -> ([[:alpha:]])$").unwrap();
    let and_regex = Regex::new(r"^(.+) AND (.+) -> ([[:alpha:]]+)$").unwrap();
    let or_regex = Regex::new(r"^(.+) OR (.+) -> ([[:alpha:]]+)$").unwrap();
    let leftshift_regex = Regex::new(r"^(.+) LSHIFT (\d+) -> ([[:alpha:]]+)$").unwrap();
    let rightshift_regex = Regex::new(r"^(.+) RSHIFT (\d+) -> ([[:alpha:]]+)$").unwrap();
    let not_regex = Regex::new(r"^NOT (.+) -> ([[:alpha:]]+)$").unwrap();
    for line in raw_input.lines() {
        // Trim leading and trailing whitespace, then ignore empty lines
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if input_wire_regex.is_match(line) {
            let captures = input_wire_regex.captures(line).unwrap();
            let output_wire = captures[2].to_string();
            let wire_id = captures[1].to_string();
            let instruction = Instruction::InputWire{wire_id};
            circuit_wires.insert(output_wire, instruction);
        } else if input_value_regex.is_match(line) {
            let captures = input_value_regex.captures(line).unwrap();
            let output_wire = captures[2].to_string();
            let input_value = captures[1].parse::<u16>().unwrap();
            let instruction = Instruction::InputValue{input_value};
            circuit_wires.insert(output_wire, instruction);
        } else if and_regex.is_match(line) {
            let captures = and_regex.captures(line).unwrap();
            let output_wire = captures[3].to_string();
            if captures[1].parse::<u16>().is_ok() && captures[2].parse::<u16>().is_ok() {
                let l_input_value = captures[1].parse::<u16>().unwrap();
                let r_input_value = captures[2].parse::<u16>().unwrap();
                let instruction = Instruction::AndInputs{l_input_value, r_input_value};
                circuit_wires.insert(output_wire, instruction);
            } else if captures[1].parse::<u16>().is_ok() && !captures[2].parse::<u16>().is_ok() {
                let input_value = captures[1].parse::<u16>().unwrap();
                let wire_id = captures[2].to_string();
                let instruction = Instruction::AndWireInput{wire_id, input_value};
                circuit_wires.insert(output_wire, instruction);
            } else if !captures[1].parse::<u16>().is_ok() && captures[2].parse::<u16>().is_ok() {
                let input_value = captures[2].parse::<u16>().unwrap();
                let wire_id = captures[1].to_string();
                let instruction = Instruction::AndWireInput{wire_id, input_value};
                circuit_wires.insert(output_wire, instruction);
            } else {
                let l_wire_id = captures[1].to_string();
                let r_wire_id = captures[2].to_string();
                let instruction = Instruction::AndWires{l_wire_id, r_wire_id};
                circuit_wires.insert(output_wire, instruction);
            }
        } else if or_regex.is_match(line) {
            let captures = or_regex.captures(line).unwrap();
            let output_wire = captures[3].to_string();
            if captures[1].parse::<u16>().is_ok() && captures[2].parse::<u16>().is_ok() {
                let l_input_value = captures[1].parse::<u16>().unwrap();
                let r_input_value = captures[2].parse::<u16>().unwrap();
                let instruction = Instruction::OrInputs{l_input_value, r_input_value};
                circuit_wires.insert(output_wire, instruction);
            } else if captures[1].parse::<u16>().is_ok() && !captures[2].parse::<u16>().is_ok() {
                let input_value = captures[1].parse::<u16>().unwrap();
                let wire_id = captures[2].to_string();
                let instruction = Instruction::OrWireInput{wire_id, input_value};
                circuit_wires.insert(output_wire, instruction);
            } else if !captures[1].parse::<u16>().is_ok() && captures[2].parse::<u16>().is_ok() {
                let input_value = captures[2].parse::<u16>().unwrap();
                let wire_id = captures[1].to_string();
                let instruction = Instruction::OrWireInput{wire_id, input_value};
                circuit_wires.insert(output_wire, instruction);
            } else {
                let l_wire_id = captures[1].to_string();
                let r_wire_id = captures[2].to_string();
                let instruction = Instruction::OrWires{l_wire_id, r_wire_id};
                circuit_wires.insert(output_wire, instruction);
            }
        } else if leftshift_regex.is_match(line) {
            let captures = leftshift_regex.captures(line).unwrap();
            let output_wire = captures[3].to_string();
            if captures[1].parse::<u16>().is_ok() {
                let input_value = captures[1].parse::<u16>().unwrap();
                let shift_value = captures[2].parse::<u16>().unwrap();
                let instruction = Instruction::LeftshiftInput{input_value, shift_value};
                circuit_wires.insert(output_wire, instruction);
            } else {
                let wire_id = captures[1].to_string();
                let shift_value = captures[2].parse::<u16>().unwrap();
                let instruction = Instruction::LeftshiftWire{wire_id, shift_value};
                circuit_wires.insert(output_wire, instruction);
            }
        } else if rightshift_regex.is_match(line) {
            let captures = rightshift_regex.captures(line).unwrap();
            let output_wire = captures[3].to_string();
            if captures[1].parse::<u16>().is_ok() {
                let input_value = captures[1].parse::<u16>().unwrap();
                let shift_value = captures[2].parse::<u16>().unwrap();
                let instruction = Instruction::RightshiftInput{input_value, shift_value};
                circuit_wires.insert(output_wire, instruction);
            } else {
                let wire_id = captures[1].to_string();
                let shift_value = captures[2].parse::<u16>().unwrap();
                let instruction = Instruction::RightshiftWire{wire_id, shift_value};
                circuit_wires.insert(output_wire, instruction);
            }
        } else if not_regex.is_match(line) {
            let captures = not_regex.captures(line).unwrap();
            let output_wire = captures[2].to_string();
            if captures[1].parse::<u16>().is_ok() {
                let input_value = captures[1].parse::<u16>().unwrap();
                let instruction = Instruction::NotInput{input_value};
                circuit_wires.insert(output_wire, instruction);
            } else {
                let wire_id = captures[1].to_string();
                let instruction = Instruction::NotWire{wire_id};
                circuit_wires.insert(output_wire, instruction);
            }
        } else {
            panic!("Day 7 - input line found with invalid format!");
        }
    }
    // for (k, v) in circuit_wires.iter() {
    //     println!(">>> Output: {} // Instruction: {:?}", k, v);
    // }
    return circuit_wires;
}

#[aoc(day7, part1)]
fn solve_part_1(circuit_wires: &HashMap<String, Instruction>) -> u16 {
    unimplemented!();
}

#[aoc(day7, part2)]
fn solve_part_2(circuit_wires: &HashMap<String, Instruction>) -> u16 {
    unimplemented!();
}
