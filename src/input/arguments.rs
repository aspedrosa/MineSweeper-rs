use std::io::Write;
use std::{io, process};

pub struct Parameters {
    rows: u8,
    columns: u8,
    mines: u8,
}

impl Parameters {
    pub fn new(rows: u8, columns: u8, mines: u8) -> Parameters {
        return Parameters {
            rows,
            columns,
            mines,
        };
    }

    pub fn rows(&self) -> u8 {
        return self.rows;
    }

    pub fn columns(&self) -> u8 {
        return self.columns;
    }

    pub fn mines(&self) -> u8 {
        return self.mines;
    }
}

// TODO add difficulty option that have defined values
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
