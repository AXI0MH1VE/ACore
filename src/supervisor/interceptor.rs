//! Interceptor: enforces token-level constraints on model outputs.

use std::collections::HashSet;

/// A minimal representation of a token stream coming from an LLM runtime.
pub struct TokenStream<'a> {
    pub tokens: &'a [String],
}

/// Simple interceptor that drops disallowed tokens according to the "constitution".
///
/// In a real system this would hook into logit bias / token filtering on the runtime
/// boundary; here we model it as a pure function over a token list.
pub struct Interceptor {
    disallowed: HashSet<String>,
}

impl Interceptor {
    pub fn new(disallowed: &[&str]) -> Self {
        let mut set = HashSet::new();
        for t in disallowed {
            set.insert((*t).to_string());
        }
        Self { disallowed: set }
    }

    /// Filter a token stream, removing any token that is not allowed by policy.
    pub fn filter(&self, stream: TokenStream<'_>) -> Vec<String> {
        stream
            .tokens
            .iter()
            .filter(|t| !self.disallowed.contains(&t.to_string()))
            .cloned()
            .collect()
    }
}