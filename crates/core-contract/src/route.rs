#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RoutePrivacy {
    /// No special biofield constraints; standard corridor / FPIC rules apply.
    PublicLike,
    /// Biofield-adjacent but exportable under strict FPIC / policy.
    BiofieldScoped,
    /// Must remain on-SoC; only anonymized / distilled outputs may leave.
    BiofieldPrivate,
}

#[derive(Clone, Debug)]
pub struct RouteId(pub String);

/// Minimal corridor / consent view needed for routing.
#[derive(Clone, Debug)]
pub struct RouteCorridorContext {
    pub route_id: RouteId,
    pub corridor_id: String,      // e.g. "protected-desert-phoenix"
    pub fpic_allowed_export: bool, // true iff FPIC / policy permits raw export from this corridor
}

/// Motion / XR route descriptor used by the router.
#[derive(Clone, Debug)]
pub struct MotionRouteDescriptor {
    pub id: RouteId,
    pub privacy: RoutePrivacy,
    pub corridor: RouteCorridorContext,
    /// True when this route carries raw / high-fidelity motion or biophysical XR signals.
    pub carries_biofield_signal: bool,
}
