#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int t;
  cin >> t;

  while (t--) {
    int n;
    cin >> n;

    int result = 0;
    while (n > 2) {
      result += n % 2 == 1;
      n += n % 2 == 1;
      n /= 2;
    }
    cout << result << '\n';
  }

  return 0;
}
