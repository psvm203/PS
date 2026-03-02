#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  auto check = [&]() {
    string k;
    cin >> k;

    if (k.size() <= 2) {
      return true;
    }

    int dist = k[1] - k[0];
    for (int i = 2; i < k.size(); i++) {
      if (k[i] - k[i - 1] != dist) {
        return false;
      }
    }

    return true;
  };

  string result = check() ? "◝(⑅•ᴗ•⑅)◜..°♡ 뀌요미!!" : "흥칫뿡!! <(￣ ﹌ ￣)>";
  cout << result;

  return 0;
}
