pub mod boyer_moore;
pub mod kmp;

fn main() {
    let res = kmp::kmp("ABC ABCDAB ABCDABCDABDE", "ABCDABD");
    println!("{res:?}");
    //let res = boyer_moore::boyer_moore("Hoola-Hoola girls like Hooligans", "Hooligan");
    //println!("{res:?}");
}
