use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

/// Simple bridge that calls the Python BranchManager CLI.
///
/// It invokes a `python` binary (or `$PYTHON_BIN` if set) with
/// `src/runtime/cli.py`, writes the prompt to stdin, and reads the
/// chosen text from stdout.
pub struct ExternalPythonRuntime {
    python_bin: String,
}

impl ExternalPythonRuntime {
    pub fn new() -> Self {
        let python_bin = env::var("PYTHON_BIN").unwrap_or_else(|_| "python".to_string());
        Self { python_bin }
    }

    /// Convenience constructor using defaults.
    pub fn default_instance() -> Self {
        Self::new()
    }

    /// Generate text for a prompt by delegating to the Python runtime.
    pub fn generate(&self, prompt: &str) -> Result<String, String> {
        let mut child = Command::new(&self.python_bin)
            .arg("src/runtime/cli.py")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| format!("failed to spawn python runtime: {e}"))?;

        {
            let stdin = child
                .stdin
                .as_mut()
                .ok_or_else(|| "failed to open stdin for python runtime".to_string())?;
            stdin
                .write_all(prompt.as_bytes())
                .map_err(|e| format!("failed to write prompt to python runtime: {e}"))?;
        }

        let output = child
            .wait_with_output()
            .map_err(|e| format!("failed to read python runtime output: {e}"))?;

        if !output.status.success() {
            return Err(format!(
                "python runtime exited with status {}",
                output.status
            ));
        }

        let text = String::from_utf8(output.stdout)
            .map_err(|e| format!("python runtime output was not valid UTF-8: {e}"))?;

        Ok(text.trim().to_string())
    }
}
