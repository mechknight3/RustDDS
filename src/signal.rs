
#[derive(Clone)] 
pub struct Signal {
    pub uid: u32,
    pub data: f64,

}

impl Signal {
    // Constructor for Signal
    pub fn new(uid: u32, data: f64) -> Self {
        Signal { uid, data }
    }

    // Method to convert Signal to Vec<u8>
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // Append UID (4 bytes)
        bytes.extend_from_slice(&self.uid.to_le_bytes());
        // Append data (8 bytes)
        bytes.extend_from_slice(&self.data.to_bits().to_le_bytes());
        bytes
    }

    // Method to convert Vec<u8> back to Signal
    pub fn from_bytes(bytes: &[u8]) -> Option<Signal> {
        if bytes.len() != 12 {
            return None; // Ensure we have exactly 12 bytes for u32 + f64
        }
        let uid = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let data = f64::from_bits(u64::from_le_bytes(bytes[4..12].try_into().unwrap()));
        Some(Signal { uid, data })
    }
}