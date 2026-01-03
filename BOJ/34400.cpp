#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int t;
    cin >> t;

    while (t--) {
        int time;
        cin >> time;
        time %= 25;
        string state = time < 17 ? "ONLINE" : "OFFLINE";
        cout << state << '\n';
    }
    return 0;
}
