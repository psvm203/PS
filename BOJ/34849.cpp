#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(0)->sync_with_stdio(0);
    int n;
    cin >> n;
    string answer = n <= 1e4 ? "Accepted" : "Time limit exceeded";
    cout << answer;
    return 0;
}
