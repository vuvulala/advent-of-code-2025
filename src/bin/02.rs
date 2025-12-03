use std::str::FromStr;

use fancy_regex::Regex;

advent_of_code::solution!(2);

fn parse_range(s: &str) -> (u64, u64) {
    let mut nums = s.trim().split("-");
    let num1 = nums.next().unwrap().parse::<u64>();
    let num2 = nums.next().unwrap().parse::<u64>();
    (num1.unwrap(), num2.unwrap())
}

fn is_valid_id(id: u64, reg: &Regex) -> bool {
    let id_as_str = id.to_string();

    return !reg.is_match(&id_as_str).unwrap();
    if id_as_str.len() % 2 == 1 {
        return true;
    }

    let middle = id_as_str.len() / 2;

    for i in 0..middle {
        if id_as_str.chars().nth(i).unwrap() != id_as_str.chars().nth(i + middle).unwrap() {
            return true;
        }
    }
    return false;
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut ranges = Vec::new();

    for range_str in input.split(",") {
        ranges.push(parse_range(range_str));
    }
    let r = Regex::from_str("^(.+)\\1$").unwrap();

    let mut sum = 0;
    for range in ranges {
        for i in range.0..=range.1 {
            if !is_valid_id(i, &r) {
                sum += i
            }
        }
    }
    //println!("{sum}");
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges = Vec::new();

    for range_str in input.split(",") {
        ranges.push(parse_range(range_str));
    }
    let r = Regex::from_str("^(.+)\\1+$").unwrap();

    let mut sum = 0;
    for range in ranges {
        for i in range.0..=range.1 {
            if !is_valid_id(i, &r) {
                sum += i
            }
        }
    }
    //println!("{sum}");
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
