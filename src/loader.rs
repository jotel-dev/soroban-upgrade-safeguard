use anyhow::{bail, Context, Result};
use std::path::Path;
use wasmparser::{Parser, Payload};

/// Holds raw WASM bytes alongside the validated file path.
#[derive(Debug)]
pub struct WasmModule {
    pub path: String,
    pub bytes: Vec<u8>,
}

/// Reads a WASM file from disk, validates it is a valid WASM binary,
/// and returns a `WasmModule` ready for further analysis.
pub fn load_wasm(path: &Path) -> Result<WasmModule> {
    // 1. Check the file exists
    if !path.exists() {
        bail!("File not found: {}", path.display());
    }

    // 2. Read all bytes into memory
    let bytes =
        std::fs::read(path).with_context(|| format!("Failed to read file: {}", path.display()))?;

    // 3. Validate the WASM magic header (0x00 0x61 0x73 0x6d)
    if bytes.len() < 4 || &bytes[0..4] != b"\0asm" {
        bail!(
            "'{}' does not appear to be a valid WASM binary (bad magic bytes)",
            path.display()
        );
    }

    // 4. Do a full structural parse to detect any deeper format errors
    validate_wasm_structure(&bytes)
        .with_context(|| format!("WASM validation failed for '{}'", path.display()))?;

    Ok(WasmModule {
        path: path.to_string_lossy().into_owned(),
        bytes,
    })
}

/// Iterates through all WASM payloads and fails fast on any parse error.
fn validate_wasm_structure(bytes: &[u8]) -> Result<()> {
    let parser = Parser::new(0);
    for payload in parser.parse_all(bytes) {
        match payload.context("Malformed WASM payload encountered")? {
            // We just want to iterate; real analysis happens in later modules
            Payload::Version { .. } => {}
            Payload::TypeSection(_) => {}
            Payload::FunctionSection(_) => {}
            Payload::TableSection(_) => {}
            Payload::MemorySection(_) => {}
            Payload::GlobalSection(_) => {}
            Payload::ExportSection(_) => {}
            Payload::ImportSection(_) => {}
            Payload::ElementSection(_) => {}
            Payload::DataSection(_) => {}
            Payload::CodeSectionStart { .. } => {}
            Payload::CodeSectionEntry(_) => {}
            Payload::CustomSection(_) => {}
            Payload::End(_) => {}
            _ => {}
        }
    }
    Ok(())
}
