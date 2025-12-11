import hashlib
import time
from axiom.engine.mamba_driver import MambaEngine
from axiom.core.monitor import RuntimeMonitor
from axiom.crypto.receipt import sign_receipt

class ZeroEntropyExecutor:
    """
    The ZEEF Orchestrator.
    Enforces the Zero Entropy Law: No action without verification.
    """
    def __init__(self):
        self.engine = MambaEngine(model="mamba-2-2.7b-local")
        self.monitor = RuntimeMonitor(spec_path="verification/specs/zero_entropy.tla")
        self.session_id = self._generate_session_id()

    def execute(self, prompt: str, constraints: list):
        """
        The Genefication Loop:
        1. Genesis (Mamba) -> Generates candidate
        2. Verification (TLA+) -> Checks invariants
        3. Collapse -> Returns proven result or HALT
        """
        print(f"[*] INITIATING ZEEF EXECUTION [Session: {self.session_id}]")
        
        # 1. Genesis
        candidate_output = self.engine.generate(prompt)
        
        # 2. Verification
        is_safe, violation = self.monitor.verify(candidate_output, constraints)
        
        if not is_safe:
            # HALT_AND_PRUNE Logic
            self._log_failure(candidate_output, violation)
            return {
                "status": "HALTED",
                "error": "Zero Entropy Violation",
                "violation_trace": violation
            }

        # 3. Cryptographic Receipt
        receipt = sign_receipt(candidate_output, self.session_id)
        
        return {
            "status": "EXECUTED",
            "output": candidate_output,
            "receipt": receipt, # SHA-256 digest
            "proof": "verified_tla_trace"
        }

    def _generate_session_id(self):
        return hashlib.sha256(str(time.time()).encode()).hexdigest()[:12]

    def _log_failure(self, output, violation):
        print(f"[!] FATAL: Entropy Drift Detected. Pruning branch. Violation: {violation}")
