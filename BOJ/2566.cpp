#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int mx = -1;
  int r, c;
  for (int y = 0; y < 9; y++) {
    for (int x = 0; x < 9; x++) {
      int num;
      cin >> num;
      if (num > mx) {
        mx = num;
        r = y;
        c = x;
      }
    }
  }
  cout << mx << '\n' << r + 1 << ' ' << c + 1;

  return 0;
}
