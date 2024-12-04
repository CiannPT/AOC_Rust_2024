advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for (v1, v2) in Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(input)
        .map(|c| (c.get(1).unwrap(), c.get(2).unwrap()))
    {
        sum += v1.as_str().parse::<u32>().unwrap() * v2.as_str().parse::<u32>().unwrap();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut enabled = true;
    for (action, v1, v2) in Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)|(don't|do)\(\)")
        .unwrap()
        .captures_iter(input)
        .map(
            |c| match c.get(1) {
                Some(m) => (m.as_str(),c.get(2).unwrap().as_str(),c.get(3).unwrap().as_str()),
                None => (c.get(4).unwrap().as_str(),"",""),
            },
        )
    {
        if action == "mul" && enabled {
            sum += v1.parse::<u32>().unwrap() * v2.parse::<u32>().unwrap();
        } else if action == "don't" {
            enabled = false;
        } else if action == "do" {
            enabled = true
        }
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
