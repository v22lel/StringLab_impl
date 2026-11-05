#![allow(non_snake_case)]

use crate::good_suffix::{make_H, make_LP_v22, preprocessX, test_down};

pub mod boyer_moore;
pub mod kmp;
pub mod naive;
pub mod good_suffix;

fn main() {
    test_down();

    let res = kmp::kmp("ABC ABCDAB ABCDABCDABDE", "ABCDABD");
    println!("{res:?}");
    let res = naive::naive("ABC ABCDAB ABCDABCDABDE", "ABCDABD");
    println!("{res:?}");
    //let res = boyer_moore::boyer_moore("Hoola-Hoola girls like Hooligans", "Hooligan");
    //println!("{res:?}");

    let chars: Vec<_> = String::from("BBABABCCCABAB").chars().collect();
    let lprime = preprocessX(&chars);
    let lprime_v22 = make_LP_v22(&chars);
    let H = make_H(&chars, &lprime);
    println!("Nj(P): {lprime:?}");
    println!("v22:   {lprime_v22:?}");
    println!("v22 H: {H:?}");
}
