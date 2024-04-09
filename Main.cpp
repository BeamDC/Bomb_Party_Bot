#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <unordered_set>
#include <algorithm>
#include <execution>
using namespace std;
//probalby hold the values in the form:
//umap<String, vector<String>>
vector<pair<string,int>> words_and_scores;
vector<string> used_words;

bool _sort (pair<string,int> w1, pair<string,int> w2){
    return w1.second > w2.second;
}

int calculate_score(string w){
    int score=0;
    unordered_map<char, int> points = {
        {'a',1},{'e',1},{'i',1},{'o',1},{'u',1},
        {'l',1},{'n',1},{'s',1},{'t',1},{'r',1},
        {'d',2},{'g',2},
        {'b',3},{'c',3},{'m',3},{'p',3},
        {'f',4},{'h',4},{'v',4},{'w',4},{'y',4},
        {'k',5},
        {'j',8},{'x',8},
        {'q',10},{'z',10},
    };
    for(auto& x : w){
        x = tolower(x);
    }
    sort(w.begin(), w.end());
    w.erase(unique(w.begin(), w.end()), w.end());
    for(auto x : w){
        score += points[x];
    }
    return score;
}

void sort_words_by_unique(){
    ifstream read("Wordlist.txt");
    string word="";
    while (getline (read, word)) {
        if(word.size() > 1){
            pair<string,int> t;
            t.first = word;
            t.second = calculate_score(word);
            words_and_scores.emplace_back(t);
        }
    }
    read.close();
    sort(execution::par, words_and_scores.begin(), words_and_scores.end(),_sort);
}

string find_best_word(string prompt){
    string best;
    for(int i=0; i<words_and_scores.size(); ++i){
        best = words_and_scores[i].first;
        if(best.find(prompt) != string::npos 
            && count(used_words.begin(), used_words.end(), best) == 0){
                used_words.emplace_back(best);
                break;
            }
        else if(i==words_and_scores.size()-1){
            return "NO MATCH FOUND!";
        }
    }
    return best;
}

int save_sorted_words(){
    ofstream outf{"Sortedwords.txt"};

    if (!outf)
    {
        cerr << "Sortedwords.txt no open!\n";
        return 1;
    }

    for(int i=0; i<words_and_scores.size(); ++i){
        outf << words_and_scores[i].first << " - " << words_and_scores[i].second << '\n';
    }
    return 0;
}

int main(){
    sort_words_by_unique();

    // for(int i=0; i<words_and_scores.size(); ++i){
    //     cout << words_and_scores[i].first << " - " << words_and_scores[i].second << '\n';
    // }

    cout << '\n' << find_best_word("op");
    cout << '\n' << find_best_word("op");
    cout << '\n' << find_best_word("he");
    // save_sorted_words();
    return 0;
}
