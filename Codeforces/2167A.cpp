#include <iostream>

using namespace std;

int main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  int t;
  cin >> t;

  while (t--) {
    int a, b, c, d;
    cin >> a >> b >> c >> d;

    string result = a == b && b == c && c == d ? "YES" : "NO";
    cout << result << '\n';
  }

  return 0;
}
