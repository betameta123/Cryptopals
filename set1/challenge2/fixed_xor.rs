extern crate hex;
fn main() {
    let m = "1c0111001f010100061a024b53535009181c";
    let k = "686974207468652062756c6c277320657965";
    let c = fixed_xor(m,k);
    println!("{}", c)
}

fn fixed_xor(m: &str, k: &str) -> String {
    let m = hex::decode(m).unwrap();
    let k = hex::decode(k).unwrap();
    let c: Vec<u8> = m
        .iter()
        .zip(k.iter())
        .map(| (&m,&k) | m^k)
        .collect();

    hex::encode(c)
}
