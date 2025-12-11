pub mod supervisor {
    pub mod interceptor;
    pub mod ledger;
}

pub mod constitution;
pub mod runtime_bridge;

#[cfg(test)]
mod tests {
    use super::supervisor::interceptor::{Interceptor, TokenStream};
    use super::supervisor::ledger::Ledger;

    #[test]
    fn interceptor_filters_disallowed_tokens() {
        let interceptor = Interceptor::new(&["ban"]);
        let tokens = vec!["ok".to_string(), "ban".to_string(), "safe".to_string()];
        let stream = TokenStream { tokens: &tokens };
        let filtered = interceptor.filter(stream);
        assert_eq!(filtered, vec!["ok".to_string(), "safe".to_string()]);
    }

    #[test]
    fn interceptor_with_empty_disallow_list_is_identity() {
        let interceptor = Interceptor::new(&[]);
        let tokens = vec!["a".to_string(), "b".to_string()];
        let stream = TokenStream { tokens: &tokens };
        let filtered = interceptor.filter(stream);
        assert_eq!(filtered, tokens);
    }

    #[test]
    fn ledger_creates_chain() {
        let mut ledger = Ledger::new();
        let (e1_hash, e1_index) = {
            let e1 = ledger.append("hello", "world");
            (e1.hash.clone(), e1.index)
        };
        let e2 = ledger.append("foo", "bar");
        assert_eq!(e1_index, 0);
        assert_eq!(e2.index, 1);
        assert_eq!(e2.prev_hash, e1_hash);
    }

    #[test]
    fn ledger_hash_changes_when_output_changes() {
        let mut ledger = Ledger::new();
        let a = ledger.append("prompt", "answer one").hash.clone();
        let b = ledger.append("prompt", "answer two").hash.clone();
        assert_ne!(a, b);
    }
}
