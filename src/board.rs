use itertools::Itertools;
use rand::Rng;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

use crate::input::arguments::Parameters;

pub struct Cell(u8, u8);

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0 && self.1 == other.1;
    }
}

impl Eq for Cell {}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl Cell {
    pub fn new(x: u8, y: u8) -> Cell {
        return Cell(x, y);
    }
}

pub enum GameResult {
    Lost,
    Won,
    None,
}

pub struct Board {
    rows: u8,
    columns: u8,
    mines_count: u8,

    display: Vec<Vec<i8>>,
    board: Vec<Vec<i8>>,

    marks: HashSet<Cell>,
}

impl Board {
    pub fn new(params: &Parameters) -> Board {
        let mut board = Vec::<Vec<i8>>::with_capacity(params.rows() as usize);
        let mut display = Vec::<Vec<i8>>::with_capacity(params.rows() as usize);

        for _ in 0..params.rows() {
            let mut row1 = Vec::<i8>::with_capacity(params.columns() as usize);
            let mut row2 = Vec::<i8>::with_capacity(params.columns() as usize);
            for _ in 0..params.columns() {
                row1.push(0);
                row2.push(-2);
            }
            board.push(row1);
            display.push(row2)
        }

        Board {
            rows: params.rows(),
            columns: params.columns(),
            mines_count: params.mines(),
            display,
            board,
            marks: HashSet::new(),
        }
    }

    pub fn build(&mut self, start_cell: Cell) {
        self.display[start_cell.0 as usize][start_cell.1 as usize] = 0;

        let EMPTY_CELLS = 40; // TODO calculate this according to difficulty

        let mut initial_empty_cells = Vec::with_capacity(EMPTY_CELLS);

        initial_empty_cells.push(start_cell);

        let expand_possibilities = [[1, 0], [0, 1], [-1, 0], [0, -1]];

        let mut random = rand::thread_rng();
        while initial_empty_cells.len() < EMPTY_CELLS {
            let cell_index = random.gen_range(0..initial_empty_cells.len());
            let cell = initial_empty_cells.get(cell_index).unwrap();

            let expand_dir = expand_possibilities[random.gen_range(0..4)];
            let x_diff = expand_dir[0];
            let y_diff = expand_dir[1];

            let new_cell_x = cell.0 as i16 + x_diff;
            let new_cell_y = cell.1 as i16 + y_diff;
            if !(new_cell_x < 0
                || new_cell_x >= self.columns as i16
                || new_cell_y < 0
                || new_cell_y >= self.rows as i16)
            {
                let new_cell = Cell(new_cell_x as u8, new_cell_y as u8);

                self.display[new_cell.0 as usize][new_cell.1 as usize] = 0;

                if !initial_empty_cells.contains(&new_cell) {
                    initial_empty_cells.push(new_cell);
                }
            }
        }

        let initial_empty_cells = HashSet::<Cell>::from_iter(initial_empty_cells.into_iter());
        let mines = self.place_mines(&initial_empty_cells);
        self.place_numbers(mines);
        self.show_numbers_border(initial_empty_cells);
    }

    fn place_mines(&mut self, initial_empty_cells: &HashSet<Cell>) -> HashSet<Cell> {
        let mut mines = HashSet::with_capacity(self.mines_count as usize);

        let mut current_mines = 0;

        let mut random = rand::thread_rng();
        while current_mines < mines.capacity() {
            let row = random.gen_range(0..self.rows) as u8;
            let column = random.gen_range(0..self.columns) as u8;

            let m = Cell(row, column);

            if mines.contains(&m) || initial_empty_cells.contains(&m) {
                continue;
            }

            mines.insert(m);

            current_mines += 1;

            self.board[row as usize][column as usize] = -1;
        }

        return mines;
    }

    fn place_numbers(&mut self, mines: HashSet<Cell>) {
        let columns = self.columns;
        let rows = self.rows;

        // TODO all show all numbers on the initial border
        for mine in mines.iter() {
            for cell in (-1..=1)
                .chain(-1..=1)
                .combinations_with_replacement(2)
                .unique()
                .filter(|v| {
                    let cell_x = mine.0 as i16 + v[0];
                    let cell_y = mine.1 as i16 + v[1];

                    return cell_x >= 0
                        && cell_x < rows as i16
                        && cell_y >= 0
                        && cell_y < columns as i16;
                })
                .map(|v| Cell((mine.0 as i16 + v[0]) as u8, (mine.1 as i16 + v[1]) as u8))
                .filter(|cell| !mines.contains(&cell))
            {
                self.board[cell.0 as usize][cell.1 as usize] += 1;
            }
        }
    }

    fn show_numbers_border(&mut self, initial_empty_cells: HashSet<Cell>) {
        for empty_cell in initial_empty_cells.iter() {
            for adjacent_cell in (-1..=1)
                .chain(-1..=1)
                .combinations_with_replacement(2)
                .unique()
                .filter(|v| {
                    let cell_x = empty_cell.0 as i16 + v[0];
                    let cell_y = empty_cell.1 as i16 + v[1];

                    return cell_x >= 0
                        && cell_x < self.rows as i16
                        && cell_y >= 0
                        && cell_y < self.columns as i16;
                })
                .filter(|v| self.board[v[0] as usize][v[1] as usize] == -1) {
                self.board[adjacent_cell[0] as usize][adjacent_cell[1] as usize] += 1;
            }
        }
    }

    pub fn dig(&mut self, cell: Cell) -> GameResult {
        if self.display[cell.0 as usize][cell.1 as usize] == -2 {
            match self.board[cell.0 as usize][cell.1 as usize] {
                -1 => return GameResult::Lost,
                0 => {
                    // propagate dig
                }
                v => self.display[cell.0 as usize][cell.1 as usize] = v,
            }

            // check if won
        }

        return GameResult::None;
    }

    pub fn mark(&mut self, cell: Cell) {
        self.display[cell.0 as usize][cell.1 as usize] =
            match self.display[cell.0 as usize][cell.1 as usize] {
                -2 => -1,
                -1 => -2,
                v => v,
            };
    }

    pub fn print_board(&self) {
        for row in self.board.iter() {
            for (i, v) in row.iter().enumerate() {
                if *v == -1 {
                    print!("#");
                } else if *v == 0 {
                    print!(" ");
                } else {
                    print!("{}", v);
                }
                if i != row.len() {
                    print!(" ");
                }
            }
            println!();
        }
    }

    pub fn print_display(&self) {
        for row in self.display.iter() {
            for (i, v) in row.iter().enumerate() {
                if *v == -2 {
                    print!("?");
                } else if *v == -1 {
                    print!("X");
                } else if *v == 0 {
                    print!(" ");
                } else {
                    print!("{}", v);
                }
                if i != row.len() {
                    print!(" | ");
                }
            }
            println!();
        }
    }
}
