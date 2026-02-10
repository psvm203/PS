#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  string n;
  cin >> n;

  sort(n.begin(), n.end(), greater<>());
  cout << n;

  return 0;
}
