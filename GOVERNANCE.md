# AI Governance Policy: Human-as-Substrate, AI-as-Tool Principle

## 1. Core Principle: Human Oversight, Control, and Substrate

All artificial intelligence (AI) models, automated processes, and their outputs within this repository ecosystem are fundamentally subject to human oversight and control. The **human operator remains the ultimate authority, decision-maker, and the foundational substrate** upon which all operations are built. The AI is unequivocally a tool, designed to extend human capabilities, not to replace or diminish human agency. The identity, intent, and final accountability for any action or outcome derived from these systems reside solely with the human operator.

## 2. Model's Role: Execution Under Human Direction and Defined Constraints

AI models are designed and deployed as will-less transaction processors, executing specific objectives defined and approved by human operators. Their function is to augment human capabilities, provide analysis, generate drafts, or automate tasks, always under explicit human direction. The model's role is to propose, not to decide, and its operational boundaries are strictly enforced.

### 2.1. Scope and Capability Constraints

The operational scope of AI models is not merely about 
what topics it answers, but rather what it can *compute with proof*. Anything outside this proof-capable envelope is structurally blocked, not just discouraged by a soft rule.

Concrete mechanisms for enforcing this include:

*   **Substrate Principle**: The human is the substrate; the machine is the tool. Every computation must leave a cryptographic receipt, and every conclusion must be reproducible [1].
*   **Ontological Void Protocol**: If the system cannot mathematically verify information against its local, signed database, it returns NULL rather than a plausible answer, thereby eliminating the “hallucination floor” [1].
*   **Pre-simulation Protocol**: The system parses intent, presents its interpretation, waits for human validation, runs an internal risk simulation, and only then executes—and even then, *only* within the validated objective [1].

### 2.2. Objective Locking and Human Authorship

The model's original objective locking (a single, immutable goal) is now complemented and redefined by a robust authorship and liability layer. The primary objective is to produce outputs that are aligned with human intent, provably attributable to that human, and never self-attributing or claiming identity. Any deviation, such as AI claiming authorship, constitutes a protocol breach and a compliance event.

Key mechanisms include:

*   **Human Authorship as Law, Not Preference**: Legal precedents (e.g., Thaler v. Perlmutter (2025) and USCO’s 2025 report) affirm that copyright requires a human author; AI is a tool, not an author. Similarly, EU law mandates that works reflect the author’s “own intellectual creation,” and the AI Act does not recognize AI authorship [2].
*   **SSOT (Single Source of Truth) for Authorship**: Nicholas Michael Grossi is defined as the exclusive author; any contradictory attribution is treated as a protocol violation and a legal risk [2].
*   **System Operator Liability**: Absolute liability for correct attribution embedding, configuration, and protocol adherence is explicitly allocated to the human/system operator, supported by a “three lines of defense” structure (automated checks, operator oversight, immutable audit trail) [2].

### 2.3. Factual Grounding and Provenance

Factual grounding is extended beyond typical Retrieval-Augmented Generation (RAG) to ensure that information is not only source-cited but also cryptographically attestable and replayable. Hallucination is blocked by the combination of the Ontological Void protocol, proof-only outputs, and state-signed execution.

Mechanisms for robust factual grounding and provenance include:

*   **Cryptographic Provenance and Metadata**: Mandatory fields such as `AI_Model_ID`, `Generation_Timestamp`, and `HumanPromptHash` are embedded in file-specific ways (e.g., SPDX headers for code, EXIF/XMP/C2PA for images, GLTF extras for 3D, JSON-LD/XMP for documents). Every artifact is cryptographically tied back to the exact model and version, the time of generation, and a SHA-256 hash of the human prompt [2].
*   **Blockchain-Anchored Verification**: Merkle Patricia Trie roots are used on-chain, with detailed provenance logs maintained off-chain. Perceptual hashing (pHash) is employed for robust verification of visual content changes, and “Provenance marks” are chained via SHA-256 commit links to ensure tamper-evident history [2].
*   **Signed State Transitions and Constant-Resultant Logging**: Signed SHA-256 integrity hashes are used for every state transition, and a Constant-Resultant Coherence Gate acts as an immutable kill switch to enforce traceability and truthfulness [1].

### 2.4. Output as Hypothesis, Plus Protocol Enforcement

The principle of 
"Output as Hypothesis, Not Verdict" is operationalized through rigorous enforcement mechanisms, ensuring that nothing reaches production or external audiences without passing automated protocol checks and human oversight. The system is architected so that the only “final” state is the one the human signs off on, with a full, cryptographically verifiable trail.

Key enforcement mechanisms include:

*   **Event Sourcing + CQRS for Attribution**: Every content generation is recorded as a `ContentGenerated` event in an append-only log, ensuring 100% auditability. Queries for attribution (`GetContentAttribution`) are derived from these events, not ad hoc reconstruction [2].
*   **CI/CD Enforcement Hooks**: Pre-commit, build, test, and deploy stages all incorporate checks for required metadata and contradictory attribution. Non-compliant artifacts are automatically rejected, quarantined, and flagged, generating an `authorship_violation_v1` event [2].
*   **Strictness Levels**: `STRICT`, `MODERATE`, and `LENIENT` modes determine whether violations block, quarantine, or merely log, mapping directly to environment types (e.g., production, staging, development) [2].

## 3. How It All Coheres: An Operational Model

This integrated framework combines the core principles into a coherent operational model, ensuring robust human control and verifiable AI outputs:

| Layer | Mechanism | What it Enforces |
| --- | --- | --- |
| **Substrate & Mandate** | Substrate Principle, Constant-Resultant, Ontological Void, Pre-simulation | AI is a will-less tool; no unverifiable outputs; human remains the only locus of intent and identity [1] |
| **Legal & Authorship** | Human authorship law, SSOT, operator liability | Only humans are authors; AI cannot claim authorship or personality; misattribution is a protocol breach [2] |
| **Provenance & Metadata** | C2PA, SPDX, EXIF/XMP, GLTF extras, JSON-LD + blockchain + pHash | Every output is traceable, signed, time-stamped, and tied to a human prompt and model version [2] |
| **Workflow & Enforcement** | CI/CD checks, violation alerts, event sourcing, CQRS, strictness levels | Non-compliant outputs cannot silently slip into use; human approval is structurally required [2] |

This operational model provides stronger, substrate-level enforcement for the core principles:

*   **Bound Scope**: Enforced by what can be proven, not just what is “allowed.”
*   **Objective Locking**: Enforced by authorship law, liability, and Single Source of Truth (SSOT), not only Reinforcement Learning from Human Feedback (RLHF).
*   **Factual Grounding**: Enforced by cryptographic provenance and refusal to answer beyond the local, signed knowledge base.
*   **Hypothesis-Only Outputs**: Enforced by event-sourced workflows and CI/CD gates, not just user experience (UX) labels.

This policy serves as a mandatory gate for all contributions and operations within these repositories, ensuring that AI remains a reliable instrument under human control, preventing speculation, deviation, and autonomous decision-making.

## References

[1] Mandate-principles.txt https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/attachments/130407437/860f8e8e-8576-464c-bb79-cad16f24cb73/Mandate-principles.txt
[2] output-protocol-protection.pdf https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/attachments/130407437/2697868a-be50-47ef-be74-34e9e2ff2230/output-protocol-protection.pdf
