#include <string>
#include <vector>
#include <unordered_map>

using namespace std;

int solution(vector<vector<string>> clothes) {
    unordered_map<string, int> counts;
    counts.reserve(20);
    for (vector<string> &c: clothes) {
        counts[c[1]]++;
    }
    
    int answer = 1;
    for (auto [_, count]: counts) {
        answer *= count + 1;
    }
    answer--;
    return answer;
}