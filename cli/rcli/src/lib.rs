mod cli;
mod process;

pub use cli::{Opts, SubCommand};
pub use process::{read_csv, write_data, process_genpass};