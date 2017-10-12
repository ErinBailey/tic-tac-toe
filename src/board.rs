#![allow(dead_code)]
use std::fmt;

#[derive(Debug, Default)]
pub struct Board {
    player: Player,
    rows: [[Option<Player>; 3]; 3]
}
// [[None, None, None],
//  [None, None, None],
//  [None, None, None]]

impl Board {
    pub fn new() -> Board {
        Default::default()
    }
    pub fn make_move(&mut self, position: usize) {
        let column = (position-1)%3;
        let row = (position-1)/3;
        self.rows[row][column] = Some(self.player);
        self.player = self.player.opponent();
    }
    pub fn calculate_winner(&self) -> Option<Player> {
        for row in self.rows.iter() {
            if row.iter().all(|x| x == &Some(Player::X)) {
                return Some(Player::X);
            }
        }
        None
    }

}

impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut count = 0;
        let board = self.rows.into_iter().map(|row| {
            row.into_iter().map(|cell| {
                count += 1;
                match *cell {
                    None => count.to_string(),
                    Some(Player::X) => "X".to_owned(),
                    Some(Player::O) => "O".to_owned(),
                }
            }).collect::<Vec<String>>().join("|")
        }).collect::<Vec<String>>().join("\n-----\n");
        write!(formatter, "{}", board)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O
}

impl Player {
    fn opponent(self) -> Player {
        match self {
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

    #[test]
    fn there_is_no_winner_when_no_moves_are_made() {
        let board = Board::new();
        assert_eq!(board.calculate_winner(), None)
    }

    #[test]
    fn there_is_no_winner_mid_game() {
        let mut board = Board::new();
        board.make_move(1);
        board.make_move(2);
        board.make_move(3);
        board.make_move(4);
        board.make_move(5);
        assert_eq!(board.calculate_winner(), None);
    }

    #[test]
    fn player_x_wins_horizontally_1_2_3() {
        let mut board = Board::new();
        board.make_move(1);
        board.make_move(5);
        board.make_move(2);
        board.make_move(7);
        board.make_move(3);
        assert_eq!(board.calculate_winner(), Some(Player::X));
    }

    #[test]
    fn player_x_wins_horizontally_4_5_6() {
        let mut board = Board::new();
        board.make_move(4);
        board.make_move(8);
        board.make_move(5);
        board.make_move(7);
        board.make_move(6);
        assert_eq!(board.calculate_winner(), Some(Player::X));
    }

    #[test]
    fn player_x_wins_horizontally_7_8_9() {
        let mut board = Board::new();
        board.make_move(7);
        board.make_move(3);
        board.make_move(8);
        board.make_move(5);
        board.make_move(9);
        assert_eq!(board.calculate_winner(), Some(Player::X));
    }
}


