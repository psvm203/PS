#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int n;
    cin >> n;

    int result = (1 << n) - n * 2;
    cout << result;

    return 0;
}
