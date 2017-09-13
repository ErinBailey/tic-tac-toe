#![allow(dead_code)]

#[derive(Debug)]
pub struct Board {

}

impl Board {
    pub fn new() -> Board {
        Board{}
    }
    pub fn print(self) -> String {
        let rows: [[Cell; 3]; 3] = [[Cell::Open, Cell::Open, Cell::Open], [Cell::Open, Cell::Open, Cell::Open], [Cell::Open, Cell::Open, Cell::Open]];
        let mut count = 0;
        rows.into_iter().map(|row| {
            row.into_iter().map(|_| {
                count += 1;
                count.to_string()
            }).collect::<Vec<String>>().join("|")
        }).collect::<Vec<String>>().join("\n-----\n")
    }
}

enum Cell {
    Open,
    X,
    O
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_can_create_a_board() {
        let board = Board::new();
        let printed_board = board.print();

        assert_eq!(printed_board, "1|2|3
-----
4|5|6
-----
7|8|9")
    } 
}


