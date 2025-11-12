#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

int main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  int t;
  cin >> t;

  while (t--) {
    int n, k;
    cin >> n >> k;

    vector<int> nums(n);
    for (int &num : nums) {
      cin >> num;
    }

    unordered_map<int, int> all_counts;
    for (int num : nums) {
      all_counts[num]++;
    }

    auto solve = [&]() {
      for (int num : nums) {
        if (all_counts[num] % k != 0) {
          return 0LL;
        }
      }

      int l = 0;
      int r = 0;
      long long result = 0;
      unordered_map<int, int> cur_counts;
      while (l <= r && r < n) {
        int num = nums[r];
        cur_counts[num]++;
        while (cur_counts[num] * k > all_counts[num]) {
          cur_counts[nums[l]]--;
          l++;
        }
        result += 1LL + r - l;
        r++;
      }

      return result;
    };

    cout << solve() << '\n';
  }

  return 0;
}
