use std::io::{Write, stdout};

use crossterm::{
    cursor::{MoveLeft, MoveToColumn, position},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
};

use crate::state::ShellState;

pub fn read_line(state: &mut ShellState) -> Option<String> {
    let mut out = stdout();
    terminal::enable_raw_mode().ok()?;

    write!(out, "{}", state.prompt).ok();
    out.flush().ok();

    let line = read_key_code(&mut out, state);
    execute!(out, crossterm::cursor::MoveToNextLine(1)).ok();
    terminal::disable_raw_mode().ok()?;

    if let Some(ref l) = line {
        state.history.push(l);
    }

    line
}

fn read_key_code(out: &mut std::io::Stdout, state: &ShellState) -> Option<String> {
    let prompt = state.prompt.clone();
    let mut history = state.history.clone();
    let mut buffer = String::new();
    loop {
        match event::read() {
            Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => break Some(buffer.to_string()),

                KeyCode::Backspace => {
                    buffer.pop();
                    execute!(out, MoveLeft(1), Clear(ClearType::UntilNewLine)).ok();
                    out.flush().ok();
                }

                KeyCode::Up => {
                    if let Some(entry) = history.up(&buffer) {
                        buffer = entry.to_string();
                        redraw(out, prompt.as_str(), &buffer);
                    }
                }

                KeyCode::Down => {
                    if let Some(entry) = history.down() {
                        buffer = entry.to_string();
                        redraw(out, prompt.as_str(), &buffer);
                    }
                }

                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    break None;
                }

                KeyCode::Char('d') if buffer.is_empty() => {
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
