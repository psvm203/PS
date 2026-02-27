#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  string word;
  cin >> word;

  int sum = 0;
  for (char ch : word) {
    if (islower(ch)) {
      sum += ch - 'a' + 1;
    } else {
      sum += ch - 'A' + 27;
    }
  }

  bool is_prime = true;
  for (int i = 2; i < sum; i++) {
    if (sum % i == 0) {
      is_prime = false;
      break;
    }
  }

  string result = is_prime ? "It is a prime word." : "It is not a prime word.";
  cout << result;

  return 0;
}
