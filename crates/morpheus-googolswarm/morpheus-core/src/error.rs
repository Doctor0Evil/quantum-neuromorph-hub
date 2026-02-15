use thiserror::Error;

#[derive(Debug, Error)]
pub enum MorpheusError {
    #[error("rights violation: {0}")]
    RightsViolation(String),
    #[error("capability violation: {0}")]
    CapabilityViolation(String),
    #[error("reversal forbidden: {0}")]
    ReversalForbidden(String),
    #[error("parsing error: {0}")]
    ParseError(String),
    #[error("ledger error: {0}")]
    LedgerError(String),
}
