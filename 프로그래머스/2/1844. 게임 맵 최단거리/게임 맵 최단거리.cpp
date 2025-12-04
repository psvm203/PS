#include <vector>
#include <queue>
#include <tuple>

using namespace std;

int solution(vector<vector<int>> maps) {
    int r = maps.size();
    int c = maps[0].size();
    vector<vector<int>> visited(r, vector<int>(c));
    
    queue<tuple<int, int, int>> q;
    q.emplace(0, 0, 1);
    int dy[4] {-1, 0, 1, 0};
    int dx[4] {0, -1, 0, 1};
    while (q.size()) {
        auto [y, x, dist] = q.front();
        q.pop();
        if (y == r - 1 && x == c - 1) {
            return dist;
        }
        for (int d = 0; d < 4; d++) {
            int ny = y + dy[d];
            int nx = x + dx[d];
            if (ny < 0 || ny >= r || nx < 0 || nx >= c || !maps[ny][nx] || visited[ny][nx]) {
                continue;
            }
            visited[ny][nx] = true;
            q.emplace(ny, nx, dist + 1);
        }
    }
    
    return -1;
}