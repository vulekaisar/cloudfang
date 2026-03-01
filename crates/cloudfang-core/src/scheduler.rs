//! Scheduler — Cron-based task scheduling for Hands.

use anyhow::Result;
use chrono::{DateTime, Utc};

/// A scheduled job.
pub struct ScheduledJob {
    pub name: String,
    pub cron_expr: String,
    pub last_run: Option<DateTime<Utc>>,
    pub enabled: bool,
}

impl ScheduledJob {
    pub fn new(name: &str, cron_expr: &str) -> Self {
        Self {
            name: name.to_string(),
            cron_expr: cron_expr.to_string(),
            last_run: None,
            enabled: true,
        }
    }

    /// Check if this job should run now based on a simple interval approach.
    /// For Phase 1, we use interval-based scheduling instead of full cron parsing.
    pub fn should_run(&self, interval_secs: u64) -> bool {
        if !self.enabled {
            return false;
        }
        match self.last_run {
            None => true,
            Some(last) => {
                let elapsed = Utc::now().signed_duration_since(last);
                elapsed.num_seconds() >= interval_secs as i64
            }
        }
    }

    pub fn mark_run(&mut self) {
        self.last_run = Some(Utc::now());
    }
}

/// The scheduler that manages multiple jobs.
pub struct Scheduler {
    jobs: Vec<ScheduledJob>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub fn add_job(&mut self, job: ScheduledJob) {
        tracing::info!("📅 Scheduled job: {} ({})", job.name, job.cron_expr);
        self.jobs.push(job);
    }

    /// Get jobs that should run now.
    pub fn due_jobs(&mut self, interval_secs: u64) -> Vec<&str> {
        self.jobs
            .iter()
            .filter(|j| j.should_run(interval_secs))
            .map(|j| j.name.as_str())
            .collect()
    }

    pub fn mark_completed(&mut self, name: &str) {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.name == name) {
            job.mark_run();
        }
    }

    pub fn enable(&mut self, name: &str) -> Result<()> {
        match self.jobs.iter_mut().find(|j| j.name == name) {
            Some(job) => {
                job.enabled = true;
                Ok(())
            }
            None => anyhow::bail!("Job '{}' not found", name),
        }
    }

    pub fn disable(&mut self, name: &str) -> Result<()> {
        match self.jobs.iter_mut().find(|j| j.name == name) {
            Some(job) => {
                job.enabled = false;
                Ok(())
            }
            None => anyhow::bail!("Job '{}' not found", name),
        }
    }

    pub fn list_jobs(&self) -> &[ScheduledJob] {
        &self.jobs
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
