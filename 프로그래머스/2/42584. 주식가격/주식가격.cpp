#include <string>
#include <vector>

using namespace std;

vector<int> solution(vector<int> prices) {
    int n = prices.size();
    vector<int> answer(n);
    vector<vector<int>> stack;
    stack.reserve(n);
    for (int i = 0; i < n; i++) {
        while (stack.size() && stack.back()[1] > prices[i]) {
            answer[stack.back()[0]] = i - stack.back()[0];
            stack.pop_back();
        }
        stack.emplace_back(vector{i, prices[i]});
    }
    
    while (stack.size()) {
        vector<int> cur = stack.back();
        stack.pop_back();
        answer[cur[0]] = n - cur[0] - 1;
    }
    
    return answer;
}