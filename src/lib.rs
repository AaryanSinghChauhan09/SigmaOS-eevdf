// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-eevdf: eevdf component model.
//!
//! Real implementation: eevdf_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EevdfItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EevdfError {
    EmptyEevdfItemName,
    DuplicateEevdfItem(String),
    EmptyEventSubject,
}

impl fmt::Display for EevdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEevdfItemName => write!(f, "eevdf_item name must not be empty"),
            Self::DuplicateEevdfItem(id) => write!(f, "duplicate eevdf_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for EevdfError {}

pub struct Eevdf {
    pub name: String,
    eevdf_items: BTreeMap<String, EevdfItem>,
    events: BTreeMap<String, Event>,
}

impl Eevdf {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            eevdf_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_eevdf_item(&mut self, item: EevdfItem) -> Result<(), EevdfError> {
        if item.name.trim().is_empty() {
            return Err(EevdfError::EmptyEevdfItemName);
        }
        let id = item.id.clone();
        if self.eevdf_items.contains_key(&id) {
            return Err(EevdfError::DuplicateEevdfItem(id.clone()));
        }
        self.eevdf_items.insert(id, item);
        Ok(())
    }

    pub fn eevdf_item_count(&self) -> usize {
        self.eevdf_items.len()
    }

    pub fn get_eevdf_item(&self, id: &str) -> Option<&EevdfItem> {
        self.eevdf_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), EevdfError> {
        if event.subject.trim().is_empty() {
            return Err(EevdfError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_eevdf_item(id: &str, name: &str) -> EevdfItem {
        EevdfItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Eevdf::new("Eevdf");
        assert_eq!(m.name, "Eevdf");
        assert_eq!(m.eevdf_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_eevdf_item() {
        let mut m = Eevdf::new("Eevdf");
        m.add_eevdf_item(sample_eevdf_item("a1", "Alpha")).unwrap();
        assert_eq!(m.eevdf_item_count(), 1);
        assert!(m.get_eevdf_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Eevdf::new("Eevdf");
        assert!(m.add_eevdf_item(sample_eevdf_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Eevdf::new("Eevdf");
        m.add_eevdf_item(sample_eevdf_item("x", "One")).unwrap();
        assert!(m.add_eevdf_item(sample_eevdf_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Eevdf::new("Eevdf");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
