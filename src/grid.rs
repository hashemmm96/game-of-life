extern crate colored;
use colored::*;

use std::fmt;
use std::fs;
use std::ops;
use std::string::String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Cell {
    x: i32,
    y: i32,
    alive: bool,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let square = "\u{2B1B}";
        if self.alive {
            write!(f, "{}", square.on_blue().blue())
        } else {
            write!(f, "{}", square.on_white().white())
        }
    }
}

#[derive(Eq, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
    NE,
    NW,
    SE,
    SW,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    size_x: i32,
    size_y: i32,
}

impl Grid {
    /// Mutate grid to its next state.
    /// Return false if the next state is identical to the current state.
    pub fn next_state(&mut self) -> bool {
        /* We first calculate the next state of each cell in the grid,
         * then we apply the state at the end.
         */
        let mut new_grid = Grid {
            cells: Vec::new(),
            size_x: self.size_x,
            size_y: self.size_y,
        };

        let mut expand_directions = Vec::new();

        for row in &self.cells {
            let mut new_row: Vec<Cell> = Vec::new();
            for cell in row {
                let direction = self.on_edge(*cell);
                if cell.alive && direction.is_some() {
                    expand_directions.push(direction.unwrap());
                }
                let new_cell = Cell {
                    x: cell.x,
                    y: cell.y,
                    alive: self.get_next_state(*cell),
                };
                new_row.push(new_cell);
            }
            new_grid.cells.push(new_row);
        }

        for direction in expand_directions {
            new_grid.expand(direction);
        }

        let ret = if *self != new_grid { true } else { false };
        *self = new_grid;
        ret
    }

    /// Render the grid.
    pub fn render(&self) {
        use std::fmt::Write;
        top_of_screen();
        let image_size = self.size_x * self.size_y * 4;
        let mut image = String::with_capacity(image_size as usize);
        for row in &self.cells {
            for cell in row {
                write!(&mut image, "{}", cell).expect("Fatal error in render()");
            }
            write!(&mut image, "\n").expect("Fatal error in render()");
        }
        print!("{}", image);
    }

    fn get_neighbors(&self, cell: Cell) -> Vec<Cell> {
        let mut neighbors: Vec<Cell> = Vec::new();
        for x in calc_range(cell.x, self.size_x - 1) {
            for y in calc_range(cell.y, self.size_y - 1) {
                if (cell.x, cell.y) != (x, y) {
                    neighbors.push(self.cells[y as usize][x as usize]);
                }
            }
        }
        neighbors
    }

    fn get_live_neighbors(&self, cell: Cell) -> i32 {
        let neighbors = self.get_neighbors(cell);
        neighbors
            .into_iter()
            .filter(|c| c.alive)
            .collect::<Vec<Cell>>()
            .len() as i32
    }

    /* If the cell is alive, then it stays alive if it has either 2 or 3 live neighbors
     *
     * If the cell is dead, then it springs to life only in the case that it has 3 live neighbors
     */
    fn get_next_state(&self, cell: Cell) -> bool {
        let alive = self.get_live_neighbors(cell);
        if cell.alive && !(2..4).contains(&alive) {
            false
        } else if !cell.alive && alive == 3 {
            true
        } else {
            cell.alive
        }
    }

    fn on_edge(&self, cell: Cell) -> Option<Direction> {
        if cell.x == 0 && cell.y == 0 {
            Some(Direction::NW)
        } else if cell.y == 0 && cell.x == self.size_x - 1 {
            Some(Direction::NE)
        } else if cell.x == 0 && cell.y == self.size_y - 1 {
            Some(Direction::SW)
        } else if cell.x == self.size_x - 1 && cell.y == self.size_y - 1 {
            Some(Direction::SE)
        } else if cell.x == 0 {
            Some(Direction::W)
        } else if cell.x == self.size_x - 1 {
            Some(Direction::E)
        } else if cell.y == 0 {
            Some(Direction::N)
        } else if cell.y == self.size_y - 1 {
            Some(Direction::S)
        } else {
            None
        }
    }

    fn add_column_east(&mut self) {
        for row_number in 0..self.size_y {
            let new_cell = Cell {
                x: self.size_x,
                y: row_number,
                alive: false,
            };
            self.cells[row_number as usize].push(new_cell);
        }
        self.size_x += 1;
    }

    fn add_column_west(&mut self) {
        // Need to shift every cell in every row to the right
        for row_number in 0..self.size_y {
            let mut shifted_row = Vec::new();
            let new_cell = Cell {
                x: 0,
                y: row_number,
                alive: false,
            };
            shifted_row.push(new_cell);
            for cell in &self.cells[row_number as usize] {
                let mut shifted_cell = *cell;
                shifted_cell.x += 1;
                shifted_row.push(shifted_cell);
            }
            self.cells[row_number as usize] = shifted_row;
        }
        self.size_x += 1;
    }

    fn add_row_north(&mut self) {
        // Need to shift every element in every row downwards
        for row_number in 0..self.size_y as usize {
            let mut shifted_row = Vec::new();
            for cell in &self.cells[row_number] {
                let mut shifted_cell = *cell;
                shifted_cell.y += 1;
                shifted_row.push(shifted_cell);
            }
            self.cells[row_number] = shifted_row;
        }
        // Then add new row
        let mut new_row = Vec::new();
        for col_number in 0..self.size_x {
            let new_cell = Cell {
                x: col_number,
                y: 0,
                alive: false,
            };
            new_row.push(new_cell);
        }
        self.cells.insert(0, new_row);
        self.size_y += 1;
    }

    fn add_row_south(&mut self) {
        let mut new_row = Vec::new();
        for col_number in 0..self.size_x {
            let new_cell = Cell {
                x: col_number,
                y: self.size_y,
                alive: false,
            };
            new_row.push(new_cell);
        }
        self.cells.push(new_row);
        self.size_y += 1;
    }

    fn expand(&mut self, direction: Direction) {
        // Add rows
        match direction {
            Direction::N | Direction::NE | Direction::NW => self.add_row_north(),
            Direction::S | Direction::SE | Direction::SW => self.add_row_south(),
            _ => {}
        }
        // Add columns
        match direction {
            Direction::E | Direction::NE | Direction::SE => self.add_column_east(),
            Direction::W | Direction::NW | Direction::SW => self.add_column_west(),
            _ => {}
        }
    }
}

pub fn create_grid(filename: &str) -> Grid {
    let contents = fs::read_to_string(filename).unwrap();
    parse_pattern_file(&contents)
}

fn parse_pattern_file(grid_string: &str) -> Grid {
    let mut cells: Vec<Vec<Cell>> = Vec::new();
    let mut x = 0;
    let mut y = 0;

    for line in str::lines(grid_string) {
        let mut row: Vec<Cell> = Vec::new();
        x = 0;

        for char in line.chars() {
            let alive;
            match char {
                'x' => alive = true,
                '-' => alive = false,
                _ => panic!("Invalid file format."),
            }
            let cell = Cell { x, y, alive };
            row.push(cell);
            x += 1;
        }
        y += 1;
        cells.push(row);
    }

    let grid = Grid {
        cells,
        size_x: x,
        size_y: y,
    };

    grid
}

fn top_of_screen() {
    let escape: u8 = 27;
    print!("{}[H", escape as char);
}

fn calc_range(x: i32, max: i32) -> ops::RangeInclusive<i32> {
    let start = if x > 0 { x - 1 } else { x };
    let end = if x < max { x + 1 } else { x };
    start..=end
}
