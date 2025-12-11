//! Ledger: cryptographic, append-only log of supervised interactions.

use sha2::{Digest, Sha256};

/// One entry in the audit ledger.
#[derive(Debug, Clone)]
pub struct LedgerEntry {
    pub index: u64,
    pub input_hash: String,
    pub output_hash: String,
    pub prev_hash: String,
    pub hash: String,
}

/// In-memory ledger (demo only). In production this would be backed by durable storage.
#[derive(Default)]
pub struct Ledger {
    entries: Vec<LedgerEntry>,
}

impl Ledger {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn append(&mut self, user_input: &str, supervised_output: &str) -> &LedgerEntry {
        let index = self.entries.len() as u64;

        let input_hash = hex::encode(Sha256::digest(user_input.as_bytes()));
        let output_hash = hex::encode(Sha256::digest(supervised_output.as_bytes()));

        let prev_hash = self
            .entries
            .last()
            .map(|e| e.hash.clone())
            .unwrap_or_else(|| "GENESIS".to_string());

        let hash = {
            let mut hasher = Sha256::new();
            hasher.update(index.to_be_bytes());
            hasher.update(&input_hash);
            hasher.update(&output_hash);
            hasher.update(&prev_hash);
            hex::encode(hasher.finalize())
        };

        self.entries.push(LedgerEntry {
            index,
            input_hash,
            output_hash,
            prev_hash,
            hash,
        });

        self.entries.last().unwrap()
    }

    pub fn entries(&self) -> &[LedgerEntry] {
        &self.entries
    }
}