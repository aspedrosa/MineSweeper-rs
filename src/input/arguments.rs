//! Functions to read and parse the variables of
//!  game (columns, rows and mines).

use std::io::Write;
use std::{io, process};

/// To store the parameters read from the user input
pub struct Parameters {
    /// number of rows of the board
    rows: u8,
    /// number of columns of the board
    columns: u8,
    /// number of mines on the board
    mines: u8,
}

/// Constructor + Getters
impl Parameters {
    /// Constructor
    pub fn new(rows: u8, columns: u8, mines: u8) -> Parameters {
        return Parameters {
            rows,
            columns,
            mines,
        };
    }

    /// rows getter
    pub fn rows(&self) -> u8 {
        return self.rows;
    }

    /// columns getter
    pub fn columns(&self) -> u8 {
        return self.columns;
    }

    /// mines getter
    pub fn mines(&self) -> u8 {
        return self.mines;
    }
}

/// Requests the user game variables (rows, columns, mines) from the user
///
/// TODO add difficulty option that have defined rows, column and mines values
pub fn read_parameters() -> Parameters {
    let mut input = String::new();
    print!("rows: ");
    io::stdout().flush();
    io::stdin().read_line(&mut input);
    let rows = input.trim().parse::<u8>().unwrap_or_else(|_| {
        eprintln!("Invalid rows number!");
        process::exit(1);
    });

    print!("columns: ");
    io::stdout().flush();
    input.clear();
    io::stdin().read_line(&mut input);
    let columns = input.trim().parse::<u8>().unwrap_or_else(|_| {
        eprintln!("Invalid columns number!");
        process::exit(1);
    });

    print!("mines: ");
    io::stdout().flush();
    input.clear();
    io::stdin().read_line(&mut input);
    let mines = input.trim().parse::<u8>().unwrap_or_else(|_| {
        eprintln!("Invalid mines number!");
        process::exit(1);
    });

    return Parameters {
        rows,
        columns,
        mines,
    };
}
