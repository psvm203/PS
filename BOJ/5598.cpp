#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  string word;
  cin >> word;

  string result;
  for (char ch : word) {
    int code = ch - 'A';
    char decryted = (code + 23) % 26 + 'A';
    result += decryted;
  }

  cout << result;

  return 0;
}
