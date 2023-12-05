use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    // style,
    terminal::{
        self, ClearType, DisableLineWrap, EnterAlternateScreen, LeaveAlternateScreen, SetSize,
    },
    // ExecutableCommand, QueueableCommand,
};
use std::io::{self, Write};

fn main() {
    // Setup terminal
    terminal::enable_raw_mode().unwrap();
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        DisableLineWrap,
        cursor::Hide
    )
    .unwrap();

    // Set terminal size to fill the entire screen
    execute!(
        io::stdout(),
        SetSize(terminal::size().unwrap().0, terminal::size().unwrap().1)
    )
    .unwrap();

    loop {
        // Handle terminal resize events
        if event::poll(std::time::Duration::from_millis(50)).unwrap() {
            if let event::Event::Resize(_width, _height) = event::read().unwrap() {
                // Handle resize event, if needed
                // For simplicity, this example does not handle resizing
                draw_ui();
            }
        }

        // Draw UI
        draw_ui();

        // Handle user input
        if event::poll(std::time::Duration::from_millis(50)).unwrap() {
            if let event::Event::Key(KeyEvent {
                code,
                state: _,
                modifiers: _,
                kind: _,
            }) = event::read().unwrap()
            {
                match code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {
                        // Do nothing
                    }
                }
            }
        }
    }

    // Cleanup terminal
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        cursor::Show,
        terminal::EnableLineWrap
    )
    .unwrap();
}

fn draw_ui() {
    // Clear the screen
    print!("{}", terminal::Clear(ClearType::All));

    // Draw top border
    print!("{}", cursor::MoveTo(0, 0));
    print!(
        "▛{}▜",
        "▀".repeat((terminal::size().unwrap().0 - 2) as usize)
    );

    // Draw the left and right edges
    for line in 1..(terminal::size().unwrap().1 - 1) {
        print!("{}", cursor::MoveTo(0, line));
        println!(
            "▌{}▐",
            " ".repeat((terminal::size().unwrap().0 - 2) as usize)
        );
    }

    // Draw bottom border
    print!("{}", cursor::MoveTo(0, terminal::size().unwrap().1 - 1));
    print!(
        "▙{}▟",
        "▄".repeat((terminal::size().unwrap().0 - 2) as usize)
    );

    // Flush the output to the terminal
    std::io::stdout().flush().unwrap();
}
