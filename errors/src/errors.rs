use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellErrors {
    #[error("Command not found {0}")]
    CommandNotFound(String),
    #[error("This command {0} requires an argument")]
    InvalidCommand(String),

    #[error(transparent)]
    Io(#[from] tokio::io::Error),
}

pub type ShellErrorResault<T> = anyhow::Result<T>;
