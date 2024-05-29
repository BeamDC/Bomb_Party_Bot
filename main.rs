use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::io;
use std::io::{BufRead, Write};
use lazy_static::lazy_static;

lazy_static! {
        static ref SCORES: HashMap<char, u8> = {
            let mut m = HashMap::new();
            m.insert('A', 1); m.insert('E', 1); m.insert('I', 1); m.insert('O', 1); m.insert('U', 1);
            m.insert('R', 1); m.insert('S', 1); m.insert('T', 1); m.insert('L', 1); m.insert('N', 1);
            m.insert('D', 2); m.insert('G', 2); m.insert('B', 3); m.insert('C', 3); m.insert('M', 3);
            m.insert('P', 3); m.insert('F', 4); m.insert('H', 4); m.insert('V', 4); m.insert('W', 4);
            m.insert('Y', 4); m.insert('K', 5); m.insert('J', 8); m.insert('Q', 10); m.insert('X', 0);
            m.insert('Z', 0);
            m
        };
    }

fn file_to_vec(path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|line| line.unwrap()).collect())
}

// fn to score a word
fn score(word: &str) -> u8 {
    let mut score = 0;
    for c in word.chars() {
        match SCORES.get(&c) {
            Some(s) => score += *s,
            None => (),
        }
    }
    score
}

fn sort_and_save() -> io::Result<()>{
    let mut words: Vec<String> = match file_to_vec("F:\\Programming\\Ethan\\Rust\\Bomb_Party_Solver\\src\\Wordlist.txt"){
        Ok(words) => words,
        Err(e) => panic!("Error reading file: {}", e),
    };
    words.sort_unstable_by(|a, b| score(b).cmp(&score(a))); // sort values by score
    let mut file = File::options().write(true).open("F:\\Programming\\Ethan\\Rust\\Bomb_Party_Solver\\src\\Sorted_Words.txt")?;

    for word in words{
        writeln!(file, "{}", word)?;
    }
    Ok(())
}

fn main() {
    sort_and_save().expect("no work :(");
}
