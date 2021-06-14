use regex::Regex;

// Represents a three-dimensional object defined by its length, width and height (all in feet).
struct Present {
    length: u64,
    width: u64,
    height: u64
}

impl Present {
    pub fn new(length: u64, width: u64, height: u64) -> Self {
        Self {
            length: length,
            width: width,
            height: height
        }
    }

    pub fn calculate_surface_area(&self) -> u64 {
        let mut sides: Vec<u64> = vec![];
        // Calculate side areas
        let bottom_area = self.length * self.width;
        let small_side_area = self.width * self.height;
        let large_side_area = self.length * self.height;
        // Determine size of the smallest side
        sides.push(bottom_area);
        sides.push(small_side_area);
        sides.push(large_side_area);
        let smallest_side = sides.iter().min().unwrap();
        return 2 * bottom_area + 2 * small_side_area + 2 * large_side_area + smallest_side;
    }

    pub fn calculate_volume(&self) -> u64 {
        return self.length * self.width * self.height;
    }

    pub fn calculate_ribbon_length(&self) -> u64 {
        let mut ribbon_length = 0;
        // Add ribbon required to tie bow
        ribbon_length += self.calculate_volume();
        // Add ribbon required to wrap present
        let mut distances: Vec<u64> = vec![];
        distances.push(2 * self.height + 2 * self.width);
        distances.push(2 * self.height + 2 * self.length);
        distances.push(2 * self.length + 2 * self.width);
        ribbon_length += distances.iter().min().unwrap();
        return ribbon_length;
    }
}

#[aoc_generator(day2)]
fn generate_input(raw_input: &str) -> Vec<Present> {
    let present_regex = Regex::new(r"^(\d+)x(\d+)x(\d+)$").unwrap();
    let mut presents_result = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if present_regex.is_match(line) {
            let captures = present_regex.captures(line).unwrap();
            let length = captures[1].parse::<u64>().unwrap();
            let width = captures[2].parse::<u64>().unwrap();
            let height = captures[3].parse::<u64>().unwrap();
            let present = Present::new(length, width, height);
            presents_result.push(present);
        }
    }
    return presents_result;
}

#[aoc(day2, part1)]
fn solve_part_1(input: &Vec<Present>) -> u64 {
    let mut total_area = 0;
    for present in input {
        total_area += present.calculate_surface_area();
    }
    return total_area;
}

#[aoc(day2, part2)]
fn solve_part_2(input: &Vec<Present>) -> u64 {
    let mut total_ribbon_length = 0;
    for present in input {
        total_ribbon_length += present.calculate_ribbon_length();
    }
    return total_ribbon_length;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d02_p1_proper() {
        let input = generate_input(&read_to_string("./input/2015/day2.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(1588178, result);
    }

    #[test]
    fn test_d02_p2_proper() {
        let input = generate_input(&read_to_string("./input/2015/day2.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(3783758, result);
    }
}
