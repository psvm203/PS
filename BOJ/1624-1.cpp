#include <bits/stdc++.h>
#define int int64_t

using namespace std;

int dx[]{1, 0, -1, 0};
int dy[]{0, 1, 0, -1};

signed main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int tx, ty;
    string commands;
    cin >> tx >> ty >> commands;

    int rx = 0;
    int ry = 0;
    int min_dist = -1;
    auto calc = [&](int x, int y) {
        int dist = (tx - x) * (tx - x) + (ty - y) * (ty - y);
        if (dist == 0) {
            cout << -1;
            exit(0);
        }
        if (min_dist == -1 || dist < min_dist) {
            min_dist = dist;
            rx = x;
            ry = y;
        }
    };
    calc(0, 0);

    int cx = 0;
    int cy = 0;
    int dir = 0;
    stringstream ss(commands);
    char cmd;
    int count;
    while (ss >> cmd >> count) {
        if (cmd == 'S') {
            int nx = cx + dx[dir] * count;
            int ny = cy + dy[dir] * count;
            int mx = clamp(tx, min(cx, nx), max(cx, nx));
            int my = clamp(ty, min(cy, ny), max(cy, ny));
            calc(mx, my);
            cx = nx;
            cy = ny;
        } else {
            dir += count;
            dir %= 4;
        }
    }

    cout << rx << ' ' << ry;

    return 0;
}
