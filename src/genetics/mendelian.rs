/// Returns probability of two randomly selected organisms producing an individual possessing a dominant allele.
pub fn mendelian_inheritance_dominant(homozygous_dominant: u32, heterozygous_dominant: u32, homozygous_recesive: u32) -> f64 {
    let m = heterozygous_dominant as f64;
    let n = homozygous_recesive as f64;
    let t = homozygous_dominant as f64 + heterozygous_dominant as f64 + homozygous_recesive as f64;


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