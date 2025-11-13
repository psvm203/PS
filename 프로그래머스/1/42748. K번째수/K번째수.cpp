#include <string>
#include <vector>
#include <algorithm>

using namespace std;

vector<int> solution(vector<int> array, vector<vector<int>> commands) {
    vector<int> answer;
    answer.reserve(commands.size());
    for (vector<int> command: commands) {
        vector<int> slice = vector(array.begin() + command[0] - 1, array.begin() + command[1]);
        sort(slice.begin(), slice.end());
        answer.emplace_back(slice[command[2] - 1]);
    }
    return answer;
}