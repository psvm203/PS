#include <string>
#include <vector>
#include <map>
#include <queue>

using namespace std;

int solution(vector<int> priorities, int location) {
    int n = priorities.size();
    map<int, int, greater<>> counts;
    for (int priority: priorities) {
        counts[priority]++;
    }
    
    queue<vector<int>> q;
    for (int i = 0; i < n; i++) {
        q.emplace(vector{i, priorities[i]});
    }
    
    int answer = 1;
    while (q.size()) {
        vector<int> cur = q.front();
        q.pop();
        int max = counts.begin()->first;
        if (cur[1] < max) {
            q.emplace(cur);
            continue;
        }
        
        if (cur[0] == location) {
            return answer;
        }
        
        counts[max]--;
        if (counts[max] == 0) {
            counts.erase(max);
        }
        answer++;
    }
    
    return -1;
}