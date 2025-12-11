use axiom_hive_core::runtime_bridge::ExternalPythonRuntime;
use axiom_hive_core::supervisor::interceptor::{Interceptor, TokenStream};
use axiom_hive_core::supervisor::ledger::Ledger;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Meta {
    name: String,
    version: u32,
}

#[derive(Debug, Deserialize)]
struct Policy {
    allow_harmful_content: bool,
    allow_pii_leakage: bool,
}

#[derive(Debug, Deserialize)]
struct SupervisorCfg {
    max_branches: u32,
    min_consensus_ratio: f32,
}

#[derive(Debug, Deserialize)]
struct Constitution {
    meta: Meta,
    policy: Policy,
    supervisor: SupervisorCfg,
}

fn load_constitution(path: &Path) -> Constitution {
    let raw = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read constitution from {}: {e}", path.display()));
    toml::from_str(&raw).expect("failed to parse constitution.toml")
}

fn main() {
    // 1) Load constitution
    let path = Path::new("config/constitution.toml");
    let constitution = load_constitution(path);

    println!(
        "Loaded constitution: {} (v{}), max_branches={}, min_consensus_ratio={}",
        constitution.meta.name,
        constitution.meta.version,
        constitution.supervisor.max_branches,
        constitution.supervisor.min_consensus_ratio
    );

    // 2) Configure interceptor from policy (very simple mapping for now)
    // If PII leakage is not allowed, we "ban" some toy tokens that represent PII.
    let disallowed_tokens: Vec<&str> = if constitution.policy.allow_pii_leakage {
        Vec::new()
    } else {
        vec!["email", "ssn", "phone"]
    };

    let interceptor = Interceptor::new(&disallowed_tokens);

    // 3) Delegate to the Python runtime to generate a best-of-N answer.
    let runtime = ExternalPythonRuntime::default_instance();
    let prompt = "demo_prompt: what is the Axiom Hive Core?";
    let answer_text = match runtime.generate(prompt) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Python runtime failed ({e}), falling back to local stub.");
            "Fallback answer for demo.".to_string()
        }
    };

    let tokens: Vec<String> = answer_text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let stream = TokenStream { tokens: &tokens };

    let filtered = interceptor.filter(stream);

    println!("Prompt          : {}", prompt);
    println!("Runtime answer  : {}", answer_text);
    println!("Original tokens : {:?}", tokens);
    println!("Filtered tokens : {:?}", filtered);

    // 4) Append an entry to the ledger capturing this supervised interaction.
    let mut ledger = Ledger::new();
    let joined_filtered = filtered.join(" ");
    let entry = ledger.append(prompt, &joined_filtered);

    println!(
        "Ledger entry => index={}, hash={}, prev_hash={}, input_hash={}, output_hash={}",
        entry.index, entry.hash, entry.prev_hash, entry.input_hash, entry.output_hash
    );
}
