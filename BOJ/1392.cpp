#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int n, q;
    cin >> n >> q;

    int indices[10001];
    int cur_time = 0;
    for (int i = 1; i <= n; i++) {
        int time;
        cin >> time;
        for (int j = cur_time; j < cur_time + time; j++) {
            indices[j] = i;
        }
        cur_time += time;
    }
    for (int i = 0; i < q; i++) {
        int query;
        cin >> query;
        cout << indices[query] << '\n';
    }

    return 0;
}
