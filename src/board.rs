//! All the game logic is in this module

use ansi_term;
use itertools::{enumerate, Itertools};
use rand::Rng;
use std::collections::HashSet;
use std::iter::FromIterator;

use crate::input::arguments::Parameters;
use core::fmt;
use std::borrow::{Borrow, BorrowMut};
use std::fmt::Formatter;

/// Returned by this module so the main
///  function know if the game should continue.
pub enum GameResult {
    Lost,
    Won,
    Continue,
}

/// Possible value of a cell
#[derive(PartialEq, Eq, Clone, Copy)]
enum CellValue {
    Empty,
    Mine,
    Number(u8),
}

impl fmt::Display for CellValue {
    /// Color based on the ones present in http://minesweeperonline.com/
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CellValue::Empty => write!(f, " "),
            CellValue::Mine => write!(f, "X"),
            CellValue::Number(n) => match n {
                1 => write!(f, "{}", ansi_term::Color::RGB(0, 0, 255).paint("1")),
                2 => write!(f, "{}", ansi_term::Color::RGB(0, 123, 0).paint("2")),
                3 => write!(f, "{}", ansi_term::Color::RGB(255, 0, 0).paint("3")),
                4 => write!(f, "{}", ansi_term::Color::RGB(0, 0, 123).paint("4")),
                5 => write!(f, "{}", ansi_term::Color::RGB(123, 0, 0).paint("5")),
                6 => write!(f, "{}", ansi_term::Color::RGB(0, 123, 123).paint("6")),
                _ => write!(f, "{}", n),
            },
        }
    }
}

impl CellValue {
    /// Is self type Number.
    ///
    /// TODO can I do `var == CellValue::Number(_)` ?
    fn is_number(&self) -> bool {
        match self {
            CellValue::Number(_) => true,
            _ => false,
        }
    }
}

/// A cell on the playing board.
struct Cell {
    /// If the user already dug this cell.
    dug: bool,
    /// If the user marked this cell as a mine.
    marked: bool,
    value: CellValue,
}

impl fmt::Display for Cell {
    /// if the cell is mark print a red `X`,
    ///  else if the user already dug the cell use
    ///  its fmt method else print `?`
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.marked {
            write!(f, "{}", ansi_term::Color::Red.paint("X"))
        } else {
            if self.dug {
                self.value.fmt(f)
            } else {
                write!(f, "?")
            }
        }
    }
}

/// Contains all the game data.
pub struct Board {
    /// Number of rows of the board
    rows: u8,
    /// Number of columns of the board
    columns: u8,
    /// Number of min on the board
    mines_count: u8,
    /// Remaining cell to dig.
    /// Used to know if the used won.
    cells_to_dig: u16,
    /// After the user loses this will contain
    ///  the coordinates for the mine that he
    ///  dug. Before that contains `None`.
    deadly_mine: Option<(u8, u8)>,

    /// Board of cells
    board: Vec<Vec<Cell>>,
}

impl Board {
    /// Constructor
    ///
    /// Fills the board with cell with [Empty](CellValue::Empty) value.
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
            cells_to_dig: params.rows() as u16 * params.columns() as u16,
            board,
            deadly_mine: None,
        }
    }

    /// Places mines and numbers on the board.
    ///
    /// Called only after the first play.
    ///
    /// Before placing mines and numbers creates
    ///  an empty area around the cell that the
    ///  user choose on the first play.
    ///
    /// TODO number of cells of the empty area is hardcoded
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
        self.propagate_dig(start_cell);
    }

    /// Place the mines, avoiding the calculated initial empty area.
    ///
    /// The mines coordinates are returned to be used on the
    ///  [place_numbers](Board::place_numbers) method.
    ///
    /// TODO bug: a mine with mines on all fours straight directions
    ///  can happen
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
            self.cells_to_dig -= 1;
        }

        return mines;
    }

    /// Go over all mines and turn their values into
    ///  [numbers](CellValue::Number) incrementing their count.
    fn place_numbers(&mut self, mines: HashSet<(u8, u8)>) {
        for mine in mines.iter() {
            for cell in self
                .generate_ring(*mine)
                .filter(|cell| !mines.contains(&(cell.0 as u8, cell.1 as u8)))
                .collect::<Vec<(i16, i16)>>()
            {
                match self.board[cell.0 as usize][cell.1 as usize].value {
                    CellValue::Number(ref mut n) => *n = *n + 1,
                    _ => self.board[cell.0 as usize][cell.1 as usize].value = CellValue::Number(1),
                }
            }
        }
    }

    /// Method called for the [Dig](crate::input::plays::PlayMode::Dig) play.
    ///
    /// If the target cell is already dug or marked nothing happens.
    ///
    /// If the target is a mine the game ends and the user loses.
    ///
    /// If the target is a number decrement cells to dig
    ///  and set dug to true.
    ///
    /// If the target is empty propagate the dig action
    ///  to all empty adjacent cells.
    ///
    /// If there is no more cell to dig, the game ends and the
    ///  user wins.
    pub fn dig(&mut self, play: (u8, u8)) -> GameResult {
        let mut cell = &mut self.board[play.0 as usize][play.1 as usize];
        if !cell.dug && !cell.marked {
            match cell.value {
                CellValue::Mine => {
                    self.deadly_mine = Some(play);
                    return GameResult::Lost;
                }
                CellValue::Number(_) => {
                    self.cells_to_dig -= 1;
                    cell.dug = true
                }
                CellValue::Empty => self.propagate_dig(play),
            }
        }

        if self.cells_to_dig == 0 {
            for row in self.board.iter_mut() {
                row.iter_mut()
                    .filter(|cell| cell.value == CellValue::Mine)
                    .for_each(|cell| cell.marked = true);
            }

            return GameResult::Won;
        }

        return GameResult::Continue;
    }

    /// Auxiliary method to generate all adjacent cells of a specific cell.
    fn generate_ring(&self, (row, col): (u8, u8)) -> impl Iterator<Item = (i16, i16)> + '_ {
        return (-1..=1)
            .chain(-1..=1)
            .combinations_with_replacement(2)
            .unique()
            .map(move |v| (row as i16 + v[0], col as i16 + v[1]))
            .filter(move |(r, c)| {
                *r >= 0 && *r < self.rows as i16 && *c >= 0 && *c < self.columns as i16
            });
    }

    /// Method to show all cells around a cell with value [Empty](CellValue::Empty).
    ///
    /// If its neighbours are also cell with [Empty](CellValue::Empty) it propagates
    ///  the effect.
    fn propagate_dig(&mut self, (initial_row, initial_col): (u8, u8)) {
        let mut to_propagate = Vec::new();
        to_propagate.push((initial_row as i16, initial_col as i16));

        let mut seen = HashSet::<(i16, i16)>::new();

        while let Some((r, c)) = to_propagate.pop() {
            let cell = &mut self.board[r as usize][c as usize];
            cell.dug = true;
            cell.marked = false;
            self.cells_to_dig -= 1;

            seen.insert((r as i16, c as i16));

            if cell.value.is_number() {
                continue;
            }

            to_propagate.extend(
                self.generate_ring((r as u8, c as u8))
                    .filter(|(r, c)| self.board[*r as usize][*c as usize].value != CellValue::Mine)
                    .filter(|(r, c)| !self.board[*r as usize][*c as usize].dug)
                    .filter(|cell| !seen.contains(cell)),
            )
        }
    }

    /// Method called for the [Mark](crate::input::plays::PlayMode::Mark) play.
    ///
    /// If the the cell is not dug nothing happens.
    pub fn mark(&mut self, (row, col): (u8, u8)) {
        let cell = &mut self.board[row as usize][col as usize];
        if !cell.dug {
            cell.marked = !(cell.marked ^ cell.marked)
        }
    }

    /// Method called for the [Unmark](crate::input::plays::PlayMode::Unmark) play.
    ///
    /// If the the cell is not dug nothing happens.
    pub fn unmark(&mut self, (row, col): (u8, u8)) {
        let cell = &mut self.board[row as usize][col as usize];
        if !cell.dug {
            cell.marked ^= cell.marked
        }
    }

    /// Method called when the game ends to print a compact version of the board.
    pub fn finish(&self) {
        for (i, row) in enumerate(&self.board) {
            for (j, cell) in enumerate(row) {
                if !cell.marked {
                    match cell.value {
                        CellValue::Mine => match self.deadly_mine {
                            Some(deadly_mine) if deadly_mine == (i as u8, j as u8) => {
                                print!("{}", ansi_term::Color::Red.paint("O"))
                            }
                            _ => print!("{}", ansi_term::Color::Yellow.paint("O")),
                        },
                        v if cell.dug => print!("{}", v),
                        _ => print!("?"),
                    }
                } else {
                    match cell.value {
                        CellValue::Mine => print!("{}", ansi_term::Color::Green.paint("X")),
                        _ => print!("{}", ansi_term::Color::Red.paint("X")),
                    }
                }
                if j != row.len() - 1 {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

/// Auxiliary function to print horizontal bars above and under the board
///  so its easier to know the coordinates of a cell.
fn print_horizontal_bar(f: &mut Formatter<'_>, columns: i32) -> fmt::Result {
    write!(f, "   ")?;
    for i in 0..columns {
        write!(f, "{:2} ", i)?;
    }
    writeln!(f)?;

    Ok(())
}

impl fmt::Display for Board {
    /// Used to print the board between plays.
    ///
    /// Prints auxiliary numbers around the board so its easier
    ///  for the user to know the coordinates of a cell.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        print_horizontal_bar(f, self.columns as i32)?;

        for (i, row) in enumerate(self.board.iter()) {
            write!(f, "{:2}  ", i)?;  // auxiliary bar on the left of coordinates
            for cell in row {
                cell.fmt(f)?;
                write!(f, "  ")?;
            }
            writeln!(f, "{}", i)?;  // auxiliary bar on the right of coordinates
        }

        print_horizontal_bar(f, self.columns as i32)?;

        Ok(())
    }
}
