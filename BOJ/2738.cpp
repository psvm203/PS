#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int n, m;
    cin >> n >> m;
    vector matrix(n, vector<int>(m));
    for (auto &row: matrix) {
        for (int &num: row) {
            cin >> num;
        }
    }
    for (auto &row: matrix) {
        for (int &num: row) {
            int tmp = num;
            cin >> num;
            num += tmp;
        }
    }

    for (auto &row: matrix) {
        for (int num: row) {
            cout << num << ' ';
        }
        cout << '\n';
    }

    return 0;
}
