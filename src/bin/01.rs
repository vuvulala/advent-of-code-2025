advent_of_code::solution!(1);

#[derive(Debug)]
enum Instruction {
    Left(isize),
    Right(isize),
}

struct Dial {
    current_number: isize,
}

impl Dial {
    pub fn new() -> Self {
        Self { current_number: 50 }
    }

    pub fn spin(&mut self, instruction: &Instruction) -> isize {
        let start = self.current_number;
        // Returns times visited 0, not including a potential landing
        let turn_count;
        let turn_direction;
        let mut times_visited_0;

        match instruction {
            Instruction::Left(steps) => {
                times_visited_0 = steps.abs() / 100;
                turn_count = steps % 100;
                turn_direction = -1;
            }
            Instruction::Right(steps) => {
                times_visited_0 = steps.abs() / 100;
                turn_count = steps % 100;
                turn_direction = 1;
            }
        }

        self.current_number += turn_count * turn_direction;
        if self.current_number == 0 {
            times_visited_0 += 1;
        }
        if self.current_number < 0 {
            self.current_number += 100;
            if start != 0 {
                times_visited_0 += 1;
            }
            if self.current_number == 0 {
                times_visited_0 += 1;
            }
        }
        if self.current_number >= 100 {
            self.current_number -= 100;
            times_visited_0 += 1;
        }

        return times_visited_0;
    }

    fn is_0(&self) -> bool {
        self.current_number == 0
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut steps = Vec::new();
    for line in input.lines() {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let count = chars.collect::<String>().parse::<isize>().unwrap();

        steps.push(match direction {
            'L' => Instruction::Left(count),
            'R' => Instruction::Right(count),
            _ => panic!(),
        });
    }

    steps
}

pub fn part_one(input: &str) -> Option<u64> {
    let steps = parse_input(input);

    let mut dial = Dial::new();
    let mut landed_0_count = 0;

    for step in steps {
        dial.spin(&step);
        if dial.is_0() {
            landed_0_count += 1;
        }
    }

    Some(landed_0_count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let steps = parse_input(input);
    let mut dial = Dial::new();

    let mut visited_0_count = 0;

    for step in steps {
        visited_0_count += dial.spin(&step);
    }

    Some(visited_0_count as u64)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_example() {
        let mut d = Dial::new();
        assert_eq!(d.spin(&Instruction::Left(68)), 1);
        assert_eq!(d.spin(&Instruction::Left(30)), 0);
        assert_eq!(d.spin(&Instruction::Right(48)), 1);
        assert_eq!(d.spin(&Instruction::Left(5)), 0);
        assert_eq!(d.spin(&Instruction::Right(60)), 1);
        assert_eq!(d.spin(&Instruction::Left(55)), 1);
        assert_eq!(d.spin(&Instruction::Left(1)), 0);
        assert_eq!(d.spin(&Instruction::Left(99)), 1);
        assert_eq!(d.spin(&Instruction::Right(14)), 0);
        assert_eq!(d.spin(&Instruction::Left(82)), 1);
    }
}
