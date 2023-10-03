use num::Complex;
use num::Num;
use num::Zero;

#[derive(PartialEq)]
pub(crate) struct ComplexWrapper<T: Num>(Complex<T>);

trait BridgeNum: Num {}

impl<T: Num> BridgeNum for T {}

impl<T: Num> From<Complex<T>> for ComplexWrapper<T> {
    fn from(value: Complex<T>) -> Self {
        Self(value)
    }
}

// let zeroes = [x1, x2, x3, ... , xn]
//
// direct expansion = (x - x1) * (x - x2) * .. * (x- xn)
//
//
pub fn poly<T: Num + Copy>(zeroes: &[T]) -> Vec<T> {
    let mut coeff = vec![T::one()];
    for z in zeroes {
        let mut clone = coeff.clone();
        mul_by_x(&mut coeff);
        mul_by_scalar(&mut clone, *z);
        sub_coeff(&mut coeff[1..], &clone)
    }
    coeff
}

fn mul_by_x<T: Num>(coeff: &mut Vec<T>) {
    coeff.push(T::zero());
}

fn mul_by_scalar<T: Num + Copy>(coeff: &mut Vec<T>, scalar: T) {
    coeff.iter_mut().for_each(move |a| *a = *a * scalar);
}

fn sub_scalar<T: Num + Copy>(coeff: &mut Vec<T>, scalar: T) {
    coeff.iter_mut().for_each(|a| *a = *a - scalar);
}

fn sub_coeff<T: Num + Copy>(coeff: &mut [T], ar: &[T]) {
    for (i, c) in coeff.iter_mut().enumerate() {
        *c = *c - ar[i]
    }
}

#[cfg(test)]
mod tests {
    use num::Complex;

    use super::poly;

    #[test]
    fn test_polynomials() {
        let zeroes = [2, 3];

        let coeffs = poly(&zeroes);
        assert_eq!(coeffs, [1, -5, 6]);

        let zeroes = [Complex::new(2, 0), Complex::new(3, 1), Complex::new(-3, 0)];

        let coeffs = poly(&zeroes);
        println!("{coeffs:?}")
    }
}
