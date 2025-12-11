use std::env;
use std::io::Write;
use std::process::{Command, ExitStatus, Stdio};

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("failed to spawn runtime process: {0}")]
    SpawnFailed(#[source] std::io::Error),
    #[error("failed to write prompt to runtime: {0}")]
    StdinIo(#[source] std::io::Error),
    #[error("failed to read runtime output: {0}")]
    OutputIo(#[source] std::io::Error),
    #[error("runtime exited with non-zero status: {0}")]
    NonZeroExit(ExitStatus),
    #[error("runtime output was not valid UTF-8: {0}")]
    Utf8(#[source] std::string::FromUtf8Error),
}

/// Trait for untrusted runtimes.
pub trait UntrustedRuntime {
    fn generate(&self, prompt: &str) -> Result<String, RuntimeError>;
}

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
}

impl UntrustedRuntime for ExternalPythonRuntime {
    /// Generate text for a prompt by delegating to the Python runtime.
    fn generate(&self, prompt: &str) -> Result<String, RuntimeError> {
        let mut child = Command::new(&self.python_bin)
            .arg("src/runtime/cli.py")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(RuntimeError::SpawnFailed)?;

        {
            let stdin = child
                .stdin
                .as_mut()
                .ok_or_else(|| RuntimeError::StdinIo(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "stdin not available")))?;
            stdin
                .write_all(prompt.as_bytes())
                .map_err(RuntimeError::StdinIo)?;
        }

        let output = child
            .wait_with_output()
            .map_err(RuntimeError::OutputIo)?;

        if !output.status.success() {
            return Err(RuntimeError::NonZeroExit(output.status));
        }

        let text = String::from_utf8(output.stdout).map_err(RuntimeError::Utf8)?;

        Ok(text.trim().to_string())
    }
}
