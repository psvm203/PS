#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int a, b;
  string c;
  cin >> a >> b >> c;

  int result = a;
  if ((c.back() - '0') % 2 == 1) {
    result ^= b;
  }
  cout << result;

  return 0;
}
