#include <iostream>

using namespace std;

int main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  int t;
  cin >> t;

  while (t--) {
    int x, n;
    cin >> x >> n;

    int result = n % 2 == 0 ? 0 : x;
    cout << result << '\n';
  }

  return 0;
}
