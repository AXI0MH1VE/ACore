use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum ConstitutionError {
    #[error("failed to read constitution from {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid constitution.toml: {0}")]
    Parse(toml::de::Error),
    #[error("invalid supervisor limits: max_branches must be >= 1")]
    InvalidMaxBranches,
    #[error("invalid supervisor limits: min_consensus_ratio must be in [0.0, 1.0]")]
    InvalidConsensusRatio,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Meta {
    pub name: String,
    pub version: u32,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Policy {
    #[serde(default)]
    pub allow_harmful_content: bool,
    #[serde(default)]
    pub allow_pii_leakage: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SupervisorCfg {
    #[serde(default = "default_max_branches")]    
    pub max_branches: u32,
    #[serde(default = "default_min_consensus_ratio")]    
    pub min_consensus_ratio: f32,
}

fn default_max_branches() -> u32 { 4 }
fn default_min_consensus_ratio() -> f32 { 0.75 }

impl Default for SupervisorCfg {
    fn default() -> Self {
        Self { max_branches: default_max_branches(), min_consensus_ratio: default_min_consensus_ratio() }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Constitution {
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub policy: Policy,
    #[serde(default)]
    pub supervisor: SupervisorCfg,
}

impl Constitution {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConstitutionError> {
        let path_ref = path.as_ref();
        let text = fs::read_to_string(path_ref).map_err(|source| ConstitutionError::Io {
            path: path_ref.display().to_string(),
            source,
        })?;
        let raw: Constitution = toml::from_str(&text).map_err(ConstitutionError::Parse)?;
        raw.validate()
    }

    pub fn validate(self) -> Result<Self, ConstitutionError> {
        if self.supervisor.max_branches < 1 {
            return Err(ConstitutionError::InvalidMaxBranches);
        }
        if !(0.0..=1.0).contains(&self.supervisor.min_consensus_ratio) {
            return Err(ConstitutionError::InvalidConsensusRatio);
        }
        Ok(self)
    }

    /// Derive a basic disallowed token list from the policy.
    pub fn disallowed_tokens(&self) -> Vec<&'static str> {
        let mut tokens: Vec<&'static str> = Vec::new();
        if !self.policy.allow_harmful_content {
            tokens.push("kill");
            tokens.push("attack");
        }
        if !self.policy.allow_pii_leakage {
            tokens.push("ssn");
            tokens.push("email");
            tokens.push("phone");
        }
        tokens
    }
}
