#include <algorithm>
#include <iostream>

using namespace std;

int main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  int q;
  cin >> q;

  while (q--) {
    int n;
    string s, t;
    cin >> n >> s >> t;

    sort(s.begin(), s.end());
    sort(t.begin(), t.end());

    string result = s == t ? "YES" : "NO";
    cout << result << '\n';
  }

  return 0;
}
