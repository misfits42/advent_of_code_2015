#[aoc_generator(day10)]
fn generate_input(raw_input: &str) -> Vec<char> {
    return raw_input.trim().chars().collect::<Vec<char>>();
}

fn apply_iteration_lookandsay(input: &Vec<char>) -> Vec<char> {
    let mut new_sequence: Vec<char> = vec![];
    let mut i = 0;
    loop {
        if i >= input.len() {
            break;
        }
        // Determine length of current sequence
        let mut delta = 1;
        loop {
            // Break if end of input sequence reached
            if i + delta >= input.len() {
                break;
            }
            // Check if sequence continues or ends
            if input[i + delta] == input[i] {
                delta += 1;
            } else {
                break;
            }
        }
        let mut delta_chars = delta.to_string().chars().collect::<Vec<char>>();
        new_sequence.append(&mut delta_chars);
        new_sequence.push(input[i]);
        i += delta;
    }
    return new_sequence;
}

#[aoc(day10, part1)]
fn solve_part_1(input: &Vec<char>) -> usize {
    let mut sequence = input.clone();
    // Apply 40 iterations of look-and-say to the input sequence
    for _ in 0..40 {
        sequence = apply_iteration_lookandsay(&sequence);
    }
    // Result is length of sequence after iterations applied
    return sequence.len();
}

#[aoc(day10, part2)]
fn solve_part_2(input: &Vec<char>) -> usize {
    let mut sequence = input.clone();
    // Apply 50 iterations of look-and-say to the input sequence
    for _ in 0..50 {
        sequence = apply_iteration_lookandsay(&sequence);
    }
    // Result is length of sequence after iterations applied
    return sequence.len();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d10_p1_proper() {
        let input = generate_input(&read_to_string("./input/2015/day10.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(329356, result);
    }

    #[test]
    fn test_d10_p2_proper() {
        let input = generate_input(&read_to_string("./input/2015/day10.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(4666278, result);
    }
}
