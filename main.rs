/*
* TODO:
* - make macros to read keypresses,
*   - have one to set the location of the bomb
*   - have another to got to that location and copy the prompt
* - when the prompt is copied, pass it to the search fn
* - when the best word is found use enigo to automatically type it
*   - this might require a macro to set the input area
* - repurpose the egui to be a setting menu for the bot
*   - once the setting are set, close the window and start the bot
* - check speed using wordlist sorted by score and by alpha, score sorting might be pointless
*/

// data related
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;

// keyboard & mouse related (and reading from clipboard)
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::{
    event::{Event, KeyCode, poll, read}
    ,
    terminal::enable_raw_mode,
};
use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings
};
use rayon::prelude::*;

/// scoring, sorting, saving words
static ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn file_to_vec(path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|line| line.unwrap()).collect())
}

// get the score of a word
fn score(word: &str, scores: &HashMap<char, u8>) -> i8 {
    let mut score = 0;
    let mut word: String = word.into();
    let mut seen = HashSet::new();
    word.retain(|c| {
        let is_first =!seen.contains(&c);
        seen.insert(c);
        is_first
    });
    //score the unique letters
    for c in word.chars() {
        match scores.get(&c) {
            Some(s) => score += *s,
            None => (),
        }
    }
    // println!("{} -> {}",word,score);
    score as i8
}

// sort all words by number of unique letters, output to file
fn sort_and_save() -> io::Result<()> {
    let mut words: Vec<String> = match file_to_vec("F:\\Programming\\Ethan\\Rust\\Bomb_Party_Solver\\src\\Wordlist.txt") {
        Ok(words) => words,
        Err(e) => panic!("Error reading file: {}", e),
    };
    words.sort_unstable_by(|a, b| {
        let a_unique: HashSet<char> = a.chars().filter(|c| c.is_alphabetic()).collect();
        let b_unique: HashSet<char> = b.chars().filter(|c| c.is_alphabetic()).collect();

        b_unique.len().cmp(&a_unique.len())});
    let mut file = File::options().write(true).open("F:\\Programming\\Ethan\\Rust\\Bomb_Party_Solver\\src\\Sorted_Words.txt")?;

    for word in words {
        writeln!(file, "{}", word)?;
    }
    Ok(())
}

/// loading words, searching by prompt, output handling
fn load_words() -> Vec<String> {
    let words: Vec<String> = match file_to_vec("F:\\Programming\\Ethan\\Rust\\Bomb_Party_Solver\\src\\Sorted_Words.txt") {
        Ok(words) => words,
        Err(e) => panic!("Error reading file: {}", e),
    };
    words
}

fn find_best_word(vec: &Vec<(usize, String)>, scores: &HashMap<char, u8>) -> Option<(usize, String)> {
    let max_score = Arc::new(Mutex::new(i8::MIN));
    let result = Arc::new(Mutex::new(None));

    let max_score_clone = Arc::clone(&max_score);
    let result_clone = Arc::clone(&result);

    vec.par_iter().for_each(|&(index, ref word)| {
        let current_score = score(word, scores);
        let mut max_score_guard = max_score_clone.lock().unwrap();
        let mut result_guard = result_clone.lock().unwrap();

        if current_score > *max_score_guard {
            *max_score_guard = current_score;
            *result_guard = Some((index, word.clone()));
        }
    });

    let result_value = result.lock().unwrap();
    Some(<Option<(usize, String)> as Clone>::clone(&result_value).unwrap_or_default())
}

fn search_by_prompt(words: &[String], prompt: &str, scores: &HashMap<char, u8>) -> (usize, String) {
    let start = Instant::now();
    let filter_start = Instant::now();
    let mut matches: Vec<(usize, String)> = words.par_iter()
        .enumerate() // Add enumeration to get the index
        .filter(|&(index, word)| word.contains(prompt)) // Filter based on whether the word contains the prompt
        .map(|(index, word)| (index, word.clone())) // Map to keep the index and clone the word
        .collect(); // Collect the results into a vector
    let filter_elapsed = filter_start.elapsed();
    println!("word filtering completed in: {:.2?}",filter_elapsed);


    return if matches.is_empty() { (0, "NO MATCH".to_string()) } else {
        // println!("{:?}",matches);
        let sorting_start = Instant::now();
        let best_word = find_best_word(&mut matches, scores);
        let sorting_elapsed = sorting_start.elapsed();
        let elapsed = start.elapsed();
        println!("max search completed in: {:.2?}", sorting_elapsed);
        println!("search for top {} words completed in: {:.2?}\n", matches.len(), elapsed);
        // matches[0].clone()
        best_word.unwrap_or_else(|| (0, "NO MATCH".to_string()))
    }
}


fn main() {
    // setup things
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut scores = HashMap::with_capacity(24);
    let mut ctx = ClipboardContext::new().unwrap();
    scores.insert('A', 1); scores.insert('B', 1);
    scores.insert('C', 1); scores.insert('D', 1);
    scores.insert('E', 1); scores.insert('F', 1);
    scores.insert('G', 1); scores.insert('H', 1);
    scores.insert('I', 1); scores.insert('J', 1);
    scores.insert('K', 1); scores.insert('L', 1);
    scores.insert('M', 1); scores.insert('N', 1);
    scores.insert('O', 1); scores.insert('P', 1);
    scores.insert('Q', 1); scores.insert('R', 1);
    scores.insert('S', 1); scores.insert('T', 1);
    scores.insert('U', 1); scores.insert('V', 1);
    scores.insert('W', 1); scores.insert('X', 1);
    scores.insert('Y', 1); scores.insert('Z', 1);
    let mut saved_pos: (i32, i32) = (0, 0);
    let mut prompt: String = String::new();
    let mut words_vec: Vec<String> = Vec::new();
    let mut best_word: String = String::new();
    let mut word_index: usize = 0usize;
    match file_to_vec("F:\\Programming\\Ethan\\Rust\\Bomb_Party_Solver\\src\\Sorted_Words.txt") {
                Ok(words) => words_vec = words,
                Err(e) => panic!("Error reading file: {}", e),
            };

    enable_raw_mode().expect("Failed to enter raw mode");
    //gets the current mose pos once the button is pressed

    // use f2 and f4 as function keys
    loop {
        if poll(std::time::Duration::from_millis(100)).unwrap() {
            match read().unwrap() {
                Event::Key(key) => match key.code {
                    KeyCode::F(2) => {
                        saved_pos = enigo.location().unwrap();
                        println!("Mouse Position: {:?}", saved_pos);
                    }
                    KeyCode::F(4) => {
                        println!("F4 key pressed th");
                        // go to the set position
                        enigo.move_mouse(saved_pos.0, saved_pos.1, Coordinate::Abs).unwrap();

                        // double click
                        enigo.button(Button::Left, Click).expect("error pressing left mouse");
                        enigo.button(Button::Left, Click).expect("error pressing left mouse");

                        //copy
                        enigo.key(Key::Control, Press).expect("error pressing control");
                        enigo.key(Key::Unicode('c'), Click).expect("error pressing C");
                        enigo.key(Key::Control, Release).expect("error releasing control");

                        //get the copied prompt and save it here
                        match ctx.get_contents() {
                            Ok(contents) => {
                                prompt = contents.clone();
                                println!("Clipboard contents: {}", contents);
                            },
                            Err(e) => eprintln!("Error getting clipboard contents: {}", e),
                        }

                        // find & save the best word
                        let ans = search_by_prompt(&words_vec, &prompt, &scores);

                        best_word = ans.1;
                        word_index = ans.0;

                        //ensure best word is valid, if not, don't play
                        // (we take a knee here and lose a life lol)
                        if best_word == "NO MATCH" {
                            println!("NO MATCH FOUND!\n\n");
                            continue;
                        }

                        //write the word to the output
                        for c in best_word.chars(){
                            enigo.key(Key::Unicode(c), Click).expect("error typing word");
                            // add a delay here in case it needs to feel real
                        }
                        enigo.key(Key::Return, Click).expect("error entering word");

                        // delete the word
                        words_vec.remove(word_index);

                        // change scoring for words to align with unused letters
                        for c in best_word.chars(){
                            if let Some(x) = scores.get_mut(&c){
                                *x = 0;
                            }
                        }

                        //reset the scores if all letters have been used
                        if score(ALPHABET, &scores) == 0{
                            for c in ALPHABET.chars() {
                                if let Some(x) = scores.get_mut(&c){
                                    *x = 1;
                                }
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
