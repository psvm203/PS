#include <iostream>
#include <vector>
#include <queue>
#include <tuple>

using namespace std;

int INF = 987654321;

int solution(int n, vector<vector<int>> edge) {
    vector<vector<int>> paths(n + 1);
    for (auto& e: edge) {
        int a = e[0];
        int b = e[1];
        paths[a].emplace_back(b);
        paths[b].emplace_back(a);
    }
    
    vector<bool> visited(n + 1);
    visited[1] = true;
    
    queue<tuple<int, int>> q;
    q.emplace(1, 0);
    
    int max_dist = -1;
    int answer = 0;
    
    while (q.size()) {
        auto [cur, dist] = q.front();
        q.pop();
        if (dist > max_dist) {
            max_dist = dist;
            answer = q.size() + 1;
        }
        for (int next: paths[cur]) {
            if (!visited[next]) {
                visited[next] = true;
                q.emplace(next, dist + 1);
            }
        }
    }
    
    return answer;
}