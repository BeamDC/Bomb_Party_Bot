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

void sort_words_by_unique(){
    ifstream read("Wordlist.txt");
    string word="";
    while (getline (read, word)) {
        if(word.size() > 1){
            pair<string,int> t;
            t.first = word;
            unordered_set<char> s (word.begin(),word.end());
            t.second = s.size();
            words_and_scores.emplace_back(t);
        }
    }
    read.close();
    sort(execution::par, words_and_scores.begin(), words_and_scores.end(),_sort);
}

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
