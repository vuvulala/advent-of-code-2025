advent_of_code::solution!(3);

struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    pub fn from_str(s: &str) -> Self {
        let mut batts = Vec::new();
        let chars = s.chars();
        for c in chars {
            batts.push(c.to_digit(10).unwrap() as u8);
        }

        Self { batteries: batts }
    }

    pub fn get_max_charge(&self, quantity: usize) -> usize {
        let mut current_index = quantity;
        let mut start = 0;

        let mut charge_digits = Vec::new();
        while current_index > 0 {
            let mut largest_found = self.batteries[start];
            let mut index = start;
            for i in start..(self.batteries.len() - current_index + 1) {
                // From last found largest digit, to last digit available whilst still having enough
                if self.batteries[i] > largest_found {
                    largest_found = self.batteries[i];
                    index = i;
                }
            }

            charge_digits.push(self.batteries[index]);
            start = index + 1;
            current_index -= 1;
        }
        let mut sum: u64 = 0;
        for digit in charge_digits {
            sum += digit as u64;
            sum *= 10;
        }
        (sum / 10) as usize
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut banks = Vec::new();

    for line in input.lines() {
        banks.push(Bank::from_str(&line));
    }

    let mut sum = 0;
    for bank in banks {
        sum += bank.get_max_charge(2);
    }

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut banks = Vec::new();

    for line in input.lines() {
        banks.push(Bank::from_str(&line));
    }

    let mut sum = 0;
    for bank in banks {
        sum += bank.get_max_charge(12);
    }

    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
