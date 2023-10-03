use rust_numpy::*;

mod ecg;

pub fn ecg(signal: Vec<u8>, sampling_rate: impl Into<Option<f32>>) {
    let sampling_rate = if let Some(rate) = sampling_rate.into() {
        rate
    } else {
        1000.0
    };
}

#[derive(Debug, Clone)]
pub struct EcgReturn {
    pub ts: Vec<u64>,
    pub filtered: Vec<u8>,
    pub features_ts: Vec<u64>,
    pub theta: Vec<u8>,
    pub alpha_low: Vec<u8>,
    pub alpha_high: Vec<u8>,
    pub beta: Vec<u8>,
    pub gamma: Vec<u8>,
    pub plf_pairs: Vec<u8>,
    pub plf: Vec<u8>,
}
