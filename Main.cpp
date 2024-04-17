#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <algorithm>
using namespace std; //ik this is lazy

vector<pair<string,int>> words_and_scores;
vector<string> words(total_words);
vector<string> used_words(total_words);

const string sorted_path = "Sorted_Words.txt";
const string unsorted_path = "Wordlist.txt";

const int total_words = 267750;

bool _sort (pair<string,int> w1, pair<string,int> w2){
    return w1.second > w2.second;
}

int calculate_score(string w){
    int score=0;
    unordered_map<char, int> points = {
            {'A',1},{'E',1},{'I',1},{'O',1},{'U',1},
            {'L',1},{'N',1},{'S',1},{'T',1},{'R',1},
            {'D',2},{'G',2},
            {'B',3},{'C',3},{'M',3},{'P',3},
            {'F',4},{'H',4},{'V',4},{'W',4},{'Y',4},
            {'K',5},
            {'J',8},{'X',8},
            {'Q',10},{'Z',10},
    };
    sort(w.begin(), w.end());
    w.erase(unique(w.begin(), w.end()), w.end());
    for(auto x : w){
        score += points[x];
    }
    return score;
}

void sort_words_by_unique(){
    ifstream read("Wordlist.txt");
    string word;
    if (read.is_open())
    {
        cout<<"FILE OPENED";
    }else{
        cout<<"FILE NOT OPENED";
    }
    while (getline (read, word)) {
        if(word.size() > 1){
            pair<string,int> t;
            t.first = word;
            t.second = calculate_score(word);
            words_and_scores.emplace_back(t);
        }
    }
    read.close();
    sort(words_and_scores.begin(), words_and_scores.end(),_sort);
}

void populate_words(){
    ifstream read(sorted_path);
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
    cout << find_best_word("EX") << '\n';
    // while(true){
    //     string prompt;cin >> prompt;
    //     cout << find_best_word(prompt) << '\n';
    // }

    return 0;
}
