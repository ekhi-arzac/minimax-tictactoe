use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::stdout;

const BOARD_SIZE: usize = 3;

#[derive(Copy, Clone, PartialEq)]
enum Square {
    Empty,
    X,
    O,
}

impl Square {
    fn from_char(c: char) -> Self {
        match c {
            'X' => Square::X,
            'O' => Square::O,
            _ => Square::Empty,
        }
    }

    fn to_char(self) -> char {
        match self {
            Square::X => 'X',
            Square::O => 'O',
            Square::Empty => ' ',
        }
    }

    fn color(self) -> Color {
        match self {
            Square::X => Color::Blue,
            Square::O => Color::Red,
            Square::Empty => Color::Reset,
        }
    }
}

struct Board {
    grid: [[Square; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        Self {
            grid: [[Square::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn print(&self, stdout: &mut std::io::Stdout) -> crossterm::Result<()> {
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

    fn set_square(&mut self, x: usize, y: usize, square: Square) {
        self.grid[y][x] = square;
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        self.grid[y][x] == Square::Empty
    }
}

fn main() -> crossterm::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    let options = ['X', 'O'];
    let mut selected = 0;

    loop {
        // Clear the screen
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        execute!(stdout, Print("Select player:\t"))?;
        // Display the options
        for (i, &option) in options.iter().enumerate() {
            if i == selected {
                let bg_color = match option {
                    'X' => Color::Blue,
                    'O' => Color::Red,
                    _ => Color::Reset,
                };
                execute!(
                    stdout,
                    SetBackgroundColor(bg_color),
                    Print(format!(" {} ", option)),
                    ResetColor,
                    Print("  ")
                )?;
            } else {
                execute!(stdout, Print(format!(" {}  ", option)))?;
            }
        }

        // Wait for user input
        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Left => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Right => {
                    if selected < options.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Esc | KeyCode::Char('d') if event.modifiers == KeyModifiers::CONTROL => {
                    execute!(stdout, DisableMouseCapture)?;
                    disable_raw_mode()?;
                    return Ok(());
                }
                _ => {}
            }
        }
    }

    // Confirm the selection
    let chosen = Square::from_char(options[selected]);
    let mut board = Board::new();
    let mut current_player = chosen;
    let mut x = 0;
    let mut y = 0;

    loop {
        board.print(&mut stdout)?;

        execute!(
            stdout,
            cursor::MoveTo(0, (BOARD_SIZE * 2) as u16),
            Print("Use arrow keys to move, Enter to place, Ctrl+C to exit")
        )?;
        execute!(stdout, cursor::MoveTo((x * 4) as u16, (y * 2) as u16))?;

        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Left => {
                    if x > 0 {
                        x -= 1;
                    }
                }
                KeyCode::Right => {
                    if x < BOARD_SIZE - 1 {
                        x += 1;
                    }
                }
                KeyCode::Up => {
                    if y > 0 {
                        y -= 1;
                    }
                }
                KeyCode::Down => {
                    if y < BOARD_SIZE - 1 {
                        y += 1;
                    }
                }
                KeyCode::Enter => {
                    if board.is_empty(x, y) {
                        board.set_square(x, y, current_player);
                        current_player = if current_player == Square::X {
                            Square::O
                        } else {
                            Square::X
                        };
                    }
                }
                KeyCode::Esc | KeyCode::Char('c') if event.modifiers == KeyModifiers::CONTROL => {
                    execute!(stdout, DisableMouseCapture)?;
                    disable_raw_mode()?;
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}
