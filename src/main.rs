pub mod boyer_moore;
pub mod kmp;
pub mod naive;

fn main() {
    let res = kmp::kmp("ABC ABCDAB ABCDABCDABDE", "ABCDABD");
    println!("{res:?}");
    let res = naive::naive("ABC ABCDAB ABCDABCDABDE", "ABCDABD");
    println!("{res:?}");
    //let res = boyer_moore::boyer_moore("Hoola-Hoola girls like Hooligans", "Hooligan");
    //println!("{res:?}");
}
