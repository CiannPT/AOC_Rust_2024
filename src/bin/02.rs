advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    'lines: for line in input.split("\n").filter(|&x| !x.is_empty()) {
        let levels = line
            .split_whitespace()
            .filter(|&x| !x.is_empty())
            .map(|n| n.parse::<i32>().unwrap_or(0))
            .collect::<Vec<i32>>();
        let mut prev_diff = 0;
        for i in 1..levels.len() {
            let curr_diff = levels[i - 1] - levels[i];
            if curr_diff == 0 || curr_diff.abs() > 3 || curr_diff * prev_diff < 0 {
                continue 'lines;
            }
            prev_diff = curr_diff;
        }
        sum += 1;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    'lines: for line in input.split("\n").filter(|&x| !x.is_empty()) {
        let levels = line
            .split_whitespace()
            .filter(|&x| !x.is_empty())
            .map(|n| n.parse::<i32>().unwrap_or(0))
            .collect::<Vec<i32>>();
        'levels: for j in 0..levels.len() {
            let mut levels2 = levels.to_owned();
            levels2.remove(j);
            let mut prev_diff = 0;
            for i in 1..levels2.len() {
                let curr_diff = levels2[i - 1] - levels2[i];
                if curr_diff == 0 || curr_diff.abs() > 3 || curr_diff * prev_diff < 0 {
                    continue 'levels;
                }
                prev_diff = curr_diff;
            }
            sum+=1;
            continue 'lines;
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
