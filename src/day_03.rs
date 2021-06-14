use std::collections::HashSet;
use super::utils::carto::{CardinalDirection, Point2D};

#[aoc_generator(day3)]
fn generate_input(raw_input: &str) -> Vec<CardinalDirection> {
    let mut directions: Vec<CardinalDirection> = vec![];
    for c in raw_input.chars() {
        match c {
            '^' => directions.push(CardinalDirection::North),
            '<' => directions.push(CardinalDirection::West),
            '>' => directions.push(CardinalDirection::East),
            'v' => directions.push(CardinalDirection::South),
            _ => ()
        }
    }
    return directions;
}

#[aoc(day3, part1)]
fn solve_part_1(input: &Vec<CardinalDirection>) -> usize {
    let mut visited: HashSet<Point2D> = HashSet::new();
    let mut current_pos = Point2D::new(0, 0);
    visited.insert(current_pos);
    for dir in input {
        let unit_vec = dir.get_unit_vector();
        current_pos.move_point(unit_vec.0, unit_vec.1);
        visited.insert(current_pos);
    }
    return visited.len();
}

#[aoc(day3, part2)]
fn solve_part_2(input: &Vec<CardinalDirection>) -> usize {
    let mut visited: HashSet<Point2D> = HashSet::new();
    let mut santa_pos = Point2D::new(0, 0);
    let mut robo_santa_pos = Point2D::new(0, 0);
    visited.insert(Point2D::new(0, 0));
    let mut i = 0;
    loop {
        if i >= input.len() {
            break;
        }
        // Move santa
        let santa_unit_vec = input[i].get_unit_vector();
        santa_pos.move_point(santa_unit_vec.0, santa_unit_vec.1);
        visited.insert(santa_pos);
        // Move robo santa
        let robo_santa_unit_vec = input[i + 1].get_unit_vector();
        robo_santa_pos.move_point(robo_santa_unit_vec.0, robo_santa_unit_vec.1);
        visited.insert(robo_santa_pos);
        // Advance to next pair of directions
        i += 2;
    }
    return visited.len();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d03_p1_proper() {
        let input = generate_input(&read_to_string("./input/2015/day3.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(2572, result);
    }

    #[test]
    fn test_d03_p2_proper() {
        let input = generate_input(&read_to_string("./input/2015/day3.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(2631, result);
    }
}
