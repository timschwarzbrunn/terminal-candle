use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;

fn print_candle<W: Write>(screen: &mut W) {
    let candle = r#"
       . .
     .     .
   .         .
  .     )     .
  .    ⎛(⎞    .
   .  ⎛⎛⁜⎞⎞  .
      ╰╰╿╯╯
   ╭─═══╧═══─╮
   │        ┊│
   │        ┊│
   │        ┊│
   │        ┊│
   │        ┊│
   │         │
   │         │
   │         │
   │         │
╒══⎬═════════╪══╕ 
 ╲‗‗‗‗‗‗‗‗‗‗‗‗‗╱
        "#;

    // Get the size of the terminal.
    let (terminal_width, terminal_height) = termion::terminal_size().unwrap();

    // Get the size of the ascii art. Therefore go through all lines.
    let mut candle_width = 0;
    let mut candle_height = 0;

    for line in candle.lines() {
        let line_width = line.chars().count();
        if line_width > candle_width {
            candle_width = line_width;
        }
        candle_height += 1;
    }

    // Get the position of the candle within the terminal.
    let candle_pos_x = (terminal_width - candle_width as u16) / 2;
    let candle_pos_y = (terminal_height - candle_height as u16) / 2;

    // Print the candle centered to the screen.
    write!(screen, "{}", termion::cursor::Goto(1, 1)).unwrap();
    for (idx, line) in candle.lines().enumerate() {
        write!(
            screen,
            "{}{}",
            termion::cursor::Goto(candle_pos_x, candle_pos_y + idx as u16),
            line
        )
        .unwrap();
    }
    screen.flush().unwrap();
}

fn main() {
    // Initialize alternate screen in raw mode.
    let stdin = stdin();
    let mut screen = stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();
    write!(screen, "{}", termion::cursor::Hide).unwrap();
    screen.flush().unwrap();

    // Print the candle.
    print_candle(&mut screen);

    // The candle will light until q is pressed.
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _ => {}
        }
        screen.flush().unwrap();
    }
    write!(screen, "{}", termion::cursor::Show).unwrap();
}
