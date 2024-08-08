/*
* TODO: idk
*/

/*
* - input method inspired by: https://github.com/nicolaspiet/jklm-word-bot/tree/main
*/

/*
* CURRENTLY UNDERWAY:
* make sure that the words vec is within main
* because removing the word from the vector does nothing
* if it is re-initialized every call
*/

// data related
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::{io, thread, time};
use std::io::{BufRead, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use rayon::prelude::*;

// keyboard & mouse related (and reading from clipboard)
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use rdev::{listen, Event, simulate, Button, EventType, Key, SimulateError};
use rdev::EventType::KeyPress;
use rdev::Key::{F4};
use autopilot;
use lazy_static;

/// scoring, sorting, saving words
static ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static TYPING_DELAY: u64 = 1; // 1 is the smallest ths can safely go

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
    let mut words: Vec<String> = match file_to_vec("Wordlist.txt") {
        Ok(words) => words,
        Err(e) => panic!("Error reading file: {}", e),
    };
    words.sort_unstable_by(|a, b| {
        let a_unique: HashSet<char> = a.chars().filter(|c| c.is_alphabetic()).collect();
        let b_unique: HashSet<char> = b.chars().filter(|c| c.is_alphabetic()).collect();

        b_unique.len().cmp(&a_unique.len())});
    let mut file = File::options().write(true).open("Sorted_Words.txt")?;

    for word in words {
        writeln!(file, "{}", word)?;
    }
    Ok(())
}

/// loading words, searching by prompt, output handling
fn load_words() -> Vec<String> {
    let words: Vec<String> = match file_to_vec("Sorted_Words.txt") {
        Ok(words) => words,
        Err(e) => panic!("Error reading file: {}", e),
    };
    words
}

// given a list of possible answers, find the highest scoring word, with the given game state
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

// given a prompt, search through the wordlist to find all words containing the prompt
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


    return if matches.is_empty() { (0, "NO MATCH".to_string()) }
    else {
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

fn type_string(string: &str){
    println!("Parsing string... [{}]", string);
    let string = string;
    println!("Creating keymap...");
    let mut keys = HashMap::with_capacity(26);
    println!("Assigning keys...");
    keys.insert('A', Key::KeyA); keys.insert('B', Key::KeyB);
    keys.insert('C', Key::KeyC); keys.insert('D', Key::KeyD);
    keys.insert('E', Key::KeyE); keys.insert('F', Key::KeyF);
    keys.insert('G', Key::KeyG); keys.insert('H', Key::KeyH);
    keys.insert('I', Key::KeyI); keys.insert('J', Key::KeyJ);
    keys.insert('K', Key::KeyK); keys.insert('L', Key::KeyL);
    keys.insert('M', Key::KeyM); keys.insert('N', Key::KeyN);
    keys.insert('O', Key::KeyO); keys.insert('P', Key::KeyP);
    keys.insert('Q', Key::KeyQ); keys.insert('R', Key::KeyR);
    keys.insert('S', Key::KeyS); keys.insert('T', Key::KeyT);
    keys.insert('U', Key::KeyU); keys.insert('V', Key::KeyV);
    keys.insert('W', Key::KeyW); keys.insert('X', Key::KeyX);
    keys.insert('Y', Key::KeyY); keys.insert('Z', Key::KeyZ);

    println!("Typing... [{}]", string);
    for c in string.chars(){
        let current_key = keys[&c];

        send(&EventType::KeyPress(current_key));
        send(&EventType::KeyRelease(current_key));
        println!("Typed -> [{}]", c);
    }

    send(&EventType::KeyPress(Key::Return));
    send(&EventType::KeyRelease(Key::Return));
}

// send keyboard and mouse events
fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(TYPING_DELAY);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("event could not be sent:\n{:?}\n", event_type);
        }
    }
    // Let the OS catch up
    thread::sleep(delay);
}

fn play(){
    let mut words_vec: Vec<String> = load_words();

    let mut clipboard_ctx = ClipboardContext::new().unwrap();
    let mut prompt: String = String::new();

    let mut best_word: String = String::new();
    let mut word_index: usize = 0usize;
    let mut scores = HashMap::with_capacity(26);

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



    // double click left mouse
    send(&EventType::ButtonPress(Button::Left));
    send(&EventType::ButtonRelease(Button::Left));
    send(&EventType::ButtonPress(Button::Left));
    send(&EventType::ButtonRelease(Button::Left));

    // copy selected text
    send(&EventType::KeyPress(Key::ControlLeft));
    send(&EventType::KeyPress(Key::KeyC));
    send(&EventType::KeyRelease(Key::KeyC));
    send(&EventType::KeyRelease(Key::ControlLeft));

    // get the copied text as a string
    match clipboard_ctx.get_contents() {
        Ok(contents) => {
            prompt = contents.clone();
            println!("Clipboard contents: {}", contents);
        },
        Err(e) => eprintln!("Error getting clipboard contents: {}", e),
    }

    // find & save the best word
    let prompt: String = prompt.to_uppercase().chars().filter(|c| c.is_alphabetic()).collect();
    let ans = search_by_prompt(&words_vec, &prompt, &scores);

    best_word = ans.1;
    word_index = ans.0;

    //ensure best word is valid, if not, don't play
    // (we take a knee here and lose a life lol)
    if best_word == "NO MATCH" {
        println!("NO MATCH FOUND!\n\n");
        // stop function here somehow
    }

    // write the word to output
    // (word, flags, wpm, noise)
    type_string(&best_word);

    // delete the word
    words_vec.remove(word_index);

    // change scoring for words to align with unused letters
    for c in best_word.chars(){
        if let Some(x) = scores.get_mut(&c){
            *x = 0;
        }
    }

    // reset the scores if all letters have been used
    if score(ALPHABET, &scores) == 0{
        for c in ALPHABET.chars() {
            if let Some(x) = scores.get_mut(&c){
                *x = 1;
            }
        }
    }
}

// callback for detecting and handling keyboard inputs
fn callback(event: Event){
    if event.event_type == KeyPress(F4){
        play();
    }
}

fn main() {
    // setup things
    // let mut words_vec: Vec<String> = Vec::new();
    // match file_to_vec("Sorted_Words.txt") {
    //     Ok(words) => words_vec = words,
    //     Err(e) => panic!("Error reading file: {}", e),
    // };

    // read from callback, notify of any errors
    if let Err(error) = listen(callback) {
        println!("Error:\n{:?}\n", error)
    }else{
        println!("SUCCESS");
    }
}
