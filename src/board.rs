use itertools::{Itertools, enumerate};
use rand::Rng;
use std::collections::HashSet;
use std::iter::FromIterator;

use crate::input::arguments::Parameters;
use core::fmt;
use std::fmt::Formatter;

pub enum GameResult {
    Lost,
    Won,
    Continue,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum CellValue {
    Empty,
    Mine,
    Number(u8),
}

impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CellValue::Empty => write!(f, " "),
            CellValue::Mine => write!(f, "X"),
            CellValue::Number(n) => write!(f, "{}", n),
        }
    }
}

struct Cell {
    dug: bool,
    marked: bool,
    value: CellValue,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.marked {
            write!(f, "X")
        }
        else {
            if self.dug {
                self.value.fmt(f)
            }
            else {
                write!(f, "?")
            }
        }
    }
}


pub struct Board {
    rows: u8,
    columns: u8,
    mines_count: u8,

    board: Vec<Vec<Cell>>,

    marks: HashSet<(u8, u8)>,
}

impl Board {
    pub fn new(params: &Parameters) -> Board {
        let mut board = Vec::<Vec<Cell>>::with_capacity(params.rows() as usize);

        for _ in 0..params.rows() {
            let mut row1 = Vec::<Cell>::with_capacity(params.columns() as usize);
            for _ in 0..params.columns() {
                row1.push(Cell {
                    dug: false,
                    marked: false,
                    value: CellValue::Empty,
                });
            }
            board.push(row1);
        }

        Board {
            rows: params.rows(),
            columns: params.columns(),
            mines_count: params.mines(),
            board,
            marks: HashSet::new(),
        }
    }

    pub fn build(&mut self, start_cell: (u8, u8)) {
        self.board[start_cell.0 as usize][start_cell.1 as usize].value = CellValue::Empty;

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
                let new_cell = (new_cell_x as u8, new_cell_y as u8);

                self.board[new_cell.0 as usize][new_cell.1 as usize].value = CellValue::Empty;

                if !initial_empty_cells.contains(&new_cell) {
                    initial_empty_cells.push(new_cell);
                }
            }
        }

        let initial_empty_cells = HashSet::<(u8, u8)>::from_iter(initial_empty_cells.into_iter());
        let mines = self.place_mines(&initial_empty_cells);
        self.place_numbers(mines);
        self.show_numbers_border(initial_empty_cells);
    }

    fn place_mines(&mut self, initial_empty_cells: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let mut mines = HashSet::with_capacity(self.mines_count as usize);

        let mut current_mines = 0;

        let mut random = rand::thread_rng();
        while current_mines < mines.capacity() {
            let row = random.gen_range(0..self.rows) as u8;
            let column = random.gen_range(0..self.columns) as u8;

            let m = (row, column);

            if mines.contains(&m) || initial_empty_cells.contains(&m) {
                continue;
            }

            mines.insert(m);

            current_mines += 1;

            self.board[row as usize][column as usize].value = CellValue::Mine;
        }

        return mines;
    }

    fn place_numbers(&mut self, mines: HashSet<(u8, u8)>) {
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
                .map(|v| ((mine.0 as i16 + v[0]) as u8, (mine.1 as i16 + v[1]) as u8))
                .filter(|cell| !mines.contains(&cell))
            {
                match self.board[cell.0 as usize][cell.1 as usize].value {
                    CellValue::Number(ref mut n) => *n = *n + 1,
                    _ => self.board[cell.0 as usize][cell.1 as usize].value = CellValue::Number(1),
                }
            }
        }
    }

    fn show_numbers_border(&mut self, initial_empty_cells: HashSet<(u8, u8)>) {
        let initial_empty_cells = initial_empty_cells.iter().filter(|c| self.board[c.0 as usize][c.1 as usize].value == CellValue::Empty).collect::<Vec<&(u8, u8)>>();
        for empty_cell in initial_empty_cells {
            for adjacent_cell in (-1..=1)
                .chain(-1..=1)
                .combinations_with_replacement(2)
                .unique()
                .map(|v| {
                    let cell_x = empty_cell.0 as i16 + v[0];
                    let cell_y = empty_cell.1 as i16 + v[1];

                    vec!(cell_x, cell_y)
                })
                .filter(|v|
                    v[0] >= 0
                        && v[0] < self.rows as i16
                        && v[1] >= 0
                        && v[1] < self.columns as i16
                )
                .filter(|v| self.board[v[0] as usize][v[1] as usize].value != CellValue::Mine)
                .collect::<Vec<Vec<i16>>>()
            {
                self.board[adjacent_cell[0] as usize][adjacent_cell[1] as usize].value = self.board[adjacent_cell[0] as usize][adjacent_cell[1] as usize].value;
            }
        }
    }

    pub fn dig(&mut self, cell: (u8, u8)) -> GameResult {
        //

        return GameResult::Continue;
    }

    pub fn mark(&mut self, cell: (u8, u8)) {
        self.board[cell.0 as usize][cell.1 as usize].marked = !(self.board[cell.0 as usize][cell.1 as usize].marked ^ self.board[cell.0 as usize][cell.1 as usize].marked)
    }

    pub fn unmark(&mut self, cell: (u8, u8)) {
        self.board[cell.0 as usize][cell.1 as usize].marked ^= self.board[cell.0 as usize][cell.1 as usize].marked
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.board.iter() {
            for (i, cell) in enumerate(row) {
                cell.fmt(f)?;
                if i != row.len() - 1 {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
