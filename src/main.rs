mod board;
use std::io;
use board::Board;

fn main() {
    let board = Board::new();

    println!("{}", board.print());
    println!("\nPlayer 1, enter a number between 1 and 9 to make your move.");

    let stdin = io::stdin();
    let mut line = String::new(); 
    stdin.read_line(&mut line).unwrap();
    line.pop();
    
    let number_given: i32 = line.parse().unwrap();
    if number_given > 9 || number_given < 0 {
        panic!("OH DEAR GOD WHAT HAVE YOU DONE");
    }
}