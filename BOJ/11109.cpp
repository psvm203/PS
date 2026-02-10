#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int t;
  cin >> t;

  while (t--) {
    int d, n, s, p;
    cin >> d >> n >> s >> p;
    auto solve = [&]() {
      int serial = n * s;
      int parallel = d + n * p;

      if (parallel < serial) {
        return "parallelize";
      }
      if (serial < parallel) {
        return "do not parallelize";
      }
      return "does not matter";
    };
    cout << solve() << '\n';
  }

  return 0;
}
