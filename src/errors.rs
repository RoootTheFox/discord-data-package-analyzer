#[derive(thiserror::Error, Debug)]
pub enum ErrorThingy {
    #[error("Error: {0}")]
    Meow(String),
}
