class EcoCorridorClient {
  constructor(config) {
    this.config = config; // { corridorId, strength, neurorightsCapsule }
  }

  buildContext(ecoDelta, fpicState) {
    return {
      corridor_id: {
        tier: this.config.corridorId.tier,
        code: this.config.corridorId.code,
        version: this.config.corridorId.version
      },
      strength: this.config.strength,
      fpic: {
        fpic_granted: fpicState.fpicGranted,
        revocable: fpicState.revocable,
        last_decision_utc: fpicState.lastDecisionUtc,
        community_veto_active: fpicState.communityVetoActive
      },
      neurorights: {
        inner_outer_enforced: this.config.neurorights.innerOuterEnforced,
        neural_data_safety_only: this.config.neurorights.neuralDataSafetyOnly,
        requires_opt_out_channels: this.config.neurorights.requiresOptOut,
        forbids_inner_for_access: this.config.neurorights.forbidsInnerForAccess
      },
      eco: {
        delta_emissions_co2e: ecoDelta.co2e,
        delta_pm25: ecoDelta.pm25,
        delta_water_use_m3: ecoDelta.waterM3,
        delta_heat_index_c: ecoDelta.heatC
      },
      jurisdiction_profile_id: this.config.jurisProfileId || null
    };
  }

  buildSncPayload(plane, intentLabel, ecoCostNj, riskScore) {
    return {
      plane,
      intent_label: intentLabel,
      eco_cost_nj: ecoCostNj,
      risk_score: riskScore
    };
  }
}

// Example wiring (Phoenix pilot under GRIC corridor):
// const client = new EcoCorridorClient({
//   corridorId: { tier: "Tribal", code: "tribal.gric-epa-2024", version: "2024.1" },
//   strength: "HardVeto",
//   neurorights: {
//     innerOuterEnforced: true,
//     neuralDataSafetyOnly: true,
//     requiresOptOut: true,
//     forbidsInnerForAccess: true
//   },
//   jurisProfileId: "state.co-neuraldata-2024"
// });
