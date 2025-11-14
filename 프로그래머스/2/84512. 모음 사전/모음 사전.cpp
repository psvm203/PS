#include <string>
#include <vector>

using namespace std;

int solution(string word) {
    vector<char> vowels{'A', 'E', 'I', 'O', 'U'};
    int weights[5][5] {
        {1, 1, 1, 1, 1},
        {782, 157, 32, 7, 2},
        {1563, 313, 63, 13, 3},
        {2344, 469, 94, 19, 4},
        {3125, 625, 125, 25, 5}
    };
    
    int answer = 0;
    for (int i = 0; i < word.size(); i++) {
        for (int j = 0; j < 5; j++) {
            if (vowels[j] != word[i]) {
                continue;
            }
            answer += weights[j][i];
            break;
        }
    }
    return answer;
}