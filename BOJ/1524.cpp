#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(0)->sync_with_stdio(0);

    int t;
    cin >> t;

    while (t--) {
        int n, m;
        cin >> n >> m;
        int max_s = 0;
        int max_b = 0;
        while (n--) {
            int power;
            cin >> power;
            max_s = max(max_s, power);
        }
        while (m--) {
            int power;
            cin >> power;
            max_b = max(max_b, power);
        }
        char result = max_s >= max_b ? 'S':'B';
        cout << result << '\n';
    }

    return 0;
}
