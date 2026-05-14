use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use stellar_xdr::curr::ScSpecEntry;

mod loader;
mod parser;

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

    println!("\n📦 Loading and Parsing contracts...");

    // Old WASM
    let old = loader::load_wasm(&args.old_wasm)?;
    let old_meta = parser::extract_metadata(&old.bytes)?;
    print_meta_summary("Old", &old.path, old.bytes.len(), &old_meta);

    // New WASM
    let new = loader::load_wasm(&args.new_wasm)?;
    let new_meta = parser::extract_metadata(&new.bytes)?;
    print_meta_summary("New", &new.path, new.bytes.len(), &new_meta);

    println!("\n✅ Functions and Types decoded successfully.");
    println!("   Next: Implementing signature comparison logic...");

    Ok(())
}

fn print_meta_summary(label: &str, path: &str, size: usize, meta: &parser::SorobanMetadata) {
    let mut functions = 0;
    let mut structs = 0;
    let mut enums = 0;
    let mut others = 0;

    for entry in &meta.spec {
        match entry {
            ScSpecEntry::FunctionV0(_) => functions += 1,
            ScSpecEntry::UdtStructV0(_) => structs += 1,
            ScSpecEntry::UdtEnumV0(_) => enums += 1,
            _ => others += 1,
        }
    }

    println!(
        "  ✅ {}: {} ({} bytes)",
        label,
        path,
        size
    );
    println!(
        "     └─ Functions: {}, Structs: {}, Enums: {}, Others: {}",
        functions, structs, enums, others
    );
}



