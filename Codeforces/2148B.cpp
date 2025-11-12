#include <iostream>

using namespace std;

int main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  int t;
  cin >> t;

  while (t--) {
    int n, m, x, y;
    cin >> n >> m >> x >> y;

    int dummy;
    for (int i = 0; i < n + m; i++) {
      cin >> dummy;
    }

    cout << n + m << '\n';
  }

  return 0;
}
