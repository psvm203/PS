#include <bits/stdc++.h>

using namespace std;

int counts[10000001];

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int n;
    cin >> n;

    for (int i = 1; i * i < n / 2; i++) {
        for (int j = i + 1; i * i + j * j <= n; j++) {
            counts[i * i + j * j]++;
        }
    }

    vector<int> goods;
    for (int i = 5; i <= n; i++) {
        if (counts[i] == 1) {
            goods.emplace_back(i);
        }
    }

    cout << goods.size() << '\n';
    for (int good: goods) {
        cout << good << ' ';
    }

    return 0;
}
