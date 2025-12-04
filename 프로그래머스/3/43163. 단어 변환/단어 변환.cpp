#include <string>
#include <vector>
#include <queue>

using namespace std;

int solution(string begin, string target, vector<string> words) {
    int s = begin.size();
    int n = words.size();
    auto is_connected = [&](string &a, string &b) {
        int diff = 0;
        for (int i = 0; i < s; i++) {
            diff += a[i] != b[i];
        }
        return diff == 1;
    };
    
    queue<pair<string, int>> q;
    q.emplace(begin, 0);
    vector<bool> visited(n);
    while (q.size()) {
        auto [cur, dist] = q.front();
        q.pop();
        if (cur == target) {
            return dist;
        }
        for (int i = 0; i < n; i++) {
            if (visited[i] || !is_connected(cur, words[i])) {
                continue;
            }
            visited[i] = true;
            q.emplace(words[i], dist + 1);
        }
    }
    
    return 0;
}