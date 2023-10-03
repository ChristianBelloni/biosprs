use num::{Complex, Num, One};

pub fn get_filter(kind: FilterType, band: Band, order: u32, sampling_rate: f32) {
    let band = _norm_freq(band, sampling_rate);
    match kind {
        FilterType::Fir => {}
        FilterType::Butter => {}
        FilterType::Cheby1 => {}
        FilterType::Cheby2 => {}
        FilterType::Elliptic => {}
        FilterType::Bessel => {}
    }
}

pub enum FilterType {
    Fir,
    Butter,
    Cheby1,
    Cheby2,
    Elliptic,
    Bessel,
}

pub enum Band {
    Lowpass(f32),
    Highpass(f32),
    Bandpass((f32, f32)),
    Bandstop((f32, f32)),
}

struct NormBand(Band);

fn _norm_freq(frequency: Band, sampling_rate: f32) -> Band {
    match frequency {
        Band::Lowpass(freq) => Band::Lowpass(_norm_freq_single(freq, sampling_rate)),
        Band::Highpass(freq) => Band::Highpass(_norm_freq_single(freq, sampling_rate)),
        Band::Bandpass((low, high)) => {
            let [low, high] = _norm_freq_array(&[low, high], sampling_rate);
            Band::Bandpass((low, high))
        }
        Band::Bandstop((low, high)) => {
            let [low, high] = _norm_freq_array(&[low, high], sampling_rate);
            Band::Bandstop((low, high))
        }
    }
}

fn _norm_freq_single(frequency: f32, sampling_rate: f32) -> f32 {
    2.0 * frequency / sampling_rate
}

fn _norm_freq_array<const T: usize>(frequency: &[f32; T], sampling_rate: f32) -> [f32; T] {
    let mut ret = frequency.clone();
    for i in ret.iter_mut() {
        *i = 2.0 * *i / sampling_rate
    }
    ret
}

pub fn lp2hp_zpk<T: Num + Copy>(
    z: &[T],
    p: &[T],
    k: T,
    wo: T,
    sampling_rate: f32,
) -> (Vec<T>, Vec<T>, T) {
    println!("lp2hp_zpk");
    let degree = _relative_degree(z, p);

    let mut z_hp = z.iter().map(|a| wo / *a).collect::<Vec<_>>();
    let p_hp = p.iter().map(|a| wo / *a).collect::<Vec<_>>();
    let z_prod: Complex<T> = z_hp
        .iter()
        .fold(Complex::one(), |acc, i| {
            let i = T::zero().sub(*i);
            acc * i
        })
        .into();

    let p_prod: Complex<T> = p_hp
        .iter()
        .fold(Complex::one(), |acc, i| {
            let i = T::zero().sub(*i);
            acc * i
        })
        .into();

    z_hp.extend(vec![T::zero(); degree]);

    let temp = z_prod / p_prod;

    let k_hp = k * temp.re;

    (z_hp, p_hp, k_hp)
}

pub fn bilinear_zpk<T: Num + Copy + From<f32>>(
    z: &[T],
    p: &[T],
    k: T,
    sampling_rate: f32,
) -> (Vec<T>, Vec<T>, T)
where
    Complex<f32>: From<T>,
{
    let fs2 = 2.0 * sampling_rate;

    let z_z = z
        .iter()
        .map(|a| (*a + fs2.into()) / (T::zero().sub(a.sub(sampling_rate.into()))))
        .collect::<Vec<T>>();

    let p_z = p
        .iter()
        .map(|a| (*a + fs2.into()) / (T::zero().sub(a.sub(sampling_rate.into()))))
        .collect::<Vec<T>>();

    let prod_z: Complex<f32> = z_z.iter().fold(Complex::<f32>::one(), |acc, i| {
        acc * Into::<Complex<f32>>::into(T::from(fs2) - *i)
    });

    let prod_p: Complex<f32> = p_z.iter().fold(Complex::<f32>::one(), |acc, i| {
        acc * Into::<Complex<f32>>::into(T::from(fs2) - *i)
    });
    let temp = prod_z / prod_p;

    let k_z = k * temp.re.into();

    (z_z, p_z, k_z)
}

fn _relative_degree<T: Num>(z: &[T], p: &[T]) -> usize {
    p.len() - z.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_norm_freq() {}
}
