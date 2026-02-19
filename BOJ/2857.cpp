#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  bool none = true;
  for (int i = 0; i < 5; i++) {
    string name;
    cin >> name;
    if (name.find("FBI") != string::npos) {
      none = false;
      cout << i + 1 << ' ';
    }
  }

  if (none) {
    cout << "HE GOT AWAY!";
  }

  return 0;
}
