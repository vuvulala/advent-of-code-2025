use std::str::FromStr;

advent_of_code::solution!(4);

#[derive(Clone, Copy, Debug)]
enum CellType {
    Floor,
    Paper,
}

impl std::fmt::Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellType::Floor => f.write_str(" "),
            CellType::Paper => f.write_str("@"),
        }
    }
}

impl FromStr for CellType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            '@' => Ok(Self::Paper),
            '.' => Ok(Self::Floor),
            _ => panic!(),
        }
    }
}

struct DynGrid<T> {
    arena: Vec<T>,
    width: usize,
}

impl<T: FromStr> FromStr for DynGrid<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let width = lines.peek().unwrap().trim().len();

        let mut s = Self::new();
        s.width = width;

        for line in lines {
            for c in line.trim().chars() {
                s.arena.push(T::from_str(&c.to_string())?)
            }
        }

        Ok(s)
    }
}

impl<T> DynGrid<T> {
    pub fn new() -> Self {
        Self {
            arena: Vec::new(),
            width: 0,
        }
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.arena.len() / self.width
    }

    pub fn get_cell_signed(&self, x: isize, y: isize) -> Option<&T> {
        if x < 0 || x >= self.width().try_into().unwrap() {
            return None;
        }
        if y < 0 || y >= self.height().try_into().unwrap() {
            return None;
        }

        return Some(&self.arena[x as usize + y as usize * self.width()]);
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width() {
            return None;
        }
        if y >= self.height() {
            return None;
        }

        return Some(&self.arena[x + y * self.width()]);
    }

    pub fn get_cell_signed_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if x < 0 || x >= self.width().try_into().unwrap() {
            return None;
        }
        if y < 0 || y >= self.height().try_into().unwrap() {
            return None;
        }

        let w = self.width();
        return Some(&mut self.arena[x as usize + y as usize * w]);
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width() {
            return None;
        }
        if y >= self.height() {
            return None;
        }
        let w = self.width();
        return Some(&mut self.arena[x + y * w]);
    }

    pub fn get_neighbours(&self, x: usize, y: usize) -> Vec<Option<&T>> {
        let mut neighbours = Vec::new();
        for iy in -1..=1 {
            for ix in -1..=1 {
                if ix == 0 && iy == 0 {
                    continue;
                }
                neighbours.push(self.get_cell_signed(x as isize + ix, y as isize + iy));
            }
        }

        neighbours
    }

    pub fn set_cell(&mut self, x: usize, y: usize, t: T) -> Result<(), ()> {
        let c = self.get_cell_mut(x, y).ok_or(())?;

        *c = t;

        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = DynGrid::<CellType>::from_str(input).unwrap();

    let mut available_count = 0;

    for y in 0..map.height() {
        for x in 0..map.width() {
            let neighbours = map.get_neighbours(x, y);
            println!("{neighbours:?}");
            let mut paper_count = 0;

            for n in neighbours {
                if let Some(t) = n {
                    match t {
                        CellType::Floor => {}
                        CellType::Paper => {
                            paper_count += 1;
                        }
                    }
                }
            }

            if paper_count < 4 {
                match map.get_cell(x, y).unwrap() {
                    CellType::Floor => {}
                    CellType::Paper => {
                        available_count += 1;
                    }
                }
            }

            println!("{x}, {y} -> {paper_count}");
        }
    }

    Some(available_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = DynGrid::<CellType>::from_str(input).unwrap();

    let mut available_count = 0;

    let mut total_changed = 1;

    while total_changed > 0 {
        total_changed = 0;
        for y in 0..map.height() {
            for x in 0..map.width() {
                let neighbours = map.get_neighbours(x, y);

                let mut paper_count = 0;

                for n in neighbours {
                    if let Some(t) = n {
                        match t {
                            CellType::Floor => {}
                            CellType::Paper => {
                                paper_count += 1;
                            }
                        }
                    }
                }

                if paper_count < 4 {
                    match map.get_cell(x, y).unwrap() {
                        CellType::Floor => {}
                        CellType::Paper => {
                            available_count += 1;
                            *map.get_cell_mut(x, y).unwrap() = CellType::Floor;

                            total_changed += 1;
                        }
                    }
                }

                //println!("{x}, {y} -> {paper_count}");
            }
        }
    }

    Some(available_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
