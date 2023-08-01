///
/// Generate contracts from json artifacts and output as a module.
/// We'll add support for crate generation, once revmup is published to crates.io
///
use clap::{Parser, ValueHint};
use std::path::PathBuf;

use revmup_abigen::multi::MultiAbigen;

// @todo add support for crates once revmup is published
//const DEFAULT_CRATE_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, Parser)]
#[command(
    about = "Generate contract bindings for revm",
    long_about = None
)]
pub struct BindArgs {
    /// Input path for contract artifacts/json files.
    #[clap(
        long = "input-path",

        short,
        value_hint = ValueHint::DirPath,
        value_name = "PATH"
    )]
    pub input_path: PathBuf,

    /// Output path for generated code
    #[clap(
        long = "output-path",
        short,
        value_hint = ValueHint::DirPath,
        value_name = "PATH"
    )]
    pub output_path: PathBuf,
    // Generate the bindings as a module
    //#[clap(long)]
    //module: bool,
}

fn main() -> eyre::Result<()> {
    let args = BindArgs::parse();

    let gen = MultiAbigen::from_json_files(args.input_path)?;
    let total = gen.len();
    println!("... generating {:} contract(s)", total);
    let bindings = gen.build()?;
    bindings.write_to_module(args.output_path, false)?;

    /*
    if args.module {
        bindings.write_to_module(args.output_path, false)?;
    } else {
        let cn = args.output_path.file_stem().ok_or(eyre::eyre!(
            "Cannot generate output.  Does the directory already exist?"
        ))?;

        bindings
            .write_to_crate(
                cn.to_str().unwrap(),
                DEFAULT_CRATE_VERSION,
                args.output_path.clone(),
                false,
            )
            .map_err(|_| {
                eyre::eyre!("Cannot generate output.  Does the directory already exist?")
            })?;
    }
    */
    Ok(())
}
