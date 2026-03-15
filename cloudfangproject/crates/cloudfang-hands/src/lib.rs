//! # CloudFang Hands — Autonomous SysOps Agents
//!
//! Pre-built autonomous capability packages that run on schedules.

pub mod backup;
pub mod monitor;
pub mod remediate;
pub mod scale;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// The lifecycle state of a Hand.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HandState {
    Inactive,
    Active,
    Running,
    Paused,
    Error(String),
}

impl std::fmt::Display for HandState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inactive => write!(f, "⚫ Inactive"),
            Self::Active => write!(f, "🟢 Active"),
            Self::Running => write!(f, "🔵 Running"),
            Self::Paused => write!(f, "🟡 Paused"),
            Self::Error(e) => write!(f, "🔴 Error: {}", e),
        }
    }
}

/// Result of a single Hand execution cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandReport {
    pub hand_name: String,
    pub timestamp: String,
    pub summary: String,
    pub actions_taken: Vec<String>,
    pub issues_found: usize,
    pub issues_resolved: usize,
}

/// Trait that all Hands must implement.
#[async_trait]
pub trait Hand: Send + Sync {
    /// The name of this Hand.
    fn name(&self) -> &str;

    /// Human-readable description.
    fn description(&self) -> &str;

    /// Current lifecycle state.
    fn state(&self) -> &HandState;

    /// Execute one cycle of this Hand's work.
    async fn execute(
        &mut self,
        session: &mut cloudfang_ops::OpenStackSession,
        store: cloudfang_store::Store,
    ) -> anyhow::Result<HandReport>;

    /// Activate this Hand.
    fn activate(&mut self);

    /// Pause this Hand (preserving state).
    fn pause(&mut self);

    /// Deactivate this Hand.
    fn deactivate(&mut self);
}
