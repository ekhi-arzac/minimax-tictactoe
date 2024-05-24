use core::fmt;

#[derive(Copy, Clone)]
pub enum Square {
    Empty = 0,
    X = 1,
    O = 2,
}
impl Square {
    pub fn from_char(c: char) -> Square {
        match c {
            'X' => Square::X,
            'O' => Square::O,
            _ => Square::Empty,
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Square::Empty => write!(f, "_"),
            Square::X => write!(f, "X"),
            Square::O => write!(f, "O"),
        }
    }
}

pub struct Board {
    board: [Square; 9],
    player: Square,
}

impl Board {
    pub fn new(player: Square) -> Board {
        Board {
            board: [Square::Empty; 9],
            player,
        }
    }
    pub fn place(&mut self, x: usize, y: usize) -> Result<(), String> {
        if x > 2 || y > 2 {
            return Err(String::from(""));
        }
        let index = 3 * x + y;
        match self.board[x + y % 3] {
            Square::Empty => {
                self.board[index] = self.player;
                Ok(())
            }
            Square::X | Square::O => Err(String::from("")),
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut res: String = "".to_owned();
        for i in 0..3 {
            res.push_str(
                &format!(
                    "{} | {} | {}\n",
                    self.board[i * 3],
                    self.board[i * 3 + 1],
                    self.board[i * 3 + 2]
                )
                .to_string(),
            );
        }
        write!(fmt, "{}", res)
    }
}
