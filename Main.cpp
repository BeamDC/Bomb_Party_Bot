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
            return "NO MATCH FOUND!";a
        }
    }
    return best;
}

// int save_sorted_words(){
//     ofstream outf{"Sortedwords.txt"};

//     if (!outf)
//     {
//         cerr << "Sortedwords.txt no open!\n";
//         return 1;
//     }

//     for(int i=0; i<words_and_scores.size(); ++i){
//         outf << words_and_scores[i].first << " - " << words_and_scores[i].second << '\n';
//     }
//     for(int i=0; i<words_and_scores.size(); ++i){
//         cout << words_and_scores[i].first << " - " << words_and_scores[i].second << '\n';
//     }
//     return 0;
// }

int main(){
    sort_words_by_unique();
    vector<string> prompts {
        "BO","UC","LM","VIA","DS","EN","EN","XP","OP","ART"
    };
    for(auto x : prompts){
        string ans = find_best_word(x);
        cout << "------------------\n";
        cout << "Prompt: " << x << '\n';
        cout << "Answer: " << ans << '\n';
        cout << "Score : " << calculate_score(ans) << '\n';
    }   cout << "------------------\n";

    cout << "\n---SORTED WORDS---\n";
    
    for(int i=0; i<words_and_scores.size(); ++i){
        cout << words_and_scores[i].first << " - " << words_and_scores[i].second << '\n';
    }

    // save_sorted_words();
    return 0;
}
