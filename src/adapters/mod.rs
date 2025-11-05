pub mod cli;
pub mod teos10;

use crate::error::AppError;
use clap::Parser;

pub fn run() -> Result<(), AppError> {
    use crate::adapters::cli::{Args, parse_inputs};
    use crate::salinity::calculator::compute_summary;

    let args = Args::parse();
    let (base_inp, ass) = parse_inputs(&args)?;

    let out = compute_summary(&base_inp, &ass);

    crate::adapters::cli::print_output(&out, &args)?;

    Ok(())
}
