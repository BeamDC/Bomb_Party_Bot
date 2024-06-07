use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;

use aho_corasick::AhoCorasick;
use eframe::egui;
use egui::Color32;

/// scoring, sorting, saving words
// also maybe make a module with all the non gui functionality, for organisation

static ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWY";

fn file_to_vec(path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|line| line.unwrap()).collect())
}

// get the score of a word
fn score(word: &str, scores: &HashMap<char, u8>) -> u8 {
    let mut score = 0;
    // remove all duplicate letters
    for c in word.chars() {
        match scores.get(&c) {
            Some(s) => score += *s,
            None => (),
        }
    }
    score
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

        b_unique.len().cmp(&a_unique.len())
    });
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

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f5e5c2d1fd10e57095c1d6ca15102036
fn search_by_prompt(words: &String, prompt: &str) -> (String, usize, usize) {
    let pattern = [prompt];

    // find first substring matching the prompt
    let ac = AhoCorasick::new(pattern).unwrap();
    let mat = ac.find(&words);

    let Some(matches) = Some(mat) else { return ("NO MATCH".to_string(), 0, 0); };
    if matches.is_none() { return ("NO MATCH".to_string(), 0, 0); }

    // expand the substring to contain the full word
    let mut start = mat.expect("still no match?").start();
    while start > 0 &&
        !words.chars().nth(start - 1).expect("to far back").is_whitespace() {
        start -= 1;
    }

    let mut end = mat.expect("still no match?").start() + 1;
    while end < words.len() &&
        !words.chars().nth(end).expect("to far forward").is_whitespace() {
        end += 1;
    }

    (words[start..end].to_string(), start, end)
}

/// GUI
struct MainWindow {
    prompt: String,
    words: String,
    scores: HashMap<char,u8>,
    best_word: String,
    start: usize,
    end: usize,
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
        scores.insert('W', 1); scores.insert('Y', 1);

        Self {
            prompt: Default::default(),
            words: load_words().join(" "),
            scores,
            best_word: Default::default(),
            start: 0,
            end: 0,
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let response = ui.add(egui::TextEdit::singleline(&mut self.prompt));
            if response.changed() {
                self.prompt = self.prompt.to_uppercase();
                let ans = search_by_prompt(&self.words, &self.prompt);
                self.best_word = ans.0;
                self.start = ans.1;
                self.end = ans.2;
            }

            if ui.button("Play Word").clicked() && self.best_word != "NO MATCH" {
                // play the word
                // replace the word with a string that will never match, so that no words are reused
                self.words.replace_range(self.start..self.end, "-");
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


    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native("Bomb Party Solver", options,
                       Box::new(|cc| {
                           Box::<MainWindow>::default()
                       }),
    )
}
