//! Remediate Hand — Auto-remediates common issues.

use crate::{Hand, HandReport, HandState};
use async_trait::async_trait;

pub struct RemediateHand {
    state: HandState,
}

impl RemediateHand {
    pub fn new() -> Self {
        Self {
            state: HandState::Inactive,
        }
    }

    async fn send_telegram_alert(&self, msg: &str) {
        if let (Ok(token), Ok(chat_id)) = (std::env::var("TELEGRAM_BOT_TOKEN"), std::env::var("TELEGRAM_CHAT_ID")) {
            let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
            let client = reqwest::Client::new();
            let _ = client.post(&url)
                .json(&serde_json::json!({
                    "chat_id": chat_id,
                    "text": msg,
                    "parse_mode": "HTML"
                }))
                .send()
                .await;
        }
    }
}

impl Default for RemediateHand {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Hand for RemediateHand {
    fn name(&self) -> &str {
        "remediate"
    }
    fn description(&self) -> &str {
        "Auto-remediates issues: restart VMs, clear disk, reconnect network"
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
        tracing::info!("🔧 Remediate Hand executing cycle...");

        let mut issues_found = 0;
        let mut issues_resolved = 0;
        let mut actions_taken = vec![];

        // 1. Get unresolved incidents
        let incidents = store.list_incidents(50)?;
        let unresolved: Vec<_> = incidents.into_iter().filter(|i| !i.resolved).collect();

        for inc in unresolved {
            issues_found += 1;
            
            // 2. Logic for VM ERROR state
            if inc.description.contains("in ERROR state") {
                tracing::info!("Attempting remediation for VM {}", inc.resource_id);
                
                let action = cloudfang_ops::nova::ServerAction::Reboot(cloudfang_ops::nova::RebootType::Hard);
                match cloudfang_ops::nova::server_action(session, &inc.resource_id, action).await {
                    Ok(_) => {
                        let msg = format!("✅ <b>Remediation Successful</b>\nWolves have rebooted VM <code>{}</code> that was in ERROR state.", inc.resource_id);
                        actions_taken.push(msg.clone());
                        store.resolve_incident(&inc.id, &msg)?;
                        issues_resolved += 1;
                        self.send_telegram_alert(&msg).await;
                    }
                    Err(e) => {
                        tracing::error!("Remediation failed for VM {}: {}", inc.resource_id, e);
                        self.send_telegram_alert(&format!("❌ <b>Remediation Failed</b>\nFailed to reboot VM <code>{}</code>: {}", inc.resource_id, e)).await;
                    }
                }
            }
        }

        let report = HandReport {
            hand_name: self.name().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            summary: format!("Remediate cycle completed. Processed {} incidents, resolved {}.", issues_found, issues_resolved),
            actions_taken,
            issues_found,
            issues_resolved,
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
