#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int n;
    cin >> n;

    int sum = 0;
    int mx = 0;
    while (n--) {
        int cost;
        cin >> cost;
        sum += cost;
        mx = max(mx, cost);
    }

    int result = sum - mx;
    cout << result;

    return 0;
}
