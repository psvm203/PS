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

    vector<int> nums(n);
    for (int &num : nums) {
      cin >> num;
    }

    int bits[30]{};
    for (int num : nums) {
      for (int i = 0; num; i++, num /= 2) {
        bits[i] += num & 1;
      }
    }

    long long max_sum = 0;
    for (int num : nums) {
      long long sum = 0;
      for (int i = 0; i < 30; i++) {
        if (num & 1) {
          sum += (1LL << i) * (n - bits[i]);
        } else {
          sum += (1LL << i) * bits[i];
        }
        num /= 2;
      }
      max_sum = max(max_sum, sum);
    }

    cout << max_sum << '\n';
  }

  return 0;
}
