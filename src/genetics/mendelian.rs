/// Returns probability of two randomly selected organisms producing an individual possessing a dominant allele.
pub fn mendelian_inheritance_dominant(homozygous_dominant: u32, heterozygous_dominant: u32, homozygous_recesive: u32) -> f64 {
    let m = f64::from(heterozygous_dominant);
    let n = f64::from(homozygous_recesive);
    let t = f64::from(homozygous_dominant + heterozygous_dominant + homozygous_recesive);


    1.0 - 1.0/t/(t - 1.0) * (n * (n - 1.0) + n * m + m * (m - 1.0)/4.0)
}

#[cfg(test)]
mod test {
    use super::mendelian_inheritance_dominant;

    #[test]
    fn dominant() {
        assert_eq!(&mendelian_inheritance_dominant(2,2,2).to_string()[..7], "0.78333");
    }
}