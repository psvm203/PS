#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int n, m;
  cin >> n >> m;

  string result;

  for (int i = 0; i < n; i++) {
    if (result.size() >= m) {
      break;
    }
    result += to_string(n);
  }

  if (result.size() > m) {
    result = result.substr(0, m);
  }

  cout << result;

  return 0;
}
