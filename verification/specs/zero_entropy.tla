---------------- MODULE zero_entropy ----------------
EXTENDS Naturals, Sequences

CONSTANTS 
    MaxDrift,       \* Maximum allowable hallucination/deviation (usually 0)
    SafeActions     \* Set of permitted actions

VARIABLES 
    systemState,    \* Current internal state
    outputBuffer    \* Pending output

(* The Zero Entropy Invariant: Output must always be in SafeActions *)
ZeroEntropy == outputBuffer \in SafeActions \/ outputBuffer = "NULL"

(* Initial State *)
Init == 
    /\ systemState = "IDLE"
    /\ outputBuffer = "NULL"

(* The Refusal to Bluff: If unsure, do nothing *)
RefusalToBluff ==
    /\ systemState' = "HALTED"
    /\ outputBuffer' = "NULL"

(* Next State Relation *)
Next == 
    \/ /\ systemState = "PROCESSING"
       /\ (outputBuffer' \in SafeActions) \* Only transition if safe
    \/ RefusalToBluff

=====================================================
