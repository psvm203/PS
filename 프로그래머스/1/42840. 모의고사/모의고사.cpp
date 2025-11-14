#include <string>
#include <vector>

using namespace std;

vector<int> solution(vector<int> answers) {
    vector<vector<int>> patterns = {
        {1, 2, 3, 4, 5},
        {2, 1, 2, 3, 2, 4, 2, 5},
        {3, 3, 1, 1, 2, 2, 4, 4, 5, 5}
    };
    int scores[3]{};
    for (int i = 0; i < answers.size(); i++) {
        for (int j = 0; j < 3; j++) {
            vector<int> &pattern = patterns[j];
            if (pattern[i % pattern.size()] == answers[i]) {
                scores[j]++;
            }
        }
    }
    
    int max_score = max(max(scores[0], scores[1]), scores[2]);
    vector<int> answer;
    answer.reserve(3);
    for (int i = 0; i < 3; i++) {
        if (scores[i] == max_score) {
            answer.emplace_back(i + 1);
        }
    }
    return answer;
}