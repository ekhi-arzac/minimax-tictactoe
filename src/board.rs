use crossterm::{cursor, execute, style::{Color, Print, ResetColor, SetBackgroundColor}, terminal::{Clear, ClearType}};

#[derive(Copy, Clone, PartialEq)]
pub enum Square {
    Empty,
    X,
    O,
}

impl Square {
    pub fn from_char(c: char) -> Self {
        match c {
            'X' => Square::X,
            'O' => Square::O,
            _ => Square::Empty,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Square::X => 'X',
            Square::O => 'O',
            Square::Empty => ' ',
        }
    }
    pub fn opposite(self) -> Self {
        match self {
            Square::X => Square::O,
            Square::O => Square::X,
            Square::Empty => Square::Empty,
        }
    }
    pub fn color(self) -> Color {
        match self {
            Square::X => Color::Blue,
            Square::O => Color::Red,
            Square::Empty => Color::Reset,
        }
    }
}

pub struct Board {
    grid: [[Square; 3]; 3],
    player: Square,
}

impl Board {
    pub fn new(player: Square) -> Self {
        Self {
            grid: [[Square::Empty; 3]; 3],
            player
        }
    }

    pub fn print(&self, stdout: &mut std::io::Stdout) -> crossterm::Result<()> {
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        let mut i = 2;
        for row in self.grid.iter() {
            for &square in row.iter() {
                let c = square.to_char();
                let color = square.color();
                execute!(
                    stdout,
                    SetBackgroundColor(color),
                    Print(format!(" {} ", c)),
                    ResetColor,
                    Print(" ")
                )?;
            }
            execute!(stdout, Print("\n\n"), cursor::MoveTo(0, i))?;
            i += 2;
        }
        Ok(())
    }

    pub fn set_square(&mut self, x: usize, y: usize, square: Square) {
        self.grid[y][x] = square;
    }

    // minimax function to determine the best move for the machine, depth is the number of moves ahead, maximizing is true if it is the machine's turn. this function
    // returns AND places the best move for the machine


    // use for loops and field of player to determine the winner, 1 if machine is winning, -1 otherwise, 0 if no winner
    pub fn check_winner(&self) -> i32 {
        for i in 0..3 {
            if self.grid[i][0] == self.player && self.grid[i][1] == self.player && self.grid[i][2] == self.player {
                return 1;
            }
            if self.grid[0][i] == self.player && self.grid[1][i] == self.player && self.grid[2][i] == self.player {
                return 1;
            }
        }

        if self.grid[0][0] == self.player && self.grid[1][1] == self.player && self.grid[2][2] == self.player {
            return 1;
        }
        if self.grid[0][2] == self.player && self.grid[1][1] == self.player && self.grid[2][0] == self.player {
            return 1;
        }

        for i in 0..3 {
            if self.grid[i][0] != Square::Empty && self.grid[i][1] != Square::Empty && self.grid[i][2] != Square::Empty {
                return -1;
            }
            if self.grid[0][i] != Square::Empty && self.grid[1][i] != Square::Empty && self.grid[2][i] != Square::Empty {
                return -1;
            }
        }

        if self.grid[0][0] != Square::Empty && self.grid[1][1] != Square::Empty && self.grid[2][2] != Square::Empty {
            return -1;
        }
        if self.grid[0][2] != Square::Empty && self.grid[1][1] != Square::Empty && self.grid[2][0] != Square::Empty {
            return -1;
        }

        0
    }
   

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.grid[y][x] == Square::Empty
    }
}
