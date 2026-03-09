#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int r, b;
  cin >> r >> b;

  int n = r / 2 + 2;
  int l = (n + sqrt(n * n - 4 * (r + b))) / 2;
  int w = n - l;

  cout << l << ' ' << w;

  return 0;
}
