#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

int main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  int t;
  cin >> t;

  while (t--) {
    int n;
    cin >> n;
    vector<int> toys(n);
    for (int &toy : toys) {
      cin >> toy;
    }

    bool all_even = true;
    bool all_odd = true;
    for (int toy : toys) {
      all_even &= toy % 2 == 0;
      all_odd &= toy % 2 == 1;
    }

    if (!all_even && !all_odd) {
      sort(toys.begin(), toys.end());
    }

    for (int toy : toys) {
      cout << toy << ' ';
    }
    cout << '\n';
  }

  return 0;
}
