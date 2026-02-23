data class EcoCorridorContext(
    val version: String = "eco-corridor-context.v1",
    val corridorId: String,
    val ecoImpact: EcoImpactMetrics,
    val fpicIdsState: VerifiableConsent?,   // null = no FPIC yet
    val neurorightsCapsule: NeurorightsCapsule
)

data class EcoImpactMetrics(
    val soilHealth: Float,
    val waterQuality: Float,
    val microbiomeDiversity: Float,
    val corridorResilience: Float
)

data class VerifiableConsent(
    val issuerDid: String,
    val subjectCorridorId: String,
    val status: ConsentStatus,
    val issuedAt: String,
    val revokedAt: String?
)

enum class ConsentStatus { Granted, Revoked, Pending }

data class NeurorightsCapsule(
    val flags: List<String> // use same enum strings as Rust
)
