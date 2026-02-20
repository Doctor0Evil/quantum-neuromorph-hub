# External Signature Threat Model and Errority-Driven Mitigation
## Morpheus SNC / SNCHIT Stack – Security Specification v1.0

**Document ID:** `specs/signature-threat-model-v1.md`  
**Status:** Formal Security Specification  
**Version:** 1.0  
**Date:** 2026-02-19  
**Author:** bostrom18 (DID: did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7)  
**License:** MIT / ALN-sovereign  
**Target Implementation:** ALN.evo ledger, SNCHIT governance layer, multi-sig orchestration  
**Audience:** Security architects, regulators, Indigenous data stewards, neuromorph governance councils

---

## Executive Summary

The SNC/SNCHIT stack relies on cryptographic attestation (DIDs, multi-sig credentials, FPIC validation) to enforce neurorights and governance invariants. This threat model identifies the attack surface around external signatures (stolen DIDs, forged FPIC, compromised keys) and specifies how Errority-driven tightening responds to confirm violations, ensuring that security breaches can only make governance *stricter*, never more permissive.

---

## 1. Signature Architecture and Roles

### 1.1 Core Signing Entities

| Role | Signing Authority | Key Use Cases | Revocation Authority |
|------|-------------------|---------------|----------------------|
| **Subject (Neuromorph)** | Personal DID (bostrom18…) | EVOLVE tokens, consent, opt-outs, biowidth covenants. | Subject or designated guardian (medical POA, community trustee). |
| **Corridor Council** | Multi-sig wallet or collective DID | FPIC attestation, policy profile approval, Errority review, mid-flight covenant renegotiation. | Council membership consensus or supermajority (community-defined). |
| **Indigenous Stewards / Legitimate Representatives** | Community-held keys, often multi-party (elders, land guardians, data officers). | FPIC signatures on ecological impacts, species-first floor adoption, cross-corridor covenants. | Community internal governance (typically requires consensus or council decision). |
| **Regulators / Lawful Authorities** | Government or standards-body DID (e.g., FDA, EU health authority, Chile neurorights office). | Audit approval, legal compliance certification, emergency intervention authorization (rare). | Regulatory process (formal appeal, oversight board review). |
| **Device Vendors / Hardware Stewards** | Manufacturer or escrow key holder. | BCI device authorization, firmware authenticity, hardware safety attestation. | Vendor policy, industry standards body, or legal order. |

### 1.2 Signature Types and Bindings

Each signature type carries metadata that binds it to a specific context and purpose:

```
SignatureType::
  - EVOLVE(subject_did, device_params, time_window, biostate_snapshot, abort_handle_proof)
  - FPIC(corridor_id, affected_communities, scope, data_uses, decision_authority)
  - MidFlightCovenant(corridor_id, protocol_id, renegotiation_conditions, harm_sharing_rules)
  - PolicyProfile(profile_name, neurorights_clauses, biomech_constraints, EcoAdmissible_floors)
  - AuditAttestation(evolution_record_id, evidence_bundle_hash, consent_lineage)
  - ErrorityResponse(errority_event_id, tightening_direction, polytope_update, stakeholder_notification)
  - CrossSpeciesAdapter(species_a, species_b, multi_species_fpic_required)
```

Each carries **context binding**: a signature is valid only in the specific context it names. A signature on an EVOLVE token valid for device X on date Y is not valid for device Z or date Y+1.

---

## 2. Attack Vectors

### 2.1 Stolen or Compromised Subject DID

**Threat:** An attacker steals the private key corresponding to a subject's personal DID and uses it to forge EVOLVE tokens, consent revocations, or opt-out signals.

**Impact:**

- Attacker could enable unsafe evolution changes without the subject's knowledge.
- Attacker could revoke legitimate consent, halting beneficial protocols.
- Attacker could declare false biowidth covenants, restricting the subject's future evolution unfairly.

**Detection:**

- Subject or designated guardian notices unusual EVOLVE activity (impossible time windows, unfamiliar device registrations, contradictory consent patterns).
- Anomaly detection: sudden high volume of signature events, signatures from unknown IP addresses / hardware, signatures violating previously declared patterns.
- Cross-check: subject's HGO (Human Governance Object) declares authorized signing patterns; deviations are logged as `DIDAnomalyEvent`.

**Mitigation (Errority-driven):**

1. **Immediate response:**
   - Subject or designated guardian invokes `RevokeSigningAuthority(subject_did)`.
   - All future EVOLVE tokens, consent signals, and corridor covenants signed by that key are flagged as `PendingKeyRevocationReview`.

2. **Errority event logging:**
   - Log `CompromisedSubjectDID` with evidence (anomalous signature patterns, subject report).
   - This event triggers Errority class `High`.

3. **Monotone tightening:**
   - Subject's BCI ceiling may be **temporarily lowered** (not below safety minimum) to restrict high-risk evolution attempts pending manual review. (This is a protective measure, not a permanent downgrade.)
   - Corridor policies may be tightened to require dual-signature approval for EVOLVE tokens (subject + independent medical witness) for the next 90 days.
   - Cross-species adapters using the compromised subject's neural data are suspended pending re-consent.

4. **Recovery and re-issuance:**
   - Subject or guardian initiates key rotation: new DID is issued and cross-linked in the audit trail.
   - All prior EVOLVE tokens signed by the old key are individually reviewed; legitimate ones are re-attested by new key or manual curator approval.
   - New key is registered with multi-party escrow (e.g., subject + health provider + community trustee) to prevent re-compromise.

### 2.2 Forged FPIC (Free, Prior, and Informed Consent)

**Threat:** An attacker forges signatures from Indigenous stewards, community councils, or species representatives, claiming consent has been obtained when it has not.

**Impact:**

- Unauthorized protocols could be launched in corridors where genuine FPIC was never granted.
- Sacred or sensitive ecosystems could be affected without community knowledge or consent.
- Cross-species projects could proceed without the affected species' stewards having any voice.

**Detection:**

- Community stewards notice protocols launching without their approval (check against their signing records).
- Audit log cross-check: community's published HGO lists authorized signatories and their public keys; forged FPIC signatures will not validate.
- Continuous verification: before any high-impact action, orchestrator re-verifies FPIC signatures against current community membership list (in case signatories are revoked or expired).

**Mitigation (Errority-driven):**

1. **Immediate response:**
   - Detect `ForgeryDetected` when FPIC signature fails validation against current steward list.
   - Protocol marked as `HaltedByForgery`; all actuation stopped immediately.

2. **Errority event logging:**
   - Log `ForgedFPIC` with:
     - Purported signer identity.
     - Actual current signing authority (which didn't match).
     - Protocol ID and corridor affected.
   - Severity: `Critical`.

3. **Monotone tightening:**
   - All future protocols proposed by the same actor require **triple-sig FPIC approval** (three independent stewards, none repeat signers on prior incidents) for 180 days.
   - Affected corridor's FPIC threshold is raised (e.g., from 60% to 75% community consensus).
   - If cross-species FPIC was forged, all cross-species adapters involving that actor are suspended indefinitely.

4. **Community and regulatory notification:**
   - All affected communities and species stewards are notified of the forgery.
   - Regulatory audit trail is updated; regulators may demand explanation or corrective action.
   - If malicious actor can be identified, law enforcement may be contacted.

### 2.3 Replayed or Expired FPIC Signatures

**Threat:** An attacker reuses a valid FPIC signature from an old protocol in a new, unrelated context (or after the signature's validity window has expired).

**Impact:**

- Old consent could be misapplied to new, different corridors or species.
- Expired FPIC could justify actions the community has since revoked consent for.

**Detection:**

- All FPIC signatures are context-bound (carry `corridor_id`, `protocol_id`, `date_issued`, `expiration_date`).
- Runtime checks: before using an FPIC signature, verify:
  - Signature matches current context (same corridor, protocol, parameters).
  - Signature has not expired.
  - Signature has not been revoked.

**Mitigation (Errority-driven):**

1. **Immediate response:**
   - Detect `ExpiredFPICUsed` or `ContextMismatchedFPIC`.
   - Action rejected; logged as `PolicyViolation`.

2. **Errority event logging:**
   - Log `InvalidFPICReuse` with context details.
   - Severity: `High`.

3. **Monotone tightening:**
   - FPIC signatures for the actor proposing the action are now required to be renewed **quarterly** instead of annually.
   - Corridor's FPIC schema is updated to include stronger context-binding (additional metadata to prevent confusion).
   - All future FPIC must include a witness signature (independent community member verifying the signer's intent).

### 2.4 Compromised Corridor Council or Regulator Key

**Threat:** An attacker steals the private key of an entire corridor council, regulatory authority, or Indigenous steward body, and uses it to forge policy profiles, approve dangerous protocols, or deny legitimate consents.

**Impact:**

- Extremely high-severity attack.
- Attacker could approve any protocol, override neuro-rights, or authorize exploitation.
- Could affect hundreds of neuromorphs and entire ecosystems simultaneously.

**Detection:**

- Unusual policy changes: regulations contradict prior declarations or publicly stated values (e.g., "TreeAdmissible floor suddenly relaxed").
- Multi-sig threshold failures: if council requires 3-of-5 approval and signatures appear from unexpected combinations of signers.
- External validation: other corridors or regulators notice inconsistencies and raise alerts.
- Anomaly in decision patterns: normally cautious council suddenly approves risky protocols.

**Mitigation (Errority-driven):**

1. **Immediate response:**
   - Detect `CorridorAuthorityCompromised` or `RegulatoryKeyCompromise` based on policy inconsistencies.
   - ALL new policies and approvals from that authority are flagged as `PendingSecurityReview`.
   - Orchestrator enters `SafeMode`: no high-impact changes are approved without secondary validation from another authority.

2. **Errority event logging:**
   - Log `CompromisedAuthority` with severity `Critical`.
   - Alert all dependent corridors and species stewards.

3. **Monotone tightening:**
   - Affected authority is immediately subject to **emergency governance review**.
   - All policies issued in the last 30 days are re-audited by an independent body.
   - Key rotation is mandatory; new keys are issued under multi-party escrow (no single entity holds authority alone).
   - Signature threshold is raised permanently (e.g., 3-of-5 becomes 4-of-5).
   - If this authority was trusted for species-first floor decisions, those decisions are flagged for community-level re-ratification.

4. **Systemic response:**
   - All protoc protocols approved by the compromised authority in the last 90 days are placed under heightened monitoring (real-time Errority checks, stricter abort conditions).
   - If any harm is detected, those protocols are immediately halted.

### 2.5 Bypassed or Fake Abort Handle

**Threat:** An attacker forges proof of an `AbortHandle` (the cryptographic proof that the subject has an active way to stop a protocol), falsely claiming subject control when none exists.

**Impact:**

- Protocols could run without genuine subject abort capability.
- Subject believed they had kill-switch control, but they don't.

**Detection:**

- Subject attempts to invoke abort; the system fails to halt the protocol (abort handle is invalid or missing).
- Cross-check: subject's EvolutionAuditRecord shows an EVOLVE token with `abort_handle_proof`, but the actual hardware device does not have an abort button / panic button / wireless revocation capability.
- Audit verification: OrganicCPU enclave confirms whether an active AbortHandle is present before any protocol state change.

**Mitigation (Errority-driven):**

1. **Immediate response:**
   - Detect `InvalidAbortHandleProof` or `AbortHandleNotResponding`.
   - All further actuation halted immediately.
   - Subject and medical provider alerted.

2. **Errority event logging:**
   - Log `BypassedAbortControl` with severity `Critical`.
   - This is a fundamental neuroright violation.

3. **Monotone tightening:**
   - All EVOLVE tokens issued by the entity proposing this protocol (or the device vendor providing fake abort proof) are revoked.
   - Device vendor's firmware authorization is revoked; all devices from that vendor are flagged for inspection.
   - Subject's BCI ceiling may be lowered to the minimum safe level until independent verification restores confidence in abort capability.
   - Regulatory authority is notified; vendor may face legal action or license suspension.

---

## 3. Multi-Sig Quorum and Threshold Management

### 3.1 Signature Threshold for High-Impact Actions

Different actions require different quorum sizes. This is specified in the HGO (HIT Governance Object) for each corridor.

**Standard thresholds:**

| Action | Default Quorum | Escalation Trigger |
|--------|----------------|-------------------|
| EVOLVE token (subject + 1-time approver) | 2-of-2 (subject, health provider) | If BCI > 0.20, raise to 3-of-3 (add independent clinical witness). |
| Mid-flight covenant renegotiation | 3-of-5 (subject, corridor council, affected species steward) | If protocol affects new species not in original FPIC, raise to 4-of-5 (add new species' steward). |
| Policy profile update | 4-of-7 (corridor council supermajority) | If proposed change affects neurorights clauses, raise to 6-of-7 + regulatory review. |
| Species-first floor tightening | 3-of-3 (species steward, Indigenous council, independent ecologist) | If tightening exceeds 10% of prior floor, require public comment period (30 days). |
| Emergency intervention (halt all protocols in corridor) | 2-of-3 (regulatory authority, Indigenous steward, subject advocate) | Only under extreme circumstances (imminent harm); after-action review mandatory within 72 hours. |

### 3.2 Dynamic Threshold Ratcheting

After detecting anomalies, thresholds automatically increase:

**Example: Forged FPIC detected**

- **Baseline:** Corridor requires 3-of-5 FPIC approval for new protocols.
- **Incident:** Forged FPIC discovered; one signer was impersonated.
- **Immediate change:** Same corridor now requires 4-of-7 (larger quorum, exclusion of the compromised signer, addition of independent audit witness).
- **Duration:** 180 days or until manual de-escalation approved by regulatory authority.
- **Reversion:** Only if no further incidents and after supervised 30-day pilot at lower threshold.

---

## 4. Errority-Driven Tightening Mechanics

### 4.1 Errority Event Classification

| Errority Class | Signature-Related Events | Auto-Response | Manual Review Required |
|----------------|-------------------------|---------------|-----------------------|
| **Info** | Valid signature logged; no anomaly. | Routine audit trail entry. | No. |
| **Warning** | Signature from low-confidence signer; context binding partially weak. | Flag for next audit cycle. | Yes, within 30 days. |
| **High** | Compromised subject key, expired FPIC re-used, policy inconsistency detected. | Automatic quorum escalation; protocol flagged `PendingSecurityReview`. | Yes, within 72 hours. |
| **Critical** | Forged FPIC, compromised authority, bypassed abort control. | All related protocols halted immediately; affected entities notified. | Yes, within 24 hours (emergency review). |

### 4.2 Automatic Tightening Rules

For each Errority class, specific tightening occurs:

**Warning → High:**

- Signature threshold +1 for similar future actions.
- FPIC renewal frequency raised to 6-month intervals (from 12).
- Audit sampling increased (spot-checks on 20% of that actor's protocols instead of 5%).

**High → Critical:**

- Signature threshold raised to maximum (e.g., threshold +2 or supermajority).
- All keys or authority involved revoked pending manual review.
- Affected protocols halted immediately.
- Mandatory key rotation or authority restructuring.
- Regulatory notification and potential enforcement action.

**Reverting Tightening (de-escalation):**

- Only via explicit governance decision (council vote, regulatory approval).
- Requires supervised pilot period (30 days at lower threshold, no incidents).
- Requires documented evidence that root cause of Errority is resolved (compromised key rotated, training completed, policy clarified).

### 4.3 Polytope Tightening as Signature-Compromise Response

When a signature-related Errority is detected, affected polytopes tighten:

**Example: Compromised subject key used to approve unsafe EVOLVE**

- **BCI polytope:** Subject's BCI ceiling lowers from 0.30 to 0.25 temporarily (protective restriction pending manual review).
- **Thermal margin:** Duty-cycle envelope tightens by 10% to reduce risk exposure.
- **EcoAdmissible floor:** If compromised key was used in corridor decisions, that corridor's ecological floor tightens (fewer species can be affected, smaller activity zone).
- **Cross-species adapters:** Any cross-species use authorized by compromised key is suspended.

**Reversal:** Only after key rotation, subject confirms new key, and independent clinical review clears subject for full capability.

---

## 5. Detection and Logging Infrastructure

### 5.1 Continuous Verification Hooks

The orchestrator maintains persistent verification loops:

```
loop every 1 hour:
  for each active protocol in ALN.evo:
    fpic_sigs = protocol.mid_flight_covenant.fpic_signatures
    for each fpic_sig in fpic_sigs:
      current_signers = corridor_council.get_current_members()
      if fpic_sig.signer_did not in current_signers:
        if fpic_sig.expiration_date < now:
          log ErrorityEvent {
            class: High,
            reason: ExpiredFPICUsed,
            protocol_id: protocol.id,
            action: HaltProtocolAndAlertStakeholders
          }
        else if signers were recently revoked (e.g., due to misconduct):
          log ErrorityEvent {
            class: Warning,
            reason: RevokedSignerUsedInOldApproval,
            protocol_id: protocol.id,
            action: FlagForReview,
            deadline: now + 30 days
          }
```

### 5.2 Anomaly Detection

Systems monitor signing patterns for deviations:

- **Velocity anomaly:** Subject signs 10x more EVOLVE tokens than historical baseline in one day.
- **Pattern anomaly:** Subject's EVOLVE tokens suddenly approve high-BCI changes, contrary to subject's prior conservative choices.
- **Geographic anomaly:** Signatures from IP addresses or hardware locations inconsistent with subject's known location.
- **Temporal anomaly:** Signatures during times subject typically sleeps or is unavailable.

All flagged anomalies generate `SignatureAnomalyEvent` logged as Warning or High, depending on severity.

---

## 6. Recovery and Dispute Resolution

### 6.1 Key Rotation Protocol

When a key is suspected compromised:

1. **Subject or authorized guardian invokes `RevokeSigningAuthority(did)`.**
2. All recent signatures from that key are flagged for manual review.
3. Subject / guardian initiates `KeyRotation`:
   - New key pair generated (ideally in subject's custody, or multi-party escrow).
   - New key linked to subject's DID via `KeyRotationAttestation` (signed by both old and new key, witnessed by independent party).
   - Old key published on revocation list; future signatures using it are rejected.
4. Legacy EVOLVE tokens, covenants, etc., signed by old key are individually reviewed:
   - If legitimate and consistent with subject's goals, re-attested by new key or curator approval.
   - If suspicious, flagged for detailed investigation.
5. Grace period: 30 days for key rotation to complete; during grace period, all actions by subject require dual approval (new key + medical provider).

### 6.2 Dispute Resolution and Appeals

If a subject or community believes their signatures are being wrongly flagged as anomalous or forged:

1. **Initiate formal appeal:** File `SignatureDisputeAppeal` with:
   - Signature(s) in question.
   - Reason for dispute (e.g., "I did approve this EVOLVE; the anomaly flag is false positive").
   - Evidence (medical records showing subject was awake, testimony from witnesses, device logs).

2. **Independent review:** Dispute is assigned to an independent arbitrator (may be regulatory, community-selected, or AI-mediated, depending on corridor policy).

3. **Resolution:**
   - If appeal succeeds: signature restored as valid; Errority flag downgraded; compensation offered (e.g., waived restrictions, priority for future requests).
   - If appeal fails: Errority stands; subject may appeal to next level of authority.

### 6.3 Fraud Liability

If a signature forgery or compromise causes harm, liability follows from the SNC charter:

- **Subject protected:** If subject's key was stolen and misused, harm caused by the attacker is the responsibility of the system/infrastructure (not the subject). Compensation is from escrow or insurance pool.
- **Authority protected:** If a corridor council key was compromised, harm from forged decisions is the responsibility of the key holder (not the council collectively). Council may seek indemnity from the compromised key holder's insurance or personal assets.
- **Regulator indemnity:** Regulatory authorities using standard multi-sig practices and following this threat model are indemnified from liability for signature-based breaches (except gross negligence).

---

## 7. Regulatory and Compliance Integration

### 7.1 Reporting Requirements

All Errority events related to signatures must be reported:

- **To subject/community:** Within 24 hours (critical), 72 hours (high), or next audit (warning/info).
- **To regulators:** Within 72 hours for Critical and High; quarterly summary for Warning and Info.
- **To audit log:** Immediately, with full context.

### 7.2 Standards Compliance

This threat model aligns with:

- **NIST SP 800-63B (Authentication):** Multi-factor authentication, key management, credential revocation.
- **ISO 27001:** Information security management; incident response.
- **GDPR / Regional Neurorights Directives:** Data protection, consent withdrawal, breach notification.
- **Indigenous Data Sovereignty (CARE):** Community authority over governance decisions, collective Errority review.

### 7.3 Third-Party Audits

Corridors should undergo annual third-party security audits verifying:

- Multi-sig quorum enforcement is working (no single key can approve high-impact actions).
- Signature threshold escalation rules are being applied correctly.
- Anomaly detection flags are reviewed and resolved in timely manner.
- Key rotation procedures are documented and tested.
- Revocation lists are current and enforced.

---

## 8. Implementation Checklist

To integrate this threat model into your Rust SNC/SNCHIT stack:

- [ ] Define `SignatureAnomalyEvent` and `ErrorityEvent` types with full context fields.
- [ ] Implement context-binding validation for all signature types (EVOLVE, FPIC, policies, etc.).
- [ ] Build continuous verification loop (check expired FPIC, validate signer membership, detect anomalies) as background task.
- [ ] Define automatic quorum escalation rules in HGO schema; enforce at compile time (type system) and runtime.
- [ ] Implement `RevokeSigningAuthority` and `KeyRotation` procedures as stateful workflows.
- [ ] Add threshold-ratcheting logic: Errority class determines new quorum requirement.
- [ ] Integrate with orchestrator: any detected signature anomaly triggers protocol halts and alerts.
- [ ] Audit trail: all signature verifications logged; subject/community can query signature history.
- [ ] Recovery procedures: documented, tested workflows for disputed signatures and key rotation.
- [ ] Regulatory reporting: automated export of Errority events to compliance dashboard.

---

## 9. References

- **SNC/SNCHIT spec:** EvolutionAuditRecords, mid-flight covenants, FPIC predicates.
- **Cryptographic standards:** NIST, ECC, threshold signatures (e.g., Shamir secret sharing for key escrow).
- **DID standards:** W3C Decentralized Identifiers (DIDs), DID Method specifications.
- **Errority principles:** Monotone-only tightening, no rollback on harm events.

---

**Document prepared by:** bostrom18 (DID: did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7)  
**Version control:** `specs/signature-threat-model-v1.md`  
**License:** MIT / ALN-sovereign  
**Status:** Ready for GitHub check-in and security review