use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the previous (on-chain) WASM contract
    #[arg(value_name = "OLD_WASM")]
    old_wasm: PathBuf,

    /// Path to the new (to be deployed) WASM contract
    #[arg(value_name = "NEW_WASM")]
    new_wasm: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("Comparing Soroban contracts:");
    println!("  Old: {:?}", args.old_wasm);
    println!("  New: {:?}", args.new_wasm);

    // TODO: Implement comparison logic in subsequent milestones
}

