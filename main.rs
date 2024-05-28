use std::fs::read_to_string;
use std::cmp::Reverse;
use std::path::Path;
use std::collections::{HashSet, HashMap};

fn file_to_vec(path: &str) -> Vec<String>{
    let path = Path::new(path);

    read_to_string(path) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

// fn to score a word
fn score(word: &str) -> u8{
    // remove duplicate chars
    let mut seen = HashSet::new();
    let word:String = word.chars().filter(|c| seen.insert(*c)).collect();
    // print!("{} <- ",word);

    // calculate score
    let mut scores = HashMap::new();
    scores.insert('A', 1);
    scores.insert('E', 1);
    scores.insert('I', 1);
    scores.insert('O', 1);
    scores.insert('U', 1);
    scores.insert('R', 1);
    scores.insert('S', 1);
    scores.insert('T', 1);
    scores.insert('L', 1);
    scores.insert('N', 1);
    scores.insert('D', 2);
    scores.insert('G', 2);
    scores.insert('B', 3);
    scores.insert('C', 3);
    scores.insert('M', 3);
    scores.insert('P', 3);
    scores.insert('F', 4);
    scores.insert('H', 4);
    scores.insert('V', 4);
    scores.insert('W', 4);
    scores.insert('Y', 4);
    scores.insert('K', 5);
    scores.insert('J', 8);
    scores.insert('Q', 10);
    scores.insert('X', 0);
    scores.insert('Z', 0);
    
    let mut val:u8 = 0;
    for c in word.chars(){
        val += scores.get(&c).unwrap();
    }
    val
}

fn main(){
    let mut vec:Vec<String> = file_to_vec("Input.txt");
    vec.sort_by_key(|s| Reverse(score(&s))); // sort values by score
    for s in vec{
        println!("{} -> {}",s, score(&s));
    }
}
