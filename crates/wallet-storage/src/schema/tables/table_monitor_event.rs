//! TableMonitorEvent - Monitor event logging
//!
//! Translates TypeScript TableMonitorEvent interface to Rust.
//! Reference: wallet-toolbox/src/storage/schema/tables/TableMonitorEvent.ts

use serde::{Deserialize, Serialize};

/// MonitorEvent table - stores monitor event logs
///
/// Matches TypeScript `TableMonitorEvent` interface
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableMonitorEvent {
    pub created_at: String,
    pub updated_at: String,
    
    pub id: i64,
    
    pub event: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl TableMonitorEvent {
    pub fn new(id: i64, event: impl Into<String>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            id,
            event: event.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_monitor_event_new() {
        let event = TableMonitorEvent::new(1, "startup");
        assert_eq!(event.id, 1);
        assert_eq!(event.event, "startup");
        assert!(event.details.is_none());
    }

    #[test]
    fn test_table_monitor_event_with_details() {
        let event = TableMonitorEvent::new(1, "error")
            .with_details("Connection failed");
        assert_eq!(event.details, Some("Connection failed".to_string()));
    }

    #[test]
    fn test_table_monitor_event_serialization() {
        let event = TableMonitorEvent::new(1, "test");
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"event\":\"test\""));
        let deserialized: TableMonitorEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }
}
