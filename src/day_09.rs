use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

#[aoc_generator(day9)]
fn generate_input(raw_input: &str) -> HashMap<String, HashMap<String, u64>> {
    // Parse input as a graph, with each node connected to others by undirected edges
    let mut graph: HashMap<String, HashMap<String, u64>> = HashMap::new();
    let line_regex = Regex::new(r"^([[:alpha:]]+) to ([[:alpha:]]+) = (\d+)$").unwrap();
    for line in raw_input.lines() {
        // Trim leading and trailing whitespace from each line, then ignore empty lines
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Only parse lines that match the required format - otherwise terminate program.
        if let Some(captures) = line_regex.captures(line) {
            let city_a = captures[1].to_string();
            let city_b = captures[2].to_string();
            let distance = captures[3].parse::<u64>().unwrap();
            // Add new key to graph if needed
            if !graph.contains_key(&city_a) {
                graph.insert(city_a.to_string(), HashMap::new());
            }
            if !graph.contains_key(&city_b) {
                graph.insert(city_b.to_string(), HashMap::new());
            }
            // Connect the cities together
            graph.get_mut(&city_a).unwrap().insert(city_b.to_string(), distance);
            graph.get_mut(&city_b).unwrap().insert(city_a.to_string(), distance);
        } else {
            panic!("Day 9 - raw input line with bad format!");
        }
    }
    return graph;
}

#[aoc(day9, part1)]
fn solve_part_1(graph: &HashMap<String, HashMap<String, u64>>) -> u64 {
    // Input has eight cities, meaning 8! possible sequences - small enough to brute force
    let cities = graph.keys().map(|x| x.to_string()).collect::<Vec<String>>();
    // Track the minimum distance observed across each tested sequence
    let mut min_distance = u64::MAX;
    // All cities are connected to each other by undirected edges
    for seq in cities.iter().permutations(cities.len()).unique() {
        let mut seq_distance = 0;
        // Add up the total distance for the current sequence
        for i in 0..seq.len()-1 {
            seq_distance += graph.get(seq[i]).unwrap().get(seq[i + 1]).unwrap();
        }
        // Check if a new minimum distance has been found
        if seq_distance < min_distance {
            min_distance = seq_distance;
        }
    }
    return min_distance;
}

#[aoc(day9, part2)]
fn solve_part_2(graph: &HashMap<String, HashMap<String, u64>>) -> u64 {
    // Input has eight cities, meaning 8! possible sequences - small enough to brute force
    let cities = graph.keys().map(|x| x.to_string()).collect::<Vec<String>>();
    // Track the maximum distance observed across each tested sequence
    let mut max_distance = 0;
    // All cities are connected to each other by undirected edges
    for seq in cities.iter().permutations(cities.len()).unique() {
        let mut seq_distance = 0;
        // Add up the total distance for the current sequence
        for i in 0..seq.len()-1 {
            seq_distance += graph.get(seq[i]).unwrap().get(seq[i + 1]).unwrap();
        }
        // Check if a new maximum distance has been found
        if seq_distance > max_distance {
            max_distance = seq_distance;
        }
    }
    return max_distance;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn test_d09_p1_proper() {
        let input = generate_input(&read_to_string("./input/2015/day9.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(141, result);
    }

    #[test]
    fn test_d09_p2_proper() {
        let input = generate_input(&read_to_string("./input/2015/day9.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(736, result);
    }
}
