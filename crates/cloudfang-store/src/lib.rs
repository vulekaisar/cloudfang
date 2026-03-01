//! # CloudFang Store — SQLite Persistence Layer
//!
//! Provides incident logging, metrics caching, and audit trails.

pub mod models;

use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;

/// The database manager.
pub struct Store {
    conn: Connection,
}

impl Store {
    /// Open (or create) the SQLite database at the given path.
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        let store = Self { conn };
        store.migrate()?;
        tracing::info!("📦 Store opened at {}", path.display());
        Ok(store)
    }

    /// Open an in-memory database (for testing).
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let store = Self { conn };
        store.migrate()?;
        Ok(store)
    }

    /// Run schema migrations.
    fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS incidents (
                id          TEXT PRIMARY KEY,
                timestamp   TEXT NOT NULL,
                severity    TEXT NOT NULL,
                resource_id TEXT NOT NULL,
                resource_name TEXT,
                description TEXT NOT NULL,
                action_taken TEXT,
                resolved    INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS snapshots_log (
                id          TEXT PRIMARY KEY,
                timestamp   TEXT NOT NULL,
                volume_id   TEXT NOT NULL,
                snapshot_id TEXT NOT NULL,
                name        TEXT,
                status      TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS metrics_cache (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp   TEXT NOT NULL,
                resource_id TEXT NOT NULL,
                metric_name TEXT NOT NULL,
                value       REAL NOT NULL,
                unit        TEXT
            );

            CREATE TABLE IF NOT EXISTS audit_log (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp   TEXT NOT NULL,
                actor       TEXT NOT NULL,
                action      TEXT NOT NULL,
                target      TEXT,
                details     TEXT,
                success     INTEGER NOT NULL DEFAULT 1
            );

            CREATE INDEX IF NOT EXISTS idx_incidents_resource ON incidents(resource_id);
            CREATE INDEX IF NOT EXISTS idx_incidents_timestamp ON incidents(timestamp);
            CREATE INDEX IF NOT EXISTS idx_metrics_resource ON metrics_cache(resource_id, metric_name);
            CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON audit_log(timestamp);
            ",
        )?;
        Ok(())
    }

    // ── Incidents ──

    pub fn log_incident(&self, incident: &models::Incident) -> Result<()> {
        self.conn.execute(
            "INSERT INTO incidents (id, timestamp, severity, resource_id, resource_name, description, action_taken, resolved)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                incident.id, incident.timestamp, incident.severity,
                incident.resource_id, incident.resource_name,
                incident.description, incident.action_taken, incident.resolved as i32
            ],
        )?;
        Ok(())
    }

    pub fn resolve_incident(&self, incident_id: &str, action: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE incidents SET resolved = 1, action_taken = ?1 WHERE id = ?2",
            rusqlite::params![action, incident_id],
        )?;
        Ok(())
    }

    pub fn list_incidents(&self, limit: usize) -> Result<Vec<models::Incident>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, severity, resource_id, resource_name, description, action_taken, resolved
             FROM incidents ORDER BY timestamp DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(rusqlite::params![limit as i64], |row| {
            Ok(models::Incident {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                severity: row.get(2)?,
                resource_id: row.get(3)?,
                resource_name: row.get(4)?,
                description: row.get(5)?,
                action_taken: row.get(6)?,
                resolved: row.get::<_, i32>(7)? != 0,
            })
        })?;
        let mut incidents = Vec::new();
        for row in rows {
            incidents.push(row?);
        }
        Ok(incidents)
    }

    // ── Audit Log ──

    pub fn log_audit(
        &self,
        actor: &str,
        action: &str,
        target: Option<&str>,
        details: Option<&str>,
        success: bool,
    ) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO audit_log (timestamp, actor, action, target, details, success)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![now, actor, action, target, details, success as i32],
        )?;
        Ok(())
    }

    // ── Metrics Cache ──

    pub fn cache_metric(
        &self,
        resource_id: &str,
        metric_name: &str,
        value: f64,
        unit: &str,
    ) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO metrics_cache (timestamp, resource_id, metric_name, value, unit)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![now, resource_id, metric_name, value, unit],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_create_and_incident() {
        let store = Store::open_in_memory().unwrap();
        let incident = models::Incident {
            id: "inc-001".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            severity: "warning".to_string(),
            resource_id: "vm-123".to_string(),
            resource_name: Some("web-server-1".to_string()),
            description: "High CPU usage detected (92%)".to_string(),
            action_taken: None,
            resolved: false,
        };
        store.log_incident(&incident).unwrap();

        let incidents = store.list_incidents(10).unwrap();
        assert_eq!(incidents.len(), 1);
        assert_eq!(incidents[0].id, "inc-001");
        assert!(!incidents[0].resolved);

        store.resolve_incident("inc-001", "Rebooted VM").unwrap();
        let incidents = store.list_incidents(10).unwrap();
        assert!(incidents[0].resolved);
    }
}
