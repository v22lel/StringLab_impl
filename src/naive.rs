pub fn naive(s: &str, pat: &str) -> Option<usize> {
    let patlen = pat.len();
    for i in 0..(s.len() - patlen) {
        let candidate = &s[i..(i + patlen)];
        if candidate == pat {
            return Some(i);
        }
    }
    None
}