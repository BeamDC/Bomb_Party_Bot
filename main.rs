/*
* TODO: 
* - make macros to read keypresses, 
*   - have one to set the location of the bomb
*   - have another to got to that location and copy the prompt
* - when the prompt is copied, pass it to the search fn
* - when the best word is found use enigo to automatically type it 
*   - this might require a macro to set the input area
* - when the program works automatically, remove the egui, as it is no longer needed
* - check speed using wordlist sorted by score and by alpha, score sorting might be pointless
*/

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use eframe::egui;
use egui::Color32;
use itertools::Itertools;
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


    if matches.is_empty() { return (0,"NO MATCH".to_string()); }
    else {
        // println!("{:?}",matches);
        let sorting_start = Instant::now();
        let best_word = find_best_word(&mut matches, scores);
        let sorting_elapsed = sorting_start.elapsed();
        let elapsed = start.elapsed();
        println!("max search completed in: {:.2?}",sorting_elapsed);
        println!("search for top {} words completed in: {:.2?}\n",matches.len(),elapsed);
        // matches[0].clone()
        match best_word {
            Some(w) => return w,
            None => return (0,"NO MATCH".to_string()),
        }

    }
}

/// GUI
struct MainWindow {
    prompt: String,
    words_vec: Vec<String>,
    scores: HashMap<char,u8>,
    best_word: String,
    word_index: usize,
}

impl Default for MainWindow {
    fn default() -> Self {
        let mut scores = HashMap::with_capacity(24);
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

        Self {
            prompt: Default::default(),
            words_vec: match file_to_vec("Sorted_Words.txt") {
                Ok(words) => words,
                Err(e) => panic!("Error reading file: {}", e),
            },
            scores,
            best_word: Default::default(),
            word_index: 0,
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::TextEdit::singleline(&mut self.prompt));

            if ui.button("Search").clicked(){
                self.prompt = self.prompt.to_uppercase();
                let ans = search_by_prompt(&self.words_vec, &self.prompt, &self.scores);
                self.word_index = ans.0;
                self.best_word = ans.1;
            }

            if ui.button("Play Word").clicked() && self.best_word != "NO MATCH" {
                // play the word
                // delete the word
                self.words_vec.remove(self.word_index);
                // change scoring for words to align with unused letters
                for c in self.best_word.chars(){
                    if let Some(x) = self.scores.get_mut(&c){
                        *x = 0;
                    }
                }
                //reset the scores if all letters have been used
                if score(ALPHABET, &self.scores) == 0{
                    for c in ALPHABET.chars() {
                        if let Some(x) = self.scores.get_mut(&c){
                            *x = 1;
                        }
                    }
                    println!("EXTRA LIFE <3");
                }
            }

            ui.horizontal(|ui| {
                if self.best_word == "NO MATCH" { ui.colored_label(Color32::RED, &self.best_word); } else { ui.colored_label(Color32::WHITE, &self.best_word); }
            });
            ui.end_row();
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    // use std::time::Instant;
    // let now = Instant::now();
    // sort_and_save().expect("TODO: panic message");
    // let elapsed = now.elapsed();
    // println!("sorted and saved in: {:.2?}", elapsed);

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native("Bomb Party Solver", options,
                       Box::new(|_cc| {
                           Box::<MainWindow>::default()
                       }),
    )
}
