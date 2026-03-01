#include <bits/stdc++.h>
#define int int64_t

using namespace std;

signed main() {
  cin.tie(0)->sync_with_stdio(0);

  int n, m, t, b;
  cin >> n >> m >> t >> b;

  int zip = n + (n - 1) / 8 * b;
  int dok = m + t + (m - 1) / 8 * (b + t * 2);

  if (zip < dok) {
    cout << "Zip\n" << zip;
  } else {
    cout << "Dok\n" << dok;
  }

  return 0;
}
