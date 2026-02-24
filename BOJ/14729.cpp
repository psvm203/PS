#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(0)->sync_with_stdio(0);
    
    int n;
    cin >> n;
    
    vector<double> scores(n);
    for (auto& score: scores) {
        cin >> score;
    }
    
    sort(scores.begin(), scores.end());
    for (int i = 0; i < 7; i++) {
        printf("%.3f\n", scores[i]);
    }

    return 0;
}