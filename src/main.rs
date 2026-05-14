use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod loader;

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

fn main() -> Result<()> {
    let args = Args::parse();

    println!("🔍 Soroban Upgrade Safeguard");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("\n📦 Loading contracts...");

    let old = loader::load_wasm(&args.old_wasm)?;
    println!("  ✅ Old: {} ({} bytes)", old.path, old.bytes.len());

    let new = loader::load_wasm(&args.new_wasm)?;
    println!("  ✅ New: {} ({} bytes)", new.path, new.bytes.len());

    println!("\n✅ Both WASM modules loaded and validated successfully.");
    println!("   Analysis coming in subsequent milestones.");

    Ok(())
}

