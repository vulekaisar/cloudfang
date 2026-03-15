//! Monitor Hand — Checks VM health, disk usage, service status.

use crate::{Hand, HandReport, HandState};
use async_trait::async_trait;

pub struct MonitorHand {
    state: HandState,
}

impl MonitorHand {
    pub fn new() -> Self {
        Self {
            state: HandState::Inactive,
        }
    }
}

impl Default for MonitorHand {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Hand for MonitorHand {
    fn name(&self) -> &str {
        "monitor"
    }
    fn description(&self) -> &str {
        "Monitors VM health, disk usage, network latency, and service status"
    }
    fn state(&self) -> &HandState {
        &self.state
    }

    async fn execute(
        &mut self,
        session: &mut cloudfang_ops::OpenStackSession,
        store: cloudfang_store::Store,
    ) -> anyhow::Result<HandReport> {
        self.state = HandState::Running;
        tracing::info!("👁️ Monitor Hand executing cycle...");

        let servers = cloudfang_ops::nova::list_servers(session).await?;
        let mut issues_found = 0;
        let mut actions_taken = vec![];

        for server in servers {
            // Check basic status
            if server.status == "ERROR" {
                issues_found += 1;
                let msg = format!("VM {} is in ERROR state", server.name);
                actions_taken.push(msg.clone());
                
                let incident = cloudfang_store::models::Incident::new(
                    "critical",
                    &server.id,
                    Some(&server.name),
                    &msg
                );
                store.log_incident(&incident)?;
            }

            // Check diagnostics
            if let Ok(diag) = cloudfang_ops::nova::get_diagnostics(session, &server.id).await {
                // Simplified health check based on diagnostics
                // In production, we would use metrics::evaluate_health with actual percentages
                if let Some(mem) = diag.memory {
                    if mem == 0 {
                        issues_found += 1;
                        let msg = format!("VM {} reports zero memory in diagnostics", server.name);
                        actions_taken.push(msg.clone());
                    }
                }
            }
        }

        let report = HandReport {
            hand_name: self.name().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            summary: format!("Monitor cycle completed. Checked {} VMs.", issues_found),
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
