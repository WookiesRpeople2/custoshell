use crate::history::History;

#[derive(Debug, Clone)]
pub struct ShellState {
    pub history: History,
}

impl ShellState {
    pub fn new() -> Self {
        Self {
            history: History::default(),
        }
    }
}
