//! Metrics Collection — Ceilometer/Gnocchi integration (stub)
//!
//! This module provides a simplified interface for collecting
//! resource metrics. In production, connect to Gnocchi or Ceilometer.

use serde::{Deserialize, Serialize};

/// A resource metric data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDataPoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub resource_id: String,
    pub metric_name: String,
    pub value: f64,
    pub unit: String,
}

/// Summary of resource health metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceHealthSummary {
    pub resource_id: String,
    pub resource_name: String,
    pub cpu_percent: Option<f64>,
    pub memory_percent: Option<f64>,
    pub disk_percent: Option<f64>,
    pub network_in_bytes: Option<u64>,
    pub network_out_bytes: Option<u64>,
    pub status: HealthStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

impl HealthStatus {
    pub fn emoji(&self) -> &str {
        match self {
            Self::Healthy => "🟢",
            Self::Warning => "🟡",
            Self::Critical => "🔴",
            Self::Unknown => "⚪",
        }
    }
}

/// Evaluate health status based on CPU/memory/disk thresholds.
pub fn evaluate_health(cpu: Option<f64>, mem: Option<f64>, disk: Option<f64>) -> HealthStatus {
    let critical_threshold = 90.0;
    let warning_threshold = 75.0;

    for val in [cpu, mem, disk].iter().flatten() {
        if *val >= critical_threshold {
            return HealthStatus::Critical;
        }
    }
    for val in [cpu, mem, disk].iter().flatten() {
        if *val >= warning_threshold {
            return HealthStatus::Warning;
        }
    }
    if cpu.is_none() && mem.is_none() && disk.is_none() {
        return HealthStatus::Unknown;
    }
    HealthStatus::Healthy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_evaluation() {
        assert_eq!(
            evaluate_health(Some(50.0), Some(60.0), Some(40.0)),
            HealthStatus::Healthy
        );
        assert_eq!(
            evaluate_health(Some(80.0), Some(60.0), Some(40.0)),
            HealthStatus::Warning
        );
        assert_eq!(
            evaluate_health(Some(95.0), Some(60.0), Some(40.0)),
            HealthStatus::Critical
        );
        assert_eq!(evaluate_health(None, None, None), HealthStatus::Unknown);
    }
}
