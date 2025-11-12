#include <string>
#include <vector>
#include <unordered_map>

using namespace std;

string solution(vector<string> participant, vector<string> completion) {
    unordered_map<string, int> counts;
    for (string &p: participant) {
        counts[p]++;
    }
    for (string &c: completion) {
        counts[c]--;
        if (counts[c] == 0) {
            counts.erase(c);
        }
    }
    
    auto it = counts.begin();
    return it->first;
}