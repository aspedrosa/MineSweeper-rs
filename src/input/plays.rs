//! Functions to read and parse the user input to know
//!  the next action and over which cell.
//!
//! A play is composed by the play mode and the
//!  coordinates to the target cell, ex:
//!  `m 3 3`
//!
//! The first play only requests the target cell.
//!

use std::io::Write;
use std::{fmt, io};

use std::error::Error;
use std::fmt::Formatter;

/// Possible plays
pub enum PlayMode {
    /// Show an undug cell.
    ///
    /// If the target cell is a mine the game ends
    ///  and the user loses.
    /// If there is no more cell to dig the game ends and
    ///  the user wins.
    Dig,
    /// Mark a cell as a mine
    Mark,
    /// Unmark a cell as a mine
    Unmark,
}

/// Built after reading play input from the user
pub struct Play {
    /// Action to apply on the target cell
    mode: PlayMode,
    /// coordinates of the cell to act on
    ///
    /// TODO The option is to avoid having
    ///  to deal with lifetimes. This can
    ///  be transformed into reference
    cell: Option<(u8, u8)>,
}

/// Just getters
impl Play {
    /// Mode getter
    pub fn mode(&self) -> &PlayMode {
        return &self.mode;
    }

    /// Cell getter
    ///
    /// TODO turn return value into reference and deal with lifetimes
    pub fn cell(&mut self) -> (u8, u8) {
        return self.cell.take().unwrap();
    }
}

/// Base Error struct
///
/// TODO create more specific errors. Is this possible?
#[derive(Debug)]
pub struct InvalidPlay;

impl Error for InvalidPlay {}

impl fmt::Display for InvalidPlay {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Invalid play")
    }
}

/// Get the play string from the user input
fn read_play(input: &mut String) {
    print!("play: ");
    io::stdout().flush();
    io::stdin().read_line(input);
}

/// Parse cell on the play string
///
/// TODO return a more specific error
fn handle_cell(inputs: &Vec<&str>, first_index: u8) -> Result<(u8, u8), InvalidPlay> {
    let row = match inputs[first_index as usize].parse::<u8>() {
        Ok(v) => v,
        Err(_) => return Err(InvalidPlay {}),
    };
    let column = match inputs[(first_index + 1) as usize].parse::<u8>() {
        Ok(v) => v,
        Err(_) => return Err(InvalidPlay {}),
    };

    return Ok((row, column));
}

/// The first play only contains the target cell and the play
///  mode is always assumed its [Dig](PlayMode::Dig)
pub fn read_first_play() -> Result<Play, InvalidPlay> {
    let mut input = String::new();
    read_play(&mut input);
    let clean_input = input.trim().to_lowercase();
    let inputs: Vec<&str> = clean_input.split_whitespace().collect();

    /// TODO return more specific error
    if inputs.len() != 2 {
        return Err(InvalidPlay {});
    }

    let cell = handle_cell(&inputs, 0)?;

    return Ok(Play {
        mode: PlayMode::Dig,
        cell: Some(cell),
    });
}

/// Following plays must contain both the play mode and the target cell
pub fn read_following_plays() -> Result<Play, InvalidPlay> {
    let mut input = String::new();
    read_play(&mut input);
    let clean_input = input.trim().to_lowercase();
    let inputs: Vec<&str> = clean_input.split_whitespace().collect();

    /// TODO return a more specific error
    if inputs.len() != 3 {
        return Err(InvalidPlay {});
    }

    let cell = handle_cell(&inputs, 1)?;

    /// TODO return a more specific error
    let mode = match inputs[0] {
        "u" => PlayMode::Unmark,
        "m" => PlayMode::Mark,
        "d" => PlayMode::Dig,
        _ => return Err(InvalidPlay {}),
    };

    Ok(Play {
        mode,
        cell: Some(cell),
    })
}
