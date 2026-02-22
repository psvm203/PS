#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int n, f;
  cin >> n >> f;

  for (int i = 0; i < 100; i++) {
    int num = n / 100 * 100 + i;

    if (num % f == 0) {
      printf("%02d", i);
      return 0;
    }
  }

  return 1;
}
