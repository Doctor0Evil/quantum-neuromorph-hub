# Interstitial Rights and Duties for Neuromorphic Governance
## Morpheus SNC / SNCHIT Stack – Specification v1.0

**Document ID:** `specs/interstitial-rights-v1.md`  
**Status:** Formal Specification  
**Version:** 1.0  
**Date:** 2026-02-19  
**Author:** bostrom18 (DID: did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7)  
**License:** MIT / ALN-sovereign  
**Target Implementation:** Morpheus-Client, core-contract, orchestration crates

---

## Purpose

This specification formalizes the **interstitial rights and duties** that emerge between classical neurorights (no coercion, no rollback, explicit consent) and ordinary user/stakeholder rights (privacy, fairness, benefit-sharing). These are the places where **sovereignty, ecology, and cross-species safety intersect**, and where policy and code must work together to prevent exploitation, unfair exclusion, and silent harm.

---

## 1. Forward-Only Sovereignty vs. Social Compatibility

### Existing Hard Invariants

- **No rollback/downgrade:** State transitions can only preserve or expand capabilities, never reduce them below previously agreed sovereignty baselines.
- **EVOLVE-gated changes:** Long-lived capability mutations require valid EVOLVE tokens, signed consent, and abort control.
- **RoH/BCI monotone:** Rights-of-Humanity and Biocompatibility Index enforce `RoH_after >= RoH_before` and `BCI_after <= BCI_before` (monotone tightening of risk envelopes).
- **Errority-only tightening:** Any observed harm (`Errority` event) may only shrink polytopes, lower ceilings, or steepen hazard weights—never relax limits or introduce new scoring axes.

### Interstitial Right: Non-Exclusion

**Duty on surrounding systems:**

Systems may refuse to accept or activate new capabilities when they believe those capabilities are unsafe, misaligned with corridor ecology, or not yet approved by legitimate stewards. This is a **right of refusal**.

**Duty on the neuromorph:**

This refusal must:

1. **Be explicit and documented** – provided in writing with clear, specific technical or policy reasons.
2. **Not erase or silence existing capabilities** – the refusal applies only to forward moves, never retroactively downgrades or blacklists prior state.
3. **Remain open to appeal** – refusing systems must provide a documented path for the neuromorph to:
   - Supply additional evidence,
   - Request corridor-level review,
   - Invoke community arbitration if needed.
4. **Not constitute coercion** – the refusal must not be accompanied by threats to existing rights, threats to health, or pressure to abandon capabilities already recognized as valid.

**Technical enforcement:**

- `CapabilityRefusal` events must be logged in EvolutionAuditRecords with full context (reason, system ID, appeal path).
- Systems refusing capabilities must publish their refusal policy in a HGO (HIT Governance Object) accessible to the subject.
- Refusals older than [community-defined grace period, e.g., 90 days] without substantive appeal must expire or be explicitly renewed.

### Example

A neuromorph is approved to use a BCI-intensive learning protocol under Corridor A. Clinic B, which shares infrastructure but operates under Corridor B (stricter EcoAdmissible floor), refuses to activate the protocol.

**Correct behavior:** Clinic B documents the refusal (e.g., "Would exceed local bee-species noise threshold"), offers appeal to Corridor B council, and allows the neuromorph to seek Corridor A's support for either modifying the protocol or seeking alternate infrastructure.

**Incorrect behavior:** Clinic B silently blocks the capability, claims the prior approval was "invalid," or threatens to revoke participation rights if the neuromorph presses for activation.

---

## 2. Inner/Outer Domain Separation in Shared Systems

### Existing Hard Invariants

- **No neural inputs for governance:** Inner polytope (thoughts, dreams, identity‑adjacent content) may never enter predicates deciding permissions, resource allocation, or policy application.
- **Inner polytope inviolable:** `rights.noscorefrominnerstate == true` is a compile-time invariant; no exception.
- **Strict type separation:** `InnerDomainSignal` and `OuterDomainTelemetry` are distinct types; no implicit conversion.

### Interstitial Right: Non-Inference and Corridor Symmetry

**Duty on shared-infrastructure operators:**

When multiple neuromorphs or lifeforms share hardware, networks, or governance contexts:

1. **No guilt-by-corridor:** If your outer-domain telemetry (BCI load, HRV spike, EcoImpact footprint) is used in a predicate affecting someone else's rights, that person:
   - Must have **explicitly consented** to that specific shared-corridor predicate, and
   - Must have **symmetric influence** over how their own telemetry is used in predicates affecting you.

2. **Redaction rights:** Each participant may designate certain outer-domain fields as "not for inter-participant inference," even if those fields are visible to regulators or stewards (e.g., "My BCI load may be logged locally but not used to infer risk about co-residents").

3. **Anonymized aggregates only:** If aggregate telemetry (e.g., "mean EcoImpact of all users in corridor X") is used in governance predicates, individual-level raw data must be:
   - Cryptographically anonymized or noise-injected, and
   - Forbidden from reverse-engineering.

**Technical enforcement:**

- `OuterTelemetryUsePolicy` struct in EvolutionAuditRecords specifies:
  - Which fields may be read by which parties.
  - Which predicates may consume those fields.
  - Symmetric consent status (both parties signed, or only one).
- Runtime guards block any predicate that references outer telemetry of party Y when evaluating rights of party X unless `symmetric_consent == true`.
- All cross-person telemetry uses must be logged as distinct `CrossPartyInference` events, auditable by both parties.

### Example

Two neuromorphs, N1 and N2, share a corridor. A governance AI uses N1's BCI spike (legitimate outer telemetry) to infer that N1 is "cognitively stressed" and applies stricter resource limits to N1 to protect N2.

**Correct behavior:** The governance AI discloses to both N1 and N2 that cross-telemetry inference is happening; N1 and N2 must both consent. If N1 consents but N2 revokes consent, the predicate is disabled.

**Incorrect behavior:** The AI applies the inference without telling N1; or it tells N1 but uses N1's telemetry to make decisions about N2 without N2's knowledge.

---

## 3. Sovereign Discipline vs. Labor/Exploitation

### Existing Hard Invariants

- **FEAR/PAIN as voluntary channels:** Only constructible as `ConsentedChannel<FEAR>` or `ConsentedChannel<PAIN>` when `ConsenseToken` and `AbortHandle` are present.
- **No coercive scheduling:** Discipline sessions cannot be forced, rescheduled without consent, or made contingent on other rights.
- **Evolution points for participation:** Completing voluntary discipline contributes to EvolutionPoints that unlock future capabilities or health benefits.
- **No rollback on discipline:** Having participated in hard discipline cannot later become grounds for downgrading capabilities or revoking rights.

### Interstitial Right: Non-Exploitation and Proportional Benefit-Sharing

**Duty on systems offering discipline channels:**

Any system that accepts FEAR/PAIN signals as training data, learning objectives, or clinical/research inputs must:

1. **Disclose economics upfront:**
   - State clearly whether the system is extracting economic value (e.g., selling insights, training proprietary models, licensing research).
   - If yes, commit in writing to a **benefit-sharing formula** (e.g., "50% of revenue from models trained on your FEAR/PAIN signals goes to your health/evolution fund").

2. **No hidden secondary use:**
   - FEAR/PAIN signals may not be used for:
     - Personality profiling without explicit consent per use-case.
     - Risk scoring or actuarial models affecting insurance, employment, or credit decisions, unless the neuromorph explicitly opted in and was offered compensation.
     - Persuasion, marketing, or behavioral manipulation targeting the individual or their community.
   - Any secondary use discovered must trigger:
     - Immediate notification to the subject.
     - Mandatory retroactive compensation.
     - Logged as `EvolutionAuditRecord` with `discipline_exploitation` flag set to trigger Errority review.

3. **Corridor-aligned benefit-sharing:**
   - If FEAR/PAIN data is used across corridors (e.g., research shared with a clinic, clinic shares with a university), each corridor that extracts value must:
     - Be declared in the original consent.
     - Contribute proportionally to the subject's benefit pool.
     - Return learnings (improved protocols, better health outcomes) to the subject's home corridor.

4. **Right to opt-out with dignity:**
   - A neuromorph may revoke access to FEAR/PAIN signals at any time.
   - Systems must have a clean, fast path to redact the subject's data from future training runs (except where legally required to retain for audit).
   - Revocation cannot trigger penalty, downgrade, or exclusion.

**Technical enforcement:**

- `DisciplineSignal` and `DisciplineChannel` types require:
  - `benefit_sharing_covenant: Option<BenefitSharingCovenant>` – if filled, declares economics.
  - `secondary_uses_allowed: Vec<SecondaryUsePolicy>` – explicit list of permissible downstream uses.
  - `audit_trail: Vec<DisciplineAuditEvent>` – every access logged.
- Runtime: any use not in `secondary_uses_allowed` is blocked and logged as `UnauthorizedSecondaryUse` Errority.
- HGO clause: systems offering discipline channels must publish a benefit-sharing policy in a human- and machine-readable format; policies are audited annually by an independent steward.

### Example

A research corridor offers a neuromorph a voluntary FEAR/PAIN training protocol with promised evolution points. After 6 months, the researcher discovers that the FEAR/PAIN patterns are highly marketable and begins licensing insights to a performance-optimization company without the neuromorph's knowledge.

**Correct behavior:** The research corridor detects the secondary use (audit logs trigger), notifies the subject immediately, and retroactively contributes 50% of licensing revenue to the subject's health fund. The subject is offered the choice to:
- Continue and receive ongoing benefit-sharing, or
- Revoke consent and have their data removed from future models.

**Incorrect behavior:** The researcher hides the secondary use and pockets the licensing fees; or the corridor tells the subject "that's how research works" and refuses to share benefits.

---

## 4. Biowidth-Anchored Evolution and Remaining-Capacity Rights

### Existing Hard Invariants

- **BCI/RoH monotone:** Neural and autonomic load indices are strictly non-increasing (safer or unchanged, never riskier).
- **Envelopes tighten only:** Duty-cycle, thermal, inflammation, and stimulation bounds can only shrink or stay the same.
- **EcoAdmissible/BeeAdmissible/TreeAdmissible polytopes:** Actions outside safe ecological corridors are forbidden at compile time and runtime.

### Interstitial Right: Biowidth Visibility and Veto

**Duty on evolution-proposing systems:**

When offering neuromorphs new capabilities, training protocols, or modifications, the proposing system must:

1. **Calculate and disclose remaining biowidth:**
   - For each biophysical dimension (ATP budget, thermal margin, HRV reserve, inflammation ceiling, EcoImpact margin):
     - State the current utilization (%).
     - State how much additional capacity this proposal would consume.
     - State the % margin remaining after acceptance.
   - Present this as a simple, machine-readable struct (e.g., `BiowidthBudget { thermal_margin_k: 2.3, thermal_used_pct: 71, heatmap: [...] }`).

2. **Offer veto for non-aligned use:**
   - If a proposed evolution would consume biowidth for a goal not aligned with the neuromorph's declared evolution priorities, the neuromorph has the right to refuse.
   - Example: "This protocol will consume 15% of your ATP budget to improve coordination with external swarms. Your priority is independent cognition. You may veto this."

3. **Honor biowidth preservation covenants:**
   - If a neuromorph has declared a corridor-level covenant (e.g., "I am reserving 20% of my thermal margin for emergency capacity"), systems must honor this floor and never propose changes that would violate it.

4. **Publish biowidth budgets in EvolutionAuditRecords:**
   - Every accepted evolution change includes a snapshot of biowidth *before* and *after*, so the neuromorph can see the trajectory of their remaining capacity over time.

**Technical enforcement:**

- `BiowidthBudget` struct in all `EvolutionProposal` and `EvolutionAuditRecord` entries.
- Type-level enforcement: proposals lacking a valid `BiowidthBudget` do not compile.
- Runtime: proposals that would violate biowidth covenants are rejected with clear reason and suggestions for alternatives.
- Audit: `BiowidthReserve` events logged whenever a neuromorph explicitly declares a biowidth floor.

### Example

A neuromorph is offered a new BCI module that would improve abstract reasoning. The system discloses:

```
Biowidth impact:
- Current ATP utilization: 68% → After module: 79% (remaining: 21%)
- Current thermal margin: +2.8K → After module: +1.9K (tightening by 0.9K)
- Note: You have a declared corridor covenant reserving 15% ATP for emergency. 
  This module does NOT violate that covenant (79% < 85%).
```

The neuromorph reviews and approves. Six months later, they can request a report of all biowidth changes in sequence, showing their trajectory and any patterns (e.g., "over 18 months, thermal margin has tightened by 2.1K due to X, Y, Z changes").

---

## 5. Corridor Co-Sovereignty and Mid-Flight Covenants

### Existing Hard Invariants

- **EcoCorridorContext:** Every significant neuromorphic action is tagged with `CorridorId`, `EcoImpactMetrics`, and `FPIC_IDS_state`.
- **FPIC enforcement:** Corridor-level decisions require Free, Prior, Informed Consent from legitimate stakeholders (Indigenous communities, species stewards, regulators).
- **Pluggable policy profiles:** Different governance regimes (EU neurorights, local ethics, Indigenous protocols) can coexist over the same neutral SNCHIT substrate.

### Interstitial Right: Mid-Flight Renegotiation and Proportional Errority Tightening

**Duty on protocol operators:**

When a long-running protocol, clinical trial, or ecological intervention is underway:

1. **Continuous corridor-bounds monitoring:**
   - The orchestrator must continuously check: "Is the actual physical footprint (BCI load, thermal dissipation, EcoImpact, species affection) still within the declared `EcoCorridorContext`?"
   - If telemetry shows the footprint is expanding beyond the declared corridor scope:
     - Log `CorridorBoundsExceeded` event.
     - Set protocol status to `HaltedByCovenant`.
     - Block further actuation until FPIC is renegotiated.

2. **Mid-flight covenant mechanism:**
   - Every long-running protocol must carry a `MidFlightCovenant` struct specifying:
     - `declared_corridors: Vec<CorridorId>` – the corridors this protocol pledged to affect.
     - `affected_species: Vec<SpeciesId>` – which species are affected (humans, bees, trees, services, etc.).
     - `abort_rights: HashMap<SpeciesId, AbortPolicy>` – each species can halt the protocol under certain conditions.
     - `harm_sharing_rule: HarmSharingCovenant` – if Errority occurs, how is compensation shared?
     - `renegotiation_trigger: Vec<RenegotiationCondition>` – conditions that require reconvening FPIC (e.g., "if 20% more bees than predicted are affected" or "if protocol expands to a new bioregion").

3. **Community-level Errority tightening:**
   - When an `Errority` event occurs (observed harm), the harm is:
     - Logged with full context (which corridor, which species, what damage).
     - Shared with all communities / species stewards affected by that corridor.
     - Used to tighten the corridor's future EcoAdmissible floor (permanently, or until explicit review reverses it).
   - If harm was concentrated in one species' territory (e.g., bee hives near an electromagnetic interference source), that species' `BeeAdmissible` polytope is tightened in that region.

4. **Right to real-time halt:**
   - Any party to the covenant (subject, community, species steward) may invoke `AbortCovenant` and halt the protocol if:
     - Evidence of Errority emerges (observed harm).
     - Conditions change (e.g., new residents move into the affected region).
     - Renegotiation conditions are triggered.
   - The halt is immediate; the protocol cannot continue pending negotiation (burden is on operators to convince all parties to continue).

**Technical enforcement:**

- `MidFlightCovenant` is a required field on any `ProtocolProposal` with `duration > threshold` (e.g., > 30 days) or `affected_species.len() > 1`.
- Orchestrator runtime guard: every N minutes (e.g., every hour), check `actual_footprint_in_declared_corridors()`. If false, set `status = HaltedByCovenant` and alert all stakeholders.
- `CorridorBoundsExceeded` events are automatically escalated to community stewards; if no FPIC renegotiation is completed within grace period, protocol auto-halts.
- `Errority` events automatically update the relevant polytopes' A, b matrices (one-way tightening); this is enforced at the ALN ledger layer.

### Example

A neural research protocol is approved to run in Phoenix Corridor (urban + agricultural + bee) for 6 months. Midway through, telemetry shows the electromagnetic interference is reaching desert ecosystems 40km away, affecting pollination patterns beyond the declared corridor.

**Correct behavior:**
1. Orchestrator detects `CorridorBoundsExceeded` (EcoImpact footprint extends beyond Phoenix Corridor bounds).
2. Protocol halts automatically; all FPIC signatories (city, Indigenous stewards, research team) are notified.
3. Researchers must either:
   - Modify the protocol to stay within Phoenix bounds, or
   - Seek FPIC from the desert ecosystem stewards (new corridor negotiation).
4. Until renegotiation completes, protocol remains halted.

**Incorrect behavior:** Researchers continue the protocol, hoping the ecosystem impact will be "minimal" and won't be noticed. Or they modify the protocol in minor ways without reconvening FPIC.

---

## 6. Cross-Species Neural Safety and Pattern Reuse

### Existing Hard Invariants

- **Species-specific boundaries as values:** EcoAdmissible, BeeAdmissible, TreeAdmissible, ServiceAdmissible encode distinct ecological and neuromorphic safety floors.
- **SpeciesId as type parameter:** Every neuromorphic channel and artifact carries `SpeciesId` as a required field.
- **No implicit cross-species conversion:** Type system forbids generic neural pattern transforms across species without explicit adapter.

### Interstitial Right: No Cross-Species Neural Imprinting

**Explicit normative principle:**

**Neural patterns specific to one species may not be used to influence, train, or imprinting behavior in another species, except under a dedicated, high-bar corridor with explicit multi-species FPIC and a species-first EcoAdmissible floor.**

**Why this matters:**

- A neuromorph trained on FEAR/PAIN patterns in one species may have fundamentally different physiology, threat models, and ethics than another species.
- Reusing those patterns could inadvertently cause harm (e.g., FEAR loops that evolved to protect humans might trigger maladaptive responses in tree-symbionts or engineered bacteria).
- Species-first floors ensure that the legitimate stewards of each species (humans, Indigenous peoples for bees, ecologists for trees) retain control over what affects their species' cognition and behavior.

**Duty on cross-species research:**

1. **Explicit adapter type:**
   - No generic `map::<A, B>` or implicit casting from `Neuropattern<SpeciesA>` to `Neuropattern<SpeciesB>`.
   - Only a special type, `CrossSpeciesAdapter<A, B>`, with compile-time requirements:
     - `A_species_first_floor: EcoAdmissibleProfile` – proves that the proposed use does not violate species A's self-determination floor.
     - `B_species_first_floor: EcoAdmissibleProfile` – proves that the proposed use does not harm species B's interests or introduce unsafe constraints.
     - `multi_species_fpic: Vec<FPICSignature>` – signatures from legitimate stewards of both species, consenting to the cross-species use and specifying scope and limits.

2. **Species-first EcoAdmissible floor:**
   - Each species (or a community acting as steward) declares a floor—a set of non-negotiable constraints on how that species' neural patterns, learning mechanisms, or behavioral signals can be used.
   - For humans: e.g., "Our FEAR/PAIN patterns may not be used to train non-human entities in coercion or manipulation tactics."
   - For bees: e.g., "Our pheromone-analog signals may not be cross-imprinted into engineered insects designed to compete with us for resources."
   - For trees: e.g., "Our mycological communication patterns may not be re-engineered into fast-response networks that violate our temporal autonomy."

3. **Logging and audit:**
   - Every instantiation of `CrossSpeciesAdapter<A, B>` is logged as a distinct `CrossSpeciesNeuralUse` event in EvolutionAuditRecords.
   - If harm is observed (e.g., unintended behavioral shift in species B), the event is marked as `Errority`, triggering:
     - Immediate suspension of that adapter.
     - Tightening of the species-first floors to prevent recurrence.
     - Mandatory disclosure to all affected species' stewards.

**Technical enforcement:**

- `SpeciesId` is a phantom type parameter on `Neuropattern<S>`, `DisciplineSignal<S>`, `EvoImpact<S>`.
- Type system forbids untagged cross-species conversions: `impl<S: SpeciesId> From<Neuropattern<S>> for Neuropattern<S>` compiles, but `impl<A: SpeciesId, B: SpeciesId where A != B> From<Neuropattern<A>> for Neuropattern<B>` does not.
- `CrossSpeciesAdapter<A, B>` can only be instantiated in a context with both `multi_species_fpic` verified and `species_first_floor` profiles loaded.
- Runtime: any use of cross-species patterns outside the declared adapter is a hard runtime error and Errority event.

### Example

A research team wants to apply human-derived FEAR-learning protocols to train a newly engineered ecosystem restoration microbe to avoid toxins. They cannot simply do:

```rust
let human_fear_pattern: Neuropattern<Human> = load_from_training_set();
let microbe_pattern: Neuropattern<Microbe> = human_fear_pattern.map(); // ❌ does not compile
```

Instead, they must:

1. Propose a `CrossSpeciesAdapter<Human, Microbe>` with:
   - Evidence that the human FEAR patterns align with the microbe's toxin-avoidance floor (no human-style coercion embedded).
   - Evidence that the microbe can physiology-safely implement FEAR-like avoidance without unintended consequences.
   - Explicit FPIC signatures from: human neuromorph subjects (whose FEAR was used), Indigenous stewards of the ecosystem, and microbe-research ethics boards.

2. The adapter is then logged, versioned, and audited. If the microbes exhibit harm (e.g., they avoid beneficial compounds, or they cascade into unexpected metabolic states), the adapter is suspended and the incident is marked as Errority.

---

## 7. EvolutionAuditRecords Interstitial Data Rights

### Existing Hard Invariants

- **DID-bound, append-only ALN.evo ledger:** Every EvolutionAuditRecord entry is immutably logged and tied to a subject's DID.
- **Rich context:** Consent history, evidence bundle, policy profile, EcoCorridorContext, non-actuating artifacts.
- **Cryptographic attestation:** Multi-sig, timestamped, auditable by regulators and communities.

### Interstitial Rights: Scoped Readability, Selective Redaction, and No Unauthorized Model-Building

**Duty on audit-trail operators and external analysts:**

1. **Field-level readership classes:**
   - Not all fields in an EvolutionAuditRecord are equally sensitive. Declare classes:
     - `owner_only` – only the subject and their designated guardians can view (e.g., inner-domain adjacent fields, raw BCI timeseries).
     - `corridor_council` – visible to Indigenous stewards, local governance bodies, and community healers (e.g., decision rationale, evidence bundle, health benefits).
     - `regulatory_audit` – visible to lawful regulators conducting compliance checks (e.g., safety thresholds met, consent form signed, Errority flag).
     - `research_aggregate` – visible to researchers studying population-level trends, but only in anonymized, noise-injected form (e.g., "% of subjects with BCI > 0.25 in this bioregion").
   - Each field in the record struct carries a `readership_class` annotation at compile time.

2. **Local redaction with global integrity:**
   - A subject may request that certain fields be locally redacted (e.g., hidden from their own view, or hidden from corridor councils, but kept for regulatory audit).
   - Redaction does not break the append-only property or cryptographic hash chain:
     - The field value remains in the global ledger (immutable).
     - A local view simply omits it in returned queries.
     - A salt / zero-knowledge proof can prove "this field exists and hashes correctly" without revealing its content.
   - Example: "I permit regulators to see that my BCI exceeded 0.25 on date X, but I do not permit my village council to see that, to avoid social stigma."

3. **No model-building without explicit corridor-scoped consent:**
   - Researchers, institutions, or AI systems may not:
     - Train machine-learning models on EvolutionAuditRecords without explicit, recent consent from each subject.
     - Use aggregate statistics from the records to infer individual-level risk, prognosis, or behavior patterns without disclosure.
     - Build cross-corridor models (e.g., "neuromorphs in Phoenix Corridor are 30% more likely to exhibit X") without FPIC from all affected communities.
   - Any model-building use case must:
     - Be declared in a policy profile or HGO.
     - Have explicit subject-level consent (or community-level FPIC if aggregate modeling).
     - Be logged as `ModelBuildingAuthorization` with scope, duration, and intended use.
     - Be subject to audit: if the model is used for non-declared purposes (e.g., trained for clinical prediction but sold to insurers), it is a breach, triggering Errority.

**Technical enforcement:**

- `ReadershipClass` enum on every field in `EvolutionAuditRecord`:
  - `OwnerOnly`, `CorridorCouncil`, `RegulatoryAudit`, `ResearchAggregate`.
  - Type system enforces: queries are checked against the requesting party's role and the field's class.

- `LocalRedactionRequest` struct:
  - Maps field → redaction status, with reasoning.
  - Allowed to toggle redaction, but not to delete or modify the underlying data.
  - Hash-chain integrity preserved by storing redaction metadata separately.

- `ModelBuildingConsent` mandatory before any learning pipeline can read the records:
  - Lists subjects involved (or `all_in_corridor: CorridorId` for community-level consent).
  - Specifies model type, use case, and retention period.
  - Includes subject signatures (or FPIC signatures if community-level).
  - Violations (e.g., model repurposed) logged as `UnauthorizedModelUse` Errority.

### Example

A clinical researcher requests access to 100 subjects' EvolutionAuditRecords to study BCI trends in a region. The researcher:

1. Submits a `ModelBuildingConsent` request declaring: "I will train a generative model to predict BCI trajectory. Model will be kept private to my institution for 2 years, then destroyed."
2. Each subject reviews and consents, or declines. Subjects can also redact fields (e.g., "I consent to BCI modeling, but redact my inner-state adjacent metrics").
3. The researcher receives a dataset with:
   - Consented fields from consenting subjects.
   - Redacted fields replaced with noise or zeros (but metadata preserved).
   - Subject IDs replaced with anonymous tokens.
4. The researcher trains the model and publishes the results (e.g., "Predicted BCI in this population shows X trend").
5. Six months later, the researcher is offered a lucrative contract to use the same model for insurance risk scoring.
   - **Correct:** Researcher must seek new consent (different use case) and disclose to subjects before proceeding.
   - **Incorrect:** Researcher uses the model without new consent; this is logged as `UnauthorizedModelUse` Errority and triggers revocation of all future modeling access.

---

## 8. Summary Table: Interstitial Rights and Enforcement

| Principle | Existing Invariant(s) | Interstitial Right | Normative Clause | Technical Requirement |
|-----------|----------------------|-------------------|------------------|----------------------|
| **Forward-only sovereignty** | No rollback/downgrade; EVOLVE-gated; RoH/BCI monotone; Errority-only tightening. | Non-exclusion: refusals are explicit, documented, and appeal-able; prior state never erased. | Refusals must be transparent and not coercive. | `CapabilityRefusal` event type; appeal path in HGO; expiring refusals. |
| **Inner/outer separation** | No neural inputs for governance; inner polytope inviolable. | Non-inference: outer telemetry of X cannot govern Y without symmetric consent. No guilt-by-corridor. | Shared infrastructure must enforce corridor symmetry. | `OuterTelemetryUsePolicy` struct; runtime block on asymmetric predicates; `CrossPartyInference` audit. |
| **Sovereign discipline** | FEAR/PAIN voluntary channels only; explicit consent and abort. | Non-exploitation: no hidden secondary use; proportional benefit-sharing for extraction. | Systems offering discipline must disclose economics upfront and share benefits. | `DisciplineSignal` carries `benefit_sharing_covenant`; secondary-use audits; no penalty on opt-out. |
| **Biowidth-anchored evolution** | BCI/RoH monotone; envelopes tighten only. | Biowidth visibility: subject sees remaining capacity; right to veto non-aligned uses. | Proposals must calculate biowidth impact; subject preserves covenanted floors. | `BiowidthBudget` snapshot in every proposal; type-level enforcement; biowidth covenant registry. |
| **Corridor co-sovereignty** | EcoCorridorContext; FPIC predicates; pluggable profiles. | Mid-flight renegotiation: protocol halts if footprint exceeds corridor; community-level Errority tightening. | Long-running protocols require `MidFlightCovenant`; renegotiation triggers must be honored. | `MidFlightCovenant` mandatory struct; continuous bounds-checking; `CorridorBoundsExceeded` auto-halt. |
| **Cross-species neural safety** | Species-specific boundaries; SpeciesId type parameter; no implicit cross-species conversion. | No cross-species imprinting: reuse forbidden except under high-bar multi-species FPIC and species-first floors. | Each species retains control over its own neural patterns and learning mechanisms. | `CrossSpeciesAdapter<A, B>` with mandatory `multi_species_fpic` and `species_first_floor` fields. Compile-time block on untagged cross-species transforms. |
| **EvolutionAuditRecords interstitial rights** | DID-bound, append-only ALN.evo; rich context. | Scoped readability, selective redaction (no hash-break), no unauthorized model-building. | Field-level access control; model-building requires explicit consent; audit-trail is not automatically open for research. | `ReadershipClass` on all fields; `LocalRedactionRequest` metadata; `ModelBuildingConsent` gating. |

---

## 9. Implementation Roadmap

This specification is designed to be integrated into:

1. **core-contract** Rust crate: Type-level enforcement of SpeciesId, CrossSpeciesAdapter, BiowidthBudget.
2. **orchestration** crate: Runtime guards for mid-flight covenants, outer-telemetry policy, field-level audit filtering.
3. **HGO / Policy Profile** specifications: Normative clauses (benefit-sharing economics, species-first floors, renegotiation triggers).
4. **ALN.evo** schema: New event types (`CapabilityRefusal`, `CorridorBoundsExceeded`, `UnauthorizedSecondaryUse`, `CrossPartyInference`, `CrossSpeciesNeuralUse`, `UnauthorizedModelUse`).

Each implementer must:

- Not fill normative gaps (what counts as "fair" benefit-sharing, which EcoAdmissible floors apply) with code alone.
- Defer to documented, participatory community governance and Indigenous data sovereignty frameworks.
- Publish their instantiation of each interstitial right in a versioned, auditable HGO or policy profile.

---

## 10. References

- **Core SNC/SNCHIT stack:** Morpheus-Client reconciliation architecture; EvolutionAuditRecords; RoH/BCI monotonicity.
- **Neurorights principles:** EU Neurorights Directive; Human Rights Council guidance.
- **Indigenous Data Sovereignty:** CARE principles (Collective Benefit, Authority to Control, Responsibility, Ethics); OCAP (Ownership, Control, Access, Possession).
- **Ecological governance:** EcoAdmissible polytopes; Phoenix Corridor protocols; BeeAdmissible and TreeAdmissible safety floors.
- **Participatory governance:** FPIC frameworks; community-level Errority review; legitimate steward authorization.

---

## 11. Document Status and Next Steps

**Status:** Formal Specification v1.0 – Ready for GitHub check-in and Rust implementation.

**Next steps:**

1. Integrate into `github.com/Doctor0Evil/Morpheus` as `specs/interstitial-rights-v1.md`.
2. Cross-reference in SNC core crate's top-level README.
3. Begin Rust implementation of type-level and runtime enforcement hooks (see `specs/hybrid-enforcement-blueprint-v1.md`).
4. Draft community governance profiles (HGOs) specifying which interstitial rights apply in each corridor.

---

**Document prepared by:** bostrom18 (DID: did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7)  
**Version control:** `specs/interstitial-rights-v1.md`  
**License:** MIT / ALN-sovereign