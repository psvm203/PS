#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int a, t;
    cin >> a >> t;

    int result = max(0, 10 + 2 * (25 - a + t));
    cout << result;

    return 0;
}
