pub type Result<T> = std::result::Result<T, Error>;

mod errors;
pub use errors::Error;

mod cli;
pub use cli::Cli;
