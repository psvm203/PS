#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  string letter;
  cin >> letter;

  unsigned char a = letter[0];
  unsigned char b = letter[1];
  unsigned char c = letter[2];

  int result = (((a & 0x0F) << 12) | ((b & 0x3F) << 6) | (c & 0x3F)) - 44031;
  cout << result;

  return 0;
}
