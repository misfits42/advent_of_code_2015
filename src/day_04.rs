use md5;

#[aoc_generator(day4)]
fn generate_input(raw_input: &str) -> String {
    return String::from(raw_input.trim());
}

#[aoc(day4, part1)]
fn solve_part_1(secret_key: &String) -> u64 {
    let mut seq_id = 1;
    loop {
        // Check if current sequence ID mines AdventCoins
        let hash_target = format!("{}{}", secret_key, seq_id);
        let digest = md5::compute(hash_target.as_bytes());
        let hash_result_hex = format!("{:x}", digest);
        // Check if hash result starts with five zeroes
        if hash_result_hex.starts_with("00000") {
            break;
        }
        // Go to the next sequence ID
        seq_id += 1;
    }
    return seq_id;
}

#[aoc(day4, part2)]
fn solve_part_2(secret_key: &String) -> u64 {
    let mut seq_id = 1;
    loop {
        // Check if current sequence ID mines AdventCoins
        let hash_target = format!("{}{}", secret_key, seq_id);
        let digest = md5::compute(hash_target.as_bytes());
        let hash_result_hex = format!("{:x}", digest);
        // Check if hash result starts with six zeroes
        if hash_result_hex.starts_with("000000") {
            break;
        }
        // Go to the next sequence ID
        seq_id += 1;
    }
    return seq_id;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d04_p1_proper() {
        let input = generate_input(&read_to_string("./input/2015/day4.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(254575, result);
    }

    #[test]
    fn test_d04_p2_proper() {
        let input = generate_input(&read_to_string("./input/2015/day4.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(1038736, result);
    }
}
