/*
 *  Input method inspired by: https://github.com/nicolaspiet/jklm-word-bot/tree/main
 */

use std::{io, thread, time};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use device_query::{DeviceQuery, DeviceState};
use lazy_static::lazy_static;
use rayon::prelude::*;
use rdev::{Button, Event, EventType, Key, listen, simulate, SimulateError};
use rdev::EventType::KeyPress;
use rdev::Key::{F2, F4};

// The English Alphabet :P
static ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

// Millisecond delay between simulated inputs
lazy_static! {
    static ref INPUT_DELAY: Arc<Mutex<u64>> = {
        let input_delay = 1;
        Arc::new(Mutex::new(input_delay))
    };
}

// Map chars to their corresponding keys on the keyboard
lazy_static!{
    static ref KEYS: HashMap<char, Key> = {
        let mut keys = HashMap::new();
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
        keys
    };
}

// This will store the words, so the vector can be saved outside the callback fn
lazy_static! {
    static ref WORDS_VEC: Arc<Mutex<Vec<String>>> = {
        let words_vec: Vec<String> = match file_to_vec("F:\\Programming\\Ethan\\Rust\\Bomb_Party_Solver\\src\\Wordlist.txt") {
            Ok(words) => words,
            Err(e) => panic!("Error reading file: {}", e),
        };
        Arc::new(Mutex::new(words_vec))
    };
}

// Store the scores of the individual letters
lazy_static! {
    static ref SCORES: Arc<Mutex<HashMap<char, u8>>> = {
        let mut scores: HashMap<char, u8> = HashMap::new();

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

        Arc::new(Mutex::new(scores))
    };
}

// Save a position on the screen for the mouse to return to
lazy_static! {
    static ref SAVED_POS: Arc<Mutex<(f64, f64)>> = {
        let pos = (0f64,0f64);
        Arc::new(Mutex::new(pos))
    };
}

// In the case of dual monitor setups, mouse movement can be tricky.
// This is because the rdev crates MoveMouse event seems to count all pixels included in the display.
// This means multiple monitors are treated as one which can throw off position calculatons.
// We store the furthest left position in the display here, and add it to the MoveMouse events
// to ensure they are in the correct position
lazy_static! {
    static ref X_AXIS_OFFEST: Arc<Mutex<f64>> = {
        let x = 0f64;
        Arc::new(Mutex::new(x))
    };
}

// Turn a given file path into a String Vector
fn file_to_vec(path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|line| line.unwrap()).collect())
}

// Get the score of a word
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
    score as i8
}

// Given a prompt, search through the wordlist to find all words containing the prompt
fn search_by_prompt(words: &[String], prompt: &str, scores: &HashMap<char, u8>) -> (usize, String) {
    let start = Instant::now();
    let filter_start = Instant::now();
    let mut matches: Vec<(usize, String)> = words.par_iter()
        // Enumeration allows us to keep the index
        .enumerate()
        // Filter based on whether the word contains the prompt
        .filter(|&(_index, word)| word.contains(prompt))
        .map(|(index, word)| (index, word.clone()))
        .collect();
    let filter_elapsed = filter_start.elapsed();
    println!("word filtering completed in: {:.2?}",filter_elapsed);


    // return if matches.is_empty() { (0, "NO MATCH".to_string()) }
    if matches.is_empty() { (0, "NO MATCH".to_string()) }
    else {
        let sorting_start = Instant::now();
        let best_word = find_best_word(&mut matches, scores);
        let sorting_elapsed = sorting_start.elapsed();
        let elapsed = start.elapsed();
        println!("max search completed in: {:.2?}", sorting_elapsed);
        println!("search for top {} words completed in: {:.2?}", matches.len(), elapsed);
        best_word.unwrap_or_else(|| (0, "NO MATCH".to_string()))
    }
}

// Given a list of possible answers, find the highest scoring word, with the given game state
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

// Send keyboard and mouse events
fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(*INPUT_DELAY.lock().unwrap());
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("event could not be sent:\n{:?}\n", event_type);
        }
    }
    // Let the OS catch up
    thread::sleep(delay);
}

// Given a string, simulate the required inputs to type it
fn type_string(string: &str){
    println!("Typing... [{}]", string);
    for c in string.chars(){
        let current_key = KEYS[&c];

        send(&EventType::KeyPress(current_key));
        send(&EventType::KeyRelease(current_key));
        // println!("Typed -> {}", c);
    }

    send(&EventType::KeyPress(Key::Return));
    send(&EventType::KeyRelease(Key::Return));
}

// Move the mouse until it hits the furthest left position
// Save this position to use as the offset for MouseMove
// Janky solution but it works  ¯\_(ツ)_/¯
fn calibrate(){
    let device_state = DeviceState::new();
    let mut mouse_pos = device_state.get_mouse().coords;
    let mut mouse_x = mouse_pos.0 as f64;
    let mut mouse_y = mouse_pos.1 as f64;
    let mut prev_mouse_x = mouse_x;
    mouse_x-= 10.0;
    send(&EventType::MouseMove { x: mouse_x, y: mouse_y});

    while prev_mouse_x != mouse_x {
        mouse_pos = device_state.get_mouse().coords;
        mouse_x = mouse_pos.0 as f64;
        mouse_y = mouse_pos.1 as f64;
        prev_mouse_x = mouse_x;
        mouse_x-= 10.0;
        send(&EventType::MouseMove { x: mouse_x, y: mouse_y});
        mouse_pos = device_state.get_mouse().coords;
        mouse_x = mouse_pos.0 as f64;
    }

    let mut x_axis_offset = *X_AXIS_OFFEST.lock().unwrap();
    x_axis_offset = mouse_x.abs();
}

// Play a round of bomb party, simulating all necessary movements
fn play(){
    let mut words_vec = WORDS_VEC.lock().unwrap();
    let mut scores = SCORES.lock().unwrap();
    let x_axis_offset = *X_AXIS_OFFEST.lock().unwrap();

    let device_state = DeviceState::new();

    let mut clipboard_ctx = ClipboardContext::new().unwrap();
    let mut prompt: String = String::new();

    let best_word: String;
    let word_index: usize;

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
            println!("\nClipboard contents: {}", contents);
        },
        Err(e) => eprintln!("Error getting clipboard contents: {}", e),
    }

    // find & save the best word
    let prompt: String = prompt.to_uppercase().chars().filter(|c| c.is_alphabetic()).collect();
    let ans = search_by_prompt(&words_vec, &prompt, &scores);

    best_word = ans.1;
    word_index = ans.0;

    // ensure best word is valid, if not, don't play
    // (we take a knee here and lose a life lol)
    if best_word == "NO MATCH" {
        println!("NO MATCH FOUND!\n\n");
        return
    }

    // move the mouse by clicking on a non text area below the cursor
    // to refocus on the text box
    let mouse_pos = device_state.get_mouse().coords;
    let mouse_x = mouse_pos.0 as f64;
    let mouse_y = mouse_pos.1 as f64;
    send(&EventType::MouseMove { x: mouse_x + x_axis_offset, y: mouse_y + 100.0});
    send(&EventType::ButtonPress(Button::Left));
    send(&EventType::ButtonRelease(Button::Left));

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
    // play a turn
    if event.event_type == KeyPress(F4){
        play();
    }
    // save the cursor location
    else if event.event_type == KeyPress(F2){
        calibrate();
    }
}

fn main() {
    // read from callback, notify of any errors
    if let Err(error) = listen(callback) {
        println!("Error:\n{:?}\n", error)
    }else{
        println!("SUCCESS");
    }
}
