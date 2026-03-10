#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int n;
  cin >> n;

  vector<int> nums(n);
  for (int &num : nums) {
    cin >> num;
  }

  for (int i = 2; i < n; i++) {
    if (nums[i - 2] - nums[i - 1] != nums[i - 1] - nums[i]) {
      cout << "NO";
      return 0;
    }
  }

  cout << "YES\n";
  for (int num : nums) {
    cout << num * 2 << ' ';
  }
  cout << '\n';
  for (int num : nums) {
    cout << -num << ' ';
  }

  return 0;
}
