use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub fn boyer_moore(str: &str, pat: &str) -> Option<usize> {
    let mut prev_char = HashMap::new();
    let chars = str.chars().collect::<Vec<_>>();
    let alphabet: HashSet<_> = str.chars().collect();
    let pat_chars = pat.chars().collect::<Vec<_>>();
    for c in alphabet {
        let entry = prev_char.entry(c);
        match entry {
            Entry::Vacant(vac) => {
                for i in (0..pat_chars.len() - 2).rev() {
                    if pat_chars[i] == c {
                        vac.insert(i);
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    let bm = BoyerMoore {
        pat: pat_chars,
        chars,
        prev_char,
    };

    bm.run()
}

struct BoyerMoore {
    pat: Vec<char>,
    chars: Vec<char>,
    prev_char: HashMap<char, usize>
}

impl BoyerMoore {
    fn run(self) -> Option<usize> {
        let mut pos = 0;
        let patlen = self.pat.len();
        let strlen = self.chars.len();

        loop {
            let patpos = patlen - 1;
            let strpos = pos + patpos;

            let strchar = self.chars[strpos];
            let patchar = self.pat[patpos];

            if strchar != patchar {
                //move pattern over to the next occurence
                if let Some(prev) = self.prev_char.get(&strchar) {
                    let to_move = patpos - *prev;
                    println!("prev: {strchar} -> {patchar}, moving by {to_move}");
                    pos += to_move;
                } else {
                    println!("moving whole pat");
                    pos += patlen;
                }

                if pos + patlen > strlen {
                    return None;
                }
            } else {
                for i in (0..(patpos - 1)).rev() {
                    let strchar = self.chars[pos + i];
                    let patchar = self.pat[i];
                    if strchar != patchar {
                        pos += 1;
                        break;
                    }
                }

                return Some(pos);
            }
        }
    }
}