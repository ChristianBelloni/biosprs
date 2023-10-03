use num::Complex;

pub enum DesiredFilterOutput {
    Ba,
    Zpk,
    Sos,
}

#[derive(Debug)]
pub enum FilterOutputType {
    Ba(BaFilterOutput),
    Zpk(ZpkFilterOutput),
    Sos(SosFilterOutput),
}

impl FilterOutputType {
    pub fn cast_ba(self) -> Option<BaFilterOutput> {
        match self {
            Self::Ba(data) => Some(data),
            _ => None,
        }
    }
    pub fn cast_zpk(self) -> Option<ZpkFilterOutput> {
        match self {
            Self::Zpk(data) => Some(data),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct BaFilterOutput {
    pub b: Vec<Complex<f32>>,
    pub a: Vec<Complex<f32>>,
}

#[derive(Debug)]
pub struct ZpkFilterOutput {
    pub z: Vec<num::Complex<f32>>,
    pub p: Vec<num::Complex<f32>>,
    pub k: f32,
}

#[derive(Debug)]
pub struct SosFilterOutput {
    pub sos: Vec<f32>,
}
