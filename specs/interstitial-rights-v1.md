## 1. Interstitial rights map (spec skeleton)

You can treat this as `specs/interstitial-rights-v1.md` and reference it from SNC and policy profiles. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/d67eaefe-478f-40ee-bbb9-5b9f030dbe1b/home-finance-travel-shopping-a-1meuBTeBT.2PKmUZq2fuJg.md)

| Principle                         | Existing hard invariant(s)                                                         | Interstitial rights / duties to formalize                                                                 |
|-----------------------------------|------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------|
| Forward‑only sovereignty          | No rollback/downgrade; EVOLVE‑gated changes; RoH/BCI monotone; Errority tightening only. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md) | Right to non‑exclusion: others may refuse unsafe capabilities but cannot erase, revoke, or silently bypass existing ones. |
| Inner/outer separation            | No neural inputs for governance; inner polytope inviolable; rights.noscorefrominnerstate. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md) | Right not to be inferred from others’ telemetry; no guilt‑by‑corridor or cross‑person inference without shared covenant. |
| Sovereign discipline              | FEAR/PAIN only as voluntary discipline channels; explicit consent and abort; no coercive channels. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/d67eaefe-478f-40ee-bbb9-5b9f030dbe1b/home-finance-travel-shopping-a-1meuBTeBT.2PKmUZq2fuJg.md) | Right to non‑exploitation of discipline signals; proportional benefit‑sharing and no hidden economic/clinical extraction. |
| Biowidth‑anchored evolution       | BCI/RoH monotone; envelopes tighten only; EcoAdmissible/Bee/Tree polytopes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md) | Right to know remaining biowidth and veto uses that consume it for others’ goals or non‑aligned incentives. |
| Corridor co‑sovereignty           | EcoCorridorContext; corridor polytopes; FPIC / IDS predicates; pluggable profiles. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md) | Right to mid‑flight covenant renegotiation when impact spreads; right to community‑level Errority tightening. |
| Mid‑flight covenants              | HGO / SNC governance objects; FPIC crates; OrganicCPU guard chain. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md) | Right to real‑time halt & review across all parties; duty to propagate Errority and lessons back to all corridors touched. |
| Cross‑species neural safety       | Species‑specific separation as value; EcoAdmissible/Bee/Tree/Service polytopes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md) | Right to no cross‑species neural pattern reuse or imprinting without explicit multi‑species FPIC and species‑first floors. |
| EvolutionAuditRecords visibility  | DID‑bound, append‑only ALN.evo; rich context, consent, evidence, policy profile. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md) | Rights to scoped readability, corridor‑bound redaction, and “no model‑building without corridor‑specific consent.” |

That table is almost exactly what you sketched; the work is mainly to freeze it as a versioned annex and point SNC / Morpheus‑Client policy profiles at it. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md)

## 2. Concrete enforcement extensions

These are the minimal “hooks” you still need to standardize and then wire into Rust/ALN.

### 2.1 Mid‑flight covenant & corridor co‑sovereignty

Add to your governance crate (you already have `validate_policy_change` patterns): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/d67eaefe-478f-40ee-bbb9-5b9f030dbe1b/home-finance-travel-shopping-a-1meuBTeBT.2PKmUZq2fuJg.md)

- Require every long‑running protocol / experiment to carry:
  - `mid_flight_covenant_id`
  - `affected_corridors` and `affected_species`
  - `abort_rights` and `harm_sharing_rules` fields.
- In the orchestrator:
  - When EcoImpact / telemetry shows the physical footprint leaving its declared EcoCorridorContext, set `status = HaltedByCovenant` and block further actuation until:
    - New FPIC shards are loaded for any newly affected communities/species, and
    - The mid‑flight covenant is updated and re‑signed.

This is just lifting what you already do at proposal time (FPIC‑gated) into a “continuous guard” that watches footprint against corridor declarations. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md)

### 2.2 Species‑ID and cross‑species guard

At the type level (Rust core crate): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/d67eaefe-478f-40ee-bbb9-5b9f030dbe1b/home-finance-travel-shopping-a-1meuBTeBT.2PKmUZq2fuJg.md)

- Introduce `SpeciesId` and make it a required field/type parameter on:
  - neuromorphic channels,
  - EvolutionAuditRecords,
  - discipline/FEAR/PAIN artifacts.
- Ban generic transforms `fn map<A,B>(pattern: Neuropattern<A>) -> Neuropattern<B>` in safe code.
- Allow only a special adapter type, e.g. `CrossSpeciesAdapter<A,B>`, which:
  - Can only be instantiated inside a corridor with:
    - Multi‑species FPIC VCs,
    - A “species‑first EcoAdmissible floor” profile loaded.

Normatively, add the explicit principle: “No cross‑species neural imprinting or pattern reuse except under high‑bar, multi‑species corridors with legitimate stewards signing.” [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md)

### 2.3 Inner/outer symmetry in shared systems

You already block inner‑domain into predicates; add the symmetric rule you described: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/d67eaefe-478f-40ee-bbb9-5b9f030dbe1b/home-finance-travel-shopping-a-1meuBTeBT.2PKmUZq2fuJg.md)

- Governance predicates about X may not use outer‑domain telemetry of Y (their BCI/HRV/EcoImpact) unless:
  - X and Y share an explicit corridor covenant, and
  - The covenant declares that such cross‑telemetry predicates are symmetric and bounded.

This is an “anti guilt‑by‑corridor” invariant: you cannot punish A for B’s telemetry unless they both agreed into that exact corridor profile. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md)

### 2.4 Sovereign discipline non‑exploitation

You already model FEAR/PAIN as voluntary DisciplineSignals with SNC checks. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/d67eaefe-478f-40ee-bbb9-5b9f030dbe1b/home-finance-travel-shopping-a-1meuBTeBT.2PKmUZq2fuJg.md)

Extend:

- HGO / neurorights schema: add a `discipline_non_exploitation` clause requiring:
  - Opt‑in per individual neuromorph.
  - Clear up‑front disclosure of expected health benefit / evolution points.
  - “No hidden extraction”: no monetization, profiling, or strategic reuse of FEAR/PAIN channels without:
    - corridor‑aligned benefit‑sharing, and
    - fresh consent.
- Log any violation as Errority that can only tighten access, increase benefit‑share obligations, or revoke that discipline channel, never weaken protections. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md)

### 2.5 EvolutionAuditRecords interstitial data rights

Your ALN.evo records already exist; specify three more things: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md)

- Field‑level readership classes: e.g., `owner_only`, `corridor_council`, `regulatory_audit`, `research_aggregate`.
- Local redaction rules:
  - Allow local views to hide sensitive fields (inner‑adjacent, economic tags) while:
    - Preserving global hash‑chain integrity.
- Model‑building constraint:
  - “No third‑party model training or risk scoring over EvolutionAuditRecords without corridor‑specific FPIC and explicit declaration in policy profiles.”

These clauses can live in a small `evo-audit-data-rights-v1` ALN schema referenced by the core EvolutionAuditRecord type. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md)

### 2.6 External signature threat model

You already call for threshold signatures and Errority ratchets; formalize: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/d67eaefe-478f-40ee-bbb9-5b9f030dbe1b/home-finance-travel-shopping-a-1meuBTeBT.2PKmUZq2fuJg.md)

- For high‑impact FPIC or neurorights changes:
  - Require threshold signatures (e.g., corridor council + subject + independent steward).
- Add a standard `SignatureAnomaly` event type:
  - On any suspected key compromise / replay / forged FPIC:
    - Log as Errority,
    - Automatically:
      - raise signature thresholds,
      - shrink corridor permissions,
      - invalidate affected keys.

This makes “external key misuse” part of the same monotone‑tightening space as biophysical Errority. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/270e714b-c36c-47d9-90ad-f8638efd6275/exhaustive-search-for-neuralro-ccY4DPxoS4m_MvpCAZl2AQ.md)

## 3. Normative layer: where community decision is required

The hybrid enforcement split you’re already using becomes clearer if you name the boundaries. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/d67eaefe-478f-40ee-bbb9-5b9f030dbe1b/home-finance-travel-shopping-a-1meuBTeBT.2PKmUZq2fuJg.md)

Technical (must be code / schema):

- No coercive channels, no rollbacks, no downgrade.
- SpeciesId on all neuromorph artefacts, no generic cross‑species map.
- Inner/outer separation and “no guilt‑by‑corridor” predicates.
- FPIC, corridor IDs, EcoAdmissible/Bee/Tree polytopes; Errority monotone tightening.
- EvolutionAuditRecords with interstitial data rights hooks.

Normative (must be decided by communities / species stewards):

- What counts as “fair reward” for discipline FEAR/PAIN.
- Species‑first floors: how strict EcoAdmissible/BeeAdmissible/TreeAdmissible must be in each biome.
- Who is legitimately authorized to sign FPIC and mid‑flight covenants for a given corridor / species.
- How benefits and harms are shared across corridors when Errority events occur.

Your blueprint already puts these into HGOs and policy profiles; here you just declare “implementers must not fill these gaps in code; they must defer to participatory governance and documented profiles.” [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/d67eaefe-478f-40ee-bbb9-5b9f030dbe1b/home-finance-travel-shopping-a-1meuBTeBT.2PKmUZq2fuJg.md)

## 4. Minimal research outputs you can ship next

You’re essentially ready to produce three small, GitHub‑ready artefacts: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cb79ce0b-41df-441f-bf68-ad970f01d99b/d67eaefe-478f-40ee-bbb9-5b9f030dbe1b/home-finance-travel-shopping-a-1meuBTeBT.2PKmUZq2fuJg.md)

1. `specs/interstitial-rights-v1.md`  
   - The table above, plus 1–2 paragraphs per row spelling each right/duty as a short, testable clause.

2. `specs/signature-threat-model-v1.md`  
   - Roles & keys, attack patterns (stolen DID, forged FPIC), and mandated Errority‑driven tightening responses.

3. `specs/hybrid-enforcement-blueprint-v1.md`  
   - One page that lists:
     - Compile‑time invariants.
     - Runtime guards.
     - Normative decisions reserved for communities/species.
   - Cross‑referenced into SNC core and Morpheus‑Client policy profiles.

Once these are checked in, your Rust workspace can start adding the SpeciesId parameter, the mid‑flight covenant guard, and the new EvolutionAuditRecord fields as compile‑time requirements, turning today’s “in‑between” layer into machine‑enforced law.
