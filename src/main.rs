use rand::Rng;
use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;

const CANDLE: &'static str = r#"
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
 ╲‗‗‗‗‗‗‗‗‗‗‗‗‗╱"#;

fn get_candle_position() -> (u16, u16) {
    // Get the size of the terminal.
    let (terminal_width, terminal_height) = termion::terminal_size().unwrap();

    // Get the size of the ascii art. Therefore go through all lines.
    let mut candle_width = 0;
    let mut candle_height = 0;

    for line in CANDLE.lines() {
        let line_width = line.chars().count();
        if line_width > candle_width {
            candle_width = line_width;
        }
        candle_height += 1;
    }

    // Get the position of the candle within the terminal.
    let candle_pos_x = (terminal_width - candle_width as u16) / 2;
    let candle_pos_y = (terminal_height - candle_height as u16) / 2;

    return (candle_pos_x, candle_pos_y);
}

fn print_candle<W: Write>(screen: &mut W) {
    // Print the candle centered to the screen.
    let (candle_pos_x, candle_pos_y) = get_candle_position();
    for (idx, line) in CANDLE.lines().enumerate() {
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

fn get_updated_candle(candle: &String, probability_flare: f64) -> String {
    let mut rng = rand::thread_rng();
    let mut updated_candle = String::new();

    // Go through all lines and update the candle flare and the flame.
    for line in candle.lines() {
        let updated_line: String = line
            .chars()
            .map(|c| {
                if c == '.' && rng.gen::<f64>() < probability_flare {
                    '*'
                } else if c == '*' {
                    '.'
                } else if c == ')' {
                    '('
                } else if c == '(' {
                    ')'
                } else {
                    c
                }
            })
            .collect();
        updated_candle.push_str(&updated_line);
        updated_candle.push('\n');
    }

    return updated_candle;
}

fn update_candle<W: Write>(screen: &mut W, candle_old: &String, candle_new: &String) {
    let (candle_pos_x, candle_pos_y) = get_candle_position();

    let lines_old = candle_old.lines();
    let lines_new = candle_new.lines();

    for (row, (line_old, line_new)) in lines_old.zip(lines_new).enumerate() {
        for (col, (char_old, char_new)) in line_old.chars().zip(line_new.chars()).enumerate() {
            if char_old != char_new {
                write!(
                    screen,
                    "{}{}",
                    termion::cursor::Goto(candle_pos_x + col as u16, candle_pos_y + row as u16),
                    char_new
                )
                .unwrap();
            }
        }
    }
    screen.flush().unwrap();
}

fn main() {
    // Initialize alternate screen in raw mode.
    let mut screen = stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();
    write!(screen, "{}", termion::cursor::Hide).unwrap();
    screen.flush().unwrap();

    // Print the initial candle.
    // The following times it will only get updated partially.
    print_candle(&mut screen);
    let mut candle_old = String::from(CANDLE);

    'main_loop: loop {
        // Update the candle.
        let candle_new = get_updated_candle(&candle_old, 0.1);
        update_candle(&mut screen, &candle_old, &candle_new);
        candle_old = candle_new;

        // How to handle user input to quit the program?

        // Wait for a little bit until the next update.
        thread::sleep(Duration::from_secs(1));
    }

    // Clean up.
    write!(screen, "{}", termion::cursor::Show).unwrap();
}
