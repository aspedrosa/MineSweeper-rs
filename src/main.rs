mod board;
mod input;

use crate::board::{Board, GameResult};
use crate::input::arguments::{read_parameters, Parameters};
use crate::input::plays::{read_first_play, read_following_plays, PlayMode};

fn main() {
    //let a = read_parameters();
    let params = Parameters::new(15, 30, 50);

    let mut board = Board::new(&params);

    loop {
        match read_first_play() {
            Ok(mut play) => {
                board.build(play.cell());
                board.print_display();
                break;
            }
            Err(e) => println!("{}", e),
        };
    }

    loop {
        match read_following_plays() {
            Ok(mut play) => {
                match play.mode() {
                    PlayMode::Dig => {
                        let end = match board.dig(play.cell()) {
                            GameResult::Won => {
                                println!("You won");
                                true
                            }
                            GameResult::Lost => {
                                println!("You lost");
                                true
                            }
                            GameResult::None => false,
                        };

                        if end {
                            break;
                        }
                    }
                    PlayMode::Mark => board.mark(play.cell()),
                };

                board.print_display();
            }
            Err(e) => println!("{}", e),
        };
    }

    board.print_board();
}
