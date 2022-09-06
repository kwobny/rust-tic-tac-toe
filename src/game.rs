use std::io::{self, Lines, StdinLock, Stdin};
use anyhow::anyhow;
use crate::board::{Board, Player, EndResult};

/// This struct is a wrapper to conveniently get lines from stdin.
#[derive(Debug)]
struct StdinLines {
    lines: Lines<StdinLock<'static>>,
}
impl StdinLines {
    fn new(stdin: Stdin) -> StdinLines {
        StdinLines {
            lines: stdin.lines(),
        }
    }
    fn next_line(&mut self) -> Result<String, anyhow::Error> {
        let line = self.lines.next()
            .ok_or_else(|| anyhow!("failed to get line"))??;
        Ok(line)
    }
}

/// This function runs/drives the tic tac toe game.
pub fn run(first_player: Player) -> Result<(), anyhow::Error> {
    // Line getter.
    let mut lines = StdinLines::new(io::stdin());

    // Enclose the whole game in a loop to allow to restart the game.
    'whole_game: loop {
        // Create a new board with width of 3 cells,
        // and create a turn variable that stores the current player (X/O),
        // and set it to the first player.
        let mut board = Board::new(3).unwrap();
        let mut turn = first_player;

        // Print the initial board before playing.
        println!("Initial board state:");
        println!();
        print!("{board}");
        println!();

        loop {
            // Prompt the user for a number that represents where
            // to place the piece.
            println!("Current turn: player {turn}");
            println!("Enter a number from 1 - 9:");
            loop {
                // Get a number from stdin.
                let num = lines.next_line()?;
                let num: Option<usize> = match num.trim().parse() {
                    Err(_) => None,
                    Ok(x) => if (1..=9).contains(&x) {
                        Some(x)
                    } else {
                        None
                    },
                };
                // Check if the number is between 1 - 9.
                if num.is_none() {
                    println!("Invalid input.");
                    continue;
                }
                let num = num.unwrap() - 1;

                // Place the piece down on the board.
                let res = board.set_position(turn, match board.position_from_index(num) {
                    Err(_) => panic!(),
                    Ok(x) => x,
                });
                if res.is_err() {
                    println!("That position is already taken.");
                    continue;
                }

                break;
            }

            // Print the current state of the board.
            println!();
            print!("{board}");
            println!();

            // Check if the game has ended.
            let possible_end = board.potential_end_result();
            if let Some(ending) = possible_end {
                // Print the result of the game.
                match ending {
                    EndResult::Tie => println!("Game ends in a tie!"),
                    EndResult::Win(player) => println!("Player {player} wins!"),
                    EndResult::BothWin => panic!(),
                }
                // Ask if the user wants to play again.
                // If yes, restart the game.
                // If no, return from this function.
                println!("Play again? (Y/N):");
                loop {
                    let reply = lines.next_line()?;
                    match reply.trim() {
                        "N" => return Ok(()),
                        "Y" => {
                            println!();
                            continue 'whole_game;
                        },
                        _ => {
                            println!();
                            println!("Invalid response.");
                            continue;
                        },
                    }
                }
            }

            // Toggle the current player.
            turn.toggle_in_place();
        }
    }
}
