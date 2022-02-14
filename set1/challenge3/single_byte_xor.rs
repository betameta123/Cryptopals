extern crate hex;

use std::collections::HashMap;

fn main() {
    let c = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    println!("{}", single_xor(c));
}

fn single_xor(c: &str) -> String{
    let m = hex::decode(c).unwrap();

    let mut best_m = String::new();
    let mut best = f32::MAX;

    for i in 1..255 {
        let mess: Vec<u8> = m
            .iter()
            .map( |&m| m^i)
            .collect();

        let message = match std::str::from_utf8(&mess) {
            Ok(m) => m,
            Err(_) => continue
        };

        let score = analyze(message);
        if score == score.min(best) && score > 0.0 {
            best_m = message.to_string();
            best = score;
        }
    }
    best_m
}

fn analyze(m: &str) -> f32 {

let letter_frequency: HashMap<char, f32> = HashMap::from(
    [ ('a', 0.082),
      ('b', 0.015),
      ('c', 0.028),
      ('d', 0.043),
      ('e', 0.13),
      ('f', 0.022),
      ('g', 0.02),
      ('h', 0.061),
      ('i', 0.07),
      ('j', 0.0015),
      ('k', 0.0077),
      ('l', 0.04),
      ('m', 0.025),
      ('n', 0.067),
      ('o', 0.075),
      ('p', 0.019),
      ('q', 0.00095),
      ('r', 0.06),
      ('s', 0.063),
      ('t', 0.091),
      ('u', 0.028),
      ('v', 0.0098),
      ('w', 0.024),
      ('x', 0.0015),
      ('y', 0.02),
      ('z', 0.0074),
    ]);

    let mut characters: HashMap<char, u8 > = HashMap::new();
    let mut ignore = 0;

    for c in m.to_lowercase().chars() {
        if c.is_ascii_digit() || c.is_ascii_punctuation() || (c as u8) == 32 {
            ignore += 1;
        }
        if !c.is_ascii_graphic() && (c as u8) != 32 {
            return f32::MAX;
        }
        let x = characters.entry(c).or_insert(1);
        *x += 1;
    }

    let mut score: f32 = 0.0;

    for (k, v) in characters.iter() {
        if letter_frequency.contains_key(k) {
            let obs = (*v as f32) / ((characters.len() as f32) - (ignore as f32));
            score += ((obs - letter_frequency[k]).powi(2)) / letter_frequency[k];
        }
    }
    score
}


