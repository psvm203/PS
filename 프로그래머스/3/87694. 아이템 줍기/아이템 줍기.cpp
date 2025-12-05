#include <string>
#include <vector>
#include <queue>
#include <tuple>

using namespace std;

bool path[101][101];
bool visited[51][51];

int solution(vector<vector<int>> rectangle, int characterX, int characterY, int itemX, int itemY) {
    for (auto& r: rectangle) {
        int bx = r[0], by = r[1], ex = r[2], ey = r[3];
        for (int y = by; y < ey; y++) {
            path[y * 2 + 1][bx * 2] = true;
            path[y * 2 + 1][ex * 2] = true;
        }
        for (int x = bx; x < ex; x++) {
            path[by * 2][x * 2 + 1] = true;
            path[ey * 2][x * 2 + 1] = true;
        }
    }
    for (auto& r: rectangle) {
        int bx = r[0], by = r[1], ex = r[2], ey = r[3];
        for (int y = by + 1; y < ey; y++) {
            for (int x = bx; x < ex; x++) {
                path[y * 2][x * 2 + 1] = false;
            }
        }
        for (int x = bx + 1; x < ex; x++) {
            for (int y = by; y < ey; y++) {
                path[y * 2 + 1][x * 2] = false;
            }
        }
    }
    
    queue<tuple<int, int, int>> q;
    q.emplace(characterY, characterX, 0);
    int dy[] {-1, 0, 1, 0};
    int dx[] {0, -1, 0, 1};
    while (q.size()) {
        auto [y, x, dist] = q.front();
        q.pop();
        if (y == itemY && x == itemX) {
            return dist;
        }
        for (int d = 0; d < 4; d++) {
            int ny = y + dy[d];
            int nx = x + dx[d];
            if (!path[y + ny][x + nx] || visited[ny][nx]) {
                continue;
            }
            visited[ny][nx] = true;
            q.emplace(ny, nx, dist + 1);
        }
    }
    
    return -1;
}