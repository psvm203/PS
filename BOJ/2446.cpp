#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int n;
  cin >> n;

  auto spaces = [&](int x) {
    string result(x, ' ');
    cout << result;
  };

  auto stars = [&](int x) {
    string result(x, '*');
    cout << result << '\n';
  };

  for (int i = n - 1; i >= 0; i--) {
    spaces(n - i - 1);
    stars(i * 2 + 1);
  }

  for (int i = 1; i < n; i++) {
    spaces(n - i - 1);
    stars(i * 2 + 1);
  }

  return 0;
}
