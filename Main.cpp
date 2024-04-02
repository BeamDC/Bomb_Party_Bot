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
vector<string> words;

bool _sort (string word1, string word2){
    unordered_set<char> a (word1.begin(), word1.end());
    unordered_set<char> b (word2.begin(), word2.end());
    return a.size() > b.size();
}

void sort_words_by_unique(){
    string word="";
    ifstream read("Wordlist.txt");
    int i=0;
    while (getline (read, word)) {
        if(word.size() > 1)words.emplace_back(word);
        ++i;
    }
    read.close();
    sort(execution::par, words.begin(), words.end(),_sort);
}

void save_sorted_words(){
    ofstream output_sorted("Sortedwords.txt", ios::out);
    for(const auto& word : words){
        output_sorted << word << '\n';
    }
    output_sorted.close();
}

int main(){
    sort_words_by_unique();
    for(const auto& x : words){
        cout<<x<<'\n';
    }
    save_sorted_words();
    return 0;
}
