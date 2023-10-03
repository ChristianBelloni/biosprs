use crate::tools::{Band, FilterType};

use super::{iir_filter, DesiredFilterOutput, FilterOutputType};

pub fn butter_filter(
    order: u32,
    band: Band,
    analog: bool,
    sampling_frequency: f32,
    desired_output: DesiredFilterOutput,
) -> FilterOutputType {
    iir_filter(
        order,
        band,
        analog,
        FilterType::Butter,
        sampling_frequency,
        desired_output,
    )
}

#[cfg(test)]
mod test {
    use crate::{
        filters::{DesiredFilterOutput, ZpkFilterOutput},
        tools::Band,
    };

    use super::butter_filter;

    #[test]
    fn test_butter_filter_ba() {
        let filter = butter_filter(
            8,
            Band::Highpass(0.2),
            false,
            200.0,
            DesiredFilterOutput::Ba,
        )
        .cast_ba()
        .unwrap();

        println!("{filter:?}")
    }

    #[test]
    fn test_butter_filter_zpk() {
        let filter = butter_filter(8, Band::Highpass(0.2), false, 2.0, DesiredFilterOutput::Zpk)
            .cast_zpk()
            .unwrap();

        let ZpkFilterOutput { z, p, k } = filter;

        println!("z: ");
        for i in &z {
            println!("{i}")
        }

        println!("p: ");
        for i in &p {
            println!("{i}")
        }
        println!("k: {k}")
    }
}
