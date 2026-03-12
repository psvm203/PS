#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int winner = -1;
  int max_score = -1;
  for (int i = 0; i < 5; i++) {
    int sum = 0;
    for (int j = 0; j < 4; j++) {
      int score;
      cin >> score;
      sum += score;
    }
    if (sum > max_score) {
      max_score = sum;
      winner = i + 1;
    }
  }

  cout << winner << ' ' << max_score;

  return 0;
}
