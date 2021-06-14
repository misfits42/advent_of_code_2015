use fancy_regex::Regex;

#[aoc_generator(day5)]
fn generate_input(raw_input: &str) -> Vec<String> {
    let mut input_strings: Vec<String> = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        input_strings.push(String::from(line));
    }
    return input_strings;
}

#[aoc(day5, part1)]
fn solve_part_1(input_strings: &Vec<String>) -> u64 {
    let mut nice_count = 0;
    // Create regexes to match against nice string properties
    let nice_1_regex = Regex::new(r"^.*[aeiou].*[aeiou].*[aeiou].*$").unwrap();
    let nice_2_regex = Regex::new(r"^.*([[:alpha:]])\1.*$").unwrap();
    let nice_3_regex = Regex::new(r"^.*(ab|cd|pq|xy).*$").unwrap();
    // Check which input strings match all nice string properties
    for candidate in input_strings {
        // Candidate must satisfy all properties to be a nice string
        if !nice_1_regex.is_match(candidate).unwrap()
            | !nice_2_regex.is_match(candidate).unwrap()
            | nice_3_regex.is_match(candidate).unwrap()
        {
            continue;
        }
        nice_count += 1;
    }
    return nice_count;
}

#[aoc(day5, part2)]
fn solve_part_2(input_strings: &Vec<String>) -> u64 {
    let mut nice_count = 0;
    // Define nice string properties with regexes
    let nice_1_regex = Regex::new(r"^.*([[:alpha:]])([[:alpha:]]).*\1\2.*$").unwrap();
    let nice_2_regex = Regex::new(r"^.*([[:alpha:]])([[:alpha:]])\1.*$").unwrap();
    // Check which input strings match all nice string properties
    for candidate in input_strings {
        // Candidate must satisfiy all properties to be a nice string
        if !nice_1_regex.is_match(candidate).unwrap() | !nice_2_regex.is_match(candidate).unwrap() {
            continue;
        }
        nice_count += 1;
    }
    return nice_count;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d05_p1_proper() {
        let input = generate_input(&read_to_string("./input/2015/day5.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(255, result);
    }

    #[test]
    fn test_d05_p2_proper() {
        let input = generate_input(&read_to_string("./input/2015/day5.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(55, result);
    }
}
