// This is the code used to sort the words
// Words are assigned a score for each of their unique letters
// Letter scores are besed upon the score for each letter in Scrabble
// This helps to prioritize less common letters

#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <algorithm>
#include <execution>
using namespace std; //ik this is lazy

vector<pair<string,int>> words_and_scores;

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
