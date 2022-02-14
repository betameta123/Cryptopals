extern crate hex;
extern crate base64;

fn main() {
    let c = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("{}",hex_to_base64(c));
}

fn hex_to_base64(c: &str) -> String {
    let c = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
    base64::encode(c)
}
