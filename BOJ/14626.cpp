#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  string code;
  cin >> code;

  int sum = 0;
  int power = 0;
  for (int i = 0; i < 13; i++) {
    if (code[i] == '*') {
      power = i % 2 * 2 + 1;
      continue;
    }
    int digit = code[i] - '0';
    sum += digit * (i % 2 * 2 + 1);
  }

  int result = 0;
  if (power == 1) {
    result = (10 - sum % 10) % 10;
  } else {
    for (int i = 0; i < 10; i++) {
      if ((sum + i * 3) % 10 == 0) {
        result = i;
        break;
      }
    }
  }
  cout << result;

  return 0;
}
