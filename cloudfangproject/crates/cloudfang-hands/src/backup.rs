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

    async fn execute(
        &mut self,
        session: &mut cloudfang_ops::OpenStackSession,
        _store: cloudfang_store::Store,
    ) -> anyhow::Result<HandReport> {
        self.state = HandState::Running;
        tracing::info!("💾 Backup Hand executing cycle...");

        let volumes = cloudfang_ops::cinder::list_volumes(session).await?;
        let mut actions_taken = vec![];
        let mut issues_found = 0;

        for vol in volumes {
            if vol.status == "available" || vol.status == "in-use" {
                let snap_name = format!("auto-backup-{}-{}", vol.id, chrono::Utc::now().format("%Y%m%d"));
                tracing::info!("Creating snapshot for volume {}: {}", vol.id, snap_name);
                
                match cloudfang_ops::cinder::create_snapshot(session, &vol.id, &snap_name, None).await {
                    Ok(snap) => {
                        actions_taken.push(format!("Created snapshot {} for volume {}", snap.id, vol.id));
                    }
                    Err(e) => {
                        tracing::error!("Failed to backup volume {}: {}", vol.id, e);
                        issues_found += 1;
                    }
                }
            }
        }

        let report = HandReport {
            hand_name: self.name().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            summary: format!("Backup cycle completed. Created {} snapshots.", actions_taken.len()),
            actions_taken,
            issues_found,
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
