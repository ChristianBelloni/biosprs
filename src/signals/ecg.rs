use ndarray::Array1;

pub fn ecg(signal: Array1<f64>, sampling_rate: f64) {
    let order = (1.5 * sampling_rate).round() as u64;
}
