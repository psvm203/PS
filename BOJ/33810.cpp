#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  string base = "SciComLove";
  string s;
  cin >> s;

  int result = 0;
  for (int i = 0; i < 10; i++) {
    result += s[i] != base[i];
  }
  cout << result;

  return 0;
}
