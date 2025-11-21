#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "approx_ct")]
pub mod manual_ct;
pub mod teos10;
#[cfg(not(feature = "approx_ct"))]
pub mod teos10_reduced;

#[cfg(feature = "cli")]
use clap::Parser;

#[cfg(feature = "cli")]
pub fn run() -> Result<(), crate::error::AppError> {
    use crate::adapters::cli::{Args, parse_inputs};
    use crate::salinity::calculator::compute_summary;

    let args = Args::parse();
    let (base_inp, ass) = parse_inputs(&args)?;

    let out = compute_summary(&base_inp, &ass);

    crate::adapters::cli::print_output(&out, &args)?;

    Ok(())
}
