use std::fs;
use std::path::Path;

use axiom_hive_core::runtime_bridge::ExternalPythonRuntime;
use axiom_hive_core::supervisor::interceptor::{Interceptor, TokenStream};
use axiom_hive_core::supervisor::ledger::Ledger;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Constitution {
    #[serde(default)]
    policy: Policy,
    #[serde(default)]
    supervisor: SupervisorCfg,
}

#[derive(Debug, Deserialize, Default)]
struct Policy {
    #[serde(default)]
    allow_harmful_content: bool,
    #[serde(default)]
    allow_pii_leakage: bool,
}

#[derive(Debug, Deserialize)]
struct SupervisorCfg {
    #[serde(default = "default_max_branches")]    
    max_branches: u32,
    #[serde(default = "default_min_consensus_ratio")]    
    min_consensus_ratio: f32,
}

impl Default for SupervisorCfg {
    fn default() -> Self {
        Self { max_branches: default_max_branches(), min_consensus_ratio: default_min_consensus_ratio() }
    }
}

fn default_max_branches() -> u32 { 4 }
fn default_min_consensus_ratio() -> f32 { 0.75 }

fn load_constitution<P: AsRef<Path>>(path: P) -> Constitution {
    let text = fs::read_to_string(path).expect("failed to read constitution.toml");
    toml::from_str(&text).expect("invalid constitution.toml")
}

fn main() {
    // Simple CLI: read prompt from args or stdin.
    let prompt = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let prompt = if prompt.is_empty() {
        eprintln!("Enter prompt, then Ctrl+D (Unix) or Ctrl+Z (Windows) to finish:");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("failed to read stdin");
        buf.trim().to_string()
    } else {
        prompt
    };

    if prompt.is_empty() {
        eprintln!("No prompt provided.");
        return;
    }

    // Load constitution-driven settings.
    let constitution = load_constitution("config/constitution.toml");

    // For demo, we derive a token blacklist from policy (very simplistic).
    let mut disallowed_tokens: Vec<&str> = vec![];
    if !constitution.policy.allow_harmful_content {
        disallowed_tokens.push("kill");
        disallowed_tokens.push("attack");
    }
    if !constitution.policy.allow_pii_leakage {
        disallowed_tokens.push("ssn");
    }

    let interceptor = Interceptor::new(&disallowed_tokens);
    let mut ledger = Ledger::new();

    // Delegate to external Python runtime (BranchManager CLI).
    let runtime = ExternalPythonRuntime::default_instance();
    let answer_text = match runtime.generate(&prompt) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Python runtime failed ({e}), falling back to local stub.");
            format!("[fallback] {prompt}")
        }
    };

    // Token-level interception.
    let tokens: Vec<String> = answer_text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let filtered_tokens = interceptor.filter(TokenStream { tokens: &tokens });
    let supervised_output = filtered_tokens.join(" ");

    // Append to ledger.
    let entry = ledger.append(&prompt, &supervised_output);

    println!("Supervised answer:\n{}\n", supervised_output);
    println!("Ledger entry:");
    println!("  index: {}", entry.index);
    println!("  input_hash: {}", entry.input_hash);
    println!("  output_hash: {}", entry.output_hash);
    println!("  prev_hash: {}", entry.prev_hash);
    println!("  hash: {}", entry.hash);
}
