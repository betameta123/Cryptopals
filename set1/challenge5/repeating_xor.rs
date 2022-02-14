extern crate hex;

fn main() {
    let s = b"Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal";

    println!("{}", repeating_xor(Vec::from(*s)));
}

fn repeating_xor(s: Vec<u8>) -> String{


    let k = b"ICE";

    let enc: Vec<u8> = s
        .into_iter()
        .enumerate()
        .map(|(i, c) | c ^ k[i%3])
        .collect();

    hex::encode(enc)

}
