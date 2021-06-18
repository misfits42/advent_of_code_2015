use regex::Regex;

enum LightInstruction {
    TurnOn,
    TurnOff,
    Toggle,
}

impl LightInstruction {
    pub fn from_string(input: &str) -> Option<LightInstruction> {
        let input = input.to_ascii_lowercase();
        match input.as_str() {
            "turn on" => return Some(LightInstruction::TurnOn),
            "turn off" => return Some(LightInstruction::TurnOff),
            "toggle" => return Some(LightInstruction::Toggle),
            _ => return None,
        }
    }
}

struct LightChangeOperation {
    i_type: LightInstruction,
    top_left: (usize, usize),
    bot_right: (usize, usize),
}

impl LightChangeOperation {
    pub fn new(i_type: LightInstruction, top_left: (usize, usize), bot_right: (usize, usize)) -> Self {
        Self {
            i_type: i_type,
            top_left: top_left,
            bot_right: bot_right,
        }
    }
}

#[aoc_generator(day6)]
fn generate_input(raw_input: &str) -> Vec<LightChangeOperation> {
    let mut operations: Vec<LightChangeOperation> = vec![];
    let operation_regex =
        Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    for line in raw_input.lines() {
        // Trim leading and trailing whitespace, then ignore empty lines
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Extract light operation fields from current line
        if operation_regex.is_match(line) {
            let captures = operation_regex.captures(line).unwrap();
            let i_type = LightInstruction::from_string(&captures[1]).unwrap();
            let top_left_x = captures[2].parse::<usize>().unwrap();
            let top_left_y = captures[3].parse::<usize>().unwrap();
            let bot_right_x = captures[4].parse::<usize>().unwrap();
            let bot_right_y = captures[5].parse::<usize>().unwrap();
            let light_op = LightChangeOperation::new(
                i_type,
                (top_left_x, top_left_y),
                (bot_right_x, bot_right_y),
            );
            operations.push(light_op);
        }
    }
    return operations;
}

#[aoc(day6, part1)]
fn solve_part_1(operations: &Vec<LightChangeOperation>) -> u64 {
    // 1000x1000 matrix starts with all lights off
    let mut light_matrix: [[bool; 1000];  1000] = [[false; 1000]; 1000];
    // Process each operation
    for op in operations {
        // Iterate over each light covered by current operation and adjust state ASREQ
        for y in op.top_left.1..=op.bot_right.1 {
            for x in op.top_left.0..=op.bot_right.0 {
                match op.i_type {
                    LightInstruction::TurnOn => light_matrix[y][x] = true,
                    LightInstruction::TurnOff => light_matrix[y][x] = false,
                    LightInstruction::Toggle => light_matrix[y][x] = !light_matrix[y][x],
                }
            }
        }
    }
    // Count number of lights that are on after all operations have been processed
    let mut on_count = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            if light_matrix[y][x] == true {
                on_count += 1;
            }
        }
    }
    return on_count;
}

#[aoc(day6, part2)]
fn solve_part_2(operations: &Vec<LightChangeOperation>) -> i64 {
    // 1000x1000 grid starts with all lights having brightness of 0
    let mut light_matrix: [[i64; 1000]; 1000] = [[0; 1000]; 1000];
    let mut total_brightness: i64 = 0;
    // Process each light change operation
    for op in operations {
        // Iterate over each light covered by the current operation
        for y in op.top_left.1..=op.bot_right.1 {
            for x in op.top_left.0..=op.bot_right.0 {
                // Determine amount by which to adjust brightness of current light
                let delta = match op.i_type {
                    LightInstruction::TurnOn => 1,
                    LightInstruction::TurnOff => {
                        if light_matrix[y][x] > 0 {
                            -1
                        } else {
                            0
                        }
                    },
                    LightInstruction::Toggle => 2,
                };
                // Adjust current light and total brightness
                light_matrix[y][x] += delta;
                total_brightness += delta;
            }
        }
    }
    return total_brightness;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d06_p1_proper() {
        let input = generate_input(&read_to_string("./input/2015/day6.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(377891, result);
    }

    /// NOTE: this test will overflow stack of default-size test thread (8 MiB). Need to increase
    /// default test stack size using:
    /// RUST_MIN_STACK=20000000 cargo test
    #[test]
    fn test_d06_p2_proper() {
        println!("!!!! 01");
        let input = generate_input(&read_to_string("./input/2015/day6.txt").unwrap());
        println!("!!!! 02");
        let result = solve_part_2(&input);
        println!("!!!! 03");
        assert_eq!(14110788, result);
    }
}
