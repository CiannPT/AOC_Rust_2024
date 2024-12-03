use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum:i32 = 0;
    let mut l1:Vec<i32> = Vec::new();
    let mut l2:Vec<i32> = Vec::new();

    for line in input.split("\n").filter(|&x| !x.is_empty()) {
        let line_values = line.split_whitespace().map(|n| n.parse::<i32>().unwrap_or(0)).collect::<Vec<i32>>();
        l1.push(line_values[0]);
        l2.push(line_values[1]);
    }
    l1.sort();
    l2.sort();
    for i in 0..l1.len() {
        sum+= (l1[i]-l2[i]).abs();
    }
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum:u32 = 0;
    let mut l1:Vec<u32> = Vec::new();
    let mut l2: HashMap<u32, u32> = HashMap::new();

    for line in input.split("\n").filter(|&x| !x.is_empty()) {
        let line_values = line.split_whitespace().map(|n| n.parse::<u32>().unwrap_or(0)).collect::<Vec<u32>>();
        l1.push(line_values[0]);
        l2.insert(line_values[1], l2.get(&line_values[1]).unwrap_or(&0)+1);
    }
    for i in 0..l1.len() {
        sum+= l1[i]*l2.get(&l1[i]).unwrap_or(&0);
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
