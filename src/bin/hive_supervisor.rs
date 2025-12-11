use axiom_hive_core::constitution::Constitution;
use axiom_hive_core::runtime_bridge::{ExternalPythonRuntime, RuntimeError, UntrustedRuntime};
use axiom_hive_core::supervisor::interceptor::{Interceptor, TokenStream};
use axiom_hive_core::supervisor::ledger::Ledger;
use std::path::Path;

fn main() {
    // 1) Load constitution
    let path = Path::new("config/constitution.toml");
    let constitution = Constitution::load(path).expect("failed to load constitution");

    println!(
        "Loaded constitution: {} (v{}), max_branches={}, min_consensus_ratio={}",
        constitution.meta.name,
        constitution.meta.version,
        constitution.supervisor.max_branches,
        constitution.supervisor.min_consensus_ratio
    );

    // 2) Configure interceptor from policy.
    let disallowed_tokens = constitution.disallowed_tokens();

    let interceptor = Interceptor::new(&disallowed_tokens);

    // 3) Delegate to the Python runtime to generate a best-of-N answer.
    let runtime = ExternalPythonRuntime::default_instance();
    let prompt = "demo_prompt: what is the Axiom Hive Core?";
    let answer_text = match runtime.generate(prompt) {
        Ok(text) => text,
        Err(e) => {
            match e {
                RuntimeError::NonZeroExit(_) => eprintln!("Runtime exited with non-zero status: {e}"),
                _ => eprintln!("Runtime failure: {e}"),
            }
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
