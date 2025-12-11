pub mod supervisor {
    pub mod interceptor;
    pub mod ledger;
}

#[cfg(test)]
mod tests {
    use crate::supervisor::interceptor::{Interceptor, TokenStream};
    use crate::supervisor::ledger::Ledger;

    #[test]
    fn interceptor_filters_disallowed_tokens() {
        let interceptor = Interceptor::new(&["ban"]);
        let stream = TokenStream {
            tokens: &["ok".into(), "ban".into(), "safe".into()],
        };
        let filtered = interceptor.filter(stream);
        assert_eq!(filtered, vec!["ok".to_string(), "safe".to_string()]);
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
}
