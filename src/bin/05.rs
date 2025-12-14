use advent_of_code::{Endpoint, Range};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut ranges = vec![];
    let mut items: Vec<i64> = vec![];
    while let Some(cur_line) = lines.next() {
        if cur_line == "" {
            break;
        }

        let (start, stop) = cur_line.split_once("-").unwrap();
        let start_num: u64 = start.parse().unwrap();

        let end_num: u64 = stop.parse().unwrap();

        ranges.push(Range::new(
            Endpoint::Inclusive(start_num),
            Endpoint::Inclusive(end_num),
        ));
    }

    while let Some(cur_line) = lines.next() {
        items.push(cur_line.parse().unwrap());
    }
    let mut valid_items = 0;
    for item in items {
        let mut is_fresh = false;
        for range in &ranges {
            if range.includes_value(item as u64) {
                is_fresh = true;
            }
        }
        if is_fresh {
            valid_items += 1;
        }
    }

    Some(valid_items)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
