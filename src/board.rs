#![allow(dead_code)]
use std::fmt;

#[derive(Debug, Default)]
pub struct Board {
    player: Player,
    rows: [[Cell; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        Default::default()
    }
    pub fn make_move(&mut self, position: usize) {
        let column = (position-1)%3;
        let row = (position-1)/3;
        self.rows[row][column] = Cell::Taken(self.player);
        self.player = self.player.opponent();
    }
}

impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut count = 0;
        let board = self.rows.into_iter().map(|row| {
            row.into_iter().map(|cell| {
                count += 1;
                match *cell {
                    Cell::Open => count.to_string(),
                    Cell::Taken(Player::X) => "X".to_owned(),
                    Cell::Taken(Player::O) => "O".to_owned(),
                }
            }).collect::<Vec<String>>().join("|")
        }).collect::<Vec<String>>().join("\n-----\n");
        write!(formatter, "{}", board)
    }
}

#[derive(Debug)]
enum Cell {
    Open,
    Taken(Player),
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::Open
    }
}

#[derive(Debug, Clone, Copy)]
enum Player {
    X,
    O
}

impl Player {
    fn opponent(&self) -> Player {
        match *self {
            Player::O => Player::X,
            Player::X => Player::O
        }
    }
}

impl Default for Player {
    fn default() -> Player {
        Player::X
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_a_board() {
        let board = Board::new();
        let printed_board = format!("{}", board);

        assert_eq!(printed_board, vec![
            "1|2|3",
            "-----",
            "4|5|6",
            "-----",
            "7|8|9",
        ].join("\n"))
    }

    #[test]
    fn it_can_make_player1_move() {
        let mut board = Board::new();
        board.make_move(3);
        let printed_board = format!("{}", board);

        assert_eq!(printed_board, vec![
            "1|2|X",
            "-----",
            "4|5|6",
            "-----",
            "7|8|9",
        ].join("\n"))
    }

    #[test]
    fn it_can_make_player2_move() {
        let mut board = Board::new();
        board.make_move(1);
        board.make_move(5);
        let printed_board = format!("{}", board);

        assert_eq!(printed_board, vec![
            "X|2|3",
            "-----",
            "4|O|6",
            "-----",
            "7|8|9",
        ].join("\n"))
    }
    #[test]
    fn it_can_make_multiple_moves() {
        let mut board = Board::new();
        board.make_move(2);
        board.make_move(6);
        board.make_move(8);
        let printed_board = format!("{}", board);

        assert_eq!(printed_board, vec![
            "1|X|3",
            "-----",
            "4|5|O",
            "-----",
            "7|X|9",
        ].join("\n"))
    }    
}


