use crate::route::{MotionRouteDescriptor, RoutePrivacy};

/// Router decision for a motion / XR route.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RouteDecision {
    /// Route may forward packets off-SoC (subject to lower-level transport / crypto).
    AllowExport,
    /// Route may be used on-SoC only; no raw export.
    OnSocOnly,
    /// Route must not be established at all.
    Reject,
}

/// Pure, non-actuating guard for motion / XR routing.
/// Implementations may consult ALN shards, FPIC ledgers, or policy profiles,
/// but they never perform I/O or actuation themselves.
pub trait MotionRouteGuard {
    /// Decide how a given motion / XR route may be used.
    fn decide_route(&self, route: &MotionRouteDescriptor) -> RouteDecision;
}

/// Default guard encoding `biofield_private` behavior:
/// - If privacy == BiofieldPrivate and carries_biofield_signal:
///     - always OnSocOnly (never AllowExport), regardless of FPIC.
/// - Else if carries_biofield_signal and FPIC does NOT allow export:
///     - Reject.
/// - Else:
///     - AllowExport.
#[derive(Clone, Debug)]
pub struct DefaultMotionRouteGuard;

impl MotionRouteGuard for DefaultMotionRouteGuard {
    fn decide_route(&self, route: &MotionRouteDescriptor) -> RouteDecision {
        match route.privacy {
            RoutePrivacy::BiofieldPrivate if route.carries_biofield_signal => {
                RouteDecision::OnSocOnly
            }
            _ if route.carries_biofield_signal && !route.corridor.fpic_allowed_export => {
                RouteDecision::Reject
            }
            _ => RouteDecision::AllowExport,
        }
    }
}
