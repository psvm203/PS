#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int n, m;
    cin >> n >> m;

    string answer;
    if (m <= 2) {
        answer = "NEWBIE!";
    } else if (m <= n) {
        answer = "OLDBIE!";
    } else {
        answer = "TLE!";
    }

    cout << answer;

    return 0;
}
