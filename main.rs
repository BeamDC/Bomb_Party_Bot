use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;

use aho_corasick::{AhoCorasick, Match};
use eframe::egui;
use egui::Color32;
use lazy_static::lazy_static;

/// scoring, sorting, saving words
// when words are played, set the values of those letters to zero, 
// find all words that match a prompt, then sort those words by the new scores 
// instead of hash map, use a vec in main (self.score
// pass this vector ovewr to the scoring function
// this also makes changing the score easier, if lazystatics cant be modified easily
// values can be modified using let Some(c) = SCORES.get_mut(&charval); *c = newval
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=94631989e4d6ceb93a6988153c7d72d4
// probably rewite this as a normal hash map, if above method does not work
// also maybe make a module with all the non gui functionality, for organisation
lazy_static! {
        static ref SCORES: HashMap<char, u8> = {
            let mut m = HashMap::new();
            m.insert('A', 1); m.insert('B', 1); m.insert('C', 1); m.insert('D', 1); m.insert('E', 1);
            m.insert('F', 1); m.insert('G', 1); m.insert('H', 1); m.insert('I', 1); m.insert('J', 1);
            m.insert('K', 1); m.insert('L', 1); m.insert('M', 1); m.insert('N', 1); m.insert('O', 1);
            m.insert('P', 1); m.insert('Q', 1); m.insert('R', 1); m.insert('S', 1); m.insert('T', 1);
            m.insert('U', 1); m.insert('V', 1); m.insert('W', 1); m.insert('Y', 1);
            m
        };
    }

fn file_to_vec(path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|line| line.unwrap()).collect())
}

// get the score of a word
fn score(word: &str) -> u8 {
    let mut score = 0;
    // remove all diuplicate letters
    for c in word.chars() {
        match SCORES.get(&c) {
            Some(s) => score += *s,
            None => (),
        }
    }
    score
}

// sort all words by best score, output to file
fn sort_and_save() -> io::Result<()>{
    let mut words: Vec<String> = match file_to_vec("Wordlist.txt"){
        Ok(words) => words,
        Err(e) => panic!("Error reading file: {}", e),
    };
    words.sort_unstable_by(|a, b| score(b).cmp(&score(a))); // sort values by score
    let mut file = File::options().write(true).open("Sorted_Words.txt")?;

    for word in words{
        writeln!(file, "{}", word)?;
    }
    Ok(())
}

/// loading words, searching by prompt, output handling
fn load_words() -> Vec<String>{
    let words:Vec<String> = match file_to_vec("Sorted_Words.txt"){
        Ok(words) => words,
        Err(e) => panic!("Error reading file: {}", e),
    };
    words
}

fn search_by_prompt(words: &String, prompt: &str) -> (String,usize,usize){
    let pattern = [prompt];

    // find first substring matching the prompt
    let ac = AhoCorasick::new(pattern).unwrap();
    let mat = ac.find(&words);

    let Some(matches) = Some(mat) else { return ("NO MATCH".to_string(),0,0); } ;
    if matches.is_none() {return ("NO MATCH".to_string(),0,0);}

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

    (words[start..end].to_string(),start,end)
}

/// GUI
struct MainWindow {
    prompt: String,
    words: String,
    best_word: String,
    start: usize,
    end: usize,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            prompt: Default::default(),
            words: load_words().join(" "),
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
            }

            ui.horizontal(|ui| {
                if self.best_word == "NO MATCH" { ui.colored_label(Color32::RED, &self.best_word); }
                else { ui.colored_label(Color32::WHITE, &self.best_word); }
            });
            ui.end_row();
        });

    }
}

fn main() -> Result<(), eframe::Error>{
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native("Bomb Party Solver",options,
                       Box::new(|cc| {
                           Box::<MainWindow>::default()
                       }),
    )
}
