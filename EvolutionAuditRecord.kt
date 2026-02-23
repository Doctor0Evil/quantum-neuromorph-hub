data class EvolutionAuditRecord(
    val version: String = "evolution-audit-record.v1",
    val txId: String,
    val subjectDid: String?, // null when corridor_presence_only
    val corridorContext: EcoCorridorContext,
    val hitGovernance: HitGovernanceObject,
    val preSovereignty: SovereigntySnapshot,
    val postSovereignty: SovereigntySnapshot,
    val distilledKnowledgeHash: String,
    val decision: Decision
)

data class SovereigntySnapshot(
    val sovereigntyLevel: Int
)

enum class Decision { Allow, DeferToHuman, Deny }
