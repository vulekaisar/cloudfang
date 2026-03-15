//! Scale Hand — Analyzes load and recommends/performs scaling.

use crate::{Hand, HandReport, HandState};
use async_trait::async_trait;

pub struct ScaleHand {
    state: HandState,
}

impl ScaleHand {
    pub fn new() -> Self {
        Self {
            state: HandState::Inactive,
        }
    }
}

impl Default for ScaleHand {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Hand for ScaleHand {
    fn name(&self) -> &str {
        "scale"
    }
    fn description(&self) -> &str {
        "Analyzes load metrics, recommends and performs scale up/down"
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
        tracing::info!("📈 Scale Hand executing cycle...");

        let servers = cloudfang_ops::nova::list_servers(session).await?;
        let mut actions_taken = vec![];
        let issues_found = 0;

        for server in servers {
            if server.status == "ACTIVE" {
                if let Ok(diag) = cloudfang_ops::nova::get_diagnostics(session, &server.id).await {
                    // Check CPU time or memory as a proxy for load
                    if let Some(cpu_time) = diag.cpu0_time {
                        // Very basic logic: if cpu0_time exists, we log it.
                        // Real logic would compare with previous values to get %
                        tracing::debug!("VM {} CPU Time: {}", server.name, cpu_time);
                    }
                    
                    if let (Some(mem), Some(actual)) = (diag.memory, diag.memory_actual) {
                        if actual > 0 && (mem as f64 / actual as f64) < 0.2 {
                            actions_taken.push(format!("Suggestion: Scale up VM {} (Low memory efficiency)", server.name));
                        }
                    }
                }
            }
        }

        let report = HandReport {
            hand_name: self.name().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            summary: format!("Scale analysis completed for {} servers.", actions_taken.len()),
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
