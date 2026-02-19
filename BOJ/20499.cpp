#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int k, d, a;
  char dummy;
  cin >> k >> dummy >> d >> dummy >> a;

  auto check = [&]() { return k + a >= d && d; };

  string result = check() ? "gosu" : "hasu";
  cout << result;
}
