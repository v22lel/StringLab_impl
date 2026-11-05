pub fn test_down() {
    for x in down(5, 0) {
        print!("{x}, ");
    }
    println!();

    for x in down_inclusive(5, 0) {
        print!("{x}, ");
    }
    println!();
}

#[inline]
fn down(from: usize, to: usize) -> impl Iterator<Item=usize> {
    (to..from).rev()
}

#[inline]
fn down_inclusive(from: usize, to: usize) -> impl Iterator<Item=usize> {
    (to..=from).rev()
}

pub fn N(pat: &[char], j: usize) -> usize {
    let n = pat.len();
    for i in 0..=j {
        if pat[n - i - 1] != pat[j - i] {
            return i;
        }
    }
    j + 1
}

//this is appearently linear :shock:
// actually that fact is super super obvious
pub fn preprocessX(pat: &[char]) -> Vec<usize> {
    let n = pat.len();
    let mut LP = vec![0; n];

    for j in 1..n-1 {
        let Nj =  N(pat, j);
        if Nj == 0 {
            continue;
        }
        let i = n - Nj;
        LP[i] = j;
    }

    LP
}

// this is O(n^2) i think
// naive approach to generation of L' array
pub fn preprocess(pat: &[char]) -> Vec<usize> {

    let n = pat.len();

    if n == 0 {
        return vec![];
    } else if n == 1 {
        return vec![0];
    }

    let mut L = vec![0usize; n];

    let mut last = n - 2;
    for i in down(n, 0) {
        let suffix = &pat[i..n];
        let suffix_len = n - i;

        for j in down_inclusive(last, suffix_len - 1) {
            let cand = &pat[j + 1 - suffix_len..=j];
            if *cand == *suffix {
                L[i] = j;
                last = j;
                break;
            }
        }

        if L[i] == 0 {
            break;
        }
    }

    let mut L_prime = L.clone();

    for i in down(n, 1) {
        let l = L[i];
        let suffix_len = n - i;
        if l == 0 || l < suffix_len {
            break;
        }

        let prev = i - 1;

        if pat[prev] == pat[l - suffix_len] {
            let suffix = &pat[i..n];

            let mut found = false;
            for j in down_inclusive(l, suffix_len - 1) {
                let cand = &pat[j + 1 - suffix_len..=j];
                if (j < suffix_len || pat[prev] != pat[j - suffix_len]) && *cand == *suffix {
                    L_prime[i] = j;
                    found = true;
                    break;
                }
            }

            if !found {
                L_prime[i] = 0;
            }
        }
    }

    L_prime
}

pub fn make_LP_v22(pat: &[char]) -> Vec<usize> {
    let n = pat.len();
    let mut LP = vec![0; n];

    if n == 0 { return vec![]; }

    for i in down(n, 1) {
        let suff_len = n - i;
        let suff = &pat[i..n];
        for j in down(i, suff_len) {
            let start = j - suff_len + 1;
            let cand = &pat[start..start + suff_len];
            if pat[start - 1] != pat[i - 1] && *cand == *suff {
                LP[i] = j;
                break;
            }
        }
    }

    LP
}

pub fn make_H(pat: &[char], LP: &[usize]) -> Vec<usize> {
    let n = pat.len();
    let mut H = vec![0; n];
    for i in 0..n {
        if LP[i] != 0 { continue; }
        let right = n - i;
        for j in down_inclusive(i, 0) {
            let suff = &pat[right + j..n];
            let pref = &pat[0..j];
            if *suff == *pref {
                H[i] = j;
                break;
            }
        }
    }

    H
}