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

//currently non functional
// void save_sorted_words(){
//     ofstream output_sorted("Sortedwords.txt", ios::out);
//     for(const auto& word : words){
//         output_sorted << word << '\n';
//     }
//     output_sorted.close();
// }

int main(){
    sort_words_by_unique();
    for(const auto& x : words_and_scores){
        cout << x.first << " - " << x.second << '\n';
    }
    //save_sorted_words();
    return 0;
}
