#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int n;
  cin >> n;

  string result = string(n, '*') + '\n';

  for (int r = 1; r < n - 1; r++) {
    result += '*';
    for (int c = 1; c < n - 1; c++) {
      int left = r;
      int right = n - r - 1;
      result += c == left || c == right ? '*' : ' ';
    }
    result += "*\n";
  }

  if (n != 1)
    result += string(n, '*');

  cout << result;

  return 0;
}
