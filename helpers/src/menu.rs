use std::io::{Stdout, Write, stdout};

use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show, position},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color as CtColor, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, ScrollUp},
};

pub struct MenuItem {
    pub label: String,
    pub color: Option<CtColor>,
}

impl MenuItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            color: None,
        }
    }

    pub fn colored(label: impl Into<String>, color: CtColor) -> Self {
        Self {
            label: label.into(),
            color: Some(color),
        }
    }
}

pub fn show_menu(items: &[MenuItem]) -> Option<usize> {
    if items.is_empty() {
        return None;
    }

    let mut out = stdout();
    terminal::enable_raw_mode().expect("failed to enable raw mode");
    execute!(out, Hide).ok();

    let (_, mut start_row) = position().unwrap_or((0, 0));
    let (_, height) = terminal::size().unwrap_or((80, 24));

    let needed_rows = items.len() as u16;

    if start_row + needed_rows >= height {
        let scroll = start_row + needed_rows - height + 1;

        execute!(out, ScrollUp(scroll)).ok();

        start_row = start_row.saturating_sub(scroll);
    }

    let mut selected = 0usize;
    let result = loop {
        draw_menu(&mut out, start_row, items, selected);

        match event::read() {
            Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Up => {
                    selected = if selected == 0 {
                        items.len() - 1
                    } else {
                        selected - 1
                    };
                }
                KeyCode::Down => {
                    selected = (selected + 1) % items.len();
                }
                KeyCode::Enter => break Some(selected),
                KeyCode::Esc | KeyCode::Char('q') => break None,
                _ => {}
            },
            Ok(_) => {}
            Err(_) => break None,
        }
    };

    cleanup(&mut out, start_row, items.len());
    terminal::disable_raw_mode().expect("failed to disable raw mode");

    result
}

fn draw_menu(out: &mut Stdout, start_row: u16, items: &[MenuItem], selected: usize) {
    execute!(out, MoveTo(0, start_row)).ok();
    for (i, item) in items.iter().enumerate() {
        execute!(out, Clear(ClearType::CurrentLine)).ok();
        let marker = if i == selected { ">" } else { " " };

        if let Some(color) = item.color {
            execute!(out, SetForegroundColor(color)).ok();
        }
        execute!(out, Print(format!("{marker} {}", item.label))).ok();
        if item.color.is_some() {
            execute!(out, ResetColor).ok();
        }
        execute!(out, MoveToNextLine(1)).ok();
    }
    out.flush().ok();
}

fn cleanup(out: &mut Stdout, start_row: u16, count: usize) {
    execute!(out, MoveTo(0, start_row)).ok();
    for _ in 0..count {
        execute!(out, Clear(ClearType::CurrentLine), MoveToNextLine(1)).ok();
    }
    execute!(out, MoveTo(0, start_row), Show).ok();
}
