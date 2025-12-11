use axiom_hive_core::constitution::{Constitution, ConstitutionError};
use axiom_hive_core::runtime_bridge::{ExternalPythonRuntime, RuntimeError, UntrustedRuntime};
use axiom_hive_core::supervisor::interceptor::{Interceptor, TokenStream};
use axiom_hive_core::supervisor::ledger::Ledger;

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
        std::process::exit(1);
    }

    // Load constitution-driven settings.
    let constitution = match Constitution::load("config/constitution.toml") {
        Ok(c) => c,
        Err(e) => {
            match e {
                ConstitutionError::Io { .. } | ConstitutionError::Parse(_) => {
                    eprintln!("Configuration error: {e}");
                }
                _ => {
                    eprintln!("Invalid constitution: {e}");
                }
            }
            std::process::exit(2);
        }
    };

    // Derive a token blacklist from policy.
    let disallowed_tokens = constitution.disallowed_tokens();

    let interceptor = Interceptor::new(&disallowed_tokens);
    let mut ledger = Ledger::new();

    // Delegate to external Python runtime (BranchManager CLI).
    let runtime = ExternalPythonRuntime::default_instance();
    let answer_text = match runtime.generate(&prompt) {
        Ok(text) => text,
        Err(e) => {
            match e {
                RuntimeError::NonZeroExit(_) => eprintln!("Runtime exited with non-zero status: {e}"),
                _ => eprintln!("Runtime failure: {e}"),
            }
            // Fallback keeps the pipeline deterministic and auditable.
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
