use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use once_cell::sync::Lazy;
use crate::signal::Signal;

pub struct SignalMap {
    signals: HashMap<u32, Signal>,
    signals_to_update: Vec<u32>,  // Track UIDs of changed signals
}

pub static SIGNAL_MAP: Lazy<Arc<Mutex<SignalMap>>> = Lazy::new(|| {
    Arc::new(Mutex::new(SignalMap::new()))
});

impl SignalMap {
    // Constructor to create a new SignalMap
    pub fn new() -> Self {
        SignalMap {
            signals: HashMap::new(),
            signals_to_update: Vec::new(),
        }
    }

    // Method to add a Signal to the map
    pub fn add_signal(&mut self, signal: Signal) {
        let uid = signal.uid;
        self.signals.insert(uid, signal);
        self.signals_to_update.push(uid);  // Mark the signal as changed
    }

    // Method to update an existing Signal
    pub fn update_signal(&mut self, uid: u32, new_data: f64) -> Option<()> {
        if let Some(signal) = self.signals.get_mut(&uid) {
            signal.data = new_data;
            if !self.signals_to_update.contains(&uid) {
                self.signals_to_update.push(uid);  // Mark the signal as changed if not already
            }
            Some(())
        } else {
            None
        }
    }

    // Method to retrieve a Signal by its UID
    pub fn get_signal_data(&self, uid: u32) -> Option<&f64> {
        self.signals.get(&uid).map(|signal| &signal.data)
    }

    // Method to check if a Signal exists by its UID
    pub fn has_signal(&self, uid: u32) -> bool {
        self.signals.contains_key(&uid)
    }
    
    // Getter method to retrieve all changed signals' UIDs
    pub fn signals_to_update(&self) -> Vec<Signal> {
        self.signals_to_update
        .iter()
        .filter_map(|uid| self.signals.get(uid).cloned()) // Clone each Signal
        .collect()

    }

    // Method to clear the list of changed signals
    pub fn clear_signals_to_update(&mut self) {
        self.signals_to_update.clear();
    }
}
