#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int a;
  cin >> a;

  int results[]{1, 2, 4, 5, 9, 14, 29};
  for (int n : results) {
    if (n > a) {
      break;
    }
    cout << n << '\n';
  }

  return 0;
}
