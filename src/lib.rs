// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-aether.nvim: aether.nvim component model.
//!
//! Real implementation: aether_nvim_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AetherNvimItem {
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
pub enum AetherNvimError {
    EmptyAetherNvimItemName,
    DuplicateAetherNvimItem(String),
    EmptyEventSubject,
}

impl fmt::Display for AetherNvimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAetherNvimItemName => write!(f, "aether_nvim_item name must not be empty"),
            Self::DuplicateAetherNvimItem(id) => write!(f, "duplicate aether_nvim_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AetherNvimError {}

pub struct AetherNvim {
    pub name: String,
    aether_nvim_items: BTreeMap<String, AetherNvimItem>,
    events: BTreeMap<String, Event>,
}

impl AetherNvim {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            aether_nvim_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_aether_nvim_item(&mut self, item: AetherNvimItem) -> Result<(), AetherNvimError> {
        if item.name.trim().is_empty() {
            return Err(AetherNvimError::EmptyAetherNvimItemName);
        }
        let id = item.id.clone();
        if self.aether_nvim_items.contains_key(&id) {
            return Err(AetherNvimError::DuplicateAetherNvimItem(id.clone()));
        }
        self.aether_nvim_items.insert(id, item);
        Ok(())
    }

    pub fn aether_nvim_item_count(&self) -> usize {
        self.aether_nvim_items.len()
    }

    pub fn get_aether_nvim_item(&self, id: &str) -> Option<&AetherNvimItem> {
        self.aether_nvim_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AetherNvimError> {
        if event.subject.trim().is_empty() {
            return Err(AetherNvimError::EmptyEventSubject);
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

    fn sample_aether_nvim_item(id: &str, name: &str) -> AetherNvimItem {
        AetherNvimItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = AetherNvim::new("AetherNvim");
        assert_eq!(m.name, "AetherNvim");
        assert_eq!(m.aether_nvim_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_aether_nvim_item() {
        let mut m = AetherNvim::new("AetherNvim");
        m.add_aether_nvim_item(sample_aether_nvim_item("a1", "Alpha")).unwrap();
        assert_eq!(m.aether_nvim_item_count(), 1);
        assert!(m.get_aether_nvim_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = AetherNvim::new("AetherNvim");
        assert!(m.add_aether_nvim_item(sample_aether_nvim_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = AetherNvim::new("AetherNvim");
        m.add_aether_nvim_item(sample_aether_nvim_item("x", "One")).unwrap();
        assert!(m.add_aether_nvim_item(sample_aether_nvim_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = AetherNvim::new("AetherNvim");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
