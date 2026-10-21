pub fn kmp(s: &str, pat: &str) -> Option<usize> {
    let s = s.chars().collect::<Vec<_>>();
    let pat = pat.chars().collect::<Vec<_>>();
    let mut lps: Vec<isize> = vec![0; pat.len()];

    lps[0] = -1;
    for i in 0..(pat.len() - 1) {
        let mut longest = 0;
        for lpos in (0..=i).rev() {
            let rpos = i + 1 - lpos;

            let lslice = &pat[0..lpos];
            let rslice = &pat[rpos..=i];

            if lslice == rslice {
                longest = lpos;
                break;
            }
        }

        lps[i + 1] = longest as isize;
    }

    let mut m = 0;
    let mut base_i = 0;

    'outer:
    loop {
        for i in base_i..pat.len() {
            let str_char = s[m + i];
            let pat_char = pat[i];

            if str_char != pat_char {
                let lps_val = lps[i];
                if lps_val == -1 {
                    base_i = 0;
                    m += 1;
                } else {
                    base_i = lps_val as usize;
                    m += pat.len() - lps_val as usize;
                }
                continue 'outer;
            }
        }

        return Some(m);
    }
}