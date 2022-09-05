use std::io::{self, Lines, StdinLock, Stdin};
use anyhow::anyhow;
use crate::board::{Board, Player, WinKind};

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

pub fn run(first_player: Player) -> Result<(), anyhow::Error> {
    let mut lines = StdinLines::new(io::stdin());

    'whole_game: loop {
        let mut board = Board::new(3).unwrap();
        let mut turn = first_player;

        println!("Initial board state:");
        println!();
        print!("{board}");
        println!();

        loop {
            println!("Current turn: player {turn}");
            println!("Enter a number from 1 - 9:");
            loop {
                let num: usize = lines.next_line()?
                    .trim().parse()?;
                if ! (1..=9).contains(&num) {
                    println!("Number is out of bounds.");
                    continue;
                }
                let num = num-1;

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

            println!();
            print!("{board}");
            println!();

            let possible_end = board.winner();
            if let Some(ending) = possible_end {
                match ending {
                    WinKind::Tie => println!("Game ends in a tie!"),
                    WinKind::Win(player) => println!("Player {player} wins!"),
                }
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

            turn.toggle_in_place();
        }
    }
}
