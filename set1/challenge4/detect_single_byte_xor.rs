use std::fs;
use std::collections::HashMap;

fn main() {

    let c = fs::read_to_string("cipher.txt").unwrap();

    let mut plain_scores = Vec::new();

    for cipher in c.lines() {
        plain_scores.extend(single_xor(cipher))
    }
    plain_scores.sort_by(|a,b| b.0.partial_cmp(&a.0).unwrap());
    for i in 0..5 {
        println!("{}: {}", plain_scores[i].0,plain_scores[0].1);
    }
}

fn single_xor(c: &str) -> Vec<(f32, String)> {
    let m = hex::decode(c).unwrap();
    let mut plain_scores = Vec::new();

    for i in 1..=255 {
        let message: Vec<u8> = m
            .iter()
            .map( |&m| m^i)
            .collect();

        let message = match std::str::from_utf8(&message) {
            Ok(m) => m,
            Err(_) => continue
        };
        let score = analyze(message);
        let message = message.to_string();
        plain_scores.push((score, message));
    }
    plain_scores

}

fn analyze(m: &str) -> f32 {

let letter_frequency: HashMap<char, f32> = HashMap::from(
    [
    ('a', 0.08167),
    ('b', 0.01492),
    ('c', 0.02782),
    ('d', 0.04253),
    ('e', 0.12702),
    ('f', 0.02228),
    ('g', 0.02015),
    ('h', 0.06094),
    ('i', 0.06094),
    ('j', 0.00153),
    ('k', 0.00772),
    ('l', 0.04025),
    ('m', 0.02406),
    ('n', 0.06749),
    ('o', 0.07507),
    ('p', 0.01929),
    ('q', 0.00095),
    ('r', 0.05987),
    ('s', 0.06327),
    ('t', 0.09056),
    ('u', 0.02758),
    ('v', 0.00978),
    ('w', 0.02360),
    ('x', 0.00150),
    ('y', 0.01974),
    ('z', 0.00074),
    (' ', 0.13000)
    ]);

    let mut score: f32 = 0.0;

    for c in m.to_lowercase().chars() {
        if letter_frequency.contains_key(&c) {
            score += letter_frequency[&c];
        }
    }
    score
}
