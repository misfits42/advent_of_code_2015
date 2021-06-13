#[aoc_generator(day1)]
fn generate_input(raw_input: &str) -> String {
    return String::from(raw_input);
}

#[aoc(day1, part1)]
fn solve_part_1(input: &String) -> i64 {
    let mut floor: i64 = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("D1_P1: invalid character in input!"),
        }
    }
    return floor;
}

#[aoc(day1, part2)]
fn solve_part_2(input: &String) -> usize {
    let mut pos: usize = 0;
    let mut floor: i64 = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("D1_P1: invalid character in input!"),
        }
        // Check if current character resulted in basement floor being reached
        pos += 1;
        if floor == -1 {
            return pos;
        }
    }
    panic!("D1_P2: did not reach basement by end of input!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d01_p1_proper() {
        let input = generate_input(&read_to_string("./input/2015/day1.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(232, result);
    }

    #[test]
    fn test_d01_p2_proper() {
        let input = generate_input(&read_to_string("./input/2015/day1.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(1783, result);
    }
}
