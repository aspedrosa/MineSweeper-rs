use std::io::Write;
use std::{fmt, io};

use std::error::Error;
use std::fmt::Formatter;

pub enum PlayMode {
    Dig,
    Mark,
}

pub struct Play {
    mode: PlayMode,
    cell: Option<(u8, u8)>,
}

impl Play {
    pub fn mode(&self) -> &PlayMode {
        return &self.mode;
    }

    pub fn cell(&mut self) -> (u8, u8) {
        return self.cell.take().unwrap();
    }
}

#[derive(Debug)]
pub struct InvalidPlay {}

impl Error for InvalidPlay {}

impl fmt::Display for InvalidPlay {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Invalid play")
    }
}

fn read_play(input: &mut String) {
    print!("play: ");
    io::stdout().flush();
    io::stdin().read_line(input);
}

fn handle_cell(inputs: &Vec<&str>, first_index: u8) -> Result<(u8, u8), InvalidPlay> {
    let row = match inputs[first_index as usize].parse::<u8>() {
        Ok(v) => v,
        Err(_) => return Err(InvalidPlay {}),
    };
    let column = match inputs[(first_index + 1) as usize].parse::<u8>() {
        Ok(v) => v,
        Err(_) => return Err(InvalidPlay {}),
    };

    return Ok(
        (row, column)
    );
}

pub fn read_first_play() -> Result<Play, InvalidPlay> {
    let mut input = String::new();
    read_play(&mut input);
    let clean_input = input.trim().to_lowercase();
    let inputs: Vec<&str> = clean_input.split_whitespace().collect();

    if inputs.len() != 2 {
        return Err(InvalidPlay {});
    }

    let cell = handle_cell(&inputs, 0)?;

    return Ok(
        Play {
            mode: PlayMode::Dig,
            cell: Some(cell),
        }
    );
}

pub fn read_following_plays() -> Result<Play, InvalidPlay> {
    let mut input = String::new();
    read_play(&mut input);
    let clean_input = input.trim().to_lowercase();
    let inputs: Vec<&str> = clean_input.split_whitespace().collect();

    if inputs.len() != 3 {
        return Err(InvalidPlay {});
    }

    let cell = handle_cell(&inputs, 1)?;

    let mode = match inputs[0] {
        "m" => PlayMode::Mark,
        "d" => PlayMode::Dig,
        _ => return Err(InvalidPlay {}),
    };

    Ok(Play {
        mode,
        cell: Some(cell),
    })
}
