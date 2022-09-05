use std::io;
use anyhow::anyhow;
use crate::board::Board;

pub fn run(board: Board) -> Result<(), anyhow::Error> {
    let mut lines = io::stdin().lines();

    println!("Initial board state:");
    print!("{board}");
    loop {
        println!("Enter a number from 1 - 9");
        let num: usize = lines.next()
            .ok_or_else(|| anyhow!("failed to get line"))??
            .trim().parse()?;
    }
}
