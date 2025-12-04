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

#[derive(Debug)]
struct Map {
    map: Vec<CellType>,
    pub width: usize,
}

impl Map {
    pub fn from_str(s: &str) -> Self {
        let mut cells = Vec::new();
        let mut width = 0;
        for line in s.lines() {
            width = line.trim().len();

            for c in line.trim().chars() {
                cells.push(match c {
                    '@' => CellType::Paper,
                    '.' => CellType::Floor,
                    _ => panic!(),
                });
            }
        }

        Self { map: cells, width }
    }

    pub fn get_cell(&self, x: isize, y: isize) -> Option<CellType> {
        if x < 0 {
            return None;
        }
        if y < 0 {
            return None;
        }
        if x >= self.width.try_into().unwrap() {
            return None;
        }
        if y >= (self.map.len() / self.width).try_into().unwrap() {
            return None;
        }

        return Some(self.map[x as usize + y as usize * self.width]);
    }

    pub fn get_neighbours(&self, x: isize, y: isize) -> Vec<Option<CellType>> {
        let mut neighbours = Vec::new();
        for ix in -1..=1 {
            for iy in -1..=1 {
                if ix == 0 && iy == 0 {
                    continue;
                }

                neighbours.push(self.get_cell(x + ix, y + iy));
            }
        }

        neighbours
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.map.len() / self.width
    }

    pub fn set_cell(&mut self, x: isize, y: isize, t: CellType) -> Result<(), ()> {
        if x < 0 {
            return Err(());
        }
        if y < 0 {
            return Err(());
        }
        if x >= self.width.try_into().unwrap() {
            return Err(());
        }
        if y >= (self.map.len() / self.width).try_into().unwrap() {
            return Err(());
        }

        self.map[x as usize + y as usize * self.width] = t;
        Ok(())
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height() {
            for cell in 0..self.width() {
                //println!("get cell {row}, {cell}");
                self.get_cell(cell as isize, row as isize)
                    .unwrap()
                    .fmt(f)
                    .unwrap();
            }
            f.write_str("\n").unwrap();
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = Map::from_str(input);

    let mut available_count = 0;

    for y in 0..map.height() {
        for x in 0..map.width() {
            let neighbours = map.get_neighbours(x as isize, y as isize);

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
                match map.get_cell(x as isize, y as isize).unwrap() {
                    CellType::Floor => {}
                    CellType::Paper => {
                        available_count += 1;
                    }
                }
            }

            //println!("{x}, {y} -> {paper_count}");
        }
    }

    Some(available_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = Map::from_str(input);

    let mut available_count = 0;

    let mut total_changed = 1;

    while total_changed > 0 {
        total_changed = 0;
        for y in 0..map.height() {
            for x in 0..map.width() {
                let neighbours = map.get_neighbours(x as isize, y as isize);

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
                    match map.get_cell(x as isize, y as isize).unwrap() {
                        CellType::Floor => {}
                        CellType::Paper => {
                            available_count += 1;
                            map.set_cell(x as isize, y as isize, CellType::Floor)
                                .unwrap();
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
