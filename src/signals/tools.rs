use ndarray::{array, Array1};
use num::Complex;
use sciport_rs::signal::{
    self, band_filter::BandFilter, bessel::BesselNorm, output_type::Ba, windows::WindowType,
};

pub fn norm_freq(frequencies: Array1<f64>, sampling_rate: impl Into<Option<f64>>) -> Array1<f64> {
    fn inner(frequencies: Array1<f64>, sampling_rate: f64) -> Array1<f64> {
        2.0 * frequencies / sampling_rate
    }

    inner(frequencies, sampling_rate.into().unwrap_or(1000.0))
}

pub fn norm_freq_band(
    frequencies: BandFilter,
    sampling_rate: impl Into<Option<f64>>,
) -> BandFilter {
    fn inner(frequencies: BandFilter, sampling_rate: f64) -> BandFilter {
        frequencies * 2.0 / sampling_rate
    }

    inner(frequencies, sampling_rate.into().unwrap_or(1000.0))
}

pub fn filter_signal(
    signal: Array1<f64>,
    filter_type: FilterType,
    band: BandFilter,
    order: u64,
    sampling_rate: f64,
) {
    let filter = get_filter(filter_type, band, order, sampling_rate);
}

pub(crate) enum FilterType {
    Firwin,
    Butter,
    Cheb1 { rp: f64 },
    Cheb2 { rs: f64 },
    Ellipse,
    Bessel { norm: BesselNorm },
}

pub fn get_filter(
    filter_type: FilterType,
    mut band: BandFilter,
    mut order: u64,
    sampling_rate: f64,
) -> Ba {
    band = norm_freq_band(band, sampling_rate);

    match filter_type {
        FilterType::Firwin => {
            if order % 2 == 0 {
                order += 1;
            }
            let a = array![1.0];
            let b = signal::firwin1::firwin(
                order as _,
                band,
                None,
                WindowType::Hamming,
                false,
                Some(sampling_rate),
            );
            let a = a.mapv(Into::into);
            let b = b.mapv(Into::into);
            Ba { a, b }
        }
        FilterType::Butter => {
            let filter = signal::butter::ButterFilterStandalone::<Ba>::filter(
                order as _,
                band,
                signal::Analog::False { fs: sampling_rate },
            );
            filter
        }
        FilterType::Cheb1 { rp } => {
            let filter = signal::cheby1::Cheby1FilterStandalone::<Ba>::filter(
                order as _,
                band,
                signal::Analog::False { fs: sampling_rate },
                rp,
            );
            filter
        }
        FilterType::Cheb2 { rs } => {
            let filter = signal::cheby2::Cheby2FilterStandalone::<Ba>::filter(
                order as _,
                band,
                signal::Analog::False { fs: sampling_rate },
                rs,
            );
            filter
        }
        FilterType::Ellipse => {
            todo!()
        }
        FilterType::Bessel { norm } => {
            let filter = signal::bessel::BesselFilterStandalone::<Ba>::filter(
                order as _,
                band,
                signal::Analog::False { fs: sampling_rate },
                norm,
            );
            filter
        }
    }
}
/*
pub fn filter_init(b: Array1<f64>, a: Array1<f64>, alpha: impl Into<Option<f64>>) -> Array1<f64> {
    fn inner(b:Array1<f64>, a: Array1<f64>, alpha: f64) -> Array1<f64> {
        zi = alpha
    }
}
*/
