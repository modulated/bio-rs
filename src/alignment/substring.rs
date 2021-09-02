// TODO: Could be optimised - currently O(n * m) time
pub fn substring<N, H>(needle: &[N], haystack: &[H]) -> Vec<usize>
    where
        N: PartialEq,
        H: PartialEq<N> {

    let mut out = vec![];

    if needle.len() > haystack.len() {
        return out;
    }

    for i in 0 .. (haystack.len() - needle.len() + 1) {
        for j in 0 .. needle.len() {
            if haystack[i+j] != needle[j] {
                break;
            }

            if j == needle.len() - 1 {
                out.push(i + 1);
            }
        }
    }

    out
}

#[cfg(test)]
mod test {
    use crate::alignment::substring::substring;

    #[test]
    fn test_substring() {
        let hs = vec![1,2,3,4,5,6,4,5,6];
        let nd = vec![4,5,6];

        assert_eq!(vec![4,7], substring(&nd, &hs));
    }
}