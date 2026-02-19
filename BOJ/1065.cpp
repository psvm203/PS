#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int n;
  cin >> n;

  auto is_arithmetic = [&](vector<int> &digits) {
    if (digits.size() == 1)
      return true;
    int distance = digits[1] - digits[0];
    for (int i = 2; i < digits.size(); i++) {
      if (digits[i] - digits[i - 1] != distance) {
        return false;
      }
    }
    return true;
  };

  auto is_hansu = [&](int x) {
    vector<int> digits;
    while (x) {
      digits.emplace_back(x % 10);
      x /= 10;
    }
    return is_arithmetic(digits);
  };

  int result = 0;
  for (int i = 1; i <= n; i++) {
    result += is_hansu(i);
  }
  cout << result;

  return 0;
}
