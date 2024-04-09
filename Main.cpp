#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <algorithm>

using namespace std;
//probalby hold the values in the form:
//umap<String, vector<String>>
const string path = "Sorted_Words.txt";

const int total_words = 267750;

//unordered_map<string, string> prompts_and_words;
vector<string> words(total_words);
vector<string> used_words(total_words);

void populate_words(){
    ifstream read(path);
    string word;

    for (int i = total_words; i >= 0; --i) {
        getline(read, word);
        words[total_words - i] = word;
    }
}

string find_best_word(string prompt){
    for (int i = 0; i < prompt.size(); ++i) {
        prompt[i] = ::toupper(prompt[i]);
    }
    for (auto word : words) {
        if(word.find(prompt) != string::npos
           && count(used_words.begin(), used_words.end(), word) == 0){
            used_words.emplace_back(word);
            return word;
        }
    }
    return "NO MATCH FOUND!";
}

int main(){
    populate_words();

    while(true){
        string prompt;cin >> prompt;
        cout << find_best_word(prompt) << '\n';
    }

    return 0;
}
