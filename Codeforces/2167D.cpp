#include <iostream>
#include <vector>

using namespace std;

int main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  const int MAX = 54;
  vector<bool> is_prime(MAX, true);
  for (int i = 2; i * i < MAX; i++) {
    if (!is_prime[i]) {
      continue;
    }
    for (int j = i * i; j < MAX; j += i) {
      is_prime[j] = false;
    }
  }

  vector<int> primes;
  for (int i = 2; i < MAX; i++) {
    if (is_prime[i]) {
      primes.emplace_back(i);
    }
  }

  int t;
  cin >> t;

  while (t--) {
    int n;
    cin >> n;
    vector<long long> nums(n);
    for (long long &num : nums) {
      cin >> num;
    }

    auto solve = [&]() {
      for (int prime : primes) {
        for (long long num : nums) {
          if (num % prime != 0) {
            return prime;
          }
        }
      }

      return -1;
    };

    cout << solve() << '\n';
  }

  return 0;
}
