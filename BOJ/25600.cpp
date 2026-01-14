#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int n;
    cin >> n;

    int max_score = 0;
    while (n--) {
        int a, d, g;
        cin >> a >> d >> g;
        int score = a * (d + g) * ((a == d + g) + 1);
        max_score = max(max_score, score);
    }

    cout << max_score;

    return 0;
}
