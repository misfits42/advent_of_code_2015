#[aoc_generator(day11)]
fn generate_input(raw_input: &str) -> String {
    return raw_input.trim().to_string();
}

/// Increments the input string - assuming it only contains lowercase alphabetical characters.
fn increment_alphabetic_string(input: &String) -> String {
    let mut output: Vec<char> = vec![];
    let mut carry = true;
    for c in input.chars().rev() {
        if carry == false {
            output.push(c);
            continue;
        }
        if c == 'z' {
            // Maintain the carry to next character
            output.push('a')
        } else {
            // Incremented character, but don't need to carry over
            let inc_c = std::char::from_u32(c as u32 + 1).unwrap();
            output.push(inc_c);
            carry = false;
        }
    }
    return output.iter().rev().collect::<String>();
}

#[aoc(day11, part1)]
fn solve_part_1(input: &String) -> String {
    let test = increment_alphabetic_string(input);

    return test;
}

#[aoc(day11, part2)]
fn solve_part_2(input: &String) -> String {
    unimplemented!();
}
