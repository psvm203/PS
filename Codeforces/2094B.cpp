#include <iostream>

using namespace std;

int main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  int t;
  cin >> t;

  while (t--) {
    int n, m, l, r;
    cin >> n >> m >> l >> r;

    int res_l = r - m;
    int res_r = r;

    if (res_l > 0) {
      res_r -= res_l;
      res_l = 0;
    }

    cout << res_l << ' ' << res_r << '\n';
  }

  return 0;
}
