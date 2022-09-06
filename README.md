# Rust Tic Tac Toe

This is a command line interactive tic tac toe game programmed in Rust.
Run it by running `cargo run --release`.

## How to play

The game will start off at an empty board, and player X goes first.
It then prompts you to enter a number from 1 - 9. This number represents
where on the board to place down the X piece. The precise mapping is shown below.

 1 | 2 | 3
---+---+---
 4 | 5 | 6
---+---+---
 7 | 8 | 9

After the piece is placed, the game switches to player O and then asks
for another number from 1 - 9. This time, the number represents where to
place the O. The game continues to alternate between X and O, until the
game ends in a win or a tie. When this happens, the game will announce
the result, and ask you whether you want to play again or not. If you
choose to play again, the board will be reset and player X will be first
again.

## How it works

This game is programmed in 3 modules, a board module, a render module,
and a game module.
The board module handles the logic of the board itself, the render module
handles displaying the board on screen, and the game module handles running
the game. The entry point of the program is `main` in `src/main.rs`,
which calls `run` in `src/lib.rs`, which calls `run` in `src/game.rs`.
`src/game.rs` is the effective entry point of the game itself.
