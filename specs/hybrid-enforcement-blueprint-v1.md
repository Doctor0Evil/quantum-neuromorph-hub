# Hybrid Enforcement Blueprint: Technical + Normative Layer Split
## Morpheus SNC / SNCHIT Stack – Governance Architecture v1.0

**Document ID:** `specs/hybrid-enforcement-blueprint-v1.md`  
**Status:** Formal Architecture Specification  
**Version:** 1.0  
**Date:** 2026-02-19  
**Author:** bostrom18 (DID: did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7)  
**License:** MIT / ALN-sovereign  
**Target Audience:** Implementers, governance councils, regulators, Indigenous stewards  
**Purpose:** Clarify where code enforces invariants vs. where communities must decide values

---

## Executive Summary

The SNC/SNCHIT stack's strength lies in its **hybrid enforcement model**: some rights and constraints are encoded as compile-time invariants or runtime guards (code cannot run without them), while others depend on community and species input (code defers to documented governance decisions). This blueprint clarifies the boundary, ensuring implementers know when they must NOT fill gaps with unilateral code choices, but must instead call for participatory governance.

---

## 1. Three-Layer Enforcement Model

### Layer 1: Compile-Time Invariants (Cannot Be Bypassed)

These are encoded as **type system requirements** in Rust / ALN. A program that violates them cannot be compiled or loaded.

| Invariant | Why Compile-Time | Technical Encoding |
|-----------|------------------|-------------------|
| **No coercive channels** | Foundational right; should never be accidentally added. | `ConsentedChannel<T>` requires `ConsenseToken` and `AbortHandle` as type parameters; impossible to construct FEAR/PAIN channel without both. |
| **No rollback/downgrade** | Sovereignty is absolute; code must prevent regressions. | State versions `NeuromorphV1 → NeuromorphV2` only; `impl From<V2> for V1` explicitly forbidden. `forbidsdowngradeorrollback == true` is a const-validated field. |
| **Mandatory corridor ID on high-impact actions** | Prevents "truth in a vacuum"; ecology is non-negotiable context. | `EffectfulAction<C: CorridorId>` is generic over corridor type; any Effectful action missing CorridorId does not compile. |
| **Inner/outer domain separation** | Inner polytope is sacred; no neural data in governance predicates. | `InnerDomainSignal` and `OuterDomainTelemetry` are distinct type families; no implicit conversion trait. `rights.noscorefrominnerstate` is a phantom type enforcing this at compile time. |
| **Species-ID on neuromorphic artifacts** | Cross-species imprinting is the highest-level risk; prevent accidental mixing. | Every `Neuropattern<S>` and `DisciplineSignal<S>` carries `S: SpeciesId` type parameter; generic cross-species map `map::<A, B>` does not exist. Only explicit `CrossSpeciesAdapter<A, B>` with multi-species FPIC can convert. |
| **EVOLVE token presence and validity** | Consent is not optional; every long-lived change requires active proof. | Long-lived parameters (polytope coefficients, BCI ceiling, duty cycle) require `EVOLVE { token: ActiveEvolveToken, ... }` wrapper type; read-only access without EVOLVE is allowed, but write requires proof. |
| **Errority-only polytope tightening** | Harm cannot justify relaxing limits; only tightening allowed. | `PolytopeTightening` type only implements `tighten(A, b)` methods, never `loosen()`. `update_ceilings()` function only allows `new_value <= old_value`. |

### Layer 2: Runtime Guards (Can Be Checked and Logged)

These are enforced by the orchestrator and enclave at **runtime**, and violations are logged as `Errority` events or audit trail entries. They can be monitored and reported, but cannot be compiled out.

| Guard | Why Runtime | Technical Encoding | Violation Response |
|-------|-------------|-------------------|-------------------|
| **Consent freshness & revocation** | Consent expires and can be revoked dynamically; must check current state. | `ConsentToken` has `issued_at`, `expires_at`, `revoked` fields. Runtime checks: if `now > expires_at` or `revoked == true`, action blocked. | `StaleConsentToken` logged as Warning or High Errority; action rejected. |
| **Abort control liveness** | Hardware abort handle must actually be present and responsive; can fail at runtime. | `AbortHandle` is validated against OrganicCPU enclave: can issue cancel signal? Does device have physical panic button? | `AbortHandleNotResponding` logged as Critical Errority; protocol halted immediately. |
| **Corridor bounds verification** | Physical footprint can exceed declared corridor; must continuously check telemetry. | Orchestrator loop: every 1 hour, check `actual_ecoimpact in declared_corridor_polytope()`. | `CorridorBoundsExceeded` logged as High; protocol halted until FPIC renegotiated. |
| **Policy profile loading** | Profiles are documents (JSON/ALN); must be validated before use. | Profile validator: check for required fields (neurorights clauses, EcoAdmissible floors), verify signatures, check for conflicts with existing policies. | `InvalidPolicyProfile` logged as High; profile rejected, protocol using it halted. |
| **FPIC signature validation** | Signatures must be checked against current steward list; stewards can be revoked or rotated. | Before using FPIC sig, re-verify against current HGO (Human Governance Object) steward list. Cross-check: is signer still active? Has signature expired? | `InvalidFPIC` or `ExpiredFPICUsed` logged as High Errority. |
| **Biowidth consumption tracking** | Subject may have declared biowidth covenants; must be tracked and respected. | Runtime: aggregate all active protocols' biowidth usage; check against declared covenants (e.g., "keep 20% ATP reserve"). | `BiowidthCovenantViolation` logged as High; new protocol proposal rejected or existing protocol downscaled. |
| **BCI/RoH monotonicity** | Neural and autonomic load must trend safer over time; checked against historical data. | Runtime: before accepting evolution change, compute `BCI_after` and verify `BCI_after <= BCI_before`. Similarly for RoH. | `MonotoneBCIViolation` logged as High; change rejected. |
| **Outer-telemetry policy enforcement** | Cross-party telemetry use requires symmetric consent; must check at time of access. | Runtime: any predicate trying to read `OuterTelemetry<PartyB>` in context of `PartyA` decision checks `symmetric_consent == true`. | `AsymmetricTelemetryUse` logged as High; predicate blocked. |
| **Mid-flight covenant renegotiation triggers** | Conditions change; must detect and act. | Orchestrator monitors conditions: `new_species_affected?`, `ecoimpact > 120% of baseline?`, `errority_threshold_crossed?`. If any true, trigger renegotiation. | `MidFlightRenegotiationRequired` logged as Info; protocol auto-halts pending stakeholder re-approval. |
| **Cross-species adapter validation** | Multi-species FPIC and species-first floors must be present and valid before adapter is used. | Runtime: before instantiating `CrossSpeciesAdapter<A, B>`, verify both `multi_species_fpic` sigs are valid and both `species_first_floor` policies are loaded. | `InvalidCrossSpeciesAdapter` logged as High; adapter rejected. |

### Layer 3: Normative Governance Decisions (Community/Species Decides)

These are NOT encoded in code as invariants or guards. Instead, they are **documented in versioned HGOs and policy profiles**, which the code loads and respects, but does not unilaterally define.

| Decision | Why Community Decides | Where It Lives | How Code Respects It |
|----------|----------------------|-----------------|----------------------|
| **What counts as "fair" benefit-sharing for FEAR/PAIN signals** | Fairness is culturally and contextually specific; varies by community, species, health model. | `BenefitSharingCovenant` in HGO; specifies: "X% of revenue to subject's health fund," "free access to clinics," "priority for future upgrades," etc. | Code enforces the covenant (e.g., revenue tracking, enforcement on secondary use), but does not define what % is "fair"; that's in the HGO. |
| **Species-first EcoAdmissible floors** | Each species (or steward community) decides what is acceptable impact in their domain. | `SpeciesFirstFloor` profile (JSON/ALN), signed by species steward. Declares: "BeeAdmissible polytope must exclude actions that reduce hive stability by >10%." | Code checks actions against the declared floor (is action in polytope?), but does not invent floors; they come from steward governance. |
| **Which communities count as "affected" by a protocol** | Depends on local knowledge, historical relationships, spiritual significance. | `AffectedCommunities` list in mid-flight covenant; populated by corridor council and Indigenous stewards, not by technical analysis alone. | Code requires the list to be present and allows any community in the list to invoke `AbortCovenant`. |
| **Who is legitimately authorized to sign FPIC** | Varies by culture: some communities use elder consensus, others use elected representatives, others recognize hereditary land stewards. | `AuthorizedSigners` in community's HGO; lists by name/title, or delegates to community's own governance process. | Code verifies that FPIC signer matches this list; does not invent who should sign. |
| **How to share Errority and lessons across corridors** | Communities decide if/how to share harm learnings; data sovereignty may restrict sharing. | `ErrorityResponsePolicy` in HGO; e.g., "When harm occurs, notify affected community within 24h; sharing with regulators requires community consent; research use forbidden without explicit opt-in." | Code enforces these policies (notifications, consent checks), but does not unilaterally share data. |
| **Biowidth covenant definitions (how much reserve to keep)** | Subject's personal choice, possibly influenced by their health goals or cultural values. | Declared by subject in their personal HGO or in a shared covenant with their health provider / community trustee. | Code tracks biowidth consumption against these declared reserves; subject/trustee adjusts reserves via governance process. |
| **Quorum thresholds for FPIC and policy changes** | Communities choose thresholds; some prefer consensus, others plurality, others consensus-minus-one. | `FPICQuorumRule` in HGO: e.g., "3-of-5 corridor council members must approve." Or "Tribal council by consensus." | Code enforces the stated rule and escalates thresholds on Errority (e.g., forged FPIC → 4-of-7), but does not invent thresholds. |
| **Emergency intervention authorization** | Who can halt all protocols in a corridor under extreme circumstances? Varies widely. | `EmergencyAuthority` in HGO; e.g., "Public health officer + Indigenous council + subject advocate" or "Only tribal council." | Code obeys the list; any party not on the list cannot invoke emergency halt. |
| **Dispute resolution process** | Varies by jurisdiction and culture: mediation, arbitration, regulatory appeal, tribal review board, etc. | `DisputeResolutionProcess` in HGO; specifies steps, timeline, authorities, appeal path. | Code provides audit logs and event data to support whatever process is defined; does not choose the process. |

---

## 2. Boundary Rules: When to Code vs. When to Defer

### Rule 1: Hard Constraints vs. Value Choices

**Hard constraint (CODE):** A constraint that, if violated, breaks a foundational right or makes the system unsafe.
- Example: "No rollback" is a hard constraint. Code must enforce it.

**Value choice (DEFER TO GOVERNANCE):** A choice that depends on community preferences, cultural context, or contextual fairness.
- Example: "What % of revenue is fair?" is a value choice. Code enforces whatever % is chosen, but does not pick it.

**Test:** If you can describe the choice as "X is always better than not-X," it's likely a hard constraint → code. If the choice is "X vs. Y, both reasonable, depends on context," it's a value choice → governance.

### Rule 2: Prevention vs. Detection

**Prevention (CODE):** Use compile-time or runtime guards to make bad states unrepresentable or impossible to reach.
- Example: `ConsentedChannel` type prevents FEAR channel without consent; impossible to construct one without it.

**Detection (CODE + GOVERNANCE):** Bad states are possible, but immediately logged and escalated. Governance processes then decide response.
- Example: A subject's biowidth covenant can be violated (bad state), but it's immediately logged as Errority, triggering manual review and policy tightening.

**Pattern:** Neurorights use prevention (no coercion, no rollback, no downgrade). Fairness and benefit-sharing use detection + escalation (code flags violations; governance decides remedy).

### Rule 3: When to Add New Compile-Time Invariants

Before adding a new compile-time constraint (e.g., "all proposals must include X field"), ask:

1. **Is this about a fundamental right?** (Neurorights, sovereignty, species safety → YES, add to compile time)
2. **Will violating it cause direct harm?** (Breach of consent, unknown downside risk → YES)
3. **Or is it about fairness / contextual choice?** (What benefit-share %, which species floors → NO, keep in governance)

**Example: Adding "AllProtocolsMustIncludeMidFlightCovenant"**

- Long-running protocols (> 30 days) benefit from mid-flight renegotiation capability.
- Missing covenants means harm can't be halted mid-course if new species are affected.
- This is about **preventing hidden harm**, not about value choices.
- **Decision:** Add to compile time as type requirement on `ProtocolProposal`.

**Example: NOT adding "AllDisciplineChannelsMustOffer50PercentRevenueSplitToBeneficiaries"**

- Revenue split is fairness, not safety.
- What % is fair varies by corridor, cultural context, health model.
- **Decision:** Define in HGO and policy profile; code enforces whatever % is declared, but does not enforce 50%.

---

## 3. Structural Integration: Code Layers and Governance Coupling

### 3.1 Rust/ALN Type System (Compile-Time Layer)

```
File structure:
crates/
  core-contract/
    src/
      lib.rs
      neurorights.rs           ← Neurorights invariants (no coerce, no rollback, explicit consent)
      corridor.rs               ← CorridorId, EcoCorridorContext type defs
      species.rs                ← SpeciesId, species-specific separation
      evolution.rs              ← EVOLVE tokens, forward-only evolution types
      bio_safety.rs             ← BCI/RoH monotonicity types, polytope constraints
      governance.rs             ← HGO loading, policy profile types (normative values live here, not in code logic)
```

**Key principle:** Types define *structure and constraints*; they do NOT embed policy values.

```rust
// ✓ CORRECT: Type system enforces structure, policy value loaded from HGO
pub struct ConsentToken {
  subject_did: DID,
  device_id: DeviceId,
  valid_until: Timestamp,
  revoked: bool,
  // Value of "what counts as valid consent" comes from HGO, not hardcoded
}

// ✗ WRONG: Hardcoding fairness value in code
pub const MANDATORY_DISCIPLINE_REVENUE_SHARE: f64 = 0.50;  // Fairness is NOT a code constant

// ✓ CORRECT: Structure defined in code; value loaded from governance
pub struct BenefitSharingCovenant {
  beneficiary_percent: f64,  // Loaded from HGO at runtime
  max_secondary_uses: usize, // Loaded from HGO at runtime
}
```

### 3.2 Orchestration Runtime (Runtime Guard Layer)

```
crates/orchestration/
  src/
    lib.rs
    guards.rs                 ← Runtime policy checks (consent freshness, corridor bounds, Errority escalation)
    hgo_loader.rs             ← Load HGO and policy profiles; merge with code defaults
    errority.rs               ← Errority event generation and escalation
    covenant_monitor.rs       ← Mid-flight covenant checks; abort triggers
```

**Key principle:** Guards check constraints from HGO/policies; they do NOT invent constraints.

```rust
// ✓ CORRECT: Load policy from HGO; enforce it
pub fn validate_benefit_sharing(covenant: &BenefitSharingCovenant, hgo: &HGO) -> Result<()> {
  let policy = hgo.discipline_policy()?;  // Get from governance, not hardcoded
  if covenant.beneficiary_percent < policy.minimum_percent {
    Err("Benefit-sharing below HGO minimum")
  } else {
    Ok(())
  }
}

// ✗ WRONG: Hardcoding policy
pub fn validate_benefit_sharing(covenant: &BenefitSharingCovenant) -> Result<()> {
  const MIN_PERCENT: f64 = 0.50;
  if covenant.beneficiary_percent < MIN_PERCENT {
    Err("Must share at least 50%")
  } else {
    Ok(())
  }
}
```

### 3.3 HGO / Policy Profile Layer (Governance Values)

```
directory structure (versioned in Git):
hgos/
  phoenix-corridor-hgo-v1.5.json
  indigenous-stewards-hgo-v2.0.json
  species-first-floors-v1.0.json
  benefit-sharing-policy-v1.0.json
  quorum-rules-v1.0.json
```

**Key principle:** Every normative decision lives here, version-controlled and auditable.

```json
{
  "hgo_id": "phoenix-corridor-2026",
  "discipline_policy": {
    "minimum_beneficiary_percent": 0.40,
    "allowed_secondary_uses": ["clinical_research", "personalized_health_optimization"],
    "forbidden_uses": ["insurance_underwriting", "employment_screening", "behavioral_targeting"]
  },
  "benefit_sharing_models": [
    { "name": "standard", "beneficiary_percent": 0.50, "implementation": "annual_payout" },
    { "name": "health_priority", "beneficiary_percent": 0.60, "implementation": "clinic_access_credit" }
  ],
  "species_first_floors": {
    "bee": { "pheromone_interference_limit_ppm": 0.01, "hive_stability_floor": 0.8 },
    "tree": { "root_electromagnetic_exposure_max": 100, "mycological_disruption_floor": 0.9 }
  },
  "quorum_rules": {
    "fpic_approval": { "baseline": "3-of-5", "escalation_on_critical_errority": "4-of-7" },
    "policy_update": { "baseline": "4-of-7", "escalation": "6-of-7_plus_regulatory_review" }
  },
  "authorized_signers": [
    { "did": "did:phoenix:council-1", "role": "corridor_council", "term_end": "2026-12-31" },
    { "did": "did:indigenous:steward-1", "role": "species_steward", "term_end": "2026-06-30" }
  ],
  "version": "1.0",
  "signed_by": ["did:phoenix:council-1", "did:indigenous:steward-1"],
  "effective_date": "2026-02-19"
}
```

---

## 4. Implementation Workflow: The Cycle

### Step 1: Identify the New Right or Constraint

**Conversation with community:** "We need to ensure that FEAR/PAIN data isn't exploited without benefit-sharing."

### Step 2: Classify (Hard Constraint vs. Value Choice)

- **Hard constraint aspect:** "Data cannot be used without consent" → Belongs in code (ConsentToken runtime check).
- **Value choice aspect:** "What % is fair to share?" → Belongs in governance (HGO).

### Step 3: Implement Hard Constraint in Type System (if applicable)

```rust
pub struct DisciplineSignalUsage {
  signal: DisciplineSignal,
  secondary_use_policy: SecondaryUsePolicy,  // Type enforces that policy is present
  benefit_sharing_covenant: BenefitSharingCovenant,  // Type enforces proof of covenant
  audit_trail: Vec<DisciplineAuditEvent>,
}
```

### Step 4: Implement Runtime Guards (if applicable)

```rust
pub fn validate_secondary_use(
  usage: &DisciplineSignalUsage,
  intended_use: SecondaryUse,
  hgo: &HGO,
) -> Result<()> {
  let allowed = hgo.discipline_policy()?.allowed_secondary_uses;
  if allowed.contains(&intended_use) {
    Ok(())
  } else {
    Err(format!("Secondary use {:?} not in HGO approved list", intended_use))
  }
}
```

### Step 5: Define Normative Values in HGO (if applicable)

```json
{
  "discipline_policy": {
    "minimum_beneficiary_percent": 0.40,
    "allowed_secondary_uses": ["clinical_research"]
  }
}
```

### Step 6: Test with Mock Governance

Before shipping, simulate governance scenarios:

- Scenario A: Strict corridor (high beneficiary %, narrow secondary uses) → Verify code respects tighter HGO.
- Scenario B: Looser corridor (lower beneficiary %, more secondary uses) → Verify code still enforces consent and audit.
- Scenario C: HGO change over time → Verify code can load new HGO without recompilation.

---

## 5. Enforcement Roadmap: Phases

### Phase 1: Neurorights Core (Compile-Time, Unavoidable)

**Deliverable:** core-contract crate with types for:
- `ConsentedChannel<T>` with mandatory `ConsenseToken + AbortHandle`.
- No-downgrade state machines (`NeuromorphV1 → V2 only`).
- CorridorId as required type parameter on Effectful actions.
- InnerDomainSignal / OuterDomainTelemetry type separation.

**Governance input:** Minimal. These are universal neurorights.

### Phase 2: Governance Structure (Runtime + HGO)

**Deliverable:** orchestration crate + HGO schema with:
- Runtime consent/FPIC validation.
- Policy profile loading and enforcement.
- Errority event generation.
- Mid-flight covenant monitoring.

**Governance input:** Corridors draft their first HGOs (benefit-sharing policy, species floors, quorum rules).

### Phase 3: Interstitial Rights (Type + Runtime + HGO)

**Deliverable:** SpeciesId type system, CrossSpeciesAdapter, biowidth tracking, outer-telemetry policy enforcement.

**Governance input:** Species stewards publish species-first floors; communities define cross-party telemetry policies.

### Phase 4: Audit and Escalation

**Deliverable:** Errority logging, automatic threshold escalation, dispute resolution workflows.

**Governance input:** Define what counts as audit-driven tightening in each corridor.

---

## 6. Key Principles Summary

1. **Neurorights are mandatory; fairness is participatory.**
   - No coercion, no rollback, no downgrade, explicit consent: code enforces universally.
   - What counts as fair benefit-sharing, acceptable species impact, etc.: communities decide via HGO.

2. **Type system prevents bad states; governance detects and escalates.**
   - Use types to make certain mistakes uncompilable (ConsentedChannel, forward-only state, CorridorId required).
   - Use runtime guards to detect dynamic violations (consent expired, FPIC forged, biowidth exceeded) and log as Errority.

3. **HGO is the source of truth for normative values.**
   - Do not hardcode policy constants in code.
   - Load policies from versioned, signed HGOs; code enforces them dynamically.

4. **Implementers must not fill normative gaps alone.**
   - If a decision is marked "governance decides," implementers must call for community input before coding a default.
   - Document which HGO clause / policy profile field controls each behavior.

5. **Escalation is monotone.**
   - Errority events trigger higher quorum, stricter policies, tighter bounds.
   - Never use Errority to justify relaxing constraints; only tightening.

---

## 7. Validation Checklist for Implementers

- [ ] Identify all new constraints or rights.
- [ ] For each, ask: "Is this a hard constraint (safety/rights) or a value choice (fairness/context)?"
- [ ] Hard constraints → implement as compile-time types or runtime guards.
- [ ] Value choices → specify as HGO fields; load dynamically.
- [ ] Every compile-time invariant has a test case proving it's uncompilable without it.
- [ ] Every runtime guard has a test case showing it detects and logs violations.
- [ ] Every HGO field is documented in the HGO schema and has at least one example HGO file showing it.
- [ ] No hardcoded policy constants (MIN_BENEFICIARY_PERCENT, MAX_SPECIES_IMPACT, etc.) in production code.
- [ ] All normative decisions are traced to an HGO or policy profile version.
- [ ] Documentation explains for each feature: "This is code-enforced" vs. "This is governance-decided."

---

## 8. References

- `specs/interstitial-rights-v1.md` – Details on each interstitial right and its enforcement split.
- `specs/signature-threat-model-v1.md` – Security layer where governance thresholds and key revocation are normative.
- SNC/SNCHIT core docs – Neurorights, EVOLVE tokens, EvolutionAuditRecords.
- HGO schema – Format for publishing governance values.

---

**Document prepared by:** bostrom18 (DID: did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7)  
**Version control:** `specs/hybrid-enforcement-blueprint-v1.md`  
**License:** MIT / ALN-sovereign  
**Status:** Ready for GitHub check-in and implementation review