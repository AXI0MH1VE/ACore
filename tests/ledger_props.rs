use axiom_hive_core::supervisor::ledger::Ledger;
use proptest::prelude::*;

proptest! {
    #[test]
    fn ledger_indices_are_monotonic(inputs in prop::collection::vec((".*", ".*"), 1..32)) {
        let mut ledger = Ledger::new();
        for (prompt, output) in inputs {
            ledger.append(&prompt, &output);
        }
        let entries = ledger.entries();
        for w in entries.windows(2) {
            prop_assert_eq!(w[0].index + 1, w[1].index);
            prop_assert_eq!(&w[0].hash, &w[1].prev_hash);
        }
    }
}