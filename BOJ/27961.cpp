#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int64_t n;
    cin >> n;

    int result = n == 0 ? 0 : log2(n * 4 - 1);
    cout << result;

    return 0;
}
