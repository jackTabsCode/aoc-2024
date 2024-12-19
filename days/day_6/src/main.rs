use std::{fmt, ops::Add};

#[allow(dead_code)]
static EXAMPLE: &str = include_str!("example.txt");
static INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy)]
struct Vector2(i32, i32);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_90(dir: &Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Add<&Direction> for Vector2 {
    type Output = Vector2;

    fn add(self, dir: &Direction) -> Self::Output {
        let dir_vec = match dir {
            Direction::Up => Vector2(-1, 0),
            Direction::Down => Vector2(1, 0),
            Direction::Left => Vector2(0, -1),
            Direction::Right => Vector2(0, 1),
        };

        Vector2(self.0 + dir_vec.0, self.1 + dir_vec.1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Guard(Direction),
    Obstructed,
    Visited,
    Empty,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Cell::Obstructed,
            '^' => Cell::Guard(Direction::Up),
            'v' => Cell::Guard(Direction::Down),
            '>' => Cell::Guard(Direction::Right),
            '<' => Cell::Guard(Direction::Left),
            '.' => Cell::Empty,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Obstructed => write!(f, "#"),
            Cell::Guard(Direction::Up) => write!(f, "^"),
            Cell::Guard(Direction::Down) => write!(f, "v"),
            Cell::Guard(Direction::Right) => write!(f, ">"),
            Cell::Guard(Direction::Left) => write!(f, "<"),
            Cell::Visited => write!(f, "X"),
            Cell::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug)]
struct Map {
    cells: Vec<Vec<Cell>>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn from_input(input: &str) -> Map {
        let cells: Vec<Vec<Cell>> = input
            .lines()
            .map(|line| line.chars().map(|ch| ch.into()).collect())
            .collect();

        Map { cells }
    }

    pub fn get_guard_pos(&mut self) -> Vector2 {
        for (row, rows) in self.cells.iter_mut().enumerate() {
            for (col, cell) in rows.iter_mut().enumerate() {
                if let Cell::Guard(_) = cell {
                    return Vector2(row as i32, col as i32);
                }
            }
        }

        unreachable!()
    }

    pub fn get_cell_at_pos(&mut self, pos: &Vector2) -> Option<&Cell> {
        self.cells.get(pos.0 as usize)?.get(pos.1 as usize)
    }

    pub fn set_cell_at_pos(&mut self, pos: &Vector2, new_cell: Cell) {
        self.cells[pos.0 as usize][pos.1 as usize] = new_cell
    }

    pub fn count_visited(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| matches!(cell, Cell::Visited))
            .count()
    }
}

fn part_1() {
    let mut map = Map::from_input(INPUT);

    loop {
        // println!("{map}");

        let guard_pos = map.get_guard_pos();
        let guard_dir = *match map.get_cell_at_pos(&guard_pos) {
            Some(Cell::Guard(dir)) => dir,
            _ => {
                continue;
            }
        };

        let next_pos = guard_pos + &guard_dir;
        let next_cell = map.get_cell_at_pos(&next_pos).cloned();

        match next_cell {
            Some(Cell::Obstructed) => {
                let next_dir = Direction::rotate_90(&guard_dir);
                let next_pos = guard_pos + &next_dir;

                map.set_cell_at_pos(&guard_pos, Cell::Visited);
                map.set_cell_at_pos(&next_pos, Cell::Guard(next_dir));
            }
            Some(_) => {
                map.set_cell_at_pos(&guard_pos, Cell::Visited);
                map.set_cell_at_pos(&next_pos, Cell::Guard(guard_dir))
            }
            None => {
                map.set_cell_at_pos(&guard_pos, Cell::Visited);
                break;
            }
        }
    }

    // println!("{map}");
    println!("{}", map.count_visited());
}

fn main() {
    part_1();
}
