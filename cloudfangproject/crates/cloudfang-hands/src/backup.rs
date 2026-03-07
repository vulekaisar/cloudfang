//! Backup Hand — Snapshots VMs/volumes on a schedule.

use crate::{Hand, HandReport, HandState};
use async_trait::async_trait;

pub struct BackupHand {
    state: HandState,
}

impl BackupHand {
    pub fn new() -> Self {
        Self {
            state: HandState::Inactive,
        }
    }
}

impl Default for BackupHand {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Hand for BackupHand {
    fn name(&self) -> &str {
        "backup"
    }
    fn description(&self) -> &str {
        "Snapshots VMs and volumes on schedule, cleans up old snapshots"
    }
    fn state(&self) -> &HandState {
        &self.state
    }

    async fn execute(&mut self) -> anyhow::Result<HandReport> {
        self.state = HandState::Running;
        tracing::info!("💾 Backup Hand executing cycle...");

        // TODO Phase 4: Connect to cloudfang-ops (cinder::create_snapshot)

        let report = HandReport {
            hand_name: self.name().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            summary: "Backup cycle completed (stub)".to_string(),
            actions_taken: vec![],
            issues_found: 0,
            issues_resolved: 0,
        };
        self.state = HandState::Active;
        Ok(report)
    }

    fn activate(&mut self) {
        self.state = HandState::Active;
    }
    fn pause(&mut self) {
        self.state = HandState::Paused;
    }
    fn deactivate(&mut self) {
        self.state = HandState::Inactive;
    }
}
