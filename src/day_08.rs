use std::str;

#[aoc_generator(day8)]
fn generate_input(raw_input: &str) -> Vec<String> {
    return raw_input
        .lines()
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();
}

#[aoc(day8, part1)]
fn solve_part_1(file_strings: &Vec<String>) -> usize {
    let mut total_code_length: usize = 0;
    let mut total_mem_length: usize = 0;
    for s in file_strings {
        // Increase total code length of strings
        total_code_length += s.len();
        // Count number of characters when stored in memory
        let mut count = 0;
        let mut i = 0;
        let s_chars = s.chars().collect::<Vec<char>>();
        loop {
            if i >= s_chars.len() {
                break;
            }
            if s_chars[i] == '\\' {
                if s_chars[i + 1] == '\\' || s_chars[i + 1] == '"' {
                    i += 2;
                } else if s_chars[i + 1] == 'x' {
                    i += 4;
                }
            } else {
                i += 1;
            }
            count += 1;
        }
        // Remove characters counted as start and end quote
        count -= 2;
        total_mem_length += count;
    }
    return total_code_length - total_mem_length;
}

#[aoc(day8, part2)]
fn solve_part_2(file_strings: &Vec<String>) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d08_p1_proper() {
        let input = generate_input(&read_to_string("./input/2015/day8.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(1371, result);
    }
}
