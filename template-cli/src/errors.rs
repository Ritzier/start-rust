use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("ColorEyre: {0:?}")]
    ColorEyre(#[from] color_eyre::Report),
}
