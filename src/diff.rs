use crate::spec::ContractSpec;
use stellar_xdr::curr::{ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef};

/// Severity of a detected issue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Critical,
    Warning,
    Info,
}

/// A single finding from the comparison analysis.
#[derive(Debug, Clone)]
pub struct Finding {
    pub severity: Severity,
    pub category: String,
    pub message: String,
}

/// Holds all findings from a comparison of two contract specs.
#[derive(Debug, Default)]
pub struct DiffReport {
    pub findings: Vec<Finding>,
}

impl DiffReport {
    pub fn critical_count(&self) -> usize {
        self.findings.iter().filter(|f| f.severity == Severity::Critical).count()
    }

    pub fn warning_count(&self) -> usize {
        self.findings.iter().filter(|f| f.severity == Severity::Warning).count()
    }

    pub fn info_count(&self) -> usize {
        self.findings.iter().filter(|f| f.severity == Severity::Info).count()
    }
}

/// Compare two contract specs and return a report of all findings.
pub fn compare(old: &ContractSpec, new: &ContractSpec) -> DiffReport {
    let mut report = DiffReport::default();

    compare_functions(old, new, &mut report);

    report
}

/// Compare function signatures between old and new contract specs.
fn compare_functions(old: &ContractSpec, new: &ContractSpec, report: &mut DiffReport) {
    // Check for removed or changed functions
    for (name, old_fn) in &old.functions {
        match new.functions.get(name) {
            None => {
                report.findings.push(Finding {
                    severity: Severity::Critical,
                    category: "Function Removed".to_string(),
                    message: format!(
                        "Function '{}' was removed. Existing callers will break.",
                        name
                    ),
                });
            }
            Some(new_fn) => {
                check_function_signature(name, old_fn, new_fn, report);
            }
        }
    }

    // Check for newly added functions (informational)
    for name in new.functions.keys() {
        if !old.functions.contains_key(name) {
            report.findings.push(Finding {
                severity: Severity::Info,
                category: "Function Added".to_string(),
                message: format!("New function '{}' added.", name),
            });
        }
    }
}

/// Compare signatures of two functions with the same name.
fn check_function_signature(
    name: &str,
    old_fn: &ScSpecFunctionV0,
    new_fn: &ScSpecFunctionV0,
    report: &mut DiffReport,
) {
    // Check input count
    let old_inputs: &[ScSpecFunctionInputV0] = old_fn.inputs.as_ref();
    let new_inputs: &[ScSpecFunctionInputV0] = new_fn.inputs.as_ref();

    if old_inputs.len() != new_inputs.len() {
        report.findings.push(Finding {
            severity: Severity::Critical,
            category: "Function Signature Changed".to_string(),
            message: format!(
                "Function '{}': parameter count changed from {} to {}.",
                name,
                old_inputs.len(),
                new_inputs.len()
            ),
        });
        return; // No point comparing individual params if count differs
    }

    // Check each input parameter
    for (i, (old_input, new_input)) in old_inputs.iter().zip(new_inputs.iter()).enumerate() {
        let old_name = old_input.name.to_string();
        let new_name = new_input.name.to_string();

        if old_name != new_name {
            report.findings.push(Finding {
                severity: Severity::Warning,
                category: "Parameter Renamed".to_string(),
                message: format!(
                    "Function '{}': parameter {} renamed from '{}' to '{}'.",
                    name, i, old_name, new_name
                ),
            });
        }

        if !types_equal(&old_input.type_, &new_input.type_) {
            report.findings.push(Finding {
                severity: Severity::Critical,
                category: "Parameter Type Changed".to_string(),
                message: format!(
                    "Function '{}': parameter {} ('{}') type changed from {:?} to {:?}.",
                    name, i, old_name, old_input.type_, new_input.type_
                ),
            });
        }
    }

    // Check output types
    let old_outputs: &[ScSpecTypeDef] = old_fn.outputs.as_ref();
    let new_outputs: &[ScSpecTypeDef] = new_fn.outputs.as_ref();

    if old_outputs.len() != new_outputs.len() {
        report.findings.push(Finding {
            severity: Severity::Critical,
            category: "Return Type Changed".to_string(),
            message: format!(
                "Function '{}': return type count changed from {} to {}.",
                name,
                old_outputs.len(),
                new_outputs.len()
            ),
        });
    } else {
        for (i, (old_out, new_out)) in old_outputs.iter().zip(new_outputs.iter()).enumerate() {
            if !types_equal(old_out, new_out) {
                report.findings.push(Finding {
                    severity: Severity::Critical,
                    category: "Return Type Changed".to_string(),
                    message: format!(
                        "Function '{}': return type {} changed from {:?} to {:?}.",
                        name, i, old_out, new_out
                    ),
                });
            }
        }
    }
}

/// Compare two ScSpecTypeDef values for equality.
/// We use the PartialEq derive on the XDR types.
fn types_equal(a: &ScSpecTypeDef, b: &ScSpecTypeDef) -> bool {
    a == b
}
