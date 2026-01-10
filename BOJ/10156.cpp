#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int k, n, m;
    cin >> k >> n >> m;

    int result = max(0, k * n - m);
    cout << result;

    return 0;
}
