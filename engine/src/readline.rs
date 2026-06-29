use std::io::{Write, stdout};
use constants::{COLORS, PROMPT_DEFAULT, PROMPT_SECTION, PROMPT_SECTION_PROMPT, PROMPT_SECTION_PROMPT_COLOR_KEY};
use crossterm::{
    cursor::{MoveLeft, MoveToColumn, position}, event::{self, Event, KeyCode, KeyEventKind, KeyModifiers}, execute, style::{Color as CtColor, Stylize}, terminal::{self, Clear, ClearType},
};
use helpers::io::{get_value, read_config};

use crate::state::ShellState;

pub fn read_line(state: &mut ShellState) -> Option<String> {
    let mut out = stdout();
    let config = read_config();
    let prompt = get_value(&config, PROMPT_SECTION, PROMPT_SECTION_PROMPT).unwrap_or(PROMPT_DEFAULT);
    let prompt_color = get_value(&config, PROMPT_SECTION, PROMPT_SECTION_PROMPT_COLOR_KEY).unwrap_or("White");
    terminal::enable_raw_mode().ok()?;

    write!(out, "{}", prompt.with(color_from_name(&prompt_color)).to_string()).ok();
    out.flush().ok();

    let line = read_key_code(&mut out, state, &prompt);
    terminal::disable_raw_mode().ok()?;

    if let Some(ref l) = line {
        state.history.push(l);
    }

    line
}

fn read_key_code(out: &mut std::io::Stdout, state: &ShellState, prompt: &str) -> Option<String> {
    let mut history = state.history.clone();
    let mut buffer = String::new();
    loop {
        match event::read() {
            Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => {
                    move_cursor(out);
                    out.flush().ok();
                    break Some(buffer.clone());
                },

                KeyCode::Backspace => {
                    if buffer.pop().is_some() {
                        execute!(out, MoveLeft(1), Clear(ClearType::UntilNewLine)).ok();
                        out.flush().ok();
                    }
                }

                KeyCode::Up => {
                    if let Some(entry) = history.up(&buffer) {
                        buffer = entry.to_string();
                        redraw(out, prompt, &buffer);
                    }
                }

                KeyCode::Down => {
                    if let Some(entry) = history.down() {
                        buffer = entry.to_string();
                        redraw(out, prompt, &buffer);
                    }
                }

                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    break None;
                }

                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) && buffer.is_empty() => {
                    break None;
                }

                KeyCode::Char(c) => {
                    buffer.push(c);
                    write!(out, "{c}").ok();
                    out.flush().ok();
                }

                _ => {}
            },
            Ok(_) => {}
            Err(_) => break None,
        }
    }
}

fn redraw(out: &mut std::io::Stdout, prompt: &str, buffer: &str) {
    let (_, _row) = position().unwrap_or((0, 0));
    execute!(out, MoveToColumn(0), Clear(ClearType::CurrentLine)).ok();
    write!(out, "{prompt}{buffer}").ok();
    out.flush().ok();
    let col = prompt.chars().count() as u16 + buffer.chars().count() as u16;
    execute!(out, MoveToColumn(col)).ok();
}

fn move_cursor(out: &mut std::io::Stdout) {
    let (_, row) = position().unwrap_or((0, 0));
    let rows = terminal::size().map(|(_, h)| h).unwrap_or(0);
    if row + 1 >= rows {
        execute!(out, terminal::ScrollUp(1), MoveToColumn(0)).ok();
    } else {
        execute!(out, crossterm::cursor::MoveToNextLine(1)).ok();
    }
}

fn color_from_name(name: &str) -> CtColor {
    COLORS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, c)| *c)
        .unwrap_or(CtColor::White)
}
