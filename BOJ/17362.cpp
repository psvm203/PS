#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int n;
    cin >> n;

    int result = (n - 1) % 8;
    if (result > 4) {
        result = 8 - result;
    }
    result++;
    cout << result;

    return 0;
}
