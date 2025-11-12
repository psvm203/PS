#include <algorithm>
#include <iostream>
#include <numeric>
#include <vector>

using namespace std;

signed main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  int t;
  cin >> t;

  while (t--) {
    int n;
    cin >> n;

    vector<long long> evens;
    vector<long long> odds;
    evens.reserve(n);
    odds.reserve(n);
    while (n--) {
      long long dandelions;
      cin >> dandelions;
      if (dandelions % 2 == 0) {
        evens.emplace_back(dandelions);
      } else {
        odds.emplace_back(dandelions);
      }
    }

    auto solve = [&]() {
      if (odds.empty()) {
        return 0LL;
      }

      sort(odds.begin(), odds.end());

      long long count =
          accumulate(evens.begin(), evens.end(), 0LL) +
          accumulate(odds.begin() + odds.size() / 2, odds.end(), 0LL);
      return count;
    };

    cout << solve() << '\n';
  }

  return 0;
}
