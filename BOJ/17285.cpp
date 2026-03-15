#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  string t;
  cin >> t;

  int key = t[0] ^ 'C';

  for (char& ch: t) {
    ch ^= key;
  }

  cout << t;

  return 0;
}
