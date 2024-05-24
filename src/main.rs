mod board;
use board::{Board, Square};
use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::stdout;

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
    let mut board = Board::new(chosen);
    let current_player = chosen;
    let mut x = 0;
    let mut y = 0;

    fn print_winner(stdout: &mut std::io::Stdout, winner: Square) -> crossterm::Result<()> {
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            Print("Game over! "),
            SetBackgroundColor(winner.color()),
            Print(format!(" {} ", winner.to_char())),
            ResetColor,
            Print(" wins! Press any key to exit")
        )?;
        Ok(())
    }

    loop {
        board.print(&mut stdout)?;

        execute!(
            stdout,
            cursor::MoveTo(0, (3 * 2) as u16),
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
                    if x < 3 - 1 {
                        x += 1;
                    }
                }
                KeyCode::Up => {
                    if y > 0 {
                        y -= 1;
                    }
                }
                KeyCode::Down => {
                    if y < 3 - 1 {
                        y += 1;
                    }
                }
                KeyCode::Enter => {
                    if board.is_empty(x, y) {
                        board.set_square(x, y, current_player);
                        let result = board.check_winner();
                        if result != 0 {
                            let winner: Square = match result {
                                1 => current_player,
                                -1 => current_player.opposite(),
                                _ => Square::Empty,
                            };
                            print_winner(&mut stdout, winner)?;
                            return Ok(());
                        }
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
