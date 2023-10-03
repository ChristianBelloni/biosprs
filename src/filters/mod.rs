use crate::{
    algo::poly,
    tools::{bilinear_zpk, lp2hp_zpk, Band, FilterType},
};
use num::{Complex, One};
mod butter;
pub mod filter_output;
pub use butter::*;

use filter_output::*;

pub fn iir_filter(
    order: u32,
    freq: Band,
    analog: bool,
    filter_type: FilterType,
    sampling_frequency: f32,
    out_type: DesiredFilterOutput,
) -> FilterOutputType {
    match out_type {
        DesiredFilterOutput::Ba => FilterOutputType::Ba(ba_iir_filter(
            order,
            freq,
            analog,
            sampling_frequency,
            filter_type,
        )),
        DesiredFilterOutput::Zpk => FilterOutputType::Zpk(zpk_iir_filter(
            order,
            freq,
            analog,
            sampling_frequency,
            filter_type,
        )),
        _ => unimplemented!(),
    }
}

fn zpk_iir_filter(
    order: u32,
    freq: Band,
    _analog: bool,
    sampling_frequency: f32,
    filter_type: FilterType,
) -> ZpkFilterOutput {
    match filter_type {
        FilterType::Butter => {
            let temp_out = zpk_butter_filter(order);
            band_adjust_zpk(temp_out, freq, sampling_frequency)
        }
        _ => unimplemented!(),
    }
}

fn zpk_butter_filter(order: u32) -> ZpkFilterOutput {
    let order = order as i32;
    let bottom = (1 - order) as i32;
    let top = order as i32;
    let mut range: Vec<Complex<f32>> = (bottom..top)
        .step_by(2)
        .map(|a| (a as f32).into())
        .collect::<Vec<_>>();
    println!("{range:?}");
    for i in range.iter_mut() {
        let temp = (num::Complex::new(0.0, 1.0) * std::f32::consts::PI * *i) / (2.0 * order as f32);
        *i = -temp.exp();
    }

    let z = vec![Complex::one(); range.len()];
    let p = range;
    let k = 1.0;

    ZpkFilterOutput { z, p, k }
}

fn ba_iir_filter(
    order: u32,
    freq: Band,
    analog: bool,
    sampling_frequency: f32,
    filter_type: FilterType,
) -> BaFilterOutput {
    let temp = zpk_iir_filter(order, freq, analog, sampling_frequency, filter_type);
    zpk2tf(&temp)
}

fn band_adjust_zpk(zpk: ZpkFilterOutput, band: Band, sampling_frequency: f32) -> ZpkFilterOutput {
    match band {
        Band::Highpass(f) => {
            let wo =
                sampling_frequency * 2.0 * (std::f32::consts::PI * f / sampling_frequency).tan();

            let (z, p, k) = lp2hp_zpk(&zpk.z, &zpk.p, zpk.k.into(), wo.into(), sampling_frequency);
            let (z, p, k) = bilinear_zpk(&z, &p, k, sampling_frequency);
            ZpkFilterOutput { z, p, k: k.re }
        }
        _ => unimplemented!(),
    }
}

fn zpk2tf(zpk: &ZpkFilterOutput) -> BaFilterOutput {
    let ZpkFilterOutput { z, p, k } = &zpk;
    let a = poly(&p);
    let mut b = poly(z);
    b.iter_mut().for_each(|a| *a = *a * k);
    BaFilterOutput { b, a }
}
